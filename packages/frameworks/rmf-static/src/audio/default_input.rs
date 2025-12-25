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
    fn read(&mut self) -> rmf_core::Result<Option<rmf_core::Content<Audio>>> {
        self.0.read()
    }
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
    fn duration(&self) -> Timestamp {
        self.0.duration()
    }
    fn sample_rate(&self) -> u32 {
        self.0.sample_rate()
    }

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
