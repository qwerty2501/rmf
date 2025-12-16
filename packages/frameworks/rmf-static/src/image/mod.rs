#[cfg(feature = "use_photon")]
pub mod photon_image;

#[cfg(feature = "photon_image_as_default")]
pub use photon_image::*;
