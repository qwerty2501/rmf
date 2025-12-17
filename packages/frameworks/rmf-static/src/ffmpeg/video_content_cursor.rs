use std::{collections::VecDeque, slice};

use anyhow::anyhow;
use rmf_core::{Content, Error, Result, Size, Timestamp, video::VideoContentCursor};
use rmf_macros::delegate_implements;
use rsmpeg::{
    avformat::AVFormatContextInput,
    avutil::{AVFrame, AVMem},
    error::RsmpegError,
    ffi::{AV_PIX_FMT_RGBA, AVMEDIA_TYPE_VIDEO, SWS_BICUBIC, av_image_get_buffer_size},
    swscale::SwsContext,
};

use crate::{
    Image,
    ffmpeg::utils::{AVFormatContentContexts, input_contexts, seek_input, to_timestamp},
};

pub struct AVFormatVideoContentCursor {
    input: AVFormatContextInput,
    video_context: AVFormatContentContexts,
    scale_context: Option<ScaleContext>,
    video_cache: VecDeque<Content<Image>>,
    fps: f64,
}
struct ScaleContext {
    sws_context: SwsContext,
    frame_rgba: AVFrame,
}

impl AVFormatVideoContentCursor {
    pub fn try_new(input: AVFormatContextInput, fps: f64) -> Result<Self> {
        let video_context = input_contexts(&input, AVMEDIA_TYPE_VIDEO)?
            .ok_or_else(|| Error::new_input(anyhow!("Can not make input context")))?;

        let scale_context = if video_context.avcodec_context.pix_fmt == AV_PIX_FMT_RGBA {
            None
        } else {
            let sws_context = SwsContext::get_context(
                video_context.avcodec_context.width,
                video_context.avcodec_context.height,
                video_context.avcodec_context.pix_fmt,
                video_context.avcodec_context.width,
                video_context.avcodec_context.height,
                AV_PIX_FMT_RGBA,
                SWS_BICUBIC,
                None,
                None,
                None,
            )
            .ok_or_else(|| Error::new_video(anyhow!("Can't get sws context")))?;
            let mut frame_rgba = AVFrame::default();
            frame_rgba.set_format(AV_PIX_FMT_RGBA);
            frame_rgba.set_width(video_context.avcodec_context.width);
            frame_rgba.set_height(video_context.avcodec_context.height);
            frame_rgba.get_buffer(0).unwrap();

            Some(ScaleContext {
                sws_context,
                frame_rgba,
            })
        };

        Ok(Self {
            input,
            video_context,
            scale_context,
            video_cache: VecDeque::default(),
            fps,
        })
    }
    fn avframe_to_image(frame: &AVFrame) -> Result<Image> {
        let width = frame.width as usize;
        let height = frame.height as usize;

        // 期待される全サイズ: width * height * 4
        let total_size = width * height * 4;
        let data = unsafe { slice::from_raw_parts(frame.data[0], total_size) };
        Image::new_size(
            Size::new(frame.height as _, frame.width as _),
            data.to_vec(),
        )
    }
}

#[delegate_implements]
impl VideoContentCursor for AVFormatVideoContentCursor {
    type Item = Image;
    #[inline]
    fn fps(&self) -> f64 {
        self.fps
    }
    fn read(&mut self) -> Result<Option<rmf_core::Content<Image>>> {
        if let Some(content) = self.video_cache.pop_front() {
            Ok(Some(content))
        } else {
            while let Some(packet) = self
                .input
                .read_packet()
                .map_err(|e| Error::new_video(e.into()))?
            {
                if packet.stream_index == self.video_context.index as _ {
                    self.video_context
                        .avcodec_context
                        .send_packet(Some(&packet))
                        .map_err(|e| Error::new_video(e.into()))?;
                    loop {
                        match self.video_context.avcodec_context.receive_frame() {
                            Ok(frame) => {
                                let presentation_timestamp =
                                    to_timestamp(frame.pts, self.video_context.time_base);
                                let duration_timestamp =
                                    to_timestamp(frame.duration, self.video_context.time_base);
                                let frame = if let Some(scale_context) = &mut self.scale_context {
                                    scale_context
                                        .sws_context
                                        .scale_frame(
                                            &frame,
                                            0,
                                            frame.height,
                                            &mut scale_context.frame_rgba,
                                        )
                                        .map_err(|e| Error::new_video(e.into()))?;
                                    &scale_context.frame_rgba
                                } else {
                                    &frame
                                };

                                let image = Self::avframe_to_image(frame)?;
                                self.video_cache.push_back(Content::new(
                                    image,
                                    presentation_timestamp,
                                    duration_timestamp,
                                ));
                            }
                            Err(err) => {
                                if err == RsmpegError::DecoderFlushedError
                                    || err == RsmpegError::DecoderDrainError
                                {
                                    break;
                                } else {
                                    Err(Error::new_video(err.into()))?
                                }
                            }
                        }
                    }
                    break;
                }
            }
            Ok(self.video_cache.pop_front())
        }
    }
    #[inline]
    fn seek(&mut self, timestamp: Timestamp) -> Result<()> {
        seek_input(&mut self.input, timestamp)
    }
}
