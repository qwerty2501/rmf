use std::path::PathBuf;

use derive_new::new;

use crate::Result;

pub trait Service {}

#[derive(new)]
pub enum InputSource {
    Path(PathBuf),
}

pub trait InputService: Service {}

impl<T: InputService> Service for T {}

pub trait InputServiceProvider {
    type InputService: InputService;
    fn try_new(source: InputSource) -> Result<Self::InputService>;
}

pub trait OutputService: Service {}
