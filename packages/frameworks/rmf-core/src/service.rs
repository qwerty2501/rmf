use std::path::PathBuf;

use derive_new::new;

#[derive(new)]
pub enum InputSource {
    Path(PathBuf),
}

pub trait OutputService {}
