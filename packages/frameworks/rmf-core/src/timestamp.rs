use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    time::{Duration, SystemTime},
};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Default, PartialOrd)]
pub struct Timestamp {
    raw_microseconds: i64,
}

const SECONDS_RATIO: i64 = 1000 * MILLISECONDS_RATIO;

const MILLISECONDS_RATIO: i64 = 1000;

impl Timestamp {
    pub const fn as_seconds_float64(&self) -> f64 {
        self.raw_microseconds as f64 / SECONDS_RATIO as f64
    }
    pub const fn as_seconds_float32(&self) -> f32 {
        self.raw_microseconds as f32 / SECONDS_RATIO as f32
    }
    pub const fn as_seconds(&self) -> i64 {
        self.raw_microseconds / SECONDS_RATIO
    }
    pub const fn as_milliseconds(&self) -> i64 {
        self.raw_microseconds / MILLISECONDS_RATIO
    }
    pub const fn as_microseconds(&self) -> i64 {
        self.raw_microseconds
    }

    pub const fn from_milliseconds(milliseconds: i64) -> Self {
        Self {
            raw_microseconds: milliseconds * MILLISECONDS_RATIO,
        }
    }

    pub const fn from_microseconds(micro_seconds: i64) -> Self {
        Self {
            raw_microseconds: micro_seconds,
        }
    }
    pub const fn from_seconds(seconds: i64) -> Self {
        Self {
            raw_microseconds: (seconds * SECONDS_RATIO),
        }
    }
    pub const fn from_seconds_float64(seconds: f64) -> Self {
        Self {
            raw_microseconds: (seconds * SECONDS_RATIO as f64) as i64,
        }
    }
    pub const fn from_seconds_float32(seconds: f32) -> Self {
        Self {
            raw_microseconds: (seconds * SECONDS_RATIO as f32) as i64,
        }
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_seconds_float32())
    }
}

impl Add for Timestamp {
    type Output = Timestamp;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_microseconds(self.raw_microseconds.add(rhs.raw_microseconds))
    }
}

impl AddAssign for Timestamp {
    fn add_assign(&mut self, rhs: Self) {
        self.raw_microseconds.add_assign(rhs.raw_microseconds);
    }
}

impl Sub for Timestamp {
    type Output = Timestamp;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_microseconds(self.raw_microseconds.sub(rhs.raw_microseconds))
    }
}

impl SubAssign for Timestamp {
    fn sub_assign(&mut self, rhs: Self) {
        self.raw_microseconds.sub_assign(rhs.raw_microseconds);
    }
}

impl Div for Timestamp {
    type Output = Timestamp;
    fn div(self, rhs: Self) -> Self::Output {
        Self::from_microseconds(self.raw_microseconds.div(rhs.raw_microseconds))
    }
}

impl DivAssign for Timestamp {
    fn div_assign(&mut self, rhs: Self) {
        self.raw_microseconds.div_assign(rhs.raw_microseconds)
    }
}

impl Mul for Timestamp {
    type Output = Timestamp;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_microseconds(self.raw_microseconds.mul(rhs.raw_microseconds))
    }
}

impl MulAssign for Timestamp {
    fn mul_assign(&mut self, rhs: Self) {
        self.raw_microseconds.mul_assign(rhs.raw_microseconds);
    }
}

impl From<Duration> for Timestamp {
    fn from(value: Duration) -> Self {
        Self::from_microseconds(value.as_micros() as _)
    }
}

impl From<Timestamp> for Duration {
    fn from(value: Timestamp) -> Self {
        Self::from_micros(value.as_microseconds() as _)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case(5000,Timestamp { raw_microseconds: 5000 })]
    #[case(-12000,Timestamp { raw_microseconds: -12000 })]
    fn from_microseconds_works(#[case] micro_seconds: i64, #[case] expected: Timestamp) {
        assert_eq!(Timestamp::from_microseconds(micro_seconds), expected);
    }
    #[rstest]
    #[case(5000,Timestamp { raw_microseconds: 5000000 })]
    #[case(-12000,Timestamp { raw_microseconds: -12000000 })]
    fn from_milliseconds_works(#[case] milliseconds: i64, #[case] expected: Timestamp) {
        assert_eq!(Timestamp::from_milliseconds(milliseconds), expected);
    }
    #[rstest]
    #[case(Timestamp { raw_microseconds: -12000 },-12000)]
    #[case(Timestamp { raw_microseconds: 12000 },12000)]
    fn as_microseconds_works(#[case] micro_seconds: Timestamp, #[case] expected: i64) {
        assert_eq!(micro_seconds.as_microseconds(), expected);
    }
    #[rstest]
    #[case(Timestamp { raw_microseconds: -12000 },-12)]
    #[case(Timestamp { raw_microseconds: 12000 },12)]
    #[case(Timestamp { raw_microseconds: 500 },0)]
    #[case(Timestamp { raw_microseconds: 500 },0)]
    fn as_milliseconds_works(#[case] micro_seconds: Timestamp, #[case] expected: i64) {
        assert_eq!(micro_seconds.as_milliseconds(), expected);
    }
    #[rstest]
    #[case(Timestamp { raw_microseconds: -12000000 },-12)]
    #[case(Timestamp { raw_microseconds: 12000000 },12)]
    #[case(Timestamp { raw_microseconds: -12000 },0)]
    #[case(Timestamp { raw_microseconds: 12000 },0)]
    fn as_seconds_works(#[case] micro_seconds: Timestamp, #[case] expected: i64) {
        assert_eq!(micro_seconds.as_seconds(), expected);
    }

