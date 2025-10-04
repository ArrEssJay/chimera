# Signal Chain (End-to-End Processing)

The **signal chain** is the complete path data takes from transmitter to receiver in Chimera's DSP simulation.

## Overview

```
Transmitter Side                    Channel                Receiver Side
════════════════                    ═══════                ════════════

[Data Bits]                                                  [Data Bits]
    ↓                                                            ↑
[FEC Encoder] ──→ add redundancy                         [FEC Decoder]
    ↓                                                            ↑
[Modulator] ────→ bits to symbols                       [Demodulator]
    ↓                                                            ↑
[Upconverter] ──→ baseband to RF              [Downconverter] ←┘
    ↓                                                    ↑
[TX Antenna] ───→ [Link Loss] + [AWGN Noise] ───→ [RX Antenna]
```

## Detailed Signal Chain

### 1. Data Source
**Input**: Raw information bits (0s and 1s)

```
Example: "Hello" in binary
01001000 01100101 01101100 01101100 01101111
```

**In Chimera**: Generated from presets or user input

---

### 2. FEC Encoder

**Input**: Information bits  
**Output**: Encoded bits with parity

**Purpose**: Add [[Forward Error Correction (FEC)|redundancy]] for error correction

```
Information: [1 0 1 0] (4 bits)
      ↓
[[LDPC Codes|LDPC]] Encoder (rate 1/2)
      ↓
Codeword: [1 0 1 0 1 1 0 0] (8 bits)
          └─────┘ └───────┘
           data     parity
```

**Parameters**:
- Code rate (1/2, 2/3, 3/4, etc.)
- Block length
- LDPC matrix structure

**Chimera Implementation**: `chimera-core::encoder`

---

### 3. Modulator

**Input**: Encoded bits  
**Output**: Complex symbols (I+jQ)

**Purpose**: Map bits to [[QPSK Modulation|constellation points]]

```
Bits: [0 0] → Symbol: (-1, -1) → 225°
Bits: [0 1] → Symbol: (-1, +1) → 135°
Bits: [1 0] → Symbol: (+1, -1) → 315°
Bits: [1 1] → Symbol: (+1, +1) →  45°
```

**For QPSK**: 2 bits per symbol

**[[Constellation Diagrams|Constellation]] (TX side)**:
```
      Q
      ↑
  •   |   •   ← Clean symbols
──────┼──────→ I
  •   |   •
```

**Chimera Implementation**: `chimera-core::modulation`

---

### 4. Pulse Shaping (Optional)

**Input**: Discrete symbols  
**Output**: Continuous waveform

**Purpose**: Limit bandwidth, reduce inter-symbol interference

**Filter Types**:
- Raised cosine
- Root raised cosine (most common)
- Gaussian

**Not simulated in current Chimera version** (focused on baseband)

---

### 5. Upconverter

**Input**: Baseband I/Q signal  
**Output**: RF signal at carrier frequency

**Purpose**: Shift signal to transmission frequency

```
Baseband: s(t) = I(t) + jQ(t)

RF Signal: s_RF(t) = I(t)·cos(2πf_c·t) - Q(t)·sin(2πf_c·t)

where f_c is carrier frequency (e.g., 2.4 GHz)
```

**Chimera**: Simulates baseband only (no carrier)

---

### 6. Channel Effects

#### A. Link Loss

**Input**: Transmitted signal power  
**Output**: Attenuated signal power

**Purpose**: Models distance, antenna gains, free-space path loss

```
Link Budget:
P_RX = P_TX + G_TX + G_RX - L_path - L_other

Example:
  10 dBm TX power
+ 3 dBi TX antenna gain
+ 3 dBi RX antenna gain
- 100 dB path loss
- 5 dB other losses
─────────────────────
= -89 dBm RX power
```

See: [[Link Loss vs Noise]]

**Chimera**: Simulated as amplitude scaling

---

#### B. Additive White Gaussian Noise (AWGN)

**Input**: Attenuated signal  
**Output**: Noisy signal

**Purpose**: Models thermal noise, interference

```
Received signal:
r(t) = s(t) + n(t)

where n(t) ~ N(0, σ²) is Gaussian noise
```

**[[Signal to Noise Ratio (SNR)]]** determines noise power:
```
SNR = P_signal / P_noise

σ² = P_signal / SNR
```

**Constellation (RX side with noise)**:
```
      Q
      ↑
  ☁   |  ☁   ← Noisy cloud
 ☁ ☁  | ☁ ☁
──────┼──────→ I
 ☁ ☁  | ☁ ☁
  ☁   |  ☁
```

See: [[Additive White Gaussian Noise (AWGN)]]

**Chimera Implementation**: `chimera-core::channel`

---

### 7. Downconverter

**Input**: RF signal at carrier frequency  
**Output**: Baseband I/Q signal

**Purpose**: Shift signal back to baseband for processing

```
I(t) = s_RF(t) · cos(2πf_c·t) · LPF
Q(t) = s_RF(t) · sin(2πf_c·t) · LPF

where LPF is low-pass filter
```

**Chimera**: Not needed (baseband simulation)

---

### 8. Matched Filter (Optional)

**Input**: Noisy continuous waveform  
**Output**: Discrete symbol samples

**Purpose**: Maximize [[Signal to Noise Ratio (SNR)|SNR]] before sampling

**Not simulated in current Chimera**

---

### 9. Demodulator

