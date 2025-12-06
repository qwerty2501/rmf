use std::path::PathBuf;

use derive_new::new;

pub trait Service {}

#[derive(new)]
pub enum InputSource {
    Path(PathBuf),
}

pub trait OutputService: Service {}
