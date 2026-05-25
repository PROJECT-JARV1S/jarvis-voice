use crossbeam_channel::{Sender, unbounded};
#[cfg(feature = "python")]
use pyo3::exceptions::{PyRuntimeError, PyTimeoutError};
#[cfg(feature = "python")]
use pyo3::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::config::Config;
use crate::transcription::engine::{Callback, Command, worker_thread};

const DEFAULT_MODEL_URI: &str = "https://blob.handy.computer/parakeet-v3-int8.tar.gz";
const DEFAULT_MODEL_PATH: &str = "parakeet-tdt-0.6b-v3-int8";

/// The core speech-to-text transcription engine.
///
/// This struct manages the Rust worker thread that handles audio capture, silence
/// detection (VAD), and execution of the Parakeet transcription model. It provides
/// thread-safe mechanisms to start and stop transcription asynchronously.
#[cfg_attr(feature = "python", pyclass(skip_from_py_object))]
pub struct Transcriber {
    command_tx: Sender<Command>,
    is_transcribing: Arc<AtomicBool>,
    latest_transcript: Arc<Mutex<String>>,
    completion_notifier: Arc<(Mutex<bool>, Condvar)>,
    on_complete_callback: Arc<Mutex<Option<Callback>>>,
}

impl Transcriber {
    /// Initializes a new Transcriber.
    ///
    /// Spawns a background worker thread that handles the audio capture and transcription logic.
    pub fn new(
        config: Option<Config>,
        model_uri: Option<String>,
        model_path: Option<String>,
    ) -> anyhow::Result<Self> {
        let uri = model_uri.unwrap_or_else(|| DEFAULT_MODEL_URI.to_string());
        let path = model_path.unwrap_or_else(|| DEFAULT_MODEL_PATH.to_string());

        let rust_config = config
            .map(|c| crate::core::config::Config {
                silence_duration: c.silence_duration,
                silence_threshold_rms: c.silence_threshold_rms,
            })
            .unwrap_or_else(crate::core::config::Config::default);

        let (command_tx, command_rx) = unbounded();
        let is_transcribing = Arc::new(AtomicBool::new(false));
        let latest_transcript = Arc::new(Mutex::new(String::new()));
        let completion_notifier = Arc::new((Mutex::new(false), Condvar::new()));
        let on_complete_callback = Arc::new(Mutex::new(None));

        let is_transcribing_clone = is_transcribing.clone();
        let latest_transcript_clone = latest_transcript.clone();
        let completion_notifier_clone = completion_notifier.clone();
        let on_complete_callback_clone = on_complete_callback.clone();

        thread::spawn(move || {
            worker_thread(
                command_rx,
                is_transcribing_clone,
                latest_transcript_clone,
                completion_notifier_clone,
                on_complete_callback_clone,
                rust_config,
                uri,
                path,
            );
        });

        Ok(Self {
            command_tx,
            is_transcribing,
            latest_transcript,
            completion_notifier,
            on_complete_callback,
        })
    }

    /// Manually starts the transcription process.
    ///
    /// Clears the previous transcript and signals the worker thread to begin listening
    /// to the microphone.
    pub fn start_transcription(&self) -> anyhow::Result<()> {
        self.is_transcribing.store(true, Ordering::SeqCst);
        if let Ok(mut guard) = self.latest_transcript.lock() {
            guard.clear();
        }
        self.command_tx.send(Command::Start).map_err(|e| {
            self.is_transcribing.store(false, Ordering::SeqCst);
            anyhow::anyhow!("Failed to send start command: {}", e)
        })
    }

    /// Manually stops the transcription process early.
    pub fn stop_transcription(&self) -> anyhow::Result<()> {
        self.command_tx
            .send(Command::Stop)
            .map_err(|e| anyhow::anyhow!("Failed to send stop command: {}", e))
    }

    /// Checks if the engine is currently transcribing audio.
    pub fn is_transcribing(&self) -> bool {
        self.is_transcribing.load(Ordering::SeqCst)
    }

    /// Gets the most recent transcription result.
    pub fn get_latest_transcript(&self) -> String {
        self.latest_transcript.lock().unwrap().clone()
    }

    /// Registers a callback function to be invoked when transcription completes.
    pub fn register_on_complete(&self, callback: Callback) {
        let mut guard = self.on_complete_callback.lock().unwrap();
        *guard = Some(callback);
    }

