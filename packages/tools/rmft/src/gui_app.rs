use eframe::egui::mutex::Mutex;
use eframe::egui::{self, Color32, ColorImage, TextureHandle, TextureOptions};
use rmf_host::InputSource;
use rmf_host::image::Image;
use rmf_host::service::{ContentCursorTrait, ContentStreamServiceTrait};
use rmf_host::video::VideoInputService;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

// アプリケーションの状態
pub struct VideoPlayer {
    texture: Arc<Mutex<egui::TextureHandle>>,
    path: PathBuf,
}

impl eframe::App for VideoPlayer {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let texture = self.texture.lock();
            let size = texture.size_vec2();
            let id = texture.id();
            let sized_texture = egui::load::SizedTexture::new(id, size);
            ui.add(egui::Image::new(sized_texture).fit_to_exact_size(size))
        });
    }
}

#[derive(Clone)]
pub enum Message {
    FrameReceived(Arc<ColorImage>),
}

impl VideoPlayer {
    pub fn new(path: impl AsRef<Path>, cc: &eframe::CreationContext<'_>) -> Self {
        let path = path.as_ref().to_path_buf();
        let (sender, receiver) = mpsc::channel(1);
        tokio::spawn(decode_video_loop(path.clone(), sender));

        let s = Self {
            texture: Arc::new(Mutex::new(cc.egui_ctx.load_texture(
                "video",
                egui::ColorImage::default(),
                egui::TextureOptions::NEAREST,
            ))),
            path,
        };

        tokio::spawn(Self::receive_loop(s.texture.clone(), receiver));
        s
    }
    async fn receive_loop(
        texture: Arc<Mutex<egui::TextureHandle>>,
        mut receiver: mpsc::Receiver<Message>,
    ) {
        if let Some(message) = receiver.recv().await {
            Self::receive(texture, message);
        }
    }
    fn receive(texture: Arc<Mutex<egui::TextureHandle>>, message: Message) {
        match message {
            Message::FrameReceived(color_image) => {
                texture.lock().set(color_image, TextureOptions::default());
            }
        }
    }
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
        let image = content.item();
        let pixels = image
            .data_bytes()
            .chunks_exact(4)
            .map(|data| Color32::from_rgba_premultiplied(data[0], data[1], data[2], data[3]))
            .collect();
        let color_image =
            ColorImage::new([image.size().width as _, image.size().height as _], pixels);
        sender
            .send(Message::FrameReceived(Arc::new(color_image)))
            .await?;
        let sleep_duration = Duration::from_micros(content.duration().as_microseconds() as _);
        sleep(sleep_duration).await;
    }

    Ok(())
}
