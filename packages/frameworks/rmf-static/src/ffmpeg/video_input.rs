
use anyhow::anyhow;
use rmf_core::{
    Error, InputSource, Result, Timestamp,
    video::{VideoContentCursor, VideoInput},
};
use rmf_macros::delegate_implements;
use rsmpeg::ffi::{AVMEDIA_TYPE_VIDEO, av_q2d};

use crate::{
    Image,
    ffmpeg::{AVFormatVideoContentCursor, utils::make_input},
};

#[derive(Clone)]
pub struct AVFormatVideoInput {
    source: InputSource,
    duration: Timestamp,
    video_index: usize,
    fps: f64,
}

impl AVFormatVideoInput {
    pub fn try_new(source: InputSource) -> Result<AVFormatVideoInput> {
        let input = make_input(&source)?;
        let (video_index, _) = input
            .find_best_stream(AVMEDIA_TYPE_VIDEO)
            .map_err(|e| Error::new_input(e.into()))?
            .ok_or_else(|| Error::new_input(anyhow!("not found video stream.")))?;
        let video_stream = &input.streams()[video_index];
        let fps = av_q2d(video_stream.r_frame_rate);

        Ok(AVFormatVideoInput {
            source,
            video_index,
            fps,
            duration: Timestamp::from_microseconds(input.duration),
        })
    }
}

#[delegate_implements]
impl VideoInput for AVFormatVideoInput {
    type Item = Image;
    type ContentCursor = AVFormatVideoContentCursor;
    fn fps(&self) -> f64 {
        self.fps
    }
    fn cursor(&self) -> Result<AVFormatVideoContentCursor> {
        let input = make_input(&self.source)?;
        AVFormatVideoContentCursor::try_new(input, self.fps)
    }
    fn duration(&self) -> Timestamp {
        self.duration
    }
}
