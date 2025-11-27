use crate::{Result, Size};

pub trait Image: Clone {
    fn new_size(size: Size, data: &[u8]) -> Result<Self>;
    fn data_bytes(&self) -> &[u8];
    fn size(&self) -> Size;
}
