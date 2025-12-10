#[cfg(feature = "use_opencv")]
pub mod opencv_image;

#[cfg(feature = "opencv_image_as_default")]
pub use opencv_image::*;
