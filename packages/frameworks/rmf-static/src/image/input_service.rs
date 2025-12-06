use rmf_core::image::{ImageContentCursor, ImageInputService};

use crate::{Image, ffmpeg::AVFormatImageInputService};

pub struct OpaqueImageContentCursor {
    inner: Box<dyn ImageContentCursor<Item = Image>>,
}

impl rmf_core::Service for OpaqueImageContentCursor {}
impl rmf_core::image::ImageContentCursor for OpaqueImageContentCursor {
    type Item = Image;
    fn read(&mut self) -> rmf_core::Result<Option<rmf_core::Content<Image>>> {
        self.inner.read()
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> rmf_core::Result<()> {
        self.inner.seek(timestamp)
    }
}

pub struct OpaqueImageInputService<
    C: ImageContentCursor<Item = Image> + 'static,
    S: rmf_core::image::ImageInputService<Item = Image, ContentCursor = C>,
> {
    inner: S,
}

impl<
    C: ImageContentCursor<Item = Image> + 'static,
    S: rmf_core::image::ImageInputService<Item = Image, ContentCursor = C>,
> rmf_core::Service for OpaqueImageInputService<C, S>
{
}

impl<
    C: ImageContentCursor<Item = Image> + 'static,
    S: rmf_core::image::ImageInputService<Item = Image, ContentCursor = C>,
> rmf_core::image::ImageInputService for OpaqueImageInputService<C, S>
{
    type Item = Image;
    type ContentCursor = OpaqueImageContentCursor;
    fn cursor(&self) -> rmf_core::Result<Self::ContentCursor> {
        Ok(OpaqueImageContentCursor {
            inner: Box::new(self.inner.cursor()?),
        })
    }
}

pub struct OpaqueImageInputServiceProvider;

impl OpaqueImageInputServiceProvider {
    pub fn into_opaque<C, S>(
        service: S,
    ) -> Box<dyn ImageInputService<Item = Image, ContentCursor = OpaqueImageContentCursor>>
    where
        C: ImageContentCursor<Item = Image> + 'static,
        S: ImageInputService<Item = Image, ContentCursor = C> + 'static,
    {
        Box::new(OpaqueImageInputService { inner: service })
    }
}

pub struct DefaultImageInputServiceProvider;

impl DefaultImageInputServiceProvider {
    pub fn provide_service(
        source: rmf_core::InputSource,
    ) -> rmf_core::Result<
        Box<dyn ImageInputService<Item = Image, ContentCursor = OpaqueImageContentCursor>>,
    > {
        Ok(OpaqueImageInputServiceProvider::into_opaque(
            AVFormatImageInputService::new(source),
        ))
    }
}

impl<
    C: ImageContentCursor<Item = Image> + 'static,
    S: ImageInputService<Item = Image, ContentCursor = C> + 'static,
> From<S> for OpaqueImageInputService<C, S>
{
    fn from(value: S) -> Self {
        OpaqueImageInputService { inner: value }
    }
}
