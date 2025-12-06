use std::{ffi::CString, os::unix::ffi::OsStrExt, path::Path};

use rmf_core::{
    Error, InputSource, Result,
    audio::AudioInputService,
    image::{ImageContentCursor, ImageInputService},
};
use rmf_macros::delegate_implements;
use rsmpeg::avformat::AVFormatContextInput;

use crate::{
    Audio, Image,
    ffmpeg::{AVFormatAudioContentCursor, AVFormatImageContentCursor},
};

pub struct AVFormatImageInputService {
    source: InputSource,
}

pub struct AVFormatAudioInputService {
    source: InputSource,
}

#[delegate_implements]
impl ImageInputService for AVFormatImageInputService {
    type Item = Image;
    fn cursor(&self) -> Result<Box<dyn ImageContentCursor<Item = Image>>> {
        Ok(Box::new(AVFormatImageContentCursor::try_new(make_input(
            &self.source,
        )?)?))
    }
}

#[delegate_implements]
impl AudioInputService for AVFormatAudioInputService {
    type Item = Audio;
    type ContentCursor = AVFormatAudioContentCursor;
    fn cursor(&self) -> Result<AVFormatAudioContentCursor> {
        AVFormatAudioContentCursor::try_new(make_input(&self.source)?)
    }
}

#[delegate_implements]
impl rmf_core::image::ImageInputServiceProvider for AVFormatImageInputService {
    type Item = Image;
    fn try_new(source: InputSource) -> Result<Box<dyn ImageInputService<Item = Image>>> {
        Ok(Box::new(Self { source }))
    }
}

#[delegate_implements]
impl rmf_core::audio::AudioInputServiceProvider for AVFormatAudioInputService {
    type InputService = Self;
    fn try_new(source: InputSource) -> Result<Self> {
        Ok(Self { source })
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
