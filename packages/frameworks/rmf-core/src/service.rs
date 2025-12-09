use std::path::PathBuf;

use derive_new::new;

#[derive(new, Clone)]
pub enum InputSource {
    Path(PathBuf),
}

pub trait OutputService {}
