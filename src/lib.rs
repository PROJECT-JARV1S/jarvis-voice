//! # JARVIS Voice Core
//!
//! This crate provides the performance-critical audio processing, wake word detection,
//! and speech-to-text transcription engine for the JARVIS voice system.
//!
//! It is designed primarily as a Python extension module via PyO3, exposing a robust
//! `Transcriber` interface backed by a high-performance Rust implementation.

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Audio capture and sample-rate conversion via CPAL.
pub mod audio;
/// VAD configuration exposed to consumers and Python bindings.
pub mod config;
/// Core configuration types shared across the crate.
pub mod core;
#[cfg(feature = "python")]
/// PyO3 bindings for the Python extension module.
pub mod python;
/// High-level transcription engine (public API entry point).
pub mod transcriber;
/// Speech-to-text engine and model management (implementation).
pub mod transcription;
/// Utility functions (audio downmixing, error conversion).
pub mod utils;

#[cfg(feature = "python")]
#[pymodule]
fn jarvis_transcriber(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    python::jarvis_transcriber(py, m)
}

