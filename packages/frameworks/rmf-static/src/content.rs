use rmf_core::Timestamp;
use rmf_macros::delegate_implements;

pub struct Content<I> {
    item: I,
    presentation_timestamp: Timestamp,
    duration_timestamp: Timestamp,
}

#[delegate_implements]
impl<I> rmf_core::Content for Content<I> {
    type Item = I;
    fn item(&self) -> &I {
        &self.item
    }
    fn item_mut(&mut self) -> &mut I {
        &mut self.item
    }

    fn presentation_timestamp(&self) -> Timestamp {
        self.presentation_timestamp
    }
    fn duration_timestamp(&self) -> Timestamp {
        self.duration_timestamp
    }
}

#[delegate_implements]
impl<I> rmf_core::ContentConstructor for Content<I> {
    type Item = I;
    type Content = Self;
    fn new(item: I, presentation_timestamp: Timestamp, duration_timestamp: Timestamp) -> Self {
        Self {
            item,
            presentation_timestamp,
            duration_timestamp,
        }
    }
}
