use std::path::PathBuf;

use derive_new::new;

use crate::{ContentCursor, Result};

pub trait Service {}

#[derive(new)]
pub enum InputSource {
    Path(PathBuf),
}

pub trait InputService: Service {
    type ImageContentCursor: ContentCursor;
    fn image_cursor(&self) -> Self::ImageContentCursor;
}

impl<T: InputService> Service for T {}

pub trait InputServiceProvider {
    type InputService: InputService;
    fn try_new(source: InputSource) -> Result<Self::InputService>;
}

pub trait OutputService: Service {}
