from __future__ import annotations

from typing import Callable, Optional

class Config:
    """Configuration for the voice transcriber VAD parameters."""
    silence_duration: float
    silence_threshold_rms: float

    def __init__(
        self,
        silence_duration: Optional[float] = None,
        silence_threshold_rms: Optional[float] = None,
    ) -> None:
        """
        Args:
            silence_duration: Silence required (in seconds) to stop transcription.
            silence_threshold_rms: RMS energy threshold for silence detection.
        """
        ...

    def validate(self) -> None:
        """Validates that parameters are >= 0. Raises ValueError if invalid."""
        ...

class Transcriber:
    """
    The core speech-to-text transcription engine powered by Rust.

    Usage:
        ```python
        from jarvis_voice import jarvis_transcriber

        transcriber = jarvis_transcriber.default()
        
        # Starts microphone capture and transcription in the background
        transcriber.start_transcription()
        
        # Blocks until the user stops speaking
        transcriber.wait_until_done()
        
        print(transcriber.get_latest_transcript())
        ```
    """
    def __init__(
        self,
        config: Optional[Config] = None,
        model_uri: Optional[str] = None,
        model_path: Optional[str] = None,
    ) -> None: ...

    def start_transcription(self) -> None:
        """Starts audio capture and transcription in the background."""
        ...
    def stop_transcription(self) -> None:
        """Manually interrupts and stops the ongoing transcription."""
        ...
    def is_transcribing(self) -> bool:
        """Returns True if the engine is currently transcribing."""
        ...
    def get_latest_transcript(self) -> str:
        """Returns the most recent transcription result."""
        ...
    def register_on_complete(self, callback: Callable[[str], None]) -> None:
        """Registers a callback function to run when transcription finishes."""
        ...
    def wait_until_done(self, timeout: Optional[float] = None) -> bool:
        """
        Blocks until the current transcription is complete.
        
        Args:
            timeout: Maximum time to wait in seconds. Raises TimeoutError if exceeded.
        """
        ...

def default() -> Transcriber:
    """Creates a Transcriber instance with default settings."""
    ...
def ___version() -> str:
    """Returns the internal crate version."""
    ...

