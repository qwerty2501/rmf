use std::path::PathBuf;

use anyhow::Result;
use iced::{
    Subscription, mouse,
    time::{self, milliseconds},
    widget::shader,
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
}

pub enum Message {
    Tick(time::Instant),
}

impl App {
    pub fn new(path: PathBuf) -> Result<App> {
        Ok(Self {
            cursor: VideoInputService::try_new(InputSource::new_path(path))?.cursor()?,
            current_content: None,
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

impl<Message> shader::Program<Message> for App {
    type State = ();
    type Primitive = Primitive;
    fn draw(
        &self,
        state: &Self::State,
        cursor: mouse::Cursor,
        bounds: iced::Rectangle,
    ) -> Self::Primitive {
        unimplemented!()
    }
}
#[derive(Debug)]
pub struct Primitive {}

impl shader::Primitive for Primitive {
    type Pipeline = Pipeline;
    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &iced::wgpu::Device,
        queue: &iced::wgpu::Queue,
        bounds: &iced::Rectangle,
        viewport: &shader::Viewport,
    ) {
        unimplemented!()
    }
    fn draw(
        &self,
        _pipeline: &Self::Pipeline,
        _render_pass: &mut iced::wgpu::RenderPass<'_>,
    ) -> bool {
        unimplemented!()
    }
}

pub struct Pipeline {}

impl iced::widget::shader::Pipeline for Pipeline {
    fn new(
        device: &iced::wgpu::Device,
        queue: &iced::wgpu::Queue,
        format: iced::wgpu::TextureFormat,
    ) -> Self {
        unimplemented!()
    }
}
