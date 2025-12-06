use rmf_core::image::ImageInputService;

use crate::{
    Image,
    ffmpeg::AVFormatImageInputService,
};

pub struct DefaultImageServiceProvider;

impl rmf_core::image::ImageInputServiceProvider for DefaultImageServiceProvider {
    type Item = Image;
    fn try_new(
        source: rmf_core::InputSource,
    ) -> rmf_core::Result<Box<dyn ImageInputService<Item = Image>>> {
        AVFormatImageInputService::try_new(source)
    }
}
