use std::{ffi::CString, os::unix::ffi::OsStrExt, path::PathBuf};

use rmf_core::{Error, InputService, InputServiceProvider, InputSource, Result};
use rsmpeg::avformat::AVFormatContextInput;

pub struct AVFormatInputService {
    input: AVFormatContextInput,
}

impl InputService for AVFormatInputService {}

impl AVFormatInputService {
    fn try_from_path(path: PathBuf) -> Result<Self> {
        Ok(Self {
            input: AVFormatContextInput::open(
                &CString::new(path.as_os_str().as_bytes().to_vec()).unwrap(),
            )
            .map_err(|e| Error::new_input(e.into()))?,
        })
    }
}

impl InputServiceProvider for AVFormatInputService {
    type InputService = Self;
    fn new(source: InputSource) -> Result<Self> {
        match source {
            InputSource::Path(path) => Self::try_from_path(path),
        }
    }
}
