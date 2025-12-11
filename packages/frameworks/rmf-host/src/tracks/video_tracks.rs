use std::collections::VecDeque;

use crate::context::ContextVideoContentStreamService;
use crate::image::Image;

use crate::service::{ContentCursorTrait, ServiceTrait};
use crate::service::{ContentStreamServiceTrait, VideoContentStreamServiceTrait};

#[derive(Clone)]
pub struct VideoTrack {
    sequence: VecDeque<ContextVideoContentStreamService>,
}

pub struct VideoTrackContentCursor {}

impl ContentCursorTrait for VideoTrackContentCursor {
    type Item = Image;
    fn read(&mut self) -> crate::Result<Option<rmf_core::Content<Self::Item>>> {
        unimplemented!()
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> crate::Result<()> {
        unimplemented!()
    }
}

impl ServiceTrait for VideoTrack {}

impl ContentStreamServiceTrait for VideoTrack {
    type Item = Image;
    type ContentCursor = VideoTrackContentCursor;
    fn cursor(&self) -> crate::Result<Self::ContentCursor> {
        unimplemented!()
    }
}

impl VideoContentStreamServiceTrait for VideoTrack {}
