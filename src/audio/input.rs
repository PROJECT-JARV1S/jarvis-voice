use anyhow::{Result, anyhow};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::Sender;

/// Raw audio data captured from the microphone.
///
/// Variants correspond to the sample format of the input device.
pub enum RawAudio {
    /// 32-bit float samples.
    F32(Vec<f32>),
    /// 16-bit signed integer samples.
    I16(Vec<i16>),
}

/// Captures audio from the system's default input device via CPAL.
///
/// The stream delivers raw audio chunks over a [`Sender<RawAudio>`] channel.
/// Call [`start_stream`](AudioInput::start_stream) to begin capture and
/// [`stop_stream`](AudioInput::stop_stream) to end it.
pub struct AudioInput {
    device: cpal::Device,
    /// The stream configuration (sample rate, channels, buffer size) selected
    /// from the device's default input config.
    pub stream_config: cpal::StreamConfig,
    /// The sample format of the input device (F32 or I16).
    pub sample_format: cpal::SampleFormat,
    stream: Option<cpal::Stream>,
}

impl AudioInput {
    /// Opens the default input audio device and reads its default configuration.
    ///
    /// # Errors
    /// Returns an error if no default input device is found or if the default
    /// input config cannot be read.
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow!("No input device found"))?;

        let config_cpal = device
            .default_input_config()
            .map_err(|e| anyhow!("Failed to get default input config: {}", e))?;

        let stream_config = config_cpal.config();
        let sample_format = config_cpal.sample_format();

        Ok(Self {
            device,
            stream_config,
            sample_format,
            stream: None,
        })
    }

    /// Starts capturing audio from the device.
    ///
    /// Audio chunks are sent over the provided channel until
    /// [`stop_stream`](AudioInput::stop_stream) is called.
    ///
    /// # Arguments
    /// * `tx` - Channel sender that receives [`RawAudio`] chunks.
    ///
    /// # Errors
    /// Returns an error if the input stream cannot be built or started.
    pub fn start_stream(&mut self, tx: Sender<RawAudio>) -> Result<()> {
        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = match self.sample_format {
            cpal::SampleFormat::F32 => self.device.build_input_stream(
                &self.stream_config,
                move |data: &[f32], _: &_| {
                    let _ = tx.send(RawAudio::F32(data.to_vec()));
                },
                err_fn,
                None,
            ),
            cpal::SampleFormat::I16 => self.device.build_input_stream(
                &self.stream_config,
                move |data: &[i16], _: &_| {
                    let _ = tx.send(RawAudio::I16(data.to_vec()));
                },
                err_fn,
                None,
            ),
            _ => return Err(anyhow!("Unsupported sample format")),
        }?;

        stream
            .play()
            .map_err(|e| anyhow!("Failed to play stream: {}", e))?;
        self.stream = Some(stream);
        Ok(())
    }

    /// Stops the audio capture stream and releases it.
    pub fn stop_stream(&mut self) {
        self.stream = None;
    }
}
