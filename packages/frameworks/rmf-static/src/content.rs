use std::time::Duration;

use rsmpeg::avformat::AVFormatContextInput;

pub struct VideoContentCursor {
    input: AVFormatContextInput,
}

pub struct Content<I> {
    item: I,
    timestamp: Duration,
}

impl<I> rmf_core::Content for Content<I> {
    type Item = I;
    fn item(&self) -> &Self::Item {
        &self.item
    }
    fn timestamp(&self) -> Duration {
        self.timestamp.clone()
    }
    fn owned_item(self) -> Self::Item {
        self.item
    }
}
