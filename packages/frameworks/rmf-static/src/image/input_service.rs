use rmf_core::image::ImageInputService;

use crate::{Image, ffmpeg::AVFormatImageInputService};

pub struct DefaultImageServiceProvider;

impl DefaultImageServiceProvider {
    pub fn provide_new(
        source: rmf_core::InputSource,
    ) -> rmf_core::Result<Box<dyn ImageInputService<Item = Image>>> {
        Ok(Box::new(AVFormatImageInputService::new(source)))
    }
}
