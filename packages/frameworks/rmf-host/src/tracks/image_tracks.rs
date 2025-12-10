use std::collections::VecDeque;

use crate::opaque::OpaqueImageContentStreamService;

#[derive(Clone)]
pub struct ImageTracks {
    tracks: VecDeque<OpaqueImageContentStreamService>,
}
