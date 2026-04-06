# Specification: Refactor Rust Logic for Maintainability

## Overview
This track focuses on refactoring the core Rust logic of the `jarvis-voice` project to enhance maintainability, readability, and modularity. The goal is to break down existing large files into smaller, more focused modules and implement a standardized error handling strategy using `anyhow`.

## Functional Requirements
- **Module Separation:**
    - Separate PyO3 binding logic into its own dedicated module to isolate the Rust-Python interface.
    - Extract audio processing (e.g., I/O with `cpal`, resampling with `rubato`) into a dedicated module.
    - Isolate transcription logic (e.g., `transcribe-rs`, model management) for improved testability.
    - Move configuration and common utility logic into a 'core' or 'utils' module.
    - **Test Separation:** Move all unit and integration tests into their own module or separate files to keep source code clean.
- **Error Handling:**
    - Implement the `anyhow` crate to standardize error propagation across all Rust modules.
    - Refactor existing error handling to use `anyhow::Result` and provide context where appropriate.
- **Maintain Performance:** Ensure the refactoring does not introduce significant latency or resource overhead, adhering to existing success metrics (<100ms latency).

## Non-Functional Requirements
- **Maintainability:** Code should be easier to navigate and modify due to clear module boundaries.
- **Readability:** Improved by smaller, more focused files and consistent error handling patterns.
- **Testability:** Decoupled modules should allow for more granular unit and integration testing.

## Acceptance Criteria
- The Rust codebase is reorganized into a hierarchical module structure.
- All public-facing PyO3 bindings continue to function as expected.
- The system still detects the "Jarvis" wake word and performs transcription correctly.
- Error handling is consistent throughout the crate using `anyhow`.
- Tests are organized into a separate module or files.
- CI/CD tests pass successfully.

## Out of Scope
- Adding new features to the voice processing engine.
- Modifying the Python-side logic (except where necessary for PyO3 binding changes).
- Changing the underlying audio processing libraries (e.g., `cpal`, `rubato`).
