use crate::*;

pub trait Frame {
    type Image: image::Image;
    type Audio: audio::Audio;
    fn image(&self) -> Option<&Self::Image>;
    fn audio(&self) -> Option<&Self::Audio>;
}
