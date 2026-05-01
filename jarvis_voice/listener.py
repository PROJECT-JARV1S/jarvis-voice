import os
import struct
from typing import Union

import pvporcupine
import sounddevice as sd

from . import jarvis_transcriber


class Listener:
    """
    A high-level interface for voice interaction, combining wake-word detection
    (via Porcupine) and speech-to-text transcription (via Parakeet/Rust).

    Usage:
        ```python
        from jarvis_voice import Listener

        # Will use PORCUPINE_KEY from environment variables
        listener = Listener(wake_words=["jarvis"])
        
        # Starts a blocking loop that listens for the wake word
        listener.listen()
        ```
    """
    def __init__(self, wake_words: Union[str, list[str]], access_key: str = None):
        """
        Initializes the Listener with specified wake words and Porcupine access key.

        Args:
            wake_words: A string or list of strings representing the wake words to listen for (e.g., "jarvis").
            access_key: Your Porcupine access key. If not provided, it looks for the `PORCUPINE_KEY` environment variable.
        """
        if access_key:
            self.access_key = access_key
        else:
            self.access_key = os.getenv('PORCUPINE_KEY')
            
        if not self.access_key:
            raise ValueError("Error: PORCUPINE_KEY not found in environment.")

        self.wake_words = [wake_words] if isinstance(wake_words, str) else wake_words
        self.transcriber = jarvis_transcriber.default()
        self.handle = None
        self.audio_stream = None
        self._setup_resources()

    def _setup_resources(self):
        if self.access_key is None:
            raise ValueError("Error: PORCUPINE_KEY not found in environment.")

        self.handle = pvporcupine.create(access_key=self.access_key, keywords=self.wake_words)
        self.audio_stream = sd.RawInputStream(
            samplerate=self.handle.sample_rate,
            channels=1,
            dtype='int16',
            blocksize=self.handle.frame_length,
        )
        self.audio_stream.start()

    def _get_next_audio_frame(self):
        try:
            pcm, overflowed = self.audio_stream.read(self.handle.frame_length)
            return struct.unpack_from("h" * self.handle.frame_length, pcm)
        except Exception as e:
            print(f"Audio read error: {e}")
            return None

    def listen(self):
        """
        Starts a blocking loop that continually listens for the wake word.
        Once detected, it automatically starts the transcription process,
        waits for the user to finish speaking (detected via silence),
        and prints the resulting transcript.
        """
        print("--- JARVIS Voice System Active ---")
        try:
            while True:
                audio_frame = self._get_next_audio_frame()
                if audio_frame is None:
                    continue
                    
                keyword_index = self.handle.process(audio_frame)
                
                if keyword_index >= 0:
                    print(f"\n[WAKE WORD DETECTED: {self.wake_words[keyword_index]}]")
                    self.force_start_transcribe()
                    self.transcriber.wait_until_done()
                    print(f"Transcript: \"{self.get_transcript()}\"")
        except KeyboardInterrupt:
            print("\nStopping...")
        finally:
            self.stop()
            
    def force_start_transcribe(self):
        """Manually triggers the transcription process."""
        self.transcriber.start_transcription()

    def restart(self):
        self.stop()
        self._setup_resources()

    def stop(self):
        """Stops the audio stream and releases associated resources."""
        if self.audio_stream:
            self.audio_stream.stop()
            self.audio_stream.close()
            self.audio_stream = None
        if self.handle:
            self.handle.delete()
            self.handle = None
    
    def __del__(self):
        self.stop()
        
    def get_transcript(self) -> str:
        """Returns the most recent transcription result as a string."""
        return self.transcriber.get_latest_transcript()

    def is_listening(self) -> bool:
        """Returns True if the underlying transcription engine is currently capturing audio."""
        return self.transcriber.is_transcribing()