//! # JARVIS Voice Core
//!
//! This crate provides the performance-critical audio processing, wake word detection,
//! and speech-to-text transcription engine for the JARVIS voice system.
//!
//! It is designed primarily as a Python extension module via PyO3, exposing a robust
//! `Transcriber` interface backed by a high-performance Rust implementation.

#[cfg(feature = "python")]
use pyo3::prelude::*;

pub mod audio;
pub mod config;
pub mod core;
#[cfg(feature = "python")]
pub mod python;
pub mod transcriber;
pub mod transcription;
pub mod utils;

#[cfg(feature = "python")]
#[pymodule]
fn jarvis_transcriber(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    python::jarvis_transcriber(py, m)
}

