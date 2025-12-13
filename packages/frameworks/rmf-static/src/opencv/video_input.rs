use opencv::{
    core::Mat,
    videoio::{
        CAP_ANY, CAP_PROP_FPS, CAP_PROP_FRAME_COUNT, CAP_PROP_POS_FRAMES, VideoCapture,
        VideoCaptureTrait, VideoCaptureTraitConst, prelude::*,
    },
};
use rmf_core::{
    Error, InputSource, Result, Timestamp,
    video::{VideoContentCursor, VideoInput},
};
use rmf_macros::delegate_implements;
use std::path::Path;

use crate::Image;

#[derive(Clone)]
pub struct OpenCvVideoInput {
    source: InputSource,
    duration: Timestamp,
    fps: u32,
}

pub struct OpenCvVideoContentCursor {
    cap: VideoCapture,
    fps: u32,
}

#[delegate_implements]
impl VideoContentCursor for OpenCvVideoContentCursor {
    type Item = Image;
    fn read(&mut self) -> Result<Option<rmf_core::Content<Image>>> {
        let mut mat = Mat::default();
        let offset = Timestamp::from_seconds_float64(
            self.cap
                .get(CAP_PROP_POS_FRAMES)
                .map_err(|e| Error::new_input(e.into()))?
                / self.fps as f64,
        );
        let duration = Timestamp::from_seconds_float64(1.0 / self.fps as f64);
        let ret = self
            .cap
            .read(&mut mat)
            .map_err(|e| Error::new_input(e.into()))?;
        if ret {
            let image = Image::new(mat);
            let content = rmf_core::Content::new(image, offset, duration);

            Ok(Some(content))
        } else {
            Ok(None)
        }
    }
    fn seek(&mut self, timestamp: Timestamp) -> Result<()> {
        self.cap
            .set(
                CAP_PROP_POS_FRAMES,
                (timestamp.as_seconds_float64() * self.fps as f64) as i64 as f64,
            )
            .map_err(|e| Error::new_input(e.into()))?;
        Ok(())
    }
}

impl OpenCvVideoInput {
    pub fn try_new(source: InputSource) -> Result<OpenCvVideoInput> {
        let cap = try_from_source(&source)?;
        let frame_count = cap
            .get(CAP_PROP_FRAME_COUNT)
            .map_err(|e| Error::new_input(e.into()))?;
        let fps = cap
            .get(CAP_PROP_FPS)
            .map_err(|e| Error::new_input(e.into()))?;
        Ok(OpenCvVideoInput {
            source,
            fps: fps as u32,
            duration: Timestamp::from_seconds_float64(frame_count / fps),
        })
    }
}

#[delegate_implements]
impl VideoInput for OpenCvVideoInput {
    type Item = Image;
    type ContentCursor = OpenCvVideoContentCursor;
    fn cursor(&self) -> Result<OpenCvVideoContentCursor> {
        let cap = try_from_source(&self.source)?;
        Ok(OpenCvVideoContentCursor { cap, fps: self.fps })
    }
    fn duration(&self) -> Timestamp {
        self.duration
    }
}

fn try_from_source(source: &InputSource) -> Result<VideoCapture> {
    match source {
        InputSource::Path(path) => try_from_path_input(path),
    }
}

fn try_from_path_input(path: impl AsRef<Path>) -> Result<VideoCapture> {
    let path = path.as_ref();
    let mut cap = VideoCapture::default().map_err(|e| Error::new_input(e.into()))?;
    cap.open_file(&path.to_string_lossy(), CAP_ANY)
        .map_err(|e| Error::new_input(e.into()))?;
    Ok(cap)
}
