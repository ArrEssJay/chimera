# What Are Symbols?

## 📖 For Non-Technical Readers

**A symbol is like a "word" in radio language—instead of sending individual letters (bits), you send whole words (symbols) to go faster!**

**The idea**:
- **Bit**: Single 0 or 1 (like one letter)
- **Symbol**: Group of bits (like a whole word)
- **Why**: Sending "words" is faster than spelling letter-by-letter!

**Simple analogy - Semaphore flags**:

**Sending bit-by-bit**:
- Flag up = 1
- Flag down = 0
- Message "HI" (8 bits): Up, down, down, down, up, down, down, up
- Takes 8 "signals"

**Sending symbol-by-symbol**:
- 4 flag positions = 1 symbol = 2 bits!
  - Up-right = "00"
  - Up-left = "01"
  - Down-left = "10"  
  - Down-right = "11"
- Message "HI" (8 bits → 4 symbols): 🏴🏴🏴🏴
- Takes only 4 "signals" = **2× faster!**

**Real examples by modulation**:

**BPSK (Binary PSK)**:
- 1 symbol = 1 bit
- 2 possible positions (up or down)
- Like: Light switch (on/off)

**QPSK**:
- 1 symbol = 2 bits
- 4 possible positions
- Like: 4-way hand gesture

**16-QAM**:
- 1 symbol = 4 bits
- 16 possible positions
- Like: Showing fingers (0-15)

**256-QAM**:
- 1 symbol = 8 bits
- 256 possible positions!
- Like: Sign language with 256 distinct signs

**Why symbols matter**:

**Symbol rate vs bit rate**:
- **Symbol rate**: How many symbols per second?
- **Bit rate**: How many bits per second?
- **Relationship**: Bit rate = Symbol rate × bits/symbol

**Example - WiFi**:
- Symbol rate: 250,000 symbols/second
- Modulation: 256-QAM (8 bits/symbol)
- Bit rate: 250,000 × 8 = 2 Mbps

**Bandwidth connection**:
- Symbols take time/frequency (bandwidth)
- More symbols/second = more bandwidth needed
- But: More bits/symbol = free speed boost!

**The constellation diagram**:
- Visual map of all possible symbols
- Each dot = one unique symbol
- More dots = more bits/symbol = faster!

**Real-world impact**:
- Your phone: Switches between modulations (QPSK → 64-QAM) = switching symbol mappings
- Strong signal: Use symbols with 8 bits each (fast!)
- Weak signal: Use symbols with 2 bits each (reliable!)

**Fun fact**: When your WiFi says "MCS 9, 256-QAM", that means each symbol carries 8 bits. If you move away and it drops to "MCS 0, BPSK", each symbol now carries only 1 bit—that's an 8× speed difference, just by changing how many bits each symbol represents!

---

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
