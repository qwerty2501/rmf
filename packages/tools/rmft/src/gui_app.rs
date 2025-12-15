use std::path::PathBuf;

use anyhow::Result;
use iced::{
    Renderer, Subscription, mouse,
    time::{self, milliseconds},
    widget::{
        canvas::{self, Frame},
        image::Handle,
    },
};
use rmf_host::{
    Content, InputSource, Timestamp,
    image::Image,
    service::{ContentCursorTrait, ContentStreamServiceTrait},
    video::{VideoInputContentCursor, VideoInputService},
};

pub struct App {
    cursor: VideoInputContentCursor,
    current_content: Option<Content<Image>>,
    cache: canvas::Cache,
}

pub enum Message {
    Tick(time::Instant),
}

impl App {
    pub fn new(path: PathBuf) -> Result<App> {
        Ok(Self {
            cursor: VideoInputService::try_new(InputSource::new_path(path))?.cursor()?,
            current_content: None,
            cache: canvas::Cache::default(),
        })
    }
    fn read_content(&mut self) -> Option<Content<Image>> {
        match self.cursor.read() {
            Ok(content) => content,
            Err(err) => {
                eprintln!("{err}");
                None
            }
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Tick(instant) => {
                let elapsed = Timestamp::from_microseconds(instant.elapsed().as_micros() as i64);
                let next = if let Some(current_content) = &self.current_content {
                    if current_content.offset() + current_content.duration() <= elapsed {
                        self.read_content()
                    } else {
                        return;
                    }
                } else {
                    self.read_content()
                };
                self.current_content = next;
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(milliseconds(1)).map(|v| Message::Tick(v))
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
