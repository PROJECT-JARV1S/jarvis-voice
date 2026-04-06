use anyhow::{Result, bail};

#[derive(Clone, Copy)]
pub struct Config {
    pub silence_duration: f32,
    pub silence_threshold_rms: f32,
}

impl Config {
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
