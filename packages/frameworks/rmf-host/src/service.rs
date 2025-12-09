use rmf_core::{Content, InnerContent, Timestamp};
use rmf_static::{Audio, Image};

use crate::{
    Result,
    image::{ImageInputContentCursor, ImageInputService},
};

pub trait ServiceTrait {}

pub trait ContentCursorTrait {
    type Item: InnerContent;
    fn read(&mut self) -> Result<Option<Content<Self::Item>>>;
    fn seek(&mut self, timestamp: Timestamp) -> Result<()>;
}

pub trait ContentStreamServiceTrait: ServiceTrait + Clone {
    type Item: InnerContent;
    type ContentCursor: ContentCursorTrait<Item = Self::Item>;
    fn cursor(&self) -> Result<Self::ContentCursor>;
}

pub enum ContextImageContentCursor {
    ImageInputContentCursor(ImageInputContentCursor),
}

impl ContentCursorTrait for ContextImageContentCursor {
    type Item = Image;
    fn read(&mut self) -> Result<Option<rmf_core::Content<Self::Item>>> {
        match self {
            Self::ImageInputContentCursor(c) => c.read(),
        }
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> Result<()> {
        match self {
            Self::ImageInputContentCursor(c) => c.seek(timestamp),
        }
    }
}

pub trait ImageContentStreamServiceTrait: ContentStreamServiceTrait<Item = Image> {}

#[derive(Clone)]
pub enum ContextImageContentStreamService {
    ImageInputService(ImageInputService),
}
impl ServiceTrait for ContextImageContentStreamService {}

impl ContentStreamServiceTrait for ContextImageContentStreamService {
    type Item = Image;
    type ContentCursor = ContextImageContentCursor;
    fn cursor(&self) -> Result<Self::ContentCursor> {
        Ok(match self {
            ContextImageContentStreamService::ImageInputService(s) => {
                ContextImageContentCursor::ImageInputContentCursor(s.cursor()?)
            }
        })
    }
}

pub trait ImageInputServiceTrait: ContentStreamServiceTrait<Item = Image> {}

pub trait AudioContentStreamServiceTrait: ContentStreamServiceTrait<Item = Audio> {}

pub trait AudioInputServiceTrait: ContentStreamServiceTrait<Item = Audio> {}
