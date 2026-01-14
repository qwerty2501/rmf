use crate::{
    core::{Error, Timestamp},
    ffmpeg::utils::input_contexts,
};
use anyhow::anyhow;
use rmf_core::{InputSource, Result, audio::AudioInput};
use rmf_macros::delegate_implements;
use rsmpeg::ffi::{AVMEDIA_TYPE_AUDIO, av_q2d};

use crate::{
    Audio,
    ffmpeg::{AVFormatAudioContentCursor, utils::make_input},
};

#[derive(Clone)]
pub struct AVFormatAudioInput {
    source: InputSource,
    duration: Timestamp,
    audio_index: usize,
    fps: f64,
    sample_rate: u32,
}

#[delegate_implements]
impl AudioInput for AVFormatAudioInput {
    type Item = Audio;
    type ContentCursor = AVFormatAudioContentCursor;
    #[inline]
    fn duration(&self) -> Timestamp {
        self.duration
    }
    #[inline]
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    #[inline]
    fn fps(&self) -> f64 {
        self.fps
    }

    #[inline]
    fn cursor(&self) -> Result<AVFormatAudioContentCursor> {
        AVFormatAudioContentCursor::try_new(make_input(&self.source)?)
    }
}

impl AVFormatAudioInput {
    pub fn try_new(source: InputSource) -> Result<Self> {
        let input = make_input(&source)?;
        let context = input_contexts(&input, AVMEDIA_TYPE_AUDIO)?
            .ok_or_else(|| Error::new_input(anyhow!("not found audio stream.")))?;
        let audio_stream = &input.streams()[context.index];
        let fps = av_q2d(audio_stream.r_frame_rate);

        Ok(Self {
            source,
            audio_index: audio_stream.index as _,
            sample_rate: audio_stream.codecpar().sample_rate as _,
            fps,
            duration: Timestamp::from_microseconds(input.duration),
        })
    }
}
