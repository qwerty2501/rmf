use derive_new::new;

#[derive(thiserror::Error, Debug, new)]
pub enum Error {
    #[error("image error {0}")]
    Image(Box<dyn std::error::Error>),
}
pub type Result<T> = std::result::Result<T, Error>;
