use iced::futures::channel::mpsc;
use iced::futures::{SinkExt, Stream};
use iced::widget::canvas::{self, Cache, Frame};
use iced::widget::{Canvas, column, container, text};
use iced::{Border, Color, Element, Length, Point, Renderer, Subscription, Theme, mouse, stream};
use rmf_host::InputSource;
use rmf_host::image::Image;
use rmf_host::service::{ContentCursorTrait, ContentStreamServiceTrait};
use rmf_host::video::VideoInputService;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

// アプリケーションの状態
pub struct VideoPlayer {
    frame_cache: Cache,
    frame_image: Option<Arc<Image>>,
    path: PathBuf,
}

#[derive(Clone)]
pub enum Message {
    FrameReceived(Arc<Image>), // 新しいフレームが届いた
}

impl VideoPlayer {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();

        Self {
            frame_cache: Cache::default(),
            frame_image: None,
            path,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::FrameReceived(image) => {
                self.frame_image = Some(image);
                self.frame_cache.clear();
            }
        }
    }

    // 画面の描画
    pub fn view(&self) -> Element<'_, Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    // 非同期イベントの購読
    pub fn subscription(&self) -> Subscription<Message> {
        let path: PathBuf = self.path.clone();
        Subscription::run_with(path, |path| decode_video_loop_worker(path.clone()))
    }
}
impl<Message> canvas::Program<Message> for VideoPlayer {
    type State = ();
    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let geometry = self.frame_cache.draw(renderer, bounds.size(), |frame| {
            if let Some(frame_image) = &self.frame_image {
                let size = frame_image.size();
                let im = iced::widget::canvas::Image::new(iced::widget::image::Handle::from_rgba(
                    size.width,
                    size.height,
                    frame_image.data_bytes(),
                ));
                frame.draw_image(bounds, im);
            }
        });
        vec![geometry]
    }
}

fn decode_video_loop_worker(path: PathBuf) -> impl Stream<Item = Message> {
    stream::channel(100, move |output| decode_video_loop(path, output))
}

async fn decode_video_loop(path: PathBuf, sender: mpsc::Sender<Message>) {
    if let Err(err) = inner_decode_loop(path, sender).await {
        eprintln!("{err}");
    }
}

async fn inner_decode_loop(path: PathBuf, mut sender: mpsc::Sender<Message>) -> anyhow::Result<()> {
    let input_source = InputSource::new_path(path.clone());
    let input_service = VideoInputService::try_new(input_source)?;
    let mut cursor = input_service.cursor()?;

    while let Some(content) = cursor.read()? {
        sender
            .send(Message::FrameReceived(Arc::new(content.item().clone())))
            .await?;
        let sleep_duration = Duration::from_micros(content.duration().as_microseconds() as _);
        sleep(sleep_duration).await;
    }

    Ok(())
}
