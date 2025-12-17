use std::env;

use crate::gui_app::VideoPlayer;

mod gui_app;

fn main() -> iced::Result {
    let args: Vec<String> = env::args().collect();
    let video_path = args[0].to_string();
    let app = iced::application(
        move || VideoPlayer::new(video_path.clone()),
        VideoPlayer::update,
        VideoPlayer::view,
    );
    app.run()
}
