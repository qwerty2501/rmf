use derive_new::new;

#[derive(thiserror::Error, Debug, new)]
pub enum Error {
    #[error("image error {0}")]
    Image(anyhow::Error),
    #[error("audio error {0}")]
    Audio(anyhow::Error),
    #[error("input error {0}")]
    Input(anyhow::Error),
    #[error("end of file")]
    Eof,
}
pub type Result<T> = std::result::Result<T, Error>;
