use rmf_static::Image;

use crate::{
    service::{
        ContentCursorTrait, ContentStreamServiceTrait, ServiceTrait, VideoContentStreamServiceTrait,
    },
    tracks::{VideoTrack, VideoTrackContentCursor},
    video::{VideoInputContentCursor, VideoInputService},
};

pub enum ContextVideoContentCursor {
    VideoInputContentCursor(VideoInputContentCursor),
    VideoTrackContentCursor(Box<VideoTrackContentCursor>),
}

impl ContentCursorTrait for ContextVideoContentCursor {
    type Item = Image;
    fn read(&mut self) -> crate::Result<Option<rmf_core::Content<Self::Item>>> {
        match self {
            Self::VideoInputContentCursor(v) => v.read(),
            Self::VideoTrackContentCursor(t) => t.read(),
        }
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> crate::Result<()> {
        match self {
            Self::VideoInputContentCursor(v) => v.seek(timestamp),
            Self::VideoTrackContentCursor(t) => t.seek(timestamp),
        }
    }
}

#[derive(Clone)]
pub enum ContextVideoContentStreamService {
    VideoInputService(VideoInputService),
    VideoTrack(Box<VideoTrack>),
}

impl From<VideoInputService> for ContextVideoContentStreamService {
    fn from(value: VideoInputService) -> Self {
        ContextVideoContentStreamService::VideoInputService(value)
    }
}

impl From<VideoTrack> for ContextVideoContentStreamService {
    fn from(value: VideoTrack) -> Self {
        ContextVideoContentStreamService::VideoTrack(Box::new(value))
    }
}

impl ServiceTrait for ContextVideoContentStreamService {}

impl ContentStreamServiceTrait for ContextVideoContentStreamService {
    type Item = Image;
    type ContentCursor = ContextVideoContentCursor;
    fn duration(&self) -> rmf_core::Timestamp {
        match self {
            ContextVideoContentStreamService::VideoTrack(t) => t.duration(),
            ContextVideoContentStreamService::VideoInputService(i) => i.duration(),
        }
    }
    fn cursor(&self) -> crate::Result<Self::ContentCursor> {
        Ok(match self {
            ContextVideoContentStreamService::VideoTrack(t) => {
                ContextVideoContentCursor::VideoTrackContentCursor(Box::new(t.cursor()?))
            }
            ContextVideoContentStreamService::VideoInputService(i) => {
                ContextVideoContentCursor::VideoInputContentCursor(i.cursor()?)
            }
        })
    }
}

impl VideoContentStreamServiceTrait for ContextVideoContentStreamService {}
