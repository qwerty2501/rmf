mod audio_app;
mod gui_app;

use std::{env, path::PathBuf};

use crate::{audio_app::AudioPlayer, gui_app::VideoPlayer};
use eframe::egui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = &env::args().collect::<Vec<_>>()[1];
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1040.0, 880.0]),
        ..Default::default()
    };
    let audio_task = tokio::spawn(AudioPlayer::play(PathBuf::from(&path)));
    let result = eframe::run_native(
        "video editor",
        options,
        Box::new(|cc| Ok(Box::new(VideoPlayer::new(path, cc)))),
    );
    audio_task.await?;
    result.map_err(|e| anyhow::Error::msg(e.to_string()))
}
