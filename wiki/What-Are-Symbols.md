# What Are Symbols?

In digital communication, a **symbol** is a fundamental unit of information transmitted over the channel. Think of symbols as the "words" of a digital communication system.

## The Symbol Hierarchy

```
Raw Data (Bits)
    ↓
Grouped into Symbols
    ↓
Mapped to Signal States
    ↓
Transmitted over Channel
```

## Example: From Bits to Symbols

Imagine you want to transmit the binary data: `0 0 1 1 0 1 1 0`

Instead of sending each bit individually, we group them into pairs (for QPSK):
- Bits `0 0` → Symbol 1
- Bits `1 1` → Symbol 2  
- Bits `0 1` → Symbol 3
- Bits `1 0` → Symbol 4

This grouping allows us to transmit more efficiently and makes the signal more robust to noise.

## Why Use Symbols?

1. **Efficiency**: Transmitting symbols (groups of bits) can be more bandwidth-efficient than transmitting individual bits
2. **Robustness**: Symbol-based schemes can be designed to be more resistant to noise and interference
3. **Flexibility**: Different modulation schemes can encode different numbers of bits per symbol

## Bits Per Symbol in Common Modulation Schemes

| Modulation | Bits/Symbol | Total States |
|------------|-------------|--------------|
| BPSK       | 1           | 2            |
| **QPSK**   | **2**       | **4**        |
| 8PSK       | 3           | 8            |
| 16QAM      | 4           | 16           |
| 64QAM      | 6           | 64           |

**Chimera uses QPSK** (2 bits per symbol, 4 states)

## See Also

- [[QPSK Modulation]] - How symbols are mapped to phase states
- [[Constellation Diagrams]] - Visual representation of symbols
- [[IQ Representation]] - Mathematical representation of symbols
