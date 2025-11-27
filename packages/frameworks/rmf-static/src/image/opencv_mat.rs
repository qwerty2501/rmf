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
        let s = self.mat.size().unwrap();
        Size {
            height: s.height as usize,
            width: s.width as usize,
        }
    }
    fn data_bytes(&self) -> &[u8] {
        self.mat.data_bytes().unwrap()
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
