use dyn_clone::DynClone;
use rmf_core::{Content, InnerContent, Timestamp};
use rmf_static::{Audio, Image};

use crate::Result;

pub trait ServiceTrait {}

pub trait ContentCursorTrait {
    type Item: InnerContent;
    fn read(&mut self) -> Result<Option<Content<Self::Item>>>;
    fn seek(&mut self, timestamp: Timestamp) -> Result<()>;
}

pub trait ContentStreamServiceTrait: ServiceTrait + Clone + DynClone {
    type Item: InnerContent;
    type ContentCursor: ContentCursorTrait<Item = Self::Item>;
    fn cursor(&self) -> Result<Self::ContentCursor>;
}

pub trait ImageContentStreamServiceTrait: ContentStreamServiceTrait<Item = Image> {}

pub trait ImageInputServiceTrait: ContentStreamServiceTrait<Item = Image> {}

pub trait AudioContentStreamServiceTrait: ContentStreamServiceTrait<Item = Audio> {}

pub trait AudioInputServiceTrait: ContentStreamServiceTrait<Item = Audio> {}
