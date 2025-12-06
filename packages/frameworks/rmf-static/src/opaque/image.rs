use derive_new::new;
use rmf_core::image::{ImageContentCursor, ImageInputService};

use crate::Image;

#[derive(new)]
pub(crate) struct OpaqueImageInputService {
    inner: Box<dyn ImageInputService<Item = Image>>,
}

impl rmf_core::image::ImageInputService for OpaqueImageInputService {
    type Item = Image;
    fn cursor(&self) -> rmf_core::Result<Box<dyn ImageContentCursor<Item = Image>>> {
        self.inner.cursor()
    }
}
