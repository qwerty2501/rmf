use eframe::egui::{self, Color32, ColorImage};
use rmf_host::InputSource;
use rmf_host::service::{ContentCursorTrait, ContentStreamServiceTrait};
use rmf_host::video::VideoInputService;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{self, Receiver};
use tokio::time::sleep;

// アプリケーションの状態
pub struct VideoPlayer {
    texture: egui::TextureHandle,
    receiver: Receiver<Message>,
}

impl eframe::App for VideoPlayer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut latest_frame = None;
        while let Ok(Message::FrameReceived(color_image)) = self.receiver.try_recv() {
            latest_frame = Some(color_image);
        }
        if let Some(latest_frame) = latest_frame {
            self.texture.set(latest_frame, Default::default());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let size = self.texture.size_vec2();
            let id = self.texture.id();
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
        let (sender, receiver) = mpsc::channel(100);
        let ctx = cc.egui_ctx.clone();
        tokio::spawn(decode_video_loop(path.clone(), sender, ctx));

        Self {
            texture: cc.egui_ctx.load_texture(
                "video",
                egui::ColorImage::default(),
                egui::TextureOptions::NEAREST,
            ),
            receiver,
        }
    }
}

async fn decode_video_loop(path: PathBuf, sender: mpsc::Sender<Message>, ctx: egui::Context) {
    if let Err(err) = inner_decode_loop(path, sender, ctx).await {
        eprintln!("{err}");
    }
}

async fn inner_decode_loop(
    path: PathBuf,
    sender: mpsc::Sender<Message>,
    ctx: egui::Context,
) -> anyhow::Result<()> {
    let input_source = InputSource::new_path(path.clone());
    let input_service = VideoInputService::try_new(input_source)?;
    let mut cursor = input_service.cursor()?;

    while let Some(content) = cursor.read()? {
        let image = content.item();
        let data_bytes = image.data_bytes();
        let pixels = bytemuck::cast_slice::<u8, Color32>(&data_bytes).to_vec();
        let color_image =
            ColorImage::new([image.size().width as _, image.size().height as _], pixels);
        sender
            .send(Message::FrameReceived(Arc::new(color_image)))
            .await?;
        ctx.request_repaint();
        let sleep_duration = Duration::from_micros(content.duration().as_microseconds() as _);
        sleep(sleep_duration).await;
    }

    Ok(())
}
