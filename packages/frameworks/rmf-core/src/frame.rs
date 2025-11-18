use crate::*;

pub trait Frame {
    type Image: image::Image;
    type Audio: audio::Audio;
}
