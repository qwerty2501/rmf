use std::marker::PhantomData;

use anyhow::anyhow;
use rmf_core::{Error, Result};
use rmf_macros::delegate_implements;
use rsmpeg::ffi::{
    AV_SAMPLE_FMT_DBL, AV_SAMPLE_FMT_DBLP, AV_SAMPLE_FMT_FLT, AV_SAMPLE_FMT_FLTP,
    AV_SAMPLE_FMT_S16, AV_SAMPLE_FMT_S16P, AV_SAMPLE_FMT_S32, AV_SAMPLE_FMT_S32P,
    AV_SAMPLE_FMT_S64, AV_SAMPLE_FMT_S64P, AV_SAMPLE_FMT_U8P,
};
use rsmpeg::{
    avutil::AVFrame,
    ffi::{AV_SAMPLE_FMT_NONE, AV_SAMPLE_FMT_U8},
};

#[derive(Clone)]
pub struct Audio {
    data_context: AudioDataContext,
}

impl rmf_core::InnerContent for Audio {}

#[delegate_implements]
impl rmf_core::audio::Audio for Audio {
    type U8Data = AudioData<u8>;
    type I16Data = AudioData<i16>;
    type I32Data = AudioData<i32>;
    type I64Data = AudioData<i64>;
    type F32Data = AudioData<f32>;
    type F64Data = AudioData<f64>;

    #[inline]
    fn data(&self) -> &AudioDataContext {
        &self.data_context
    }
}

#[delegate_implements]
impl rmf_core::audio::AudioConstructor for Audio {
    type U8Data = AudioData<u8>;
    type I16Data = AudioData<i16>;
    type I32Data = AudioData<i32>;
    type I64Data = AudioData<i64>;
    type F32Data = AudioData<f32>;
    type F64Data = AudioData<f64>;
    type Audio = Self;
    fn tyr_new(data_context: AudioDataContext) -> Result<Self> {
        Ok(Self { data_context })
    }
}

pub type AudioDataContext = rmf_core::audio::AudioDataContext<
    AudioData<u8>,
    AudioData<i16>,
    AudioData<i32>,
    AudioData<i64>,
    AudioData<f32>,
    AudioData<f64>,
>;

pub struct AudioDataContextBuilder;

impl AudioDataContextBuilder {
    pub fn try_new(av_frame: AVFrame) -> Result<AudioDataContext> {
        match av_frame.format {
            AV_SAMPLE_FMT_NONE => Ok(AudioDataContext::None),
            AV_SAMPLE_FMT_U8 | AV_SAMPLE_FMT_U8P => {
                Ok(AudioDataContext::U8(AudioData::<u8>::new(av_frame)))
            }
            AV_SAMPLE_FMT_S16 | AV_SAMPLE_FMT_S16P => {
                Ok(AudioDataContext::I16(AudioData::<i16>::new(av_frame)))
            }
            AV_SAMPLE_FMT_S32 | AV_SAMPLE_FMT_S32P => {
                Ok(AudioDataContext::I32(AudioData::<i32>::new(av_frame)))
            }
            AV_SAMPLE_FMT_S64 | AV_SAMPLE_FMT_S64P => {
                Ok(AudioDataContext::I64(AudioData::<i64>::new(av_frame)))
            }
            AV_SAMPLE_FMT_FLT | AV_SAMPLE_FMT_FLTP => {
                Ok(AudioDataContext::F32(AudioData::<f32>::new(av_frame)))
            }
            AV_SAMPLE_FMT_DBL | AV_SAMPLE_FMT_DBLP => {
                Ok(AudioDataContext::F64(AudioData::<f64>::new(av_frame)))
            }
            _ => Err(Error::new_audio(anyhow!("Can't convert av frame"))),
        }
    }
}

#[derive(Clone)]
pub struct AudioData<T: Clone> {
    data: Vec<Vec<T>>,
    _phantom: PhantomData<T>,
}

impl<T: Clone> AudioData<T> {
    pub(crate) fn new(audio_av_frame: AVFrame) -> Self {
        let mut data = Vec::with_capacity(audio_av_frame.ch_layout().nb_channels as _);
        for i in 0..audio_av_frame.ch_layout().nb_channels as _ {
            let len = audio_av_frame.linesize[0] as usize;
            let ch_data = unsafe {
                Vec::from_raw_parts(*audio_av_frame.extended_data.add(i) as *mut T, len, len)
            };
            data.push(ch_data);
        }
        Self {
            data,
            _phantom: PhantomData::<T>,
        }
    }
}

#[delegate_implements]
impl<T: Clone> rmf_core::audio::AudioData for AudioData<T> {
    type Item = T;
    fn channels_len(&self) -> usize {
        self.data.len()
    }
    fn get_channel_line(&self, index: usize) -> Option<&[T]> {
        if index < self.channels_len() {
            Some(self.data[index].as_slice())
        } else {
            None
        }
    }
}
