//! Audio capture and sample-rate conversion.

/// Microphone input stream via CPAL.
pub mod input;
/// Rubato-based sample-rate converter (to 16 kHz mono f32).
pub mod resampler;
