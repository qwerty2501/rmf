use rmf_static::Image;

use crate::{
    Result,
    image::{ImageInputContentCursor, ImageInputService},
    service::{ContentCursorTrait, ContentStreamServiceTrait, ServiceTrait},
};

pub struct OpaqueImageContentCursor(ContextImageContentCursor);

enum ContextImageContentCursor {
    ImageInputContentCursor(ImageInputContentCursor),
}
impl ContentCursorTrait for OpaqueImageContentCursor {
    type Item = Image;
    fn read(&mut self) -> Result<Option<rmf_core::Content<Self::Item>>> {
        match &mut self.0 {
            ContextImageContentCursor::ImageInputContentCursor(c) => c.read(),
        }
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> Result<()> {
        match &mut self.0 {
            ContextImageContentCursor::ImageInputContentCursor(c) => c.seek(timestamp),
        }
    }
}

#[derive(Clone)]
pub struct OpaqueImageContentStreamService(ContextImageContentStreamService);

#[derive(Clone)]
enum ContextImageContentStreamService {
    ImageInputService(ImageInputService),
}
impl ServiceTrait for OpaqueImageContentStreamService {}

impl ContentStreamServiceTrait for OpaqueImageContentStreamService {
    type Item = Image;
    type ContentCursor = OpaqueImageContentCursor;
    fn cursor(&self) -> Result<Self::ContentCursor> {
        Ok(match &self.0 {
            ContextImageContentStreamService::ImageInputService(s) => OpaqueImageContentCursor(
                ContextImageContentCursor::ImageInputContentCursor(s.cursor()?),
            ),
        })
    }
}

impl From<ImageInputService> for OpaqueImageContentStreamService {
    fn from(value: ImageInputService) -> Self {
        Self(ContextImageContentStreamService::ImageInputService(value))
    }
}
