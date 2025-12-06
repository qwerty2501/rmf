use std::collections::VecDeque;

use anyhow::anyhow;
use rmf_core::{Content, Error, Result, Timestamp};
use rmf_macros::delegate_implements;
use rsmpeg::{
    avcodec::AVCodecContext,
    avformat::AVFormatContextInput,
    avutil::{AVFrame, AVMem, av_rescale_q},
    error::RsmpegError,
    ffi::{
        self, AV_PIX_FMT_BGR24, AV_TIME_BASE_Q, AVMEDIA_TYPE_AUDIO, AVMEDIA_TYPE_VIDEO,
        AVPixelFormat, AVRational, AVSEEK_FLAG_BACKWARD, SWS_BICUBIC, av_image_get_buffer_size,
    },
    swscale::SwsContext,
};

use crate::{Audio, AudioDataContextBuilder, Image};

pub struct AVFormatImageContentCursor {
    input: AVFormatContextInput,
    video_context: AVFormatContentContexts,
    image_scale_context: Option<ImageScaleContext>,
    image_cache: VecDeque<Content<Image>>,
}

pub struct AVFormatAudioContentCursor {
    input: AVFormatContextInput,
    audio_context: AVFormatContentContexts,
    audio_cache: VecDeque<Content<Audio>>,
}

struct AVFormatContentContexts {
    avcodec_context: AVCodecContext,
    index: usize,
    ptr_per: f64,
}

struct ImageScaleContext {
    sws_context: SwsContext,
    frame_cache: AVFrame,
    buffer: AVMem,
}

const DEFAULT_PIX_FMT: AVPixelFormat = AV_PIX_FMT_BGR24;

impl AVFormatImageContentCursor {
    pub fn try_new(input: AVFormatContextInput) -> Result<Self> {
        let video_context = input_contexts(&input, AVMEDIA_TYPE_VIDEO)?
            .ok_or_else(|| Error::new_image(anyhow!("Can not make input context").into()))?;
        let image_scale_context = if video_context.avcodec_context.pix_fmt != DEFAULT_PIX_FMT {
            let sws_context = SwsContext::get_context(
                video_context.avcodec_context.width,
                video_context.avcodec_context.height,
                video_context.avcodec_context.pix_fmt,
                video_context.avcodec_context.width,
                video_context.avcodec_context.height,
                AV_PIX_FMT_BGR24,
                SWS_BICUBIC,
                None,
                None,
                None,
            )
            .ok_or_else(|| Error::new_image(anyhow!("Failed get sws context.").into()))?;
            let mut frame_cache = AVFrame::default();
            let mut buffer = AVMem::new(unsafe {
                av_image_get_buffer_size(
                    DEFAULT_PIX_FMT,
                    video_context.avcodec_context.width,
                    video_context.avcodec_context.height,
                    1,
                )
            } as _);
            unsafe {
                frame_cache.fill_arrays(
                    buffer.as_mut_ptr(),
                    DEFAULT_PIX_FMT,
                    video_context.avcodec_context.width,
                    video_context.avcodec_context.height,
                )
            }
            .map_err(|e| Error::new_image(e.into()))?;
            Some(ImageScaleContext {
                sws_context,
                buffer,
                frame_cache,
            })
        } else {
            None
        };
        Ok(Self {
            input,
            video_context,
            image_scale_context,
            image_cache: VecDeque::default(),
        })
    }

    fn avframe_to_image(
        frame: AVFrame,
        avcodec_context: &AVCodecContext,
        image_scale_context: &mut Option<ImageScaleContext>,
    ) -> Result<Image> {
        let avframe = if let Some(image_scale_context) = image_scale_context {
            image_scale_context
                .sws_context
                .scale_frame(
                    &frame,
                    0,
                    avcodec_context.height,
                    &mut image_scale_context.frame_cache,
                )
                .map_err(|e| Error::new_image(e.into()))?;
            &image_scale_context.frame_cache
        } else {
            &frame
        };
        let data = unsafe { std::slice::from_raw_parts(avframe.data[0], avframe.linesize[0] as _) };
        Image::new_size(
            rmf_core::Size {
                height: avcodec_context.height as _,
                width: avcodec_context.width as _,
            },
            data,
        )
    }
}

impl AVFormatAudioContentCursor {
    pub fn try_new(input: AVFormatContextInput) -> Result<Self> {
        let audio_context = input_contexts(&input, AVMEDIA_TYPE_AUDIO)?
            .ok_or_else(|| Error::new_audio(anyhow!("Can not make input context").into()))?;
        Ok(Self {
            input,
            audio_context,
            audio_cache: VecDeque::default(),
        })
    }
    fn avframe_to_audio(frame: AVFrame) -> Result<Audio> {
        let data_context = AudioDataContextBuilder::try_new(frame)?;
        Audio::tyr_new(data_context)
    }
}

