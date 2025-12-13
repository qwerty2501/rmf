use std::collections::VecDeque;

use rmf_core::*;

use crate::context::ContextVideoContentStreamService;
use crate::image::Image;

use crate::service::{
    ContentCursorTrait, ContentStreamServiceTrait, ServiceTrait, VideoContentStreamServiceTrait,
};

#[derive(Clone)]
struct ContentRange {
    content: ContextVideoContentStreamService,
    offset: Timestamp,
}

impl ContentRange {
    #[inline]
    fn offset(&self) -> Timestamp {
        self.offset
    }
    #[inline]
    fn duration(&self) -> Timestamp {
        self.content.duration()
    }
}

#[derive(Clone)]
pub struct VideoTrack {
    sequence: VecDeque<ContentRange>,
}

pub struct VideoTrackContentCursor {}

impl ContentCursorTrait for VideoTrackContentCursor {
    type Item = Image;
    fn read(&mut self) -> crate::Result<Option<Content<Self::Item>>> {
        unimplemented!()
    }
    fn seek(&mut self, timestamp: Timestamp) -> crate::Result<()> {
        unimplemented!()
    }
}

impl ServiceTrait for VideoTrack {}

impl ContentStreamServiceTrait for VideoTrack {
    type Item = Image;
    type ContentCursor = VideoTrackContentCursor;
    fn duration(&self) -> Timestamp {
        if let Some(c) = self.sequence.iter().last() {
            c.offset() + c.duration()
        } else {
            Timestamp::default()
        }
    }
    fn cursor(&self) -> crate::Result<Self::ContentCursor> {
        unimplemented!()
    }
}

impl VideoContentStreamServiceTrait for VideoTrack {
    fn fps(&self) -> u32 {
        unimplemented!()
    }
}
