use crate::{
    core::{Error, Timestamp},
    ffmpeg::utils::input_contexts,
};
use anyhow::anyhow;
use rmf_core::{InputSource, Result, audio::AudioInput};
use rmf_macros::delegate_implements;
use rsmpeg::ffi::AVMEDIA_TYPE_AUDIO;

use crate::{
    Audio,
    ffmpeg::{AVFormatAudioContentCursor, utils::make_input},
};

#[derive(Clone)]
pub struct AVFormatAudioInput {
    source: InputSource,
    duration: Timestamp,
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

        Ok(Self {
            source,
            sample_rate: audio_stream.codecpar().sample_rate as _,
            duration: Timestamp::from_microseconds(input.duration),
        })
    }
}
