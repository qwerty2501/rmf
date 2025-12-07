use rmf_core::{Content, InnerContent, Result, Timestamp};
use rmf_static::{Audio, Image};

pub trait Service {}

pub trait ContentCursor {
    type Item: InnerContent;
    fn read(&mut self) -> Result<Option<Content<Self::Item>>>;
    fn seek(&mut self, timestamp: Timestamp) -> Result<()>;
}
pub trait ContentStreamService: Service {
    type Item: InnerContent;
    type ContentCursor: ContentCursor;
    fn cursor(&self) -> Result<Self::ContentCursor>;
}

pub trait ImageContentStreamService: ContentStreamService<Item = Image> {}

pub trait ImageInputService: ImageContentStreamService {}

pub trait AudioContentStreamService: ContentStreamService<Item = Audio> {}

pub trait AudioInputService: AudioContentStreamService {}
