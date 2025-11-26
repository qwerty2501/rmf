#[inline]
pub(crate) fn calculate_frame_samples(fps: f32, sample_rate: u32, position: isize) -> isize {
    calculate_samples_to_position(fps, sample_rate, position + 1)
        - calculate_samples_to_position(fps, sample_rate, position)
}
#[inline]
pub(crate) fn calculate_samples_to_position(fps: f32, sample_rate: u32, position: isize) -> isize {
    if fps != 0.0 {
        (position as f64 * sample_rate as f64 / fps as f64
            + (if position < 0 { -0.5 } else { 0.5 })) as isize
    } else {
        0
    }
}
