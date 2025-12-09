use rmf_macros::delegate_implements;

use crate::{
    Image,
    ffmpeg::{AVFormatImageContentCursor, AVFormatImageInput},
};

pub struct DefaultImageContentCursor(AVFormatImageContentCursor);

#[delegate_implements]
impl rmf_core::image::ImageContentCursor for DefaultImageContentCursor {
    type Item = Image;
    fn read(&mut self) -> rmf_core::Result<Option<rmf_core::Content<Image>>> {
        self.0.read()
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> rmf_core::Result<()> {
        self.0.seek(timestamp)
    }
}

#[derive(Clone)]
pub struct DefaultImageInput(AVFormatImageInput);

#[delegate_implements]
impl rmf_core::image::ImageInput for DefaultImageInput {
    type Item = Image;
    type ContentCursor = DefaultImageContentCursor;
    fn cursor(&self) -> rmf_core::Result<DefaultImageContentCursor> {
        Ok(DefaultImageContentCursor(self.0.cursor()?))
    }
}

pub struct DefaultImageInputProvider;

impl DefaultImageInputProvider {
    pub fn provide(source: rmf_core::InputSource) -> rmf_core::Result<DefaultImageInput> {
        Ok(DefaultImageInput(AVFormatImageInput::new(source)))
    }
}