**Input**: Noisy symbols (I+jQ)  
**Output**: Soft bit estimates (LLRs)

**Purpose**: Convert symbols back to bits with reliability info

**Hard Decision**:
```
Symbol: (0.8, 0.7) → Bits: [1 1] → Confidence: High ✓
Symbol: (0.1, -0.05) → Bits: [0 0] → Confidence: Low ⚠
```

**Soft Decision (Log-Likelihood Ratios)**:
```
LLR > 0 → Likely a '1' bit
LLR < 0 → Likely a '0' bit
|LLR| = Confidence level

Example:
LLR = +5.2 → Very confident '1'
LLR = -0.3 → Weak '0' (could be wrong)
```

**Soft values help FEC decoder make better decisions!**

**Chimera Implementation**: `chimera-core::demodulator`

---

### 10. FEC Decoder

**Input**: Soft bit estimates  
**Output**: Corrected information bits

**Purpose**: Fix errors using [[Forward Error Correction (FEC)|redundancy]]

**[[LDPC Codes|LDPC]] Decoding**:
1. Initialize with soft values (LLRs)
2. Iterate belief propagation (typically 50 iterations)
3. Check parity constraints
4. Converge to corrected bits

```
Pre-FEC:  [1 0 1 0 1 1 0 0] ← 2 bit errors
                ↑       ↑
LDPC Decoder (50 iterations)
                ↓
Post-FEC: [1 0 1 0 1 1 0 0] ← Errors corrected! ✅
```

**Chimera Implementation**: `chimera-core::decoder`

---

### 11. Data Sink

**Output**: Recovered information bits

**Quality Metrics**:
- [[Bit Error Rate (BER)|Pre-FEC BER]]: Errors before decoding
- [[Bit Error Rate (BER)|Post-FEC BER]]: Residual errors after decoding
- Frame Error Rate: Percentage of completely corrupted frames

```
Best case:  Post-FEC BER = 0 (perfect!) 🎉
Acceptable: Post-FEC BER < 10⁻⁶
Poor:       Post-FEC BER > 10⁻³
```

---

## Signal Chain Metrics (Chimera)

### Transmitter Metrics
- Data rate (bits/second)
- Symbol rate (symbols/second)
- Code rate
- Modulation order (bits/symbol)

### Channel Metrics
- [[Link Loss vs Noise|Link loss]] (dB)
- [[Signal to Noise Ratio (SNR)|SNR]] (dB)
- [[Energy Ratios (Es/N0 and Eb/N0)|Es/N0]] (dB per symbol)
- [[Energy Ratios (Es/N0 and Eb/N0)|Eb/N0]] (dB per bit)

### Receiver Metrics
- Pre-FEC errors
- Post-FEC errors
- [[Bit Error Rate (BER)|BER]] (pre and post)
- Frame Error Rate
- LDPC iterations
- Coding gain

---

## Signal Processing Domains

Throughout the signal chain, we work in different domains:

### Time Domain
- **What**: Signal amplitude vs time
- **Where**: Pulse shaping, waveform generation
- **Tool**: Oscilloscope

### Frequency Domain
- **What**: Signal power vs frequency
- **Where**: Spectrum analysis, filtering
- **Tool**: Spectrum analyzer

### Symbol Domain
- **What**: Discrete constellation points
- **Where**: Modulation, demodulation
- **Tool**: [[Constellation Diagrams|Constellation diagram]]

### Bit Domain
- **What**: Binary data (0s and 1s)
- **Where**: FEC encoding/decoding
- **Tool**: Error counting

---

## Processing Gain

Chimera applies **processing gain** to improve SNR:

```
Processing Gain = 10 · log₁₀(Spread Factor)

Example:
- Symbol rate: 3200 symbols/sec
- Chip rate: 32000 chips/sec
- Spread factor: 10
- Processing gain: 10 dB

Effective SNR at demodulator:
SNR_eff = SNR_channel + Processing_gain
        = -25 dB + 10 dB
        = -15 dB
```

This allows operation at much lower channel SNR!

---

## End-to-End Example

```
Input: "Hi" = 01001000 01101001 (16 bits)

1. FEC Encoder (rate 1/2):
   → 32 bits (16 data + 16 parity)

2. Modulator (QPSK, 2 bits/symbol):
   → 16 symbols

3. Channel (SNR = -18 dB):
   → Noisy symbols with errors

4. Demodulator:
   → 32 soft bits (LLRs)
   → Pre-FEC errors: 4 bits

5. LDPC Decoder (50 iterations):
   → 16 corrected bits
   → Post-FEC errors: 0 bits ✅

Output: "Hi" = 01001000 01101001 (recovered!)
```

---

## Visualization in Chimera

The web interface shows:

1. **TX Constellation**: Clean transmitted symbols
2. **RX Constellation**: Noisy received symbols (scatter plot)
3. **Error Metrics**: Pre/Post-FEC BER, frame errors
4. **SNR Controls**: Adjust noise level in real-time
5. **Processing Gain**: Shows effective SNR improvement

---

## See Also

- [[Forward Error Correction (FEC)]] - Encoding/decoding details
- [[QPSK Modulation]] - Modulation scheme
- [[Constellation Diagrams]] - Symbol visualization
- [[Signal to Noise Ratio (SNR)]] - Channel quality metric
- [[Link Loss vs Noise]] - Channel impairments
- [[Bit Error Rate (BER)]] - Performance metric
- [[Reading the Constellation]] - Practical guide to interpreting plots
