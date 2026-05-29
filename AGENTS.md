# jarvis-voice

Hybrid Python + Rust voice processing engine: wake-word detection (Porcupine) powered by Python, transcription (Parakeet) powered by Rust.

## Quick start

```bash
uv sync                          # install Python deps
maturin develop                  # compile Rust cdylib into venv
```

Set `PORCUPINE_KEY` in `.env` (copy from `.env.example`).

## Architecture

```
Python: jarvis_voice.Listener          # wake-word loop via sounddevice + pvporcupine
  └─> jarvis_voice.jarvis_transcriber  # PyO3 bindings (compiled .pyd)
        └─ Rust: Transcriber           # src/transcriber.rs
              ├─ AudioInput (cpal)     # src/audio/input.rs
              ├─ AudioResampler        # src/audio/resampler.rs (rubato → 16kHz mono f32)
              ├─ VAD (RMS threshold)   # src/transcription/engine.rs
              └─ ParakeetEngine        # transcribe-rs crate
```

- Python `Listener` owns the blocking wake-word loop; `listen_async()` wraps it in a daemon thread.
- Rust `Transcriber` spawns a background worker thread; sync via `Mutex<Condvar>`.
- The `parakeet-tdt-0.6b-v3-int8/` model (~1GB) auto-downloads from `blob.handy.computer` on first use.
- VAD defaults: `silence_duration=1.0s`, `silence_threshold_rms=0.005`.

## Command prefix

Prefix `git`, `cargo`, and `cargo test` commands with `rtk` (e.g. `rtk git status`, `rtk cargo test`, `rtk cargo build`). `pytest`, `uv`, and `maturin` commands do not need it.

## Tests

```bash
rtk cargo test && pytest tests/ -v       # run both Rust and Python tests
```

- Python tests mock `pvporcupine`, `sounddevice`, and `jarvis_transcriber.default()` — no mic or API key needed.
- Rust integration tests (`tests/rust_audio.rs`, `rust_transcription.rs`) use real hardware by default; sync tests (`rust_sync.rs`) don't.
- Manual smoke tests (`jarvis_voice/test_*.py`) need a real mic and `PORCUPINE_KEY`; run via `python -m jarvis_voice.test_porcupine`.

## Important details

- **`python` feature flag** gates all PyO3 bindings (`#[cfg(feature = "python")]`). `cargo build` alone skips them; `maturin develop` enables it automatically.
- **Version** is `dynamic` in `pyproject.toml` — single source of truth: `Cargo.toml` version.
- **No CI/CD, no linters, no formatters, no pre-commit hooks** configured.
- **Windows-only**: the compiled `.pyd` targets `cp310-win_amd64`.
- The `conductor/` directory is a custom project-management system with its own workflow (`conductor/workflow.md`), phase tracking, and git notes protocol.
- Commit style: conventional commits (`feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`).
