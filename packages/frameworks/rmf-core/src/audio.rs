pub trait Audio: Clone {
    type U8Data: AudioData<u8>;
    type I16Data: AudioData<i16>;
    type I32Data: AudioData<i32>;
    type I64Data: AudioData<i64>;
    type F32Data: AudioData<f32>;
    type F64Data: AudioData<f64>;

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
    #[allow(clippy::type_complexity)]
    fn new(
        data_context: AudioDataContext<
            Self::U8Data,
            Self::I16Data,
            Self::I32Data,
            Self::I64Data,
            Self::F32Data,
            Self::F64Data,
        >,
    ) -> Self;
    fn calculate_frame_samples(fps: f32, sample_rate: u32, position: isize) -> isize;
    fn calculate_samples_to_position(fps: f32, sample_rate: u32, position: isize) -> isize;
}

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
pub enum AudioDataContext<
    U8B: AudioData<u8>,
    I16B: AudioData<i16>,
    I32B: AudioData<i32>,
    I64B: AudioData<i64>,
    F32B: AudioData<f32>,
    F64B: AudioData<f64>,
> {
    None,
    U8(U8B),
    I16(I16B),
    I32(I32B),
    I64(I64B),
    F32(F32B),
    F64(F64B),
}

pub trait AudioData<T>: Clone {
    fn get_channel_line(&self, index: usize) -> Option<&[T]>;
    fn channels_len(&self) -> usize;
}
