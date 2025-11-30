use std::ffi::c_void;

use opencv::{core::CV_8UC3, prelude::*};
use rmf_core::{Error, Result, Size};
use rmf_macros::delegate_implements;

#[derive(Clone)]
pub struct Image {
    mat: Mat,
}

#[delegate_implements]
impl rmf_core::image::Image for Image {
    fn size(&self) -> Size {
        if let Ok(s) = self.mat.size() {
            Size {
                height: s.height as _,
                width: s.width as _,
            }
        } else {
            Size::default()
        }
    }
    fn data_bytes(&self) -> &[u8] {
        if let Ok(data) = self.mat.data_bytes() {
            data
        } else {
            &[]
        }
    }
    fn new_size(size: Size, data: &[u8]) -> Result<Self> {
        let mat = unsafe {
            Mat::new_size_with_data_unsafe(
                opencv::core::Size::new(size.width as i32, size.height as i32),
                CV_8UC3,
                data.as_ptr() as *mut c_void,
                data.len(),
            )
        }
        .map_err(|e| Error::new_image(e.into()))?
        .clone();
        Ok(Self { mat })
    }
}
