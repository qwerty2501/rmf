use std::collections::VecDeque;

use anyhow::anyhow;
use rmf_core::{Content, Error, Result, Timestamp};
use rmf_macros::delegate_implements;
use rsmpeg::{
    avformat::AVFormatContextInput, avutil::AVFrame, error::RsmpegError, ffi::AVMEDIA_TYPE_AUDIO,
};

use crate::{
    Audio, AudioDataContextBuilder,
    ffmpeg::utils::{AVFormatContentContexts, input_contexts, seek_input, to_timestamp},
};

pub struct AVFormatAudioContentCursor {
    input: AVFormatContextInput,
    audio_context: AVFormatContentContexts,
    audio_cache: VecDeque<Content<Audio>>,
}

impl AVFormatAudioContentCursor {
    pub fn try_new(input: AVFormatContextInput) -> Result<Self> {
        let audio_context = input_contexts(&input, AVMEDIA_TYPE_AUDIO)?
            .ok_or_else(|| Error::new_input(anyhow!("Can not make input context")))?;
        Ok(Self {
            input,
            audio_context,
            audio_cache: VecDeque::default(),
        })
    }
    fn avframe_to_audio(frame: AVFrame) -> Result<Audio> {
        let data_context = AudioDataContextBuilder::try_new(frame)?;
        Audio::tyr_new(data_context)
    }
}

#[delegate_implements]
impl rmf_core::audio::AudioContentCursor for AVFormatAudioContentCursor {
    type Item = Audio;
    fn read(&mut self) -> Result<Option<Content<Audio>>> {
        if let Some(content) = self.audio_cache.pop_front() {
            Ok(Some(content))
        } else {
            while let Some(packet) = self
                .input
                .read_packet()
                .map_err(|e| Error::new_audio(e.into()))?
            {
                if packet.stream_index == self.audio_context.index as _ {
                    self.audio_context
                        .avcodec_context
                        .send_packet(Some(&packet))
                        .map_err(|e| Error::new_audio(e.into()))?;
                    loop {
                        match self.audio_context.avcodec_context.receive_frame() {
                            Ok(frame) => {
                                let presentation_timestamp =
                                    to_timestamp(frame.pts, frame.time_base);
                                let duration_timestamp =
                                    to_timestamp(frame.duration, frame.time_base);
                                let audio = Self::avframe_to_audio(frame)?;
                                self.audio_cache.push_back(Content::new(
                                    audio,
                                    presentation_timestamp,
                                    duration_timestamp,
                                ));
                            }
                            Err(err) => {
                                if err == RsmpegError::DecoderFlushedError
                                    || err == RsmpegError::DecoderDrainError
                                {
                                    break;
                                } else {
                                    Err(Error::new_input(err.into()))?
                                }
                            }
                        }
                    }
                    break;
                }
            }
            Ok(self.audio_cache.pop_front())
        }
    }
    #[inline]
    fn seek(&mut self, timestamp: Timestamp) -> Result<()> {
        seek_input(&mut self.input, timestamp)
    }
}
