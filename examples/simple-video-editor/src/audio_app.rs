use derive_new::new;
use rmf_host::{
    Content, InputSource, Timestamp,
    audio::{self, Audio, AudioInputService},
    service::{ContentCursorTrait, ContentStreamServiceTrait},
};
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tokio::sync::mpsc::{self, Receiver};
use tokio::time::{Instant, sleep_until};

pub struct AudioPlayer {}

impl AudioPlayer {
    pub async fn play(path: PathBuf) {
        let (sender, receiver) = mpsc::channel(100);
        tokio::join!(decode_audio_loop(path, sender), play_audio_loop(receiver));
    }
}

async fn play_audio_loop(mut receiver: mpsc::Receiver<Message>) {
    loop {
        if let Some(message) = receiver.recv().await {
            match message {
                Message::End => break,
                Message::Received(audio) => {}
            }
        }
    }
}

async fn decode_audio_loop(path: PathBuf, sender: mpsc::Sender<Message>) {
    if let Err(err) = inner_decode_loop(path, sender).await {
        eprintln!("{err}");
    }
}

#[derive(Clone)]
pub enum Message {
    Received(Arc<Audio>),
    End,
}

struct InnerContent {
    offset: Timestamp,
    audio: Audio,
}

impl InnerContent {
    fn from_rmf_content(content: Content<Audio>) -> Self {
        Self {
            offset: content.offset(),
            audio: content.item().clone(),
        }
    }
}

async fn inner_decode_loop(path: PathBuf, sender: mpsc::Sender<Message>) -> anyhow::Result<()> {
    let input_source = InputSource::new_path(path.clone());
    let input_service = AudioInputService::try_new(input_source)?;
    let mut cursor = input_service.cursor()?;
    const MAX_QUEUE_SIZE: usize = 10;
    let mut content_queue = VecDeque::<InnerContent>::with_capacity(MAX_QUEUE_SIZE);

    let mut start_instant: Option<Instant> = None;
    let mut start_offset: Option<Timestamp> = None;

    loop {
        while content_queue.len() < MAX_QUEUE_SIZE {
            if let Some(content) = cursor.read()? {
                content_queue.push_back(InnerContent::from_rmf_content(content));
            } else {
                break;
            }
        }

        if let Some(content) = content_queue.pop_front() {
            if start_instant.is_none() {
                start_instant = Some(Instant::now());
                start_offset = Some(content.offset);
            }
            let base_start = start_instant.unwrap();
            let base_offset = start_offset.unwrap();

            let audio_elapsed = content.offset - base_offset;

            let audio_elapsed_duration =
                Duration::from_millis(audio_elapsed.as_milliseconds() as u64);

            let target_time = base_start + audio_elapsed_duration;

            sleep_until(target_time).await;
            let a = Arc::new(content.audio);
            sender.send(Message::Received(a)).await?;
        } else {
            sender.send(Message::End).await?;
            break;
        }
    }

    Ok(())
}
