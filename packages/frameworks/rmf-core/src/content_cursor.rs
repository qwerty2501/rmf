use std::time::Duration;

use crate::Result;

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
    fn owned_item(self) -> Self::Item;
    fn timestamp(&self) -> Duration;
}
