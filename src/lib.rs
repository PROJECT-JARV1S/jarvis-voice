//! # JARVIS Voice Core
//!
//! This crate provides the performance-critical audio processing, wake word detection,
//! and speech-to-text transcription engine for the JARVIS voice system.
//!
//! It is designed primarily as a Python extension module via PyO3, exposing a robust
//! `Transcriber` interface backed by a high-performance Rust implementation.

use pyo3::prelude::*;

pub mod audio;
pub mod config;
pub mod core;
pub mod python;
pub mod transcriber;
pub mod transcription;
pub mod utils;

#[pymodule]
fn jarvis_transcriber(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    python::jarvis_transcriber(py, m)
}
