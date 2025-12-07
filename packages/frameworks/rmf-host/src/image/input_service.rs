use rmf_core::image::ImageContentCursor as _;
use rmf_static::{Image, OpaqueImageContentCursor, OpaqueImageInput};

use crate::{
    Result,
    service::{
        ContentCursorTrait, ContentStreamServiceTrait, ImageContentStreamServiceTrait,
        ImageInputServiceTrait, ServiceTrait,
    },
};

pub struct ImageInputService {
    inner: OpaqueImageInput,
}

pub struct ImageInputContentCursor {
    inner: OpaqueImageContentCursor,
}

impl ContentCursorTrait for ImageInputContentCursor {
    type Item = Image;
    fn read(&mut self) -> Result<Option<rmf_core::Content<Self::Item>>> {
        Ok(self.inner.read()?)
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> Result<()> {
        Ok(self.inner.seek(timestamp)?)
    }
}

impl ServiceTrait for ImageInputService {}

impl ContentStreamServiceTrait for ImageInputService {
    type Item = Image;
    type ContentCursor = ImageInputContentCursor;
    fn cursor(&self) -> Result<Self::ContentCursor> {
        let cursor = self.inner.cursor()?;
        Ok(ImageInputContentCursor { inner: cursor })
    }
}

impl ImageContentStreamServiceTrait for ImageInputService {}

impl ImageInputServiceTrait for ImageInputService {}
