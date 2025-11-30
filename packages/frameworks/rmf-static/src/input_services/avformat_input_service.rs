use std::{cmp::min, ffi::CString, os::unix::ffi::OsStrExt, path::PathBuf, sync::Arc};

use rmf_core::{Error, InputService, InputServiceProvider, InputSource, Result};
use rsmpeg::{
    avformat::{AVFormatContextInput, AVIOContextContainer, AVIOContextCustom},
    avutil::AVMem,
    ffi::{AVSEEK_SIZE, SEEK_CUR, SEEK_END, SEEK_SET},
};

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
    fn try_from_buffer(buffer: Vec<u8>) -> Result<Self> {
        let buf_len = buffer.len();
        let m = AVMem::new(buf_len);
        let mut cursor: Arc<usize> = Arc::new(0);
        let mut s_seek_cursor = cursor.clone();
        let mut set_seek_cursor = move |cursor: i64| -> i64 {
            if cursor >= 0 || cursor < buf_len as i64 {
                *Arc::make_mut(&mut s_seek_cursor) = cursor as _;
                0
            } else {
                -1
            }
        };
        let seek_cursor = cursor.clone();
        let custom = AVIOContextCustom::alloc_context(
            m,
            false,
            buffer,
            Some(Box::new(move |source, data| -> i32 {
                let source = &source[*cursor..];
                let copy_len = min(data.len(), source.len());
                data.copy_from_slice(&source[0..copy_len]);
                *Arc::make_mut(&mut cursor) += copy_len;
                0
            })),
            None,
            Some(Box::new(move |_, offset, whence| -> i64 {
                match whence as u32 {
                    AVSEEK_SIZE => buf_len as _,
                    SEEK_SET => set_seek_cursor(offset),
                    SEEK_CUR => set_seek_cursor(*seek_cursor as i64 + offset),
                    SEEK_END => set_seek_cursor(buf_len as i64 - 1 + offset),
                    _ => -1,
                }
            })),
        );
        let container = AVIOContextContainer::Custom(custom);
        let input = AVFormatContextInput::from_io_context(container)
            .map_err(|e| Error::new_input(e.into()))?;
        Ok(Self { input })
    }
}

impl InputServiceProvider for AVFormatInputService {
    type InputService = Self;
    fn try_new(source: InputSource) -> Result<Self::InputService> {
        match source {
            InputSource::Path(path) => Self::try_from_path(path),
            InputSource::Buffer(buffer) => Self::try_from_buffer(buffer),
        }
    }
}
