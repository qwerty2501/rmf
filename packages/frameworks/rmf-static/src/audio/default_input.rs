use crate::core::Timestamp;
use rmf_macros::delegate_implements;

use crate::{
    Audio,
    ffmpeg::{AVFormatAudioContentCursor, AVFormatAudioInput},
};

pub struct DefaultAudioContentCursor(AVFormatAudioContentCursor);

#[delegate_implements]
impl rmf_core::audio::AudioContentCursor for DefaultAudioContentCursor {
    type Item = Audio;
    #[inline]
    fn offset(&self) -> Timestamp {
        self.0.offset()
    }
    #[inline]
    fn read(&mut self) -> rmf_core::Result<Option<rmf_core::Content<Audio>>> {
        self.0.read()
    }
    #[inline]
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> rmf_core::Result<()> {
        self.0.seek(timestamp)
    }
}

#[derive(Clone)]
pub struct DefaultAudioInput(AVFormatAudioInput);

#[delegate_implements]
impl rmf_core::audio::AudioInput for DefaultAudioInput {
    type Item = Audio;
    type ContentCursor = DefaultAudioContentCursor;
    #[inline]
    fn duration(&self) -> Timestamp {
        self.0.duration()
    }
    #[inline]
    fn sample_rate(&self) -> u32 {
        self.0.sample_rate()
    }

    #[inline]
    fn cursor(&self) -> rmf_core::Result<DefaultAudioContentCursor> {
        Ok(DefaultAudioContentCursor(self.0.cursor()?))
    }
}

pub struct DefaultAudioInputProvider;

impl DefaultAudioInputProvider {
    pub fn provide(source: rmf_core::InputSource) -> crate::core::Result<DefaultAudioInput> {
        Ok(DefaultAudioInput(AVFormatAudioInput::try_new(source)?))
    }
}
