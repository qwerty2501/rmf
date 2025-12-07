use rmf_core::audio::{AudioContentCursor, AudioInputService};
use rmf_macros::delegate_implements;

use crate::{Audio, ffmpeg::AVFormatAudioInputService};

pub struct OpaqueAudioContentCursor {
    inner: Box<dyn AudioContentCursor<Item = Audio>>,
}

impl rmf_core::audio::AudioContentCursor for OpaqueAudioContentCursor {
    type Item = Audio;
    fn read(&mut self) -> rmf_core::Result<Option<rmf_core::Content<Audio>>> {
        self.inner.read()
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> rmf_core::Result<()> {
        self.inner.seek(timestamp)
    }
}

struct OpaqueAudioInputServiceInner<
    C: AudioContentCursor<Item = Audio> + 'static,
    S: rmf_core::audio::AudioInputService<Item = Audio, ContentCursor = C>,
> {
    inner: S,
}

impl<
    C: AudioContentCursor<Item = Audio> + 'static,
    S: rmf_core::audio::AudioInputService<Item = Audio, ContentCursor = C>,
> rmf_core::audio::AudioInputService for OpaqueAudioInputServiceInner<C, S>
{
    type Item = Audio;
    type ContentCursor = OpaqueAudioContentCursor;
    fn cursor(&self) -> rmf_core::Result<Self::ContentCursor> {
        Ok(OpaqueAudioContentCursor {
            inner: Box::new(self.inner.cursor()?),
        })
    }
}

pub struct OpaqueAudioInputService {
    inner: Box<dyn AudioInputService<Item = Audio, ContentCursor = OpaqueAudioContentCursor>>,
}

impl OpaqueAudioInputService {
    pub fn into_opaque<C, S>(service: S) -> Self
    where
        C: AudioContentCursor<Item = Audio> + 'static,
        S: AudioInputService<Item = Audio, ContentCursor = C> + 'static,
    {
        Self {
            inner: Box::new(OpaqueAudioInputServiceInner { inner: service }),
        }
    }
}

#[delegate_implements]
impl rmf_core::audio::AudioInputService for OpaqueAudioInputService {
    type Item = Audio;
    type ContentCursor = OpaqueAudioContentCursor;
    fn cursor(&self) -> rmf_core::Result<OpaqueAudioContentCursor> {
        self.inner.cursor()
    }
}

pub type DefaultAudioContentCursor = OpaqueAudioContentCursor;

pub type DefaultAudioInputService = OpaqueAudioInputService;

pub struct DefaultAudioInputServiceProvider;

impl DefaultAudioInputServiceProvider {
    pub fn provide_service(
        source: rmf_core::InputSource,
    ) -> rmf_core::Result<DefaultAudioInputService> {
        Ok(DefaultAudioInputService::into_opaque(
            AVFormatAudioInputService::new(source),
        ))
    }
}

impl<
    C: AudioContentCursor<Item = Audio> + 'static,
    S: AudioInputService<Item = Audio, ContentCursor = C> + 'static,
> From<S> for OpaqueAudioInputServiceInner<C, S>
{
    fn from(value: S) -> Self {
        OpaqueAudioInputServiceInner { inner: value }
    }
}
