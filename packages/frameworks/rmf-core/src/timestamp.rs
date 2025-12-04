#[derive(Clone, Copy)]
pub struct Timestamp {
    raw_micro_seconds: i64,
}

impl Timestamp {
    pub const fn micro_seconds(&self) -> i64 {
        self.raw_micro_seconds
    }
    pub const fn from_micro_seconds(micro_seconds: i64) -> Self {
        Self {
            raw_micro_seconds: micro_seconds,
        }
    }
}
