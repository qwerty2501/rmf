use rmf_macros::delegate_implements;

use crate::{audio::Audio, image::Image};

pub struct Frame {
    image: Option<Image>,
    audio: Option<Audio>,
}

#[delegate_implements]
impl rmf_core::Frame for Frame {
    type Image = Image;
    type Audio = Audio;
    fn image(&self) -> Option<&Image> {
        self.image.as_ref()
    }
    fn audio(&self) -> Option<&Audio> {
        self.audio.as_ref()
    }
}
