# Raman Whisper Modulation Protocol v4.2

- **FILE**: `chimera/modulation_protocol_v4.2.dat`
- **DOCUMENT TITLE**: Raman Whisper Modulation Protocol v4.2
- **LAST REVISION**: 2025-01-20

---

## Modulation of the 12.0 kHz Auditory Carrier

The core "sound" is a 12.0 kHz sine wave. The Gnostic "data" is not encoded on the THz carrier; it is encoded on this auditory-range carrier, which is then amplitude-modulated onto the THz "whisper signal."

## Modulation Parameters

- **Modulation Scheme**: Quadrature Phase-Shift Keying (QPSK) with a nested, low-rate Frequency-Shift Keying (FSK) layer. This is a non-standard protocol optimized for psychoacoustic impact, not data throughput.

### QPSK Layer (Framing and Symbol Rate)

- **Symbol Rate**: 16 symbols per second. Optimized for stability and bypassing cognitive filters.
- **Bandwidth**: Approximately 20 Hz, centered on the 12.0 kHz carrier.
- **Framing**: Transmitted in 128-symbol frames.
    - 16-symbol synchronization sequence.
    - 16-symbol Target ID (unique Gnostic signature).
    - 16-symbol Command Type (e.g., "Gnostic Inject," "Ontological Nudge," "Surveillance Query").
    - 64-symbol data payload.
    - 16-symbol error-checking code.

### FSK Layer (Gnostic Data)

- The 12.0 kHz carrier is deliberately "dithered" using a binary FSK.
- **Frequency for binary "0"**: 11,999 Hz.
- **Frequency for binary "1"**: 12,001 Hz.
- **Data Rate**: 1 bit per second. The slowness is intentional, designed for subconscious informational osmosis.
