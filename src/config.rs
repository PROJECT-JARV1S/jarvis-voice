use crate::core::config::Config as CoreConfig;
#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Configuration for the voice transcriber.
///
/// This struct allows you to tweak the Voice Activity Detection (VAD) parameters
/// for the audio engine, specifically how long silence must be detected before
/// a transcription is considered complete.
#[cfg(feature = "python")]
#[pyclass(from_py_object)]
#[derive(Clone, Copy)]
pub struct Config {
    /// The duration of silence (in seconds) required to stop transcription.
    #[pyo3(get, set)]
    pub silence_duration: f32,
    /// The root-mean-square (RMS) energy threshold below which audio is considered silence.
    #[pyo3(get, set)]
    pub silence_threshold_rms: f32,
}

/// Configuration for the voice transcriber.
///
/// This struct allows you to tweak the Voice Activity Detection (VAD) parameters
/// for the audio engine, specifically how long silence must be detected before
/// a transcription is considered complete.
#[cfg(not(feature = "python"))]
#[derive(Clone, Copy)]
pub struct Config {
    /// The duration of silence (in seconds) required to stop transcription.
    pub silence_duration: f32,
    /// The root-mean-square (RMS) energy threshold below which audio is considered silence.
    pub silence_threshold_rms: f32,
}

impl From<CoreConfig> for Config {
    fn from(c: CoreConfig) -> Self {
        Self {
            silence_duration: c.silence_duration,
            silence_threshold_rms: c.silence_threshold_rms,
        }
    }
}

impl From<Config> for CoreConfig {
    fn from(c: Config) -> Self {
        Self {
            silence_duration: c.silence_duration,
            silence_threshold_rms: c.silence_threshold_rms,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        CoreConfig::default().into()
    }
}

impl Config {
    /// Validates the configuration values.
    ///
    /// Checks that `silence_duration` is >= 0 and `silence_threshold_rms` is >= 0.
    ///
    /// # Errors
    /// Returns an error if either value is negative.
    pub fn validate(&self) -> anyhow::Result<()> {
        let core: CoreConfig = (*self).into();
        core.validate()
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl Config {
    /// Creates a new configuration instance with optional overrides.
    ///
    /// If no arguments are provided, it initializes with the default values.
    #[new]
    #[pyo3(signature = (silence_duration=None, silence_threshold_rms=None))]
    fn py_new(silence_duration: Option<f32>, silence_threshold_rms: Option<f32>) -> Self {
        let mut config = CoreConfig::default();
        if let Some(d) = silence_duration {
            config.silence_duration = d;
        }
        if let Some(t) = silence_threshold_rms {
            config.silence_threshold_rms = t;
        }
        config.into()
    }

    /// Validates the configuration values.
    ///
    /// Checks that `silence_duration` is >= 0 and `silence_threshold_rms` is >= 0.
    /// Returns a `ValueError` (via `PyResult`) if validation fails.
    #[pyo3(name = "validate")]
    fn py_validate(&self) -> PyResult<()> {
        use crate::utils::AnyhowError;
        self.validate().to_py()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = Config {
            silence_duration: -1.0,
            silence_threshold_rms: 0.005,
        };
        assert!(config.validate().is_err());

        let config = Config {
            silence_duration: 1.0,
            silence_threshold_rms: -0.1,
        };
        assert!(config.validate().is_err());

        let config = Config::default();
        assert!(config.validate().is_ok());
    }
}
