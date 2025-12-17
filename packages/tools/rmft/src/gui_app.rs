use iced::futures::channel::mpsc;
use iced::futures::{SinkExt, Stream};
use iced::widget::{column, container, image, text};
use iced::{Border, Color, Element, Length, Subscription, Theme, stream};
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
    frame_handle: Option<image::Handle>, // Icedで表示する画像ハンドル
    path: PathBuf,
}

#[derive(Clone)]
pub enum Message {
    FrameReceived(Arc<Image>), // 新しいフレームが届いた
    Tick,                      // タイマーイベント（フレームレート制御用）
}

impl VideoPlayer {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();

        Self {
            frame_handle: None,
            path,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::FrameReceived(image) => {
                let mut raw_pixels = image.data_bytes();
                let size = image.size();
                let expected_size = (size.width * size.height * 4) as usize;
                if raw_pixels.len() != expected_size {
                    // もしここを通るなら、FFmpeg側の変換（linesize）にパディングが残っています
                    eprintln!(
                        "Size mismatch! expected: {}, got: {}",
                        expected_size,
                        raw_pixels.len()
                    );
                    return;
                }
                for i in 0..100 {
                    raw_pixels[i * 4 + 1] = 255; // G
                    raw_pixels[i * 4 + 3] = 255; // A
                }
                let handle = image::Handle::from_rgba(size.width, size.height, raw_pixels);
                self.frame_handle = Some(handle);
            }
            Message::Tick => {}
        }
    }

    // 画面の描画
    pub fn view(&self) -> Element<'_, Message> {
        if let Some(handle) = &self.frame_handle {
            let content = container(image(handle))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_theme: &Theme| {
                    container::Style {
                        // Border::with_color(color, width) または border::all を使用
                        border: Border {
                            color: Color::BLACK,
                            width: 2.0,
                            radius: 0.0.into(),
                        },
                        ..Default::default()
                    }
                });
            column![text("Video View").size(30), content]
                .padding(20)
                .spacing(10)
                .into()
        } else {
            container(text("Wait..."))
                .width(640)
                .height(360)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(iced::Color::from_rgb(1.0, 0.0, 0.0).into()),
                    ..Default::default()
                })
                .into()
        }
    }

    // 非同期イベントの購読
    pub fn subscription(&self) -> Subscription<Message> {
        let path: PathBuf = self.path.clone();
        Subscription::run_with(path, |path| decode_video_loop_worker(path.clone()))
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
