use std::{ffi::CString, os::unix::ffi::OsStrExt, path::Path};

use rmf_core::{Error, InputSource, Result, Timestamp};
use rsmpeg::{
    avcodec::AVCodecContext,
    avformat::AVFormatContextInput,
    avutil::av_rescale_q,
    ffi::{self, AV_TIME_BASE_Q, AVRational, AVSEEK_FLAG_BACKWARD},
};

#[inline]
fn try_from_path_input(path: impl AsRef<Path>) -> Result<AVFormatContextInput> {
    let path = path.as_ref();
    AVFormatContextInput::open(&CString::new(path.as_os_str().as_bytes().to_vec()).unwrap())
        .map_err(|e| Error::new_input(e.into()))
}
#[inline]
pub fn make_input(source: &InputSource) -> Result<AVFormatContextInput> {
    match source {
        InputSource::Path(path) => try_from_path_input(path),
    }
}

#[inline]
pub fn seek_input(input: &mut AVFormatContextInput, timestamp: Timestamp) -> Result<()> {
    input
        .seek(-1, timestamp.as_microseconds(), AVSEEK_FLAG_BACKWARD as _)
        .map_err(|e| Error::new_input(e.into()))
}

#[inline]
pub fn to_timestamp(ts: i64, time_base: AVRational) -> Timestamp {
    Timestamp::from_microseconds(av_rescale_q(ts, time_base, AV_TIME_BASE_Q))
}

#[inline]
pub fn input_contexts(
    input: &AVFormatContextInput,
    media_type: ffi::AVMediaType,
) -> Result<Option<AVFormatContentContexts>> {
    if let Some((index, avcodec)) = input
        .find_best_stream(media_type)
        .map_err(|e| Error::new_input(e.into()))?
    {
        let avcodec_context = AVCodecContext::new(&avcodec);
        Ok(Some(AVFormatContentContexts {
            avcodec_context,
            index,
        }))
    } else {
        Ok(None)
    }
}

pub struct AVFormatContentContexts {
    pub avcodec_context: AVCodecContext,
    pub index: usize,
}
