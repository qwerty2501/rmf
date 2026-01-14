use dyn_clone::DynClone;

use crate::{Content, InnerContent, Result, Timestamp};

pub trait Audio: InnerContent + Clone {
    type U8Data: AudioData<Item = u8>;
    type I16Data: AudioData<Item = i16>;
    type I32Data: AudioData<Item = i32>;
    type I64Data: AudioData<Item = i64>;
    type F32Data: AudioData<Item = f32>;
    type F64Data: AudioData<Item = f64>;

    #[inline]
    fn format(&self) -> AudioFormat {
        match self.data() {
            AudioDataContext::None => AudioFormat::None,
            AudioDataContext::U8(_) => AudioFormat::U8,
            AudioDataContext::I16(_) => AudioFormat::I16,
            AudioDataContext::I32(_) => AudioFormat::I32,
            AudioDataContext::I64(_) => AudioFormat::I64,
            AudioDataContext::F32(_) => AudioFormat::F32,
            AudioDataContext::F64(_) => AudioFormat::F64,
        }
    }

    #[allow(clippy::type_complexity)]
    fn data(
        &self,
    ) -> &AudioDataContext<
        Self::U8Data,
        Self::I16Data,
        Self::I32Data,
        Self::I64Data,
        Self::F32Data,
        Self::F64Data,
    >;
}

pub trait AudioConstructor {
    type U8Data: AudioData<Item = u8>;
    type I16Data: AudioData<Item = i16>;
    type I32Data: AudioData<Item = i32>;
    type I64Data: AudioData<Item = i64>;
    type F32Data: AudioData<Item = f32>;
    type F64Data: AudioData<Item = f64>;
    type Audio: Audio;

    #[allow(clippy::type_complexity)]
    fn tyr_new(
        data_context: AudioDataContext<
            Self::U8Data,
            Self::I16Data,
            Self::I32Data,
            Self::I64Data,
            Self::F32Data,
            Self::F64Data,
        >,
    ) -> Result<Self::Audio>;
}
pub trait AudioContentCursor {
    type Item: Audio;
    fn read(&mut self) -> Result<Option<Content<Self::Item>>>;
    fn seek(&mut self, timestamp: Timestamp) -> Result<()>;
}

pub trait AudioInput: DynClone {
    type Item: Audio;
    type ContentCursor: AudioContentCursor;
    fn duration(&self) -> Timestamp;
    fn fps(&self) -> f64;
    fn sample_rate(&self) -> u32;
    fn cursor(&self) -> Result<Self::ContentCursor>;
}
dyn_clone::clone_trait_object!(<I,C> AudioInput<Item = I,ContentCursor=C> where I:Audio ,C:AudioContentCursor);

#[repr(C)]
pub enum AudioFormat {
    None = 0,
    U8 = 1,
    I16 = 2,
    I32 = 3,
    I64 = 4,
    F32 = 5,
    F64 = 6,
}

#[derive(Clone)]
pub enum AudioDataContext<U8B, I16B, I32B, I64B, F32B, F64B> {
    None,
    U8(U8B),
    I16(I16B),
    I32(I32B),
    I64(I64B),
    F32(F32B),
    F64(F64B),
}

pub trait AudioData: Clone {
    type Item;
    fn get_channel_line(&self, index: usize) -> Option<&[Self::Item]>;
    fn channels_len(&self) -> usize;
}
