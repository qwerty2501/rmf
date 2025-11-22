pub trait Audio {
    fn calculate_frame_samples(fps: f32, sample_rate: u32, position: isize) -> isize;
    fn calculate_samples_to_position(fps: f32, sample_rate: u32, position: isize) -> isize;
}
