use std::{
    ffi::CString,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

use rmf_core::{Error, InputSource, Result, image::ImageInputService};
use rsmpeg::{avformat::AVFormatContextInput, ffi::AVFormatContext};

use crate::{
    Audio, Image,
    ffmpeg::{AVFormatAudioContentCursor, AVFormatImageContentCursor},
};

pub struct AVFormatImageInputService {
    source: InputSource,
}

impl ImageInputService for AVFormatImageInputService {
    type Item = Image;
    type ContentCursor = AVFormatImageContentCursor;
    fn cursor(&self) -> Result<AVFormatImageContentCursor> {
        AVFormatImageContentCursor::try_new(self.make_input()?)
    }
}

impl AVFormatImageInputService {
    fn try_from_path_input(path: impl AsRef<Path>) -> Result<AVFormatContextInput> {
        let path = path.as_ref();
        AVFormatContextInput::open(&CString::new(path.as_os_str().as_bytes().to_vec()).unwrap())
            .map_err(|e| Error::new_input(e.into()))
    }
    fn make_input(&self) -> Result<AVFormatContextInput> {
        match &self.source {
            InputSource::Path(path) => Self::try_from_path_input(path),
        }
    }
}

impl rmf_core::image::ImageInputServiceProvider for AVFormatImageInputService {
    type InputService = Self;
    fn try_new(source: InputSource) -> Result<Self> {
        Ok(Self { source })
    }
}
