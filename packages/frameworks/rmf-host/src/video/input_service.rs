use crate::image::Image;
use rmf_core::video::VideoContentCursor as _;
use rmf_core::video::VideoInput as _;
use rmf_static::video::DefaultVideoContentCursor;
use rmf_static::video::DefaultVideoInput;

use crate::{
    Result,
    service::{
        ContentCursorTrait, ContentStreamServiceTrait, ServiceTrait,
        VideoContentStreamServiceTrait, VideoInputServiceTrait,
    },
};

#[derive(Clone)]
enum ContextVideoInput {
    Default(DefaultVideoInput),
}

enum ContextVideoContentCursor {
    Default(DefaultVideoContentCursor),
}

impl rmf_core::video::VideoContentCursor for ContextVideoContentCursor {
    type Item = Image;

    #[inline]
    fn fps(&self) -> u32 {
        match self {
            Self::Default(d) => d.fps(),
        }
    }
    #[inline]
    fn read(&mut self) -> rmf_core::Result<Option<rmf_core::Content<Self::Item>>> {
        match self {
            Self::Default(d) => d.read(),
        }
    }
    #[inline]
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> rmf_core::Result<()> {
        match self {
            Self::Default(d) => d.seek(timestamp),
        }
    }
}

impl rmf_core::video::VideoInput for ContextVideoInput {
    type Item = Image;
    type ContentCursor = ContextVideoContentCursor;
    #[inline]
    fn fps(&self) -> u32 {
        match self {
            Self::Default(d) => d.fps(),
        }
    }
    #[inline]
    fn duration(&self) -> rmf_core::Timestamp {
        match self {
            Self::Default(d) => d.duration(),
        }
    }
    #[inline]
    fn cursor(&self) -> rmf_core::Result<Self::ContentCursor> {
        Ok(match self {
            Self::Default(d) => ContextVideoContentCursor::Default(d.cursor()?),
        })
    }
}

#[derive(Clone)]
pub struct VideoInputService {
    inner: ContextVideoInput,
}

pub struct VideoInputContentCursor {
    inner: ContextVideoContentCursor,
}

impl ContentCursorTrait for VideoInputContentCursor {
    type Item = Image;
    fn read(&mut self) -> Result<Option<rmf_core::Content<Self::Item>>> {
        Ok(self.inner.read()?)
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> Result<()> {
        Ok(self.inner.seek(timestamp)?)
    }
}

impl ServiceTrait for VideoInputService {}

impl ContentStreamServiceTrait for VideoInputService {
    type Item = Image;
    type ContentCursor = VideoInputContentCursor;
    #[inline]
    fn duration(&self) -> rmf_core::Timestamp {
        self.inner.duration()
    }
    #[inline]
    fn cursor(&self) -> Result<Self::ContentCursor> {
        Ok(VideoInputContentCursor {
            inner: self.inner.cursor()?,
        })
    }
}

impl VideoContentStreamServiceTrait for VideoInputService {
    fn fps(&self) -> u32 {
        self.inner.fps()
    }
}

impl VideoInputServiceTrait for VideoInputService {}

impl From<DefaultVideoInput> for VideoInputService {
    fn from(value: DefaultVideoInput) -> Self {
        VideoInputService {
            inner: ContextVideoInput::Default(value),
        }
    }
}
