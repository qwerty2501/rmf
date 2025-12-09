use std::{collections::VecDeque, sync::Arc};

use crate::service::{ContextImageContentStreamService, ImageContentStreamServiceTrait};

pub struct ImageTracks {
    tracks: VecDeque<ContextImageContentStreamService>,
}
