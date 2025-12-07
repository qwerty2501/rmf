use rmf_core::{Content, InnerContent, Timestamp};
use rmf_static::{Audio, Image};

use crate::Result;

pub trait ServiceTrait {}

pub trait ContentCursorTrait {
    type Item: InnerContent;
    fn read(&mut self) -> Result<Option<Content<Self::Item>>>;
    fn seek(&mut self, timestamp: Timestamp) -> Result<()>;
}
pub trait ContentStreamServiceTrait: ServiceTrait {
    type Item: InnerContent;
    type ContentCursor: ContentCursorTrait;
    fn cursor(&self) -> Result<Self::ContentCursor>;
}

pub trait ImageContentStreamServiceTrait: ContentStreamServiceTrait<Item = Image> {}

pub trait ImageInputServiceTrait: ImageContentStreamServiceTrait {}

pub trait AudioContentStreamServiceTrait: ContentStreamServiceTrait<Item = Audio> {}

pub trait AudioInputServiceTrait: AudioContentStreamServiceTrait {}
