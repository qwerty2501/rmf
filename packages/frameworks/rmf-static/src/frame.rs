use rmf_macros::delegate_implements;

use crate::{audio::Audio, image::Image};

pub struct Frame {}

#[delegate_implements]
impl rmf_core::Frame for Frame {
    type Image = Image;
    type Audio = Audio;
}
