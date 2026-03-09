use pyo3::prelude::*;

#[pyclass]
pub struct Config {
    pub silence_duration: f32,
    pub silence_threshold_rms: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            silence_duration: 1.0,
            silence_threshold_rms: 0.005,
        }
    }
}
