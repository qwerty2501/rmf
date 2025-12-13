use std::collections::VecDeque;

use anyhow::anyhow;
use rmf_core::{Content, Error, Result, Timestamp};
use rmf_macros::delegate_implements;
use rsmpeg::{
    avcodec::AVCodecContext,
    avformat::AVFormatContextInput,
    avutil::{AVFrame, AVMem, av_rescale_q},
    error::RsmpegError,
    ffi::{
        self, AV_PIX_FMT_BGR24, AV_TIME_BASE_Q, AVMEDIA_TYPE_AUDIO, AVMEDIA_TYPE_VIDEO,
        AVPixelFormat, AVRational, AVSEEK_FLAG_BACKWARD, SWS_BICUBIC, av_image_get_buffer_size,
    },
    swscale::SwsContext,
};

use crate::{Audio, AudioDataContextBuilder, Image};

pub struct AVFormatAudioContentCursor {
    input: AVFormatContextInput,
    audio_context: AVFormatContentContexts,
    audio_cache: VecDeque<Content<Audio>>,
}

struct AVFormatContentContexts {
    avcodec_context: AVCodecContext,
    index: usize,
}

struct ImageScaleContext {
    sws_context: SwsContext,
    frame_cache: AVFrame,
    buffer: AVMem,
}

impl AVFormatAudioContentCursor {
    pub fn try_new(input: AVFormatContextInput) -> Result<Self> {
        let audio_context = input_contexts(&input, AVMEDIA_TYPE_AUDIO)?
            .ok_or_else(|| Error::new_audio(anyhow!("Can not make input context").into()))?;
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
                .map_err(|e| Error::new_image(e.into()))?
            {
                if packet.stream_index == self.audio_context.index as _ {
                    self.audio_context
                        .avcodec_context
                        .send_packet(Some(&packet))
                        .map_err(|e| Error::new_image(e.into()))?;
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
                                    Err(Error::new_image(err.into()))?
                                }
                            }
                        }
                    }
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

#[inline]
fn seek_input(input: &mut AVFormatContextInput, timestamp: Timestamp) -> Result<()> {
    input
        .seek(-1, timestamp.as_microseconds(), AVSEEK_FLAG_BACKWARD as _)
        .map_err(|e| Error::new_image(e.into()))
}

#[inline]
fn input_contexts(
    input: &AVFormatContextInput,
    media_type: ffi::AVMediaType,
) -> Result<Option<AVFormatContentContexts>> {
    if let Some((index, avcodec)) = input
        .find_best_stream(media_type)
        .map_err(|e| Error::new_image(e.into()))?
    {
        let avcodec_context = AVCodecContext::new(&avcodec);
        Ok(Some(AVFormatContentContexts {
            avcodec_context,
            index,
        }))
    } else {
        Ok(None)
    }
}

#[inline]
fn to_timestamp(ts: i64, time_base: AVRational) -> Timestamp {
    Timestamp::from_microseconds(av_rescale_q(ts, time_base, AV_TIME_BASE_Q))
}
