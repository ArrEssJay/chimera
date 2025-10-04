# QPSK Modulation

**QPSK** stands for **Quadrature Phase-Shift Keying**. It's a modulation technique that encodes 2 bits per symbol by varying the phase of the carrier wave.

## The Four QPSK States

QPSK uses four distinct phase states, each representing a unique 2-bit pattern:

```
        Q (Quadrature)
              ↑
              |
    00 •      |      • 01
              |
   -----------+-----------> I (In-phase)
              |
    11 •      |      • 10
              |
```

## Bit-to-Phase Mapping in Chimera

- `00` → Phase: 135° (upper-left quadrant)
- `01` → Phase: 45° (upper-right quadrant)
- `11` → Phase: 225° (lower-left quadrant)
- `10` → Phase: 315° (lower-right quadrant)

## Mathematical Representation

For normalized QPSK (unit energy), the four symbols are:

```
Symbol 00: I = -1/√2,  Q = +1/√2
Symbol 01: I = +1/√2,  Q = +1/√2
Symbol 11: I = -1/√2,  Q = -1/√2
Symbol 10: I = +1/√2,  Q = -1/√2
```

## Why QPSK?

- **Spectral Efficiency**: Transmits 2 bits per symbol
- **Robustness**: The large phase separation (90°) makes it resilient to noise
- **Simplicity**: Relatively simple to implement and demodulate
- **Widespread Use**: Used in many real-world systems (satellite, WiFi, LTE)

## QPSK in Chimera

Chimera's implementation:
- **Symbol Rate**: Configurable (typically 16-1000 symbols/second depending on preset)
- **Carrier Frequency**: 12.0 kHz (audio frequency for demonstration)
- **Frame Structure**: Organized into sync, command, data, and ECC sections

## See Also

- [[What Are Symbols]] - Understanding the fundamental unit
- [[IQ Representation]] - In-phase and Quadrature components
- [[Constellation Diagrams]] - Visualizing QPSK
- [[Modulation Protocol v4.2]] - Chimera's QPSK implementation details
