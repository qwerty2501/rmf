use rmf_core::audio::{AudioContentCursor, AudioInputService};

use crate::{Audio, ffmpeg::AVFormatAudioInputService};

pub struct OpaqueAudioContentCursor {
    inner: Box<dyn AudioContentCursor<Item = Audio>>,
}

impl rmf_core::Service for OpaqueAudioContentCursor {}
impl rmf_core::audio::AudioContentCursor for OpaqueAudioContentCursor {
    type Item = Audio;
    fn read(&mut self) -> rmf_core::Result<Option<rmf_core::Content<Audio>>> {
        self.inner.read()
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> rmf_core::Result<()> {
        self.inner.seek(timestamp)
    }
}

pub struct OpaqueAudioInputService<
    C: AudioContentCursor<Item = Audio> + 'static,
    S: rmf_core::audio::AudioInputService<Item = Audio, ContentCursor = C>,
> {
    inner: S,
}

impl<
    C: AudioContentCursor<Item = Audio> + 'static,
    S: rmf_core::audio::AudioInputService<Item = Audio, ContentCursor = C>,
> rmf_core::Service for OpaqueAudioInputService<C, S>
{
}

impl<
    C: AudioContentCursor<Item = Audio> + 'static,
    S: rmf_core::audio::AudioInputService<Item = Audio, ContentCursor = C>,
> rmf_core::audio::AudioInputService for OpaqueAudioInputService<C, S>
{
    type Item = Audio;
    type ContentCursor = OpaqueAudioContentCursor;
    fn cursor(&self) -> rmf_core::Result<Self::ContentCursor> {
        Ok(OpaqueAudioContentCursor {
            inner: Box::new(self.inner.cursor()?),
        })
    }
}

pub struct OpaqueAudioInputServiceProvider;

impl OpaqueAudioInputServiceProvider {
    pub fn into_opaque<C, S>(
        service: S,
    ) -> Box<dyn AudioInputService<Item = Audio, ContentCursor = OpaqueAudioContentCursor>>
    where
        C: AudioContentCursor<Item = Audio> + 'static,
        S: AudioInputService<Item = Audio, ContentCursor = C> + 'static,
    {
        Box::new(OpaqueAudioInputService { inner: service })
    }
}

pub struct DefaultAudioInputServiceProvider;

impl DefaultAudioInputServiceProvider {
    pub fn provide_service(
        source: rmf_core::InputSource,
    ) -> rmf_core::Result<
        Box<dyn AudioInputService<Item = Audio, ContentCursor = OpaqueAudioContentCursor>>,
    > {
        Ok(OpaqueAudioInputServiceProvider::into_opaque(
            AVFormatAudioInputService::new(source),
        ))
    }
}

impl<
    C: AudioContentCursor<Item = Audio> + 'static,
    S: AudioInputService<Item = Audio, ContentCursor = C> + 'static,
> From<S> for OpaqueAudioInputService<C, S>
{
    fn from(value: S) -> Self {
        OpaqueAudioInputService { inner: value }
    }
}
