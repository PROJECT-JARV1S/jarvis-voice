from os import getenv

import struct
import pyaudio
import pvporcupine
from dotenv import load_dotenv
import jarvis_transcriber

load_dotenv()

def get_next_audio_frame(stream, frame_length):
    pcm = stream.read(frame_length)
    return struct.unpack_from("h" * frame_length, pcm)

if __name__ == '__main__':
    access_key = getenv('PORCUPINE_KEY')

    print(jarvis_transcriber.___version())

    handle = pvporcupine.create(access_key=access_key, keywords=['jarvis'])
    
    pa = pyaudio.PyAudio()
    audio_stream = pa.open(
        rate=handle.sample_rate,
        channels=1,
        format=pyaudio.paInt16,
        input=True,
        frames_per_buffer=handle.frame_length)

    print("Listening...")
    while True:
        audio_frame = get_next_audio_frame(audio_stream, handle.frame_length)
        keyword_index = handle.process(audio_frame)
        if keyword_index == 0:
            print('Jarvis detected')
            audio_stream.close()
            pa.terminate()
            handle.delete()
            break
