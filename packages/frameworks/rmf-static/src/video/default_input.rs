use rmf_core::Timestamp;
use rmf_macros::delegate_implements;

use crate::{
    Image,
    opencv::{OpenCvVideoContentCursor, OpenCvVideoInput},
};

pub struct DefaultVideoContentCursor(OpenCvVideoContentCursor);

#[delegate_implements]
impl rmf_core::video::VideoContentCursor for DefaultVideoContentCursor {
    type Item = Image;
    fn read(&mut self) -> rmf_core::Result<Option<rmf_core::Content<Image>>> {
        self.0.read()
    }
    fn seek(&mut self, timestamp: rmf_core::Timestamp) -> rmf_core::Result<()> {
        self.0.seek(timestamp)
    }
}

#[derive(Clone)]
pub struct DefaultVideoInput(OpenCvVideoInput);

#[delegate_implements]
impl rmf_core::video::VideoInput for DefaultVideoInput {
    type Item = Image;
    type ContentCursor = DefaultVideoContentCursor;
    fn duration(&self) -> Timestamp {
        self.0.duration()
    }
    fn cursor(&self) -> rmf_core::Result<DefaultVideoContentCursor> {
        Ok(DefaultVideoContentCursor(self.0.cursor()?))
    }
}

pub struct DefaultVideoInputProvider;

impl DefaultVideoInputProvider {
    pub fn provide(source: rmf_core::InputSource) -> rmf_core::Result<DefaultVideoInput> {
        Ok(DefaultVideoInput(OpenCvVideoInput::try_new(source)?))
    }
}
