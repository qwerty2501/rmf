use dyn_clone::DynClone;

use crate::{Content, Result, Timestamp, image::Image};

pub trait VideoContentCursor {
    type Item: Image;
    fn read(&mut self) -> Result<Option<Content<Self::Item>>>;
    fn fps(&self) -> f64;
    fn seek(&mut self, timestamp: Timestamp) -> Result<()>;
}

pub trait VideoInput: DynClone {
    type Item: Image;
    type ContentCursor: VideoContentCursor;
    fn duration(&self) -> Timestamp;
    fn fps(&self) -> f64;
    fn cursor(&self) -> Result<Self::ContentCursor>;
}

dyn_clone::clone_trait_object!(<I,C> VideoInput<Item = I,ContentCursor=C> where I:Image ,C:VideoContentCursor);
