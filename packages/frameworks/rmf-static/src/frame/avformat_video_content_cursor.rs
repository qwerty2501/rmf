use std::collections::VecDeque;

use anyhow::anyhow;
use rmf_core::{ContentSeekFlag, Error, Result};
use rsmpeg::{
    avcodec::{AVCodecContext, AVCodecRef},
    avformat::AVFormatContextInput,
    avutil::{AVFrame, AVMem},
    error::RsmpegError,
    ffi::{
        self, AV_PIX_FMT_BGR24, AVMEDIA_TYPE_AUDIO, AVMEDIA_TYPE_VIDEO, AVPixelFormat,
        AVSEEK_FLAG_BACKWARD, SWS_BICUBIC, av_image_get_buffer_size,
    },
    swscale::SwsContext,
};

use crate::{Audio, Content, Frame, Image};

pub struct AVFormatContentCursor {
    input: AVFormatContextInput,
    video_context: Option<AVFormatContentContexts>,
    audio_context: Option<AVFormatContentContexts>,
    image_scale_context: Option<ImageScaleContext>,
    image_cache: VecDeque<Image>,
    audio_cache: VecDeque<Audio>,
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

impl AVFormatContentCursor {
    pub fn try_new(input: AVFormatContextInput) -> Result<Self> {
        let video_context = Self::input_contexts(&input, AVMEDIA_TYPE_VIDEO)?;
        let audio_context = Self::input_contexts(&input, AVMEDIA_TYPE_AUDIO)?;
        let image_scale_context = if let Some(video_context) = &video_context
            && video_context.avcodec_context.pix_fmt != DEFAULT_PIX_FMT
        {
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
            image_cache: VecDeque::default(),
            audio_cache: VecDeque::default(),
        })
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

impl rmf_core::ContentCursor for AVFormatContentCursor {
    type Content = Content<Frame>;
    fn read(&mut self) -> Result<Option<Self::Content>> {
        loop {
            if let Some(packet) = self
                .input
                .read_packet()
                .map_err(|e| Error::new_image(e.into()))?
            {
                if let Some(video_context) = &mut self.video_context
                    && packet.stream_index == video_context.index as _
                {
                    video_context
                        .avcodec_context
                        .send_packet(Some(&packet))
                        .map_err(|e| Error::new_image(e.into()))?;

                    loop {
                        match video_context.avcodec_context.receive_frame() {
                            Ok(frame) => {
                                let avframe = if let Some(image_scale_context) =
                                    &mut self.image_scale_context
                                {
                                    image_scale_context
                                        .sws_context
                                        .scale_frame(
                                            &frame,
                                            0,
                                            video_context.avcodec_context.height,
                                            &mut image_scale_context.frame_cache,
                                        )
                                        .map_err(|e| Error::new_image(e.into()))?;
                                    &image_scale_context.frame_cache
                                } else {
                                    &frame
                                };
                                let data = unsafe {
                                    std::slice::from_raw_parts(
                                        avframe.data[0],
                                        avframe.linesize[0] as _,
                                    )
                                };
                                let image = Image::new_size(
                                    rmf_core::Size {
                                        height: video_context.avcodec_context.height as _,
                                        width: video_context.avcodec_context.width as _,
                                    },
                                    data,
                                )?;
                                self.image_cache.push_back(image);
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
                if let Some(audio_context) = &self.audio_context
                    && packet.stream_index == audio_context.index as _
                {}
            }
        }
    }
    fn seek(&mut self, timestamp: i64, flag: Option<ContentSeekFlag>) -> rmf_core::Result<()> {
        self.input
            .seek(
                -1,
                timestamp,
                flag.map(|f| match f {
                    ContentSeekFlag::Backword => AVSEEK_FLAG_BACKWARD,
                })
                .unwrap_or(0) as i32,
            )
            .map_err(|e| Error::new_image(e.into()))
    }
}
