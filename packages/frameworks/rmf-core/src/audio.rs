pub trait AudioTrait {}

pub struct Audio {}

impl AudioTrait for Audio {}

impl Audio {
    pub fn calculate_frame_samples(fps: f32, sample_rate: u32, position: isize) -> isize {
        Self::calculate_samples_to_position(fps, sample_rate, position + 1)
            - Self::calculate_samples_to_position(fps, sample_rate, position)
    }

    pub fn calculate_samples_to_position(fps: f32, sample_rate: u32, position: isize) -> isize {
        if fps != 0.0 {
            (position as f64 * sample_rate as f64 / fps as f64
                + (if position < 0 { -0.5 } else { 0.5 })) as isize
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

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
