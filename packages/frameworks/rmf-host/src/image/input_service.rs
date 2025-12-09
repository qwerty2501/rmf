use rmf_core::image::ImageContentCursor as _;
use rmf_core::image::ImageInput as _;
use rmf_static::{DefaultImageContentCursor, DefaultImageInput, Image};

use crate::{
    Result,
    service::{
        ContentCursorTrait, ContentStreamServiceTrait, ImageContentStreamServiceTrait,
        ImageInputServiceTrait, ServiceTrait,
    },
};

#[derive(Clone)]
pub enum ContextImageInput {
    Default(DefaultImageInput),
}

pub enum ContextImageContentCursor {
    Default(DefaultImageContentCursor),
}

impl rmf_core::image::ImageContentCursor for ContextImageContentCursor {
    type Item = Image;
    fn read(&mut self) -> rmf_core::Result<Option<rmf_core::Content<Self::Item>>> {
        match self {
            Self::Default(d) => d.read(),
        }
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> rmf_core::Result<()> {
        match self {
            Self::Default(d) => d.seek(timestamp),
        }
    }
}

impl rmf_core::image::ImageInput for ContextImageInput {
    type Item = Image;
    type ContentCursor = ContextImageContentCursor;
    fn cursor(&self) -> rmf_core::Result<Self::ContentCursor> {
        Ok(match self {
            Self::Default(d) => ContextImageContentCursor::Default(d.cursor()?),
        })
    }
}

#[derive(Clone)]
pub struct ImageInputService {
    inner: ContextImageInput,
}

pub struct ImageInputContentCursor {
    inner: ContextImageContentCursor,
}

impl ContentCursorTrait for ImageInputContentCursor {
    type Item = Image;
    fn read(&mut self) -> Result<Option<rmf_core::Content<Self::Item>>> {
        Ok(self.inner.read()?)
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> Result<()> {
        Ok(self.inner.seek(timestamp)?)
    }
}

impl ServiceTrait for ImageInputService {}

impl ContentStreamServiceTrait for ImageInputService {
    type Item = Image;
    type ContentCursor = ImageInputContentCursor;

    fn cursor(&self) -> Result<ImageInputContentCursor> {
        Ok(ImageInputContentCursor {
            inner: self.inner.cursor()?,
        })
    }
}

impl ImageContentStreamServiceTrait for ImageInputService {}

impl ImageInputServiceTrait for ImageInputService {}
