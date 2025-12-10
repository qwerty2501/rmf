use crate::{InnerContent, Result, Size};

pub trait Image: InnerContent + Clone {
    fn new_size(size: Size, data: &[u8]) -> Result<Self>;
    fn data_bytes(&self) -> &[u8];
    fn size(&self) -> Size;
}