impl rmf_core::Service for AVFormatImageContentCursor {}

#[delegate_implements]
impl rmf_core::image::ImageContentCursor for AVFormatImageContentCursor {
    type Item = Image;
    fn read(&mut self) -> Result<Option<Content<Image>>> {
        if let Some(content) = self.image_cache.pop_front() {
            Ok(Some(content))
        } else {
            loop {
                if let Some(packet) = self
                    .input
                    .read_packet()
                    .map_err(|e| Error::new_image(e.into()))?
                {
                    if packet.stream_index == self.video_context.index as _ {
                        self.video_context
                            .avcodec_context
                            .send_packet(Some(&packet))
                            .map_err(|e| Error::new_image(e.into()))?;

                        loop {
                            match self.video_context.avcodec_context.receive_frame() {
                                Ok(frame) => {
                                    let presentation_timestamp =
                                        timestamp_to_duration(frame.pts, frame.time_base);
                                    let duration_timestamp =
                                        timestamp_to_duration(frame.duration, frame.time_base);
                                    let image = Self::avframe_to_image(
                                        frame,
                                        &self.video_context.avcodec_context,
                                        &mut self.image_scale_context,
                                    )?;
                                    self.image_cache.push_back(Content::new(
                                        image,
                                        presentation_timestamp,
                                        duration_timestamp,
                                    ));
                                }
                                Err(err) => {
                                    if err == RsmpegError::DecoderFlushedError
                                        || err == RsmpegError::DecoderFlushedError
                                    {
                                        break;
                                    } else {
                                        Err(Error::new_image(err.into()))?
                                    }
                                }
                            }
                        }
                        break;
                    }
                } else {
                    break;
                }
            }
            Ok(self.image_cache.pop_front())
        }
    }

    #[inline]
    fn seek(&mut self, timestamp: Timestamp) -> rmf_core::Result<()> {
        seek_input(&mut self.input, timestamp)
    }
}

#[delegate_implements]
impl rmf_core::audio::AudioContentCursor for AVFormatAudioContentCursor {
    type Item = Audio;
    fn read(&mut self) -> Result<Option<Content<Audio>>> {
        if let Some(content) = self.audio_cache.pop_front() {
            Ok(Some(content))
        } else {
            loop {
                if let Some(packet) = self
                    .input
                    .read_packet()
                    .map_err(|e| Error::new_image(e.into()))?
                {
                    if packet.stream_index == self.audio_context.index as _ {
                        self.audio_context
                            .avcodec_context
                            .send_packet(Some(&packet))
                            .map_err(|e| Error::new_image(e.into()))?;
                        loop {
                            match self.audio_context.avcodec_context.receive_frame() {
                                Ok(frame) => {
                                    let presentation_timestamp =
                                        timestamp_to_duration(frame.pts, frame.time_base);
                                    let duration_timestamp =
                                        timestamp_to_duration(frame.duration, frame.time_base);
                                    let audio = Self::avframe_to_audio(frame)?;
                                    self.audio_cache.push_back(Content::new(
                                        audio,
                                        presentation_timestamp,
                                        duration_timestamp,
                                    ));
                                }
                                Err(err) => {
                                    if err == RsmpegError::DecoderFlushedError
                                        || err == RsmpegError::DecoderFlushedError
                                    {
                                        break;
                                    } else {
                                        Err(Error::new_image(err.into()))?
                                    }
                                }
                            }
                        }
                    }
                    break;
                } else {
                    break;
                }
            }
            Ok(self.audio_cache.pop_front())
        }
    }
    #[inline]
    fn seek(&mut self, timestamp: Timestamp) -> Result<()> {
        seek_input(&mut self.input, timestamp)
    }
}

#[inline]
fn seek_input(input: &mut AVFormatContextInput, timestamp: Timestamp) -> Result<()> {
    input
        .seek(-1, timestamp.micro_seconds(), AVSEEK_FLAG_BACKWARD as _)
        .map_err(|e| Error::new_image(e.into()))
}

#[inline]
fn input_contexts(
    input: &AVFormatContextInput,
    media_type: ffi::AVMediaType,
) -> Result<Option<AVFormatContentContexts>> {
    if let Some((index, avcodec)) = input
        .find_best_stream(media_type)
        .map_err(|e| Error::new_image(e.into()))?
    {
        let ptr_per = {
            let stream = &input.streams()[index];
            stream.time_base.num as f64 / stream.time_base.den as f64
        };
        let avcodec_context = AVCodecContext::new(&avcodec);
        Ok(Some(AVFormatContentContexts {
            avcodec_context,
            index,
            ptr_per,
        }))
    } else {
        Ok(None)
    }
}

#[inline]
fn timestamp_to_duration(ts: i64, time_base: AVRational) -> Timestamp {
    Timestamp::from_micro_seconds(av_rescale_q(ts, time_base, AV_TIME_BASE_Q))
}
