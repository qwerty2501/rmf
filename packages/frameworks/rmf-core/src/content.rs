use crate::Timestamp;

pub trait InnerContent {}

pub struct Content<I> {
    item: I,
    offset: Timestamp,
    duration: Timestamp,
}

impl<I> Content<I> {
    pub fn item(&self) -> &I {
        &self.item
    }
    pub fn item_mut(&mut self) -> &mut I {
        &mut self.item
    }

    pub fn offset(&self) -> Timestamp {
        self.offset
    }
    pub fn duration(&self) -> Timestamp {
        self.duration
    }
    pub fn new(item: I, offset: Timestamp, duration: Timestamp) -> Self {
        Self {
            item,
            offset,
            duration,
        }
    }
}
