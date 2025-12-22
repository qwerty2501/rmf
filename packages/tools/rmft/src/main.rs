use std::env;

use crate::gui_app::VideoPlayer;
use eframe::egui;

mod gui_app;

fn main() -> eframe::Result {
    let path = env::args().next().unwrap();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 880.0]),
        ..Default::default()
    };
    eframe::run_native(
        "video editor",
        options,
        Box::new(|cc| Ok(Box::new(VideoPlayer::new(path, cc)))),
    )
}
