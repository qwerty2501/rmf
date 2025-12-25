#[inline]
pub fn calculate_frame_samples(fps: f64, sample_rate: u32, position: isize) -> isize {
    calculate_samples_to_position(fps, sample_rate, position + 1)
        - calculate_samples_to_position(fps, sample_rate, position)
}
#[inline]
pub fn calculate_samples_to_position(fps: f64, sample_rate: u32, position: isize) -> isize {
    if fps != 0.0 {
        (position as f64 * sample_rate as f64 / fps + (if position < 0 { -0.5 } else { 0.5 }))
            as isize
    } else {
        0
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
        #[case] fps: f64,
        #[case] sample_rate: u32,
        #[case] position: isize,
        #[case] expected: isize,
    ) {
        assert_eq!(
            expected,
            calculate_frame_samples(fps, sample_rate, position)
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
        #[case] fps: f64,
        #[case] sample_rate: u32,
        #[case] position: isize,
        #[case] expected: isize,
    ) {
        assert_eq!(
            expected,
            calculate_samples_to_position(fps, sample_rate, position)
        )
    }
}
