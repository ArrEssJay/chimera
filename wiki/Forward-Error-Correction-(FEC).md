# Forward Error Correction (FEC)

## 🛡️ For Non-Technical Readers

**FEC is like sending instructions with built-in spell-checker clues—even if some words get garbled, you can figure out what was meant!**

**The problem**: 
- Radio channels add noise/interference
- Bits get flipped: 1→0 or 0→1
- By the time you notice, it's too late to ask "can you repeat that?"

**The FEC solution - Add smart redundancy**:

**Simple example - Repetition**:
- Want to send: `1`
- FEC adds redundancy: `1 1 1` (send it 3 times)
- Received: `1 0 1` (middle bit corrupted!)
- Decoder: "Two 1s, one 0? Probably was `1`!" ✅

**Real codes are smarter**:
- **Hamming code**: 4 data bits + 3 parity bits = fix any single error
- **Reed-Solomon**: Used in CDs, DVDs, QR codes—can fix burst errors!
- **Turbo/LDPC codes**: Used in 4G/5G—nearly as good as theoretical limit!

**Real-world examples**:
- **QR codes**: Can work with 30% missing—that's FEC!
- **Satellite TV**: Signal is weak, so FEC fixes errors without re-sending
- **Deep space probes**: Can't retransmit from Mars! FEC is critical
- **Your WiFi**: Uses LDPC codes to fix interference from microwave ovens

**Trade-off**:
- More redundancy = fix more errors BUT slower transmission
- Your phone adjusts: strong signal = less FEC (fast), weak signal = more FEC (reliable)

**Fun fact**: The Voyager probes launched in 1977 still communicate with Earth (15+ billion miles away!) using FEC codes to fix the incredibly noisy signal.

---

**Forward Error Correction (FEC)** adds redundancy to transmitted data so the receiver can detect and correct errors without retransmission.

## How FEC Works

```
Original Data:     [100 bits]
      ↓
FEC Encoder:       [Add redundancy]
      ↓
Encoded Data:      [150 bits with parity]
      ↓
Noisy Channel:     [Errors introduced]
      ↓
FEC Decoder:       [Correct errors using redundancy]
      ↓
Recovered Data:    [100 bits, hopefully error-free]
```

## Key Concept

FEC trades **bandwidth efficiency** for **reliability**:
- **More redundancy** = better error correction but lower data rate
- **Less redundancy** = higher data rate but less error protection

## Code Rate

The **code rate** is the ratio of information bits to total bits:

```
Code Rate = Information Bits / Total Bits

Example: 
- 100 information bits
- 50 parity bits
- Total: 150 bits
- Code Rate: 100/150 = 2/3
```

Common code rates:
- **1/2**: Very robust, 50% overhead
- **2/3**: Good balance
- **3/4**: Efficient, less redundancy
- **5/6**: High efficiency, minimal redundancy

## Types of FEC Codes

### Block Codes
- Process fixed-size blocks of data
- Examples: Hamming, Reed-Solomon, BCH, **LDPC**

### Convolutional Codes
- Process continuous stream of data
- Examples: Viterbi, Turbo codes

### Modern Codes
- **LDPC**: Low-Density Parity-Check (used in Chimera!)
- **Turbo**: Parallel concatenated codes
- **Polar**: Capacity-achieving codes

## FEC Gain (Coding Gain)

**Coding Gain** measures the SNR improvement provided by FEC:

```
Without FEC: Need SNR = 10 dB for BER = 10⁻⁶
With FEC:    Need SNR = 2 dB for BER = 10⁻⁶

Coding Gain = 10 dB - 2 dB = 8 dB
```

This means FEC saves 8 dB of transmit power!

## FEC Performance Metrics

### 1. Coding Gain
How much SNR improvement at a given BER

### 2. Error Floor
Minimum BER achievable (implementation limits)

### 3. Waterfall Region
SNR range where BER drops rapidly

### 4. Threshold
SNR where FEC "turns on" effectively

```
BER Curve with FEC:

BER
 ↑
10⁰ |•
    |  ••
10⁻³ |    ••        ← Waterfall
    |      ••         Region
10⁻⁶ |        ••___  ← Threshold
    |            ╲
10⁻⁹ |             ╲___ ← Error Floor
    +───────────────────→ SNR (dB)
     Threshold
```

## FEC in Chimera

In Chimera's simulation:

### 1. Encoder
- Applies [[LDPC-Codes]] encoding to payload data
- Adds parity symbols for error correction

### 2. Channel
- [[Additive-White-Gaussian-Noise-(AWGN)]] corrupts the transmitted symbols

### 3. Decoder
- Iterative LDPC decoder corrects errors
- Uses belief propagation algorithm

### 4. Metrics
- **ECC symbols**: Number of parity/redundancy symbols added
- **Pre-FEC errors**: Errors at the demodulator output  
- **Post-FEC errors**: Residual errors after LDPC correction
- **Frame Error Rate**: Percentage of frames that couldn't be fully corrected

## Example Performance

```
Channel Conditions:
- SNR: -15 dB
- BER without FEC: 10⁻² (1%)

After LDPC FEC:
- Code rate: 1/2
- Iterations: 50
- Post-FEC BER: 10⁻⁶ (0.0001%)

Result: 40 dB coding gain! 🎉
```

## Why FEC is Essential

### Deep Space Communications
- Very weak signals (high link loss)
- Retransmission impractical (minutes of delay)
- FEC allows operation at lower SNR

### Satellite Links
- Power-limited transmitters
- FEC saves precious transmit power
- Standard in DVB-S2, GPS

### Mobile Communications
- Fading channels, interference
- FEC provides robustness
- Used in LTE, 5G, WiFi

### Data Storage
- Bits can flip over time
- FEC protects against corruption
- Used in SSDs, hard drives, QR codes

## Shannon Limit

**Claude Shannon** proved there's a theoretical limit to how much FEC can help:

```
Shannon Capacity:
C = B · log₂(1 + SNR)

where:
- C is channel capacity (bits/second)
- B is bandwidth (Hz)
- SNR is signal-to-noise ratio (linear)
```

**Modern codes like LDPC approach this limit!**

## Trade-offs

### Advantages
- ✅ Corrects errors without retransmission
- ✅ Improves reliability at low SNR
- ✅ Saves transmit power
- ✅ Enables communication over poor channels

### Disadvantages
- ❌ Adds latency (encoding/decoding delay)
- ❌ Requires computational resources
- ❌ Reduces effective data rate (overhead)
- ❌ More complex implementation

## See Also

- [[LDPC-Codes]] - Specific FEC code used in Chimera
- [[Bit-Error-Rate-(BER)]] - What FEC improves
- [[Signal-to-Noise-Ratio-(SNR)]] - FEC enables lower SNR operation
<!-- - Understanding BER Curves - Visualizing FEC performance (Coming Soon) -->
