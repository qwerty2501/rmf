mod audio_app;
mod gui_app;

use std::env;

use crate::gui_app::VideoPlayer;
use eframe::egui;

#[tokio::main]
async fn main() -> eframe::Result {
    let path = &env::args().collect::<Vec<_>>()[1];
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1040.0, 880.0]),
        ..Default::default()
    };
    eframe::run_native(
        "video editor",
        options,
        Box::new(|cc| Ok(Box::new(VideoPlayer::new(path, cc)))),
    )
}