    /// Blocks the calling thread until the current transcription is complete.
    ///
    /// Returns `true` if transcription finishes, or `false` if `timeout` is reached.
    pub fn wait_until_done(&self, timeout: Option<Duration>) -> anyhow::Result<bool> {
        let (lock, cvar) = &*self.completion_notifier;
        let mut completed = lock.lock().unwrap();

        if *completed {
            return Ok(true);
        }

        if let Some(duration) = timeout {
            let start = Instant::now();
            while !*completed {
                let elapsed = start.elapsed();
                if elapsed >= duration {
                    return Ok(false);
                }
                let remaining = duration - elapsed;
                let (guard, res) = cvar.wait_timeout(completed, remaining).map_err(|e| {
                    anyhow::anyhow!("Condvar wait error: {:?}", e)
                })?;
                completed = guard;
                if res.timed_out() && !*completed {
                    return Ok(false);
                }
            }
        } else {
            while !*completed {
                completed = cvar.wait(completed).map_err(|e| {
                    anyhow::anyhow!("Condvar wait error: {:?}", e)
                })?;
            }
        }
        Ok(true)
    }
}

impl Drop for Transcriber {
    fn drop(&mut self) {
        let _ = self.command_tx.send(Command::Shutdown);
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl Transcriber {
    /// Initializes a new Transcriber.
    ///
    /// Spawns a background worker thread that handles the audio capture and transcription logic.
    #[new]
    #[pyo3(signature = (config=None, model_uri=None, model_path=None))]
    fn py_new(
        config: Option<Py<Config>>,
        model_uri: Option<String>,
        model_path: Option<String>,
    ) -> PyResult<Self> {
        let rust_config = Python::attach(|py| {
            if let Some(c) = config {
                let b = c.borrow(py);
                Some(Config {
                    silence_duration: b.silence_duration,
                    silence_threshold_rms: b.silence_threshold_rms,
                })
            } else {
                None
            }
        });

        use crate::utils::AnyhowError;
        Self::new(rust_config, model_uri, model_path).to_py()
    }

    /// Manually starts the transcription process.
    ///
    /// Clears the previous transcript and signals the worker thread to begin listening
    /// to the microphone.
    #[pyo3(name = "start_transcription")]
    fn py_start_transcription(&self) -> PyResult<()> {
        use crate::utils::AnyhowError;
        self.start_transcription().to_py()
    }

    /// Manually stops the transcription process early.
    #[pyo3(name = "stop_transcription")]
    fn py_stop_transcription(&self) -> PyResult<()> {
        use crate::utils::AnyhowError;
        self.stop_transcription().to_py()
    }

    /// Checks if the engine is currently transcribing audio.
    #[pyo3(name = "is_transcribing")]
    fn py_is_transcribing(&self) -> bool {
        self.is_transcribing()
    }

    /// Gets the most recent transcription result.
    #[pyo3(name = "get_latest_transcript")]
    fn py_get_latest_transcript(&self) -> String {
        self.get_latest_transcript()
    }

    /// Registers a Python callback function to be invoked when transcription completes.
    #[pyo3(name = "register_on_complete")]
    fn py_register_on_complete(&self, callback: Py<PyAny>) {
        self.register_on_complete(callback);
    }

    /// Blocks the calling thread until the current transcription is complete.
    ///
    /// Returns `True` if transcription finishes, or raises `TimeoutError` if `timeout` is reached.
    #[pyo3(signature = (timeout=None), name = "wait_until_done")]
    fn py_wait_until_done(&self, py: Python, timeout: Option<f64>) -> PyResult<bool> {
        py.detach(|| {
            let timeout_duration = timeout.map(Duration::from_secs_f64);
            match self.wait_until_done(timeout_duration) {
                Ok(true) => Ok(true),
                Ok(false) => Err(PyTimeoutError::new_err("Transcription timed out")),
                Err(e) => Err(PyRuntimeError::new_err(format!("Wait error: {:?}", e))),
            }
        })
    }
}

/// Creates a default `Transcriber` instance with default configuration settings.
#[cfg(feature = "python")]
#[pyfunction]
pub fn default() -> PyResult<Transcriber> {
    use crate::utils::AnyhowError;
    Transcriber::new(None, None, None).to_py()
}
