use rmf_macros::delegate_implements;

#[derive(Clone)]
pub struct Image {
    inner: photon_rs::PhotonImage,
}

impl rmf_core::InnerContent for Image {}

#[delegate_implements]
impl rmf_core::image::Image for Image {
    fn data_bytes(&self) -> Vec<u8> {
        self.inner.get_bytes()
    }
    fn size(&self) -> rmf_core::Size {
        rmf_core::Size {
            height: self.inner.get_height(),
            width: self.inner.get_width(),
        }
    }
    fn new_size(size: rmf_core::Size, data: &[u8]) -> rmf_core::Result<Self> {
        Ok(Self {
            inner: photon_rs::PhotonImage::new(data.to_vec(), size.width, size.height),
        })
    }
}
