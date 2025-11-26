use rmf_macros::delegate_implements;

use crate::audio::utils;
#[derive(Clone)]
pub struct Audio {
    data_context: AudioDataContext,
}

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

    #[inline]
    fn new(data_context: AudioDataContext) -> Self {
        Self { data_context }
    }
    #[inline]
    fn calculate_frame_samples(fps: f32, sample_rate: u32, position: isize) -> isize {
        utils::calculate_frame_samples(fps, sample_rate, position)
    }
    #[inline]
    fn calculate_samples_to_position(fps: f32, sample_rate: u32, position: isize) -> isize {
        utils::calculate_samples_to_position(fps, sample_rate, position)
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

#[derive(Clone)]
pub struct AudioData<T: Clone> {
    data: Vec<Vec<T>>,
}

impl<T: Clone> AudioData<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }
}

#[delegate_implements]
impl<T: Clone> rmf_core::audio::AudioData for AudioData<T> {
    type Item = T;
    fn channels_len(&self) -> usize {
        self.data.len()
    }
    fn get_channel_line(&self, index: usize) -> Option<&[T]> {
        self.data.get(index).map(|v| v.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(0.0, 0, 0, 0)]
    fn calculate_frame_samples_works(
        #[case] fps: f32,
        #[case] sample_rate: u32,
        #[case] position: isize,
        #[case] expected: isize,
    ) {
        assert_eq!(
            expected,
            Audio::calculate_frame_samples(fps, sample_rate, position)
        )
    }

    #[rstest]
    #[case(0.0, 0, 0, 0)]
    #[case(0.0, 30, 22, 0)]
    #[case(0.0, 100, 121, 0)]
    #[case(0.0, 786432, 121, 0)]
    #[case(0.0, 786432, -332, 0)]
    #[case(30.0, 786432, 121, 3171942)]
    #[case(30.0, 786432, -121, -3171942)]
    fn calculate_samples_to_position_works(
        #[case] fps: f32,
        #[case] sample_rate: u32,
        #[case] position: isize,
        #[case] expected: isize,
    ) {
        assert_eq!(
            expected,
            Audio::calculate_samples_to_position(fps, sample_rate, position)
        )
    }
}
