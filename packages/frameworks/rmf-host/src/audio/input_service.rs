use rmf_core::{
    InputSource,
    audio::{AudioContentCursor, AudioInput},
};
use rmf_static::{Audio, DefaultAudioContentCursor, DefaultAudioInput, DefaultAudioInputProvider};

use crate::service::{
    AudioContentStreamServiceTrait, ContentCursorTrait, ContentStreamServiceTrait, ServiceTrait,
};

#[derive(Clone)]
enum ContextAudioInput {
    Default(DefaultAudioInput),
}

enum ContextAudioContextCursor {
    Default(DefaultAudioContentCursor),
}

impl rmf_core::audio::AudioContentCursor for ContextAudioContextCursor {
    type Item = Audio;
    #[inline]
    fn offset(&self) -> rmf_core::Timestamp {
        match self {
            Self::Default(d) => d.offset(),
        }
    }
    #[inline]
    fn read(&mut self) -> rmf_core::Result<Option<rmf_core::Content<Self::Item>>> {
        match self {
            Self::Default(d) => d.read(),
        }
    }
    #[inline]
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> rmf_core::Result<()> {
        match self {
            Self::Default(d) => d.seek(timestamp),
        }
    }
}

impl rmf_core::audio::AudioInput for ContextAudioInput {
    type ContentCursor = ContextAudioContextCursor;
    type Item = Audio;

    #[inline]
    fn cursor(&self) -> rmf_core::Result<Self::ContentCursor> {
        match self {
            Self::Default(d) => Ok(ContextAudioContextCursor::Default(d.cursor()?)),
        }
    }

    #[inline]
    fn duration(&self) -> rmf_core::Timestamp {
        match self {
            Self::Default(d) => d.duration(),
        }
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        match self {
            Self::Default(d) => d.sample_rate(),
        }
    }
}

#[derive(Clone)]
pub struct AudioInputService {
    inner: ContextAudioInput,
}

pub struct AudioInputContentCursor {
    inner: ContextAudioContextCursor,
}

impl AudioInputService {
    pub fn try_new(source: InputSource) -> crate::Result<Self> {
        Ok(Self::from(DefaultAudioInputProvider::provide(source)?))
    }
}

impl ContentCursorTrait for AudioInputContentCursor {
    type Item = Audio;
    fn read(&mut self) -> crate::Result<Option<rmf_core::Content<Self::Item>>> {
        Ok(self.inner.read()?)
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> crate::Result<()> {
        Ok(self.inner.seek(timestamp)?)
    }
}

impl From<DefaultAudioInput> for AudioInputService {
    fn from(value: DefaultAudioInput) -> Self {
        AudioInputService {
            inner: ContextAudioInput::Default(value),
        }
    }
}

impl ServiceTrait for AudioInputService {}

impl ContentStreamServiceTrait for AudioInputService {
    type Item = Audio;
    type ContentCursor = AudioInputContentCursor;

    #[inline]
    fn duration(&self) -> rmf_core::Timestamp {
        self.inner.duration()
    }
    #[inline]
    fn cursor(&self) -> crate::Result<Self::ContentCursor> {
        Ok(AudioInputContentCursor {
            inner: self.inner.cursor()?,
        })
    }
}

impl AudioContentStreamServiceTrait for AudioInputService {}
