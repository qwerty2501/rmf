use derive_new::new;
use rmf_host::InputSource;
use std::path::PathBuf;

#[derive(new)]
pub struct AudioPlayer {
    path: PathBuf,
}

impl AudioPlayer {
    pub fn play(&self) {
        decode_audio_loop(self.path.clone());
    }
}

async fn decode_audio_loop(path: PathBuf) {
    if let Err(err) = inner_decode_loop(path).await {
        eprintln!("{err}");
    }
}
async fn inner_decode_loop(path: PathBuf) -> anyhow::Result<()> {
    let input_source = InputSource::new_path(path.clone());

    Ok(())
}
