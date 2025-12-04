use std::{collections::VecDeque, time::Duration};

use anyhow::anyhow;
use rmf_core::{ContentConstructor, ContentSeekFlag, Error, Result, Timestamp};
use rsmpeg::{
    avcodec::{AVCodecContext, AVCodecRef},
    avformat::AVFormatContextInput,
    avutil::{AVFrame, AVMem, av_rescale_q},
    error::RsmpegError,
    ffi::{
        self, AV_PIX_FMT_BGR24, AV_TIME_BASE_Q, AVMEDIA_TYPE_AUDIO, AVMEDIA_TYPE_VIDEO,
        AVPixelFormat, AVRational, AVSEEK_FLAG_BACKWARD, SWS_BICUBIC, av_image_get_buffer_size,
    },
    swscale::SwsContext,
};

use crate::{Audio, AudioDataContextBuilder, Content, ContextContent, Image};

pub struct AVFormatContextContentCursor {
    input: AVFormatContextInput,
    video_context: AVFormatContentContexts,
    audio_context: AVFormatContentContexts,
    image_scale_context: Option<ImageScaleContext>,
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

impl AVFormatContextContentCursor {
    pub fn try_new(input: AVFormatContextInput) -> Result<Self> {
        let video_context = Self::input_contexts(&input, AVMEDIA_TYPE_VIDEO)?
            .ok_or_else(|| Error::new_image(anyhow!("Can not make input context").into()))?;
        let audio_context = Self::input_contexts(&input, AVMEDIA_TYPE_AUDIO)?
            .ok_or_else(|| Error::new_audio(anyhow!("Can not make input context").into()))?;
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
            audio_context,
            image_scale_context,
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
    fn avframe_to_audio(frame: AVFrame) -> Result<Audio> {
        let data_context = AudioDataContextBuilder::try_new(frame)?;
        Audio::tyr_new(data_context)
    }
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
}

fn timestamp_to_duration(ts: i64, time_base: AVRational) -> Timestamp {
    Timestamp::from_micro_seconds(av_rescale_q(ts, time_base, AV_TIME_BASE_Q))
}

impl rmf_core::ContentCursor for AVFormatContextContentCursor {
    type Content = Content<ContextContent>;
    fn read(&mut self) -> Result<Option<Self::Content>> {
        let video_context = &mut self.video_context;
        let audio_context = &mut self.audio_context;
        loop {
            if let Some(packet) = self
                .input
                .read_packet()
                .map_err(|e| Error::new_image(e.into()))?
            {
                if packet.stream_index == video_context.index as _ {
                    video_context
                        .avcodec_context
                        .send_packet(Some(&packet))
                        .map_err(|e| Error::new_image(e.into()))?;

                    loop {
                        match video_context.avcodec_context.receive_frame() {
                            Ok(frame) => {
                                let presentation_timestamp =
                                    timestamp_to_duration(frame.pts, frame.time_base);
                                let duration_timestamp =
                                    timestamp_to_duration(frame.duration, frame.time_base);
                                let image = Self::avframe_to_image(
                                    frame,
                                    &video_context.avcodec_context,
                                    &mut self.image_scale_context,
                                )?;
                                return Ok(Some(Content::<ContextContent>::new(
                                    ContextContent::new_image(image),
                                    presentation_timestamp,
                                    duration_timestamp,
                                )));
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
                if packet.stream_index == audio_context.index as _ {
                    audio_context
                        .avcodec_context
                        .send_packet(Some(&packet))
                        .map_err(|e| Error::new_image(e.into()))?;
                    loop {
                        match audio_context.avcodec_context.receive_frame() {
                            Ok(frame) => {
                                let presentation_timestamp =
                                    timestamp_to_duration(frame.pts, frame.time_base);
                                let duration_timestamp =
                                    timestamp_to_duration(frame.duration, frame.time_base);
                                let audio = Self::avframe_to_audio(frame)?;
                                return Ok(Some(Content::<ContextContent>::new(
                                    ContextContent::new_audio(audio),
                                    presentation_timestamp,
                                    duration_timestamp,
                                )));
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
            } else {
                break;
            }
        }
        Ok(None)
    }

    #[inline]
    fn seek(
        &mut self,
        timestamp: Timestamp,
        flag: Option<ContentSeekFlag>,
    ) -> rmf_core::Result<()> {
        self.input
            .seek(
                -1,
                timestamp.micro_seconds(),
                flag.map(|f| match f {
                    ContentSeekFlag::Backword => AVSEEK_FLAG_BACKWARD,
                })
                .unwrap_or(0) as i32,
            )
            .map_err(|e| Error::new_image(e.into()))
    }
}
