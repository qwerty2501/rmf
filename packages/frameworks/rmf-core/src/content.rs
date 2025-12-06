use crate::Timestamp;

pub trait InnerContent {}

pub struct Content<I> {
    item: I,
    presentation_timestamp: Timestamp,
    duration_timestamp: Timestamp,
}

impl<I> Content<I> {
    pub fn item(&self) -> &I {
        &self.item
    }
    pub fn item_mut(&mut self) -> &mut I {
        &mut self.item
    }

    pub fn presentation_timestamp(&self) -> Timestamp {
        self.presentation_timestamp
    }
    pub fn duration_timestamp(&self) -> Timestamp {
        self.duration_timestamp
    }
    pub fn new(item: I, presentation_timestamp: Timestamp, duration_timestamp: Timestamp) -> Self {
        Self {
            item,
            presentation_timestamp,
            duration_timestamp,
        }
    }
}
