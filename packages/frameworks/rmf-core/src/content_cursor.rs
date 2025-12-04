use crate::{Result, Timestamp};

pub trait ContentCursor {
    type Content: Content;
    fn read(&mut self) -> Result<Option<Self::Content>>;
    fn seek(&mut self, timestamp: Timestamp) -> Result<()>;
}

pub trait Content {
    type Item;
    fn item(&self) -> &Self::Item;
    fn item_mut(&mut self) -> &mut Self::Item;
    fn presentation_timestamp(&self) -> Timestamp;
    fn duration_timestamp(&self) -> Timestamp;
}

pub trait ContentConstructor {
    type Item;
    type Content: Content;
    fn new(
        item: Self::Item,
        presentation_timestamp: Timestamp,
        duration_timestamp: Timestamp,
    ) -> Self::Content;
}
