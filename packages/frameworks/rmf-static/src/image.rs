use rmf_macros::delegate_implements;

pub struct Image {}

#[delegate_implements]
impl rmf_core::image::Image for Image {}
