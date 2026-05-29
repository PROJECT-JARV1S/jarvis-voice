"""JARVIS Voice — wake-word detection and local speech-to-text transcription.

Combines Porcupine (Python) for wake-word spotting with a Parakeet-based
Rust backend for low-latency, on-device transcription.
"""

from . import jarvis_transcriber
from .listener import Listener

__all__ = [
    "Listener",
    "jarvis_transcriber",
]
