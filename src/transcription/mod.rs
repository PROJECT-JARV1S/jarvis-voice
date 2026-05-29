//! Speech-to-text transcription engine and model management.

/// VAD-based worker thread that drives audio capture and Parakeet inference.
pub mod engine;
/// Model download, extraction, and loading.
pub mod model;
