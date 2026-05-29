use anyhow::{Result, bail};

#[cfg(feature = "python")]
use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Extension trait for converting `anyhow::Result` into `PyResult`.
#[cfg(feature = "python")]
pub trait AnyhowError<T> {
    /// Converts `anyhow::Result<T>` to `PyResult<T>`, mapping errors to
    /// `PyRuntimeError`.
    fn to_py(self) -> PyResult<T>;
}

#[cfg(feature = "python")]
impl<T> AnyhowError<T> for Result<T> {
    fn to_py(self) -> PyResult<T> {
        self.map_err(|e| PyRuntimeError::new_err(format!("{:?}", e)))
    }
}

/// Downmixes interleaved `i16` multi-channel audio to mono `f32`.
///
/// Each channel's sample is normalised to \[-1.0, 1.0\] and then averaged
/// across channels per frame.
///
/// # Arguments
/// * `samples` - Interleaved multi-channel `i16` samples.
/// * `channels` - Number of channels.
///
/// # Returns
/// A vector of mono `f32` frames, one per original sample frame.
///
/// # Errors
/// Returns an error if `channels` is zero or if `samples.len()` is not a
/// multiple of `channels`.
pub fn interleaved_i16_to_mono(samples: &[i16], channels: usize) -> Result<Vec<f32>> {
    if channels == 0 {
        bail!("channels must be greater than zero");
    }
    if !samples.len().is_multiple_of(channels) {
        bail!(
            "expected input sample count to be divisible by {channels} channel(s), got {}",
            samples.len()
        );
    }

    let frames = samples.len() / channels;
    if channels == 1 {
        return Ok(samples
            .iter()
            .map(|sample| normalize_i16(*sample))
            .collect());
    }

    let mut mono = Vec::with_capacity(frames);
    for frame in samples.chunks_exact(channels) {
        let sum: f32 = frame.iter().map(|sample| normalize_i16(*sample)).sum();
        mono.push(sum / channels as f32);
    }

    Ok(mono)
}

/// Downmixes interleaved `f32` multi-channel audio to mono `f32`.
///
/// Each channel's sample is averaged across channels per frame.
///
/// # Arguments
/// * `samples` - Interleaved multi-channel `f32` samples.
/// * `channels` - Number of channels.
///
/// # Returns
/// A vector of mono `f32` frames, one per original sample frame.
///
/// # Errors
/// Returns an error if `channels` is zero or if `samples.len()` is not a
/// multiple of `channels`.
pub fn interleaved_f32_to_mono(samples: &[f32], channels: usize) -> Result<Vec<f32>> {
    if channels == 0 {
        bail!("channels must be greater than zero");
    }
    if !samples.len().is_multiple_of(channels) {
        bail!(
            "expected input sample count to be divisible by {channels} channel(s), got {}",
            samples.len()
        );
    }

    let frames = samples.len() / channels;
    if channels == 1 {
        return Ok(samples.to_vec());
    }

    let mut mono = Vec::with_capacity(frames);
    for frame in samples.chunks_exact(channels) {
        let sum: f32 = frame.iter().copied().sum();
        mono.push(sum / channels as f32);
    }

    Ok(mono)
}

/// Normalises an `i16` sample to the range \[-1.0, 1.0\].
///
/// `i16::MIN` (-32768) maps to -1.0; `i16::MAX` (32767) maps to ~1.0.
pub fn normalize_i16(sample: i16) -> f32 {
    if sample == i16::MIN {
        -1.0
    } else {
        sample as f32 / i16::MAX as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn downmixes_interleaved_i16_to_mono() {
        let mono = interleaved_i16_to_mono(&[i16::MAX, 0, 0, i16::MAX], 2).unwrap();
        assert_eq!(mono.len(), 2);
        assert!((mono[0] - 0.5).abs() < 1e-6);
        assert!((mono[1] - 0.5).abs() < 1e-6);
    }
}
