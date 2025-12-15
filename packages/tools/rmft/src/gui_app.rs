use std::path::PathBuf;

use anyhow::Result;
use iced::{
    Renderer, mouse,
    widget::{
        canvas::{self, Frame},
        image::Handle,
    },
};
use rmf_host::{InputSource, video::VideoInputService};

pub struct App {
    input_service: VideoInputService,
    cache: canvas::Cache,
}

impl App {
    pub fn new(path: PathBuf) -> Result<App> {
        Ok(Self {
            input_service: VideoInputService::try_new(InputSource::new_path(path))?,
            cache: canvas::Cache::default(),
        })
    }
}

impl<Message> canvas::Program<Message> for App {
    type State = ();
    fn update(
        &self,
        _state: &mut Self::State,
        _event: &iced::Event,
        _bounds: iced::Rectangle,
        _cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        unimplemented!()
    }
    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &iced::Theme,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let ret = self.cache.draw(renderer, bounds.size(), |frame| {
            let image = iced::widget::Image::new(Handle::from_bytes(unimplemented!()));
            frame.draw_image(bounds, image.into());
        });
        vec![ret]
    }
}
