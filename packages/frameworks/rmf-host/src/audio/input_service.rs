use rmf_static::{Audio, DefaultAudioContentCursor, DefaultAudioInput};

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
    fn cursor(&self) -> rmf_core::Result<Self::ContentCursor> {
        match self {
            Self::Default(d) => d.cursor(),
        }
    }
    fn duration(&self) -> rmf_core::Timestamp {
        match self {
            Self::Default(d) => d.duration(),
        }
    }
    fn sample_rate(&self) -> u32 {
        match self {
            Self::Default(d) => d.sample_rate(),
        }
    }
}
