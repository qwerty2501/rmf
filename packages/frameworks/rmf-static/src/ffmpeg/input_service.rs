use std::{ffi::CString, os::unix::ffi::OsStrExt, path::Path};

use rmf_core::{Error, InputSource, Result, audio::AudioInput, image::ImageInput};
use rmf_macros::delegate_implements;
use rsmpeg::avformat::AVFormatContextInput;

use crate::{
    Audio, Image,
    ffmpeg::{AVFormatAudioContentCursor, AVFormatImageContentCursor},
};

#[derive(Clone)]
pub struct AVFormatImageInput {
    source: InputSource,
}

#[derive(Clone)]
pub struct AVFormatAudioInput {
    source: InputSource,
}

#[delegate_implements]
impl ImageInput for AVFormatImageInput {
    type Item = Image;
    type ContentCursor = AVFormatImageContentCursor;
    fn cursor(&self) -> Result<AVFormatImageContentCursor> {
        AVFormatImageContentCursor::try_new(make_input(&self.source)?)
    }
}

#[delegate_implements]
impl AudioInput for AVFormatAudioInput {
    type Item = Audio;
    type ContentCursor = AVFormatAudioContentCursor;
    fn cursor(&self) -> Result<AVFormatAudioContentCursor> {
        AVFormatAudioContentCursor::try_new(make_input(&self.source)?)
    }
}

impl AVFormatImageInput {
    pub fn new(source: InputSource) -> Self {
        Self { source }
    }
}

impl AVFormatAudioInput {
    pub fn new(source: InputSource) -> Self {
        Self { source }
    }
}

fn try_from_path_input(path: impl AsRef<Path>) -> Result<AVFormatContextInput> {
    let path = path.as_ref();
    AVFormatContextInput::open(&CString::new(path.as_os_str().as_bytes().to_vec()).unwrap())
        .map_err(|e| Error::new_input(e.into()))
}
fn make_input(source: &InputSource) -> Result<AVFormatContextInput> {
    match source {
        InputSource::Path(path) => try_from_path_input(path),
    }
}
