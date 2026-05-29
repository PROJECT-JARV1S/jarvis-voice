use anyhow::{Result, bail};

/// Voice Activity Detection (VAD) configuration.
///
/// Controls when the transcription engine considers the speaker finished:
/// transcription stops after `silence_duration` seconds of audio whose RMS
/// energy falls below `silence_threshold_rms`.
#[derive(Clone, Copy)]
pub struct Config {
    /// Duration of silence (in seconds) required to end a transcription session.
    pub silence_duration: f32,
    /// RMS energy threshold below which audio is treated as silence.
    pub silence_threshold_rms: f32,
}

impl Config {
    /// Validates the configuration values.
    ///
    /// # Errors
    /// Returns an error if `silence_duration` or `silence_threshold_rms` is negative.
    pub fn validate(&self) -> Result<()> {
        if self.silence_duration < 0.0 {
            bail!("silence_duration must be non-negative");
        }
        if self.silence_threshold_rms < 0.0 {
            bail!("silence_threshold_rms must be non-negative");
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            silence_duration: 1.0,
            silence_threshold_rms: 0.005,
        }
    }
}
