#[derive(Clone, Copy)]
pub struct Timestamp {
    raw_micro_seconds: i64,
}

impl Timestamp {
    pub fn micro_seconds(&self) -> i64 {
        self.raw_micro_seconds
    }
    pub fn from_micro_seconds(micro_seconds: i64) -> Self {
        Self {
            raw_micro_seconds: micro_seconds,
        }
    }
}
