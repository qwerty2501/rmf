use derive_new::new;

#[derive(thiserror::Error, Debug, new)]
pub enum Error {
    #[error("image error {0}")]
    Image(Box<dyn std::error::Error>),
    #[error("audio error {0}")]
    Audio(Box<dyn std::error::Error>),
    #[error("input error {0}")]
    Input(Box<dyn std::error::Error>),
    #[error("end of file")]
    Eof,
}
pub type Result<T> = std::result::Result<T, Error>;
