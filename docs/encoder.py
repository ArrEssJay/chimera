# --- 1. SETUP: Install required audio library ---
!pip install soundfile

# --- 2. Import necessary libraries ---
import numpy as np
import soundfile as sf
from scipy.signal import butter, filtfilt
import os

# Import Colab-specific library for file handling
# This will cause an error if not run in a Google Colab environment.
# If running locally, you can remove this import and the files.download call.
from google.colab import files

# ==============================================================================
# --- USER CONFIGURATION ---
# ==============================================================================
# Set the desired parameters for the output audio files.

DURATION_SECONDS = 120      # Duration of the generated signal in seconds (default: 2 minutes)
SAMPLE_RATE = 48000         # Sample rate in Hz (default: 48kHz)
BIT_DEPTH = 'FLOAT'         # Bit depth. Options: 'PCM_16', 'PCM_24', 'PCM_32', 'FLOAT' (32-bit)
PLAINTEXT_SOURCE = "May this bring a little peace to a troubled world." # Data payload source

# ==============================================================================
# --- Raman Whisper Modulation Protocol v4.2 Parameters ---
# These parameters are taken directly from the technical specification.
# ==============================================================================
# Carrier Parameters
CARRIER_FREQ_HZ = 12000.0   # 12.0 kHz Auditory Carrier

# QPSK Layer (Framing and Symbol Rate)
QPSK_SYMBOL_RATE = 16       # 16 symbols per second
QPSK_BANDWIDTH_HZ = 20      # Approximate bandwidth, centered on the carrier

# QPSK Frame Structure (in symbols)
FRAME_TOTAL_SYMBOLS = 128
SYNC_SYMBOLS = 16
TARGET_ID_SYMBOLS = 16
COMMAND_TYPE_SYMBOLS = 16
DATA_PAYLOAD_SYMBOLS = 64
ECC_SYMBOLS = 16

# FSK Layer (Gnostic Data)
FSK_BIT_RATE = 1.0          # 1 bit per second
FSK_FREQ_ZERO_HZ = 11999.0  # Frequency for binary "0"
FSK_FREQ_ONE_HZ = 12001.0   # Frequency for binary "1"
# Calculated frequency deviation for FSK
FSK_FREQ_DEVIATION_HZ = (FSK_FREQ_ONE_HZ - CARRIER_FREQ_HZ)

# ==============================================================================
# --- Helper Functions ---
# ==============================================================================

def string_to_bitstream(text):
    """Converts a UTF-8 string to a numpy array of bits."""
    byte_array = text.encode('utf-8')
    return np.unpackbits(np.frombuffer(byte_array, dtype=np.uint8))

def generate_frame_components():
    """Generates fixed bit sequences for frame overhead."""
    # Using fixed seeds for reproducibility, as these should be constant sequences.
    rng = np.random.RandomState(seed=42)
    sync_bits = rng.randint(0, 2, SYNC_SYMBOLS * 2)
    target_id_bits = rng.randint(0, 2, TARGET_ID_SYMBOLS * 2)
    command_type_bits = rng.randint(0, 2, COMMAND_TYPE_SYMBOLS * 2)
    ecc_bits = rng.randint(0, 2, ECC_SYMBOLS * 2)
    return sync_bits, target_id_bits, command_type_bits, ecc_bits

def build_full_bitstream(payload_bits, num_frames):
    """Constructs the complete bitstream by assembling frames."""
    sync_bits, target_id_bits, command_type_bits, ecc_bits = generate_frame_components()
    
    payload_bits_per_frame = DATA_PAYLOAD_SYMBOLS * 2 # 2 bits per QPSK symbol
    
    # Repeat payload bits if not enough for the full duration
    if len(payload_bits) < payload_bits_per_frame * num_frames:
        repeats_needed = int(np.ceil((payload_bits_per_frame * num_frames) / len(payload_bits)))
        payload_bits = np.tile(payload_bits, repeats_needed)
        
    all_frames_bits = []
    for i in range(num_frames):
        start = i * payload_bits_per_frame
        end = start + payload_bits_per_frame
        current_payload = payload_bits[start:end]
        
        # Assemble the frame according to the spec
        frame = np.concatenate([
            sync_bits,
            target_id_bits,
            command_type_bits,
            current_payload,
            ecc_bits
        ])
        all_frames_bits.append(frame)
        
    return np.concatenate(all_frames_bits)
    
# ==============================================================================
# --- Main Generation Logic ---
# ==============================================================================

