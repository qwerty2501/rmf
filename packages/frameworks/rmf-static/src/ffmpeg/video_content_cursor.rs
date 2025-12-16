use std::{collections::VecDeque, slice};

use anyhow::anyhow;
use rmf_core::{Content, Error, Result, Size, Timestamp, video::VideoContentCursor};
use rmf_macros::delegate_implements;
use rsmpeg::{
    avformat::AVFormatContextInput, avutil::AVFrame, error::RsmpegError, ffi::AVMEDIA_TYPE_VIDEO,
};

use crate::{
    Image,
    ffmpeg::utils::{AVFormatContentContexts, input_contexts, seek_input, to_timestamp},
};

pub struct AVFormatVideoContentCursor {
    input: AVFormatContextInput,
    video_context: AVFormatContentContexts,
    video_cache: VecDeque<Content<Image>>,
    fps: f64,
}

impl AVFormatVideoContentCursor {
    pub fn try_new(input: AVFormatContextInput, fps: f64) -> Result<Self> {
        let video_context = input_contexts(&input, AVMEDIA_TYPE_VIDEO)?
            .ok_or_else(|| Error::new_input(anyhow!("Can not make input context")))?;
        Ok(Self {
            input,
            video_context,
            video_cache: VecDeque::default(),
            fps,
        })
    }
    fn avframe_to_image(frame: AVFrame) -> Result<Image> {
        let data = unsafe { slice::from_raw_parts(frame.data[0], frame.linesize[0] as _) };
        Image::new_size(Size::new(frame.width as _, frame.height as _), data)
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
                                    to_timestamp(frame.pts, frame.time_base);
                                let duration_timestamp =
                                    to_timestamp(frame.duration, frame.time_base);
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
