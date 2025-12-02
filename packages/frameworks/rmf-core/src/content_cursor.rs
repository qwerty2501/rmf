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
    fn seek(&mut self, timestamp: i64, flag: Option<ContentSeekFlag>) -> Result<()>;
}

#[repr(C)]
pub enum ContentSeekFlag {
    Backword = 1,
}

pub trait Content {
    type Item;
    fn item(&self) -> &Self::Item;
    fn item_mut(&mut self) -> &mut Self::Item;
    fn presentation_timestamp(&self) -> i64;
    fn duration_timestamp(&self) -> i64;
}

pub trait ContentConstructor {
    type Item;
    type Content: Content;
    fn new(item: Self::Item, presentation_timestamp: i64, duration_timestamp: i64)
    -> Self::Content;
}
