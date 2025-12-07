use rmf_core::image::{ImageContentCursor, ImageInputService};
use rmf_macros::delegate_implements;

use crate::{Image, ffmpeg::AVFormatImageInputService};

pub struct OpaqueImageContentCursor {
    inner: Box<dyn ImageContentCursor<Item = Image>>,
}

impl rmf_core::image::ImageContentCursor for OpaqueImageContentCursor {
    type Item = Image;
    fn read(&mut self) -> rmf_core::Result<Option<rmf_core::Content<Image>>> {
        self.inner.read()
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> rmf_core::Result<()> {
        self.inner.seek(timestamp)
    }
}

struct OpaqueImageInputServiceInner<
    C: ImageContentCursor<Item = Image> + 'static,
    S: rmf_core::image::ImageInputService<Item = Image, ContentCursor = C>,
> {
    inner: S,
}

impl<
    C: ImageContentCursor<Item = Image> + 'static,
    S: rmf_core::image::ImageInputService<Item = Image, ContentCursor = C>,
> rmf_core::image::ImageInputService for OpaqueImageInputServiceInner<C, S>
{
    type Item = Image;
    type ContentCursor = OpaqueImageContentCursor;
    fn cursor(&self) -> rmf_core::Result<Self::ContentCursor> {
        Ok(OpaqueImageContentCursor {
            inner: Box::new(self.inner.cursor()?),
        })
    }
}

pub struct OpaqueImageInputService {
    inner: Box<dyn ImageInputService<Item = Image, ContentCursor = OpaqueImageContentCursor>>,
}

#[delegate_implements]
impl rmf_core::image::ImageInputService for OpaqueImageInputService {
    type Item = Image;
    type ContentCursor = OpaqueImageContentCursor;
    fn cursor(&self) -> rmf_core::Result<OpaqueImageContentCursor> {
        self.inner.cursor()
    }
}

impl OpaqueImageInputService {
    pub fn into_opaque<C, S>(service: S) -> Self
    where
        C: ImageContentCursor<Item = Image> + 'static,
        S: ImageInputService<Item = Image, ContentCursor = C> + 'static,
    {
        Self {
            inner: Box::new(OpaqueImageInputServiceInner { inner: service }),
        }
    }
}

pub type DefaultImageContentCursor = OpaqueImageContentCursor;
pub type DefaultImageInputService = OpaqueImageInputService;

pub struct DefaultImageInputServiceProvider;

impl DefaultImageInputServiceProvider {
    pub fn provide_service(
        source: rmf_core::InputSource,
    ) -> rmf_core::Result<DefaultImageInputService> {
        Ok(DefaultImageInputService::into_opaque(
            AVFormatImageInputService::new(source),
        ))
    }
}

impl<
    C: ImageContentCursor<Item = Image> + 'static,
    S: ImageInputService<Item = Image, ContentCursor = C> + 'static,
> From<S> for OpaqueImageInputServiceInner<C, S>
{
    fn from(value: S) -> Self {
        OpaqueImageInputServiceInner { inner: value }
    }
}
