use iced::futures::SinkExt;
use iced::futures::channel::mpsc;
use iced::widget::{column, container, image};
use iced::{Element, Length, Settings, Subscription, Theme, executor};
use rmf_host::InputSource;
use rmf_host::image::Image;
use rmf_host::service::{ContentCursorTrait, ContentStreamServiceTrait};
use rmf_host::video::VideoInputService;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

// アプリケーションの状態
pub struct VideoPlayer {
    frame_handle: Option<image::Handle>, // Icedで表示する画像ハンドル
    receiver: Option<mpsc::Receiver<Image>>, // フレーム受信用チャンネル
}

#[derive(Clone)]
pub enum Message {
    FrameReceived(Arc<Image>), // 新しいフレームが届いた
    Tick,                      // タイマーイベント（フレームレート制御用）
}

impl VideoPlayer {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let (sender, receiver) = mpsc::channel(10);
        let path = path.as_ref().to_path_buf();

        // 動画デコードを別スレッド（タスク）で開始
        tokio::spawn(decode_video_loop(path, sender));

        Self {
            frame_handle: None,
            receiver: Some(receiver),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::FrameReceived(image) => {
                let raw_pixels = image.data_bytes();
                let size = image.size();

                let handle = image::Handle::from_rgba(size.width, size.height, raw_pixels);
                self.frame_handle = Some(handle);
            }
            Message::Tick => {}
        }
    }

    // 画面の描画
    pub fn view(&self) -> Element<'_, Message> {
        let content = if let Some(handle) = &self.frame_handle {
            image(handle.clone())
                .width(Length::Fill)
                .height(Length::Fill)
        } else {
            image(image::Handle::from_rgba(100, 100, vec![0; 100 * 100 * 4]))
        };

        container(column![content])
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    // 非同期イベントの購読
    fn subscription(&self) -> Subscription<Message> {
        // ここでチャネルを監視し、データが来たらMessage::FrameReceivedを発行する
        // ※ 実際には `iced::subscription::unfold` などを使用して、
        // self.receiverからデータを取り出し続ける実装にします。
        struct Worker;

        // 簡易的な実装: 実際にはチャネルをSubscriptionに変換するロジックが必要
        // 概念としては「デコードスレッドからの入力をリッスンする」
        Subscription::none()
    }
}

async fn decode_video_loop(path: PathBuf, sender: mpsc::Sender<Image>) {
    if let Err(err) = inner_decode_loop(path, sender).await {
        eprintln!("{err}");
    }
}

async fn inner_decode_loop(path: PathBuf, mut sender: mpsc::Sender<Image>) -> anyhow::Result<()> {
    let input_source = InputSource::new_path(path);
    let input_service = VideoInputService::try_new(input_source)?;
    let mut cursor = input_service.cursor()?;

    while let Some(content) = cursor.read()? {
        sender.send(content.item().clone()).await?;
        sleep(Duration::from_micros(
            content.duration().as_microseconds() as _
        ))
        .await;
    }

    Ok(())
}