    #[rstest]
    #[case(Timestamp { raw_microseconds: -12000000 },-12.0)]
    #[case(Timestamp { raw_microseconds: 12000000 },12.0)]
    #[case(Timestamp { raw_microseconds: -12000 },-0.012)]
    #[case(Timestamp { raw_microseconds: 12000 },0.012)]
    fn as_seconds_float64_works(#[case] micro_seconds: Timestamp, #[case] expected: f64) {
        assert_eq!(micro_seconds.as_seconds_float64(), expected);
    }

    #[rstest]
    #[case(Timestamp { raw_microseconds: -12000000 },-12.0)]
    #[case(Timestamp { raw_microseconds: 12000000 },12.0)]
    #[case(Timestamp { raw_microseconds: -12000 },-0.012)]
    #[case(Timestamp { raw_microseconds: 12000 },0.012)]
    fn as_seconds_float32_works(#[case] micro_seconds: Timestamp, #[case] expected: f32) {
        assert_eq!(micro_seconds.as_seconds_float32(), expected);
    }

    #[rstest]
    #[case(Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 0 })]
    #[case(Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: -24000000 })]
    #[case(Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 24000000 })]
    fn add_works(#[case] base: Timestamp, #[case] rhs: Timestamp, #[case] expected: Timestamp) {
        assert_eq!(base + rhs, expected)
    }
    #[rstest]
    #[case(Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 0 })]
    #[case(Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: -24000000 })]
    #[case(Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 24000000 })]
    fn add_assign_works(
        #[case] mut base: Timestamp,
        #[case] rhs: Timestamp,
        #[case] expected: Timestamp,
    ) {
        base += rhs;
        assert_eq!(base, expected)
    }

    #[rstest]
    #[case(Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: -24000000 })]
    #[case(Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: 0 })]
    #[case(Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 0 })]
    fn sub_works(#[case] base: Timestamp, #[case] rhs: Timestamp, #[case] expected: Timestamp) {
        assert_eq!(base - rhs, expected)
    }

    #[rstest]
    #[case(Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: -24000000 })]
    #[case(Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: 0 })]
    #[case(Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 0 })]
    fn sub_assign_works(
        #[case] mut base: Timestamp,
        #[case] rhs: Timestamp,
        #[case] expected: Timestamp,
    ) {
        base -= rhs;
        assert_eq!(base, expected)
    }

    #[rstest]
    #[case(Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: -1 })]
    #[case(Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: 1 })]
    #[case(Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 1 })]
    fn div_works(#[case] base: Timestamp, #[case] rhs: Timestamp, #[case] expected: Timestamp) {
        assert_eq!(base / rhs, expected)
    }
    #[rstest]
    #[case(Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: -1 })]
    #[case(Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: -12000000 },Timestamp { raw_microseconds: 1 })]
    #[case(Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 1 })]
    fn div_assign_works(
        #[case] mut base: Timestamp,
        #[case] rhs: Timestamp,
        #[case] expected: Timestamp,
    ) {
        base /= rhs;
        assert_eq!(base, expected)
    }

    #[rstest]
    #[case(Timestamp { raw_microseconds: -12 },Timestamp { raw_microseconds: 100 },Timestamp { raw_microseconds: -1200 })]
    #[case(Timestamp { raw_microseconds: -12 },Timestamp { raw_microseconds: 0 },Timestamp { raw_microseconds: 0 })]
    #[case(Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 1},Timestamp { raw_microseconds: 12000000 })]
    fn mul_works(#[case] base: Timestamp, #[case] rhs: Timestamp, #[case] expected: Timestamp) {
        assert_eq!(base * rhs, expected)
    }

    #[rstest]
    #[case(Timestamp { raw_microseconds: -12 },Timestamp { raw_microseconds: 100 },Timestamp { raw_microseconds: -1200 })]
    #[case(Timestamp { raw_microseconds: -12 },Timestamp { raw_microseconds: 0 },Timestamp { raw_microseconds: 0 })]
    #[case(Timestamp { raw_microseconds: 12000000 },Timestamp { raw_microseconds: 1},Timestamp { raw_microseconds: 12000000 })]
    fn mul_assign_works(
        #[case] mut base: Timestamp,
        #[case] rhs: Timestamp,
        #[case] expected: Timestamp,
    ) {
        base *= rhs;
        assert_eq!(base, expected)
    }

    #[rstest]
    #[case(Duration::from_secs(33),Timestamp { raw_microseconds: 33000000 })]
    fn from_duration_works(#[case] value: Duration, #[case] expected: Timestamp) {
        assert_eq!(Timestamp::from(value), expected)
    }
    #[rstest]
    fn scenario_works() {
        let value_secs = 550;
        let duration = Duration::from_secs(value_secs);
        let actual_timestamp = Timestamp::from(duration);
        let expected = Timestamp::from_microseconds(550000000);
        assert_eq!(expected, actual_timestamp);
        assert_eq!(expected.as_seconds(), value_secs as _);
    }
}
