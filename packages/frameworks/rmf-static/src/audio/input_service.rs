use rmf_core::audio::AudioInputService;

use crate::{Audio, ffmpeg::AVFormatAudioInputService};

pub struct DefaultAudioInputService;

impl DefaultAudioInputService {
    pub fn provide_service(
        source: rmf_core::InputSource,
    ) -> rmf_core::Result<Box<dyn AudioInputService<Item = Audio>>> {
        Ok(Box::new(AVFormatAudioInputService::new(source)))
    }
}
