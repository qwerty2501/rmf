#[cfg(feature = "use_ffmpeg")]
pub mod avformat_video_content_cursor;

use std::time::Duration;

#[cfg(feature = "use_ffmpeg")]
pub use avformat_video_content_cursor::*;
use rmf_macros::delegate_implements;

use crate::{Audio, Image};

pub struct Content<I> {
    item: I,
    presentation_timestamp: Duration,
    duration_timestamp: Duration,
}

pub type ContextContent = rmf_core::ContextContent<Image, Audio>;

#[delegate_implements]
impl<I> rmf_core::Content for Content<I> {
    type Item = I;
    fn item(&self) -> &I {
        &self.item
    }
    fn item_mut(&mut self) -> &mut I {
        &mut self.item
    }

    fn presentation_timestamp(&self) -> Duration {
        self.presentation_timestamp
    }
    fn duration_timestamp(&self) -> Duration {
        self.duration_timestamp
    }
}

#[delegate_implements]
impl<I> rmf_core::ContentConstructor for Content<I> {
    type Item = I;
    type Content = Self;
    fn new(item: I, presentation_timestamp: Duration, duration_timestamp: Duration) -> Self {
        Self {
            item,
            presentation_timestamp,
            duration_timestamp,
        }
    }
}