def generate_modulated_audio(duration, sample_rate, bit_depth):
    """Generates the final audio file based on the v4.2 spec."""
    try:
        num_samples = int(duration * sample_rate)
        
        print("Initializing Raman Whisper Modulation Protocol v4.2 Encoder...")
        print(f"Configuration: {duration}s duration, {sample_rate}Hz sample rate, {bit_depth} format.")

        # --- 1. Construct the full bitstream from frames ---
        print("Building frames and constructing bitstream...")
        total_symbols_needed = int(np.ceil(duration * QPSK_SYMBOL_RATE))
        num_frames_needed = int(np.ceil(total_symbols_needed / FRAME_TOTAL_SYMBOLS))
        
        source_payload_bits = string_to_bitstream(PLAINTEXT_SOURCE)
        qpsk_bitstream = build_full_bitstream(source_payload_bits, num_frames_needed)
        
        # --- 2. FSK Layer Synthesis (Frequency Dithering) ---
        print("Synthesizing FSK layer...")
        fsk_bits_needed = int(np.ceil(duration * FSK_BIT_RATE))
        # The FSK layer uses the same data source, but at a much lower rate
        fsk_bits = qpsk_bitstream[:fsk_bits_needed] 
        fsk_bits_signed = fsk_bits.astype(np.int32) * 2 - 1 # Convert 0,1 to -1,1
        
        fsk_freq_offsets = fsk_bits_signed * FSK_FREQ_DEVIATION_HZ
        samples_per_fsk_bit = int(sample_rate / FSK_BIT_RATE)
        
        instantaneous_freq = CARRIER_FREQ_HZ + np.repeat(fsk_freq_offsets, samples_per_fsk_bit)[:num_samples]

        # --- 3. QPSK Layer Synthesis (Phase Modulation) ---
        print("Synthesizing QPSK layer...")
        qpsk_bits_needed = total_symbols_needed * 2
        qpsk_bit_pairs = qpsk_bitstream[:qpsk_bits_needed].reshape(-1, 2)
        
        # Standard Gray-coded QPSK mapping: (0,0)->+45deg, (0,1)->+135deg, (1,1)->-135deg, (1,0)->-45deg
        qpsk_symbol_map = {(0,0): 1, (0,1): 0, (1,1): 2, (1,0): 3} 
        qpsk_symbols = np.array([qpsk_symbol_map[tuple(p)] for p in qpsk_bit_pairs])
        qpsk_phase_map = np.array([1, 0, 2, 3]) * (np.pi / 2) + (np.pi / 4) # Maps symbols to phase angles
        
        qpsk_phases = qpsk_phase_map[qpsk_symbols]
        samples_per_qpsk_symbol = int(sample_rate / QPSK_SYMBOL_RATE)
        instantaneous_phase_qpsk_raw = np.repeat(qpsk_phases, samples_per_qpsk_symbol)[:num_samples]

        # Filter the phase signal to limit bandwidth to ~20 Hz as per spec
        b, a = butter(4, QPSK_BANDWIDTH_HZ / (0.5 * sample_rate), btype='low')
        # To avoid discontinuities at 0/2pi, filter the Cartesian components and reconstruct the angle
        cos_phase_filtered = filtfilt(b, a, np.cos(instantaneous_phase_qpsk_raw))
        sin_phase_filtered = filtfilt(b, a, np.sin(instantaneous_phase_qpsk_raw))
        instantaneous_phase_qpsk_smoothed = np.arctan2(sin_phase_filtered, cos_phase_filtered)
        
        # --- 4. Combine FSK and QPSK layers to generate final signal ---
        print("Combining layers and generating final audio...")
        # The FSK part is integrated to become the carrier phase
        fsk_phase = 2 * np.pi * np.cumsum(instantaneous_freq) / sample_rate
        # The QPSK part is the phase offset added to the carrier phase
        total_phase = fsk_phase + instantaneous_phase_qpsk_smoothed
        
        # Generate the final sine wave signal
        final_signal = np.sin(total_phase)
        
        # Normalize to prevent clipping before writing to file
        final_signal /= np.max(np.abs(final_signal)) * 1.01

        # --- 5. Write to file and trigger download ---
        output_filename = "raman_whisper_v4.2_modulated_audio.wav"
        sf.write(output_filename, final_signal, sample_rate, subtype=bit_depth)
        print(f"\nSuccessfully generated: {output_filename}")

        print("Triggering download...")
        files.download(output_filename)

    except NameError:
        print("\nERROR: 'google.colab' not found. This script is intended to be run in a Google Colab environment.")
        print("If you are running this locally, please remove the 'google.colab' import and the 'files.download' call.")
    except Exception as e:
        print(f"\nAn unexpected error occurred during processing: {e}")

# --- START EXECUTION ---
generate_modulated_audio(DURATION_SECONDS, SAMPLE_RATE, BIT_DEPTH)