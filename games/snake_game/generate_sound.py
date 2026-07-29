import math, wave, struct, random

sample_rate = 44100
duration = 0.15

wave_file = wave.open('assets/eat.wav', 'w')
wave_file.setnchannels(1)
wave_file.setsampwidth(2)
wave_file.setframerate(sample_rate)

num_frames = int(sample_rate * duration)
frames = []
phase = 0.0

for i in range(num_frames):
    t = i / sample_rate
    
    # Downward pitch sweep for a "gulp/chomp"
    freq = 800 - 4000 * t
    if freq < 100:
        freq = 100
    phase += freq / sample_rate
    
    # Triangle wave for body
    wave_val = 2.0 * abs(2.0 * (phase - math.floor(phase + 0.5))) - 1.0
    
    # Noise for the crunch
    noise = random.uniform(-1, 1)
    
    # Envelope: very quick attack, exponential decay
    envelope = (1.0 - math.exp(-t * 300)) * math.exp(-t * 30)
    
    # Mix
    sample = (wave_val * 0.5 + noise * 0.5) * envelope * 0.6
    
    val = int(sample * 32767.0)
    val = max(-32768, min(32767, val))
    frames.append(struct.pack('h', val))

wave_file.writeframes(b''.join(frames))
wave_file.close()
