use rmf_macros::delegate_implements;

#[derive(Clone)]
pub struct Image {
    inner: photon_rs::PhotonImage,
}

impl crate::core::InnerContent for Image {}

#[delegate_implements]
impl rmf_core::image::Image for Image {
    fn data_bytes(&self) -> Vec<u8> {
        self.inner.get_raw_pixels()
    }
    fn size(&self) -> crate::core::Size {
        crate::core::Size {
            height: self.inner.get_height(),
            width: self.inner.get_width(),
        }
    }
    fn new_size(size: crate::core::Size, data: &[u8]) -> crate::core::Result<Self> {
        Ok(Self {
            inner: photon_rs::PhotonImage::new(data.to_vec(), size.width, size.height),
        })
    }
}
