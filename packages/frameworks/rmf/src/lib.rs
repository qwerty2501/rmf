mod audio;
pub use rmf_core::{Consumer, Producer, Service};

#[cfg(feature = "static_link")]
pub use rmf_static as rmf_impl;

pub use audio::*;
