#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Timestamp {
    raw_micro_seconds: i64,
}

const SECONDS_RATIO: i64 = 1000 * MILLISECONDS_RATIO;

const MILLISECONDS_RATIO: i64 = 1000;

impl Timestamp {
    pub const fn as_seconds_float64(&self) -> f64 {
        self.raw_micro_seconds as f64 / SECONDS_RATIO as f64
    }
    pub const fn as_seconds_float32(&self) -> f32 {
        self.raw_micro_seconds as f32 / SECONDS_RATIO as f32
    }
    pub const fn as_seconds(&self) -> i64 {
        self.raw_micro_seconds / SECONDS_RATIO
    }
    pub const fn as_milliseconds(&self) -> i64 {
        self.raw_micro_seconds / MILLISECONDS_RATIO
    }
    pub const fn as_microseconds(&self) -> i64 {
        self.raw_micro_seconds
    }
    pub const fn from_microseconds(micro_seconds: i64) -> Self {
        Self {
            raw_micro_seconds: micro_seconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(5000,Timestamp { raw_micro_seconds: 5000 })]
    #[case(-12000,Timestamp { raw_micro_seconds: -12000 })]
    fn from_microseconds_works(#[case] micro_seconds: i64, #[case] expected: Timestamp) {
        assert_eq!(Timestamp::from_microseconds(micro_seconds), expected);
    }
    #[rstest]
    #[case(Timestamp { raw_micro_seconds: -12000 },-12000)]
    #[case(Timestamp { raw_micro_seconds: 12000 },12000)]
    fn as_microseconds_works(#[case] micro_seconds: Timestamp, #[case] expected: i64) {
        assert_eq!(micro_seconds.as_microseconds(), expected);
    }
    #[rstest]
    #[case(Timestamp { raw_micro_seconds: -12000 },-12)]
    #[case(Timestamp { raw_micro_seconds: 12000 },12)]
    #[case(Timestamp { raw_micro_seconds: 500 },0)]
    #[case(Timestamp { raw_micro_seconds: 500 },0)]
    fn as_milliseconds_works(#[case] micro_seconds: Timestamp, #[case] expected: i64) {
        assert_eq!(micro_seconds.as_milliseconds(), expected);
    }
    #[rstest]
    #[case(Timestamp { raw_micro_seconds: -12000000 },-12)]
    #[case(Timestamp { raw_micro_seconds: 12000000 },12)]
    #[case(Timestamp { raw_micro_seconds: -12000 },0)]
    #[case(Timestamp { raw_micro_seconds: 12000 },0)]
    fn as_seconds_works(#[case] micro_seconds: Timestamp, #[case] expected: i64) {
        assert_eq!(micro_seconds.as_seconds(), expected);
    }

    #[rstest]
    #[case(Timestamp { raw_micro_seconds: -12000000 },-12.0)]
    #[case(Timestamp { raw_micro_seconds: 12000000 },12.0)]
    #[case(Timestamp { raw_micro_seconds: -12000 },-0.012)]
    #[case(Timestamp { raw_micro_seconds: 12000 },0.012)]
    fn as_seconds_float64_works(#[case] micro_seconds: Timestamp, #[case] expected: f64) {
        assert_eq!(micro_seconds.as_seconds_float64(), expected);
    }

    #[rstest]
    #[case(Timestamp { raw_micro_seconds: -12000000 },-12.0)]
    #[case(Timestamp { raw_micro_seconds: 12000000 },12.0)]
    #[case(Timestamp { raw_micro_seconds: -12000 },-0.012)]
    #[case(Timestamp { raw_micro_seconds: 12000 },0.012)]
    fn as_seconds_float32_works(#[case] micro_seconds: Timestamp, #[case] expected: f32) {
        assert_eq!(micro_seconds.as_seconds_float32(), expected);
    }
}
