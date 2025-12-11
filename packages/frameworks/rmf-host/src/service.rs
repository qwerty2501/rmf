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

pub trait ContentStreamServiceTrait: ServiceTrait + DynClone {
    type Item: InnerContent;
    fn cursor(&self) -> Result<Box<dyn ContentCursorTrait<Item = Self::Item>>>;
}

pub trait VideoContentStreamServiceTrait: ContentStreamServiceTrait<Item = Image> {}

dyn_clone::clone_trait_object!(VideoContentStreamServiceTrait);

pub trait VideoInputServiceTrait: ContentStreamServiceTrait<Item = Image> {}

pub trait AudioContentStreamServiceTrait: ContentStreamServiceTrait<Item = Audio> {}

pub trait AudioInputServiceTrait: ContentStreamServiceTrait<Item = Audio> {}

dyn_clone::clone_trait_object!(AudioInputServiceTrait);
