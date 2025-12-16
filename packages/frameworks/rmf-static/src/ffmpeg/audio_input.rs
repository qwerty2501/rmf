use rmf_core::{InputSource, Result, audio::AudioInput};
use rmf_macros::delegate_implements;

use crate::{
    Audio,
    ffmpeg::{AVFormatAudioContentCursor, utils::make_input},
};

#[derive(Clone)]
pub struct AVFormatAudioInput {
    source: InputSource,
}

#[delegate_implements]
impl AudioInput for AVFormatAudioInput {
    type Item = Audio;
    type ContentCursor = AVFormatAudioContentCursor;
    #[inline]
    fn cursor(&self) -> Result<AVFormatAudioContentCursor> {
        AVFormatAudioContentCursor::try_new(make_input(&self.source)?)
    }
}

impl AVFormatAudioInput {
    pub fn new(source: InputSource) -> Self {
        Self { source }
    }
}
