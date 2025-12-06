#[cfg(feature = "use_opencv")]
pub mod opencv_image;

mod input_service;

#[cfg(feature = "opencv_image_as_default")]
pub use opencv_image::*;

pub use input_service::*;
