use rmf_static::Image;

use crate::{
    Result,
    service::{ContentCursorTrait, ContentStreamServiceTrait, ServiceTrait},
    video::{VideoInputContentCursor, VideoInputService},
};

pub struct OpaqueImageContentCursor(ContextVideoContentCursor);

enum ContextVideoContentCursor {
    VideoInputContentCursor(VideoInputContentCursor),
}
impl ContentCursorTrait for OpaqueImageContentCursor {
    type Item = Image;
    fn read(&mut self) -> Result<Option<rmf_core::Content<Self::Item>>> {
        match &mut self.0 {
            ContextVideoContentCursor::VideoInputContentCursor(c) => c.read(),
        }
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> Result<()> {
        match &mut self.0 {
            ContextVideoContentCursor::VideoInputContentCursor(c) => c.seek(timestamp),
        }
    }
}

#[derive(Clone)]
pub struct OpaqueImageContentStreamService(ContextImageContentStreamService);

#[derive(Clone)]
enum ContextImageContentStreamService {
    VideoInputService(VideoInputService),
}
impl ServiceTrait for OpaqueImageContentStreamService {}

impl ContentStreamServiceTrait for OpaqueImageContentStreamService {
    type Item = Image;
    type ContentCursor = OpaqueImageContentCursor;
    fn cursor(&self) -> Result<Self::ContentCursor> {
        Ok(match &self.0 {
            ContextImageContentStreamService::VideoInputService(s) => OpaqueImageContentCursor(
                ContextVideoContentCursor::VideoInputContentCursor(s.cursor()?),
            ),
        })
    }
}

impl From<VideoInputService> for OpaqueImageContentStreamService {
    fn from(value: VideoInputService) -> Self {
        Self(ContextImageContentStreamService::VideoInputService(value))
    }
}
