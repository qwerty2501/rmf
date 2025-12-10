pub mod audio;
pub mod image;
pub mod video;
pub use rmf_core::OutputService;

#[cfg(feature = "static_link")]
use rmf_static as rmf_impl;
