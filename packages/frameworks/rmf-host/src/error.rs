use derive_new::new;

#[derive(thiserror::Error, Debug, new)]
pub enum Error {
    #[error("{0}")]
    Core(rmf_core::Error),
}

impl From<rmf_core::Error> for Error {
    fn from(value: rmf_core::Error) -> Self {
        Self::new_core(value)
    }
}
