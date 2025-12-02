use std::time::Duration;

use derive_new::new;

use crate::{Result, audio::Audio, image::Image};

#[derive(new)]
pub enum ContextContent<I: Image, A: Audio> {
    Image(I),
    Audio(A),
}

pub trait ContentCursor {
    type Content: Content;
    fn read(&mut self) -> Result<Option<Self::Content>>;
    fn seek(&mut self, timestamp: Duration, flag: Option<ContentSeekFlag>) -> Result<()>;
}

#[repr(C)]
pub enum ContentSeekFlag {
    Backword = 1,
}

pub trait Content {
    type Item;
    fn item(&self) -> &Self::Item;
    fn item_mut(&mut self) -> &mut Self::Item;
    fn presentation_timestamp(&self) -> Duration;
    fn duration_timestamp(&self) -> Duration;
}

pub trait ContentConstructor {
    type Item;
    type Content: Content;
    fn new(
        item: Self::Item,
        presentation_timestamp: Duration,
        duration_timestamp: Duration,
    ) -> Self::Content;
}
