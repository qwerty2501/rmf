use crate::{Content, InnerContent, InputSource, Result, Service, Size, Timestamp};

pub trait Image: InnerContent + Clone {
    fn new_size(size: Size, data: &[u8]) -> Result<Self>;
    fn data_bytes(&self) -> &[u8];
    fn size(&self) -> Size;
}
pub trait ImageContentCursor: Service {
    type Item: Image;
    fn read(&mut self) -> Result<Option<Content<Self::Item>>>;
    fn seek(&mut self, timestamp: Timestamp) -> Result<()>;
}

pub trait ImageInputService {
    type Item: Image;
    fn cursor(&self) -> Result<Box<dyn ImageContentCursor<Item = Self::Item>>>;
}

pub trait ImageInputServiceProvider {
    type Item: Image;
    fn try_new(source: InputSource) -> Result<Box<dyn ImageInputService<Item = Self::Item>>>;
}
