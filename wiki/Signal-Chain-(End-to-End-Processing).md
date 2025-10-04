# Signal Chain (End-to-End Processing)

## ⛓️ For Non-Technical Readers

**The signal chain is like a postal system for data—messages get packaged (encoded), addressed (modulated), mailed (transmitted), delivered (received), sorted (demodulated), and unpacked (decoded)!**

**The complete journey**:

**Transmitter Side** (📤 Sending):

**1. Raw Data**:
- Your message: "Hello!" (text, voice, video)
- Binary: 01001000 01100101 01101100...

**2. FEC Encoder** (🛡️ Add error protection):
- Like: Adding packing material to fragile package
- Adds redundancy so receiver can fix errors
- Input: 100 data bits → Output: 150 coded bits

**3. Modulator** (🎵 Convert to symbols):
- Like: Converting letter to semaphore flags
- Maps bits to radio signal patterns
- QPSK: Every 2 bits → 1 symbol (4 positions)

**4. Upconverter** (🚀 Shift to radio frequency):
- Like: Loading package onto airplane
- Shifts signal from baseband (near 0 Hz) to RF (GHz)
- Baseband 10 MHz → RF 2.4 GHz

**5. Transmit Antenna** (📡 Launch!):
- Converts electrical signal to electromagnetic waves
- Radiates into space
- Power: mW to kW depending on application

**Channel** (🌍 The dangerous journey):

**Link Loss** (📉 Signal weakens):
- Distance: Signal spreads out, gets weaker
- 2× distance = 1/4 power (inverse square law)

**Noise** (⚡ Random interference):
- Thermal noise (electronics are warm)
- Interference (other transmitters)
- Cosmic background radiation
- Like: Static on old radio

**Fading** (🌊 Signal fluctuates):
- Multipath: Echoes interfere
- Obstacles: Buildings, trees block signal
- Movement: Doppler shift
- Like: Sound echoing in canyon

**Receiver Side** (📥 Receiving):

**6. Receive Antenna** (📡 Catch!):
- Collects weak electromagnetic waves
- Converts to electrical signal
- Received power: Often 10^-12 watts (pW)!

**7. Downconverter** (📦 Unload from airplane):
- Shifts RF signal back to baseband
- RF 2.4 GHz → Baseband 10 MHz
- Now can process with DSP

**8. Synchronizer** (⏰ Align timing/frequency):
- Like: Aligning decoder ring
- Matches receiver clock to transmitter
- Corrects frequency/phase/timing offsets

**9. Equalizer** (🎸 Undo channel distortion):
- Like: Uncrumpling package
- Reverses distortion from multipath/fading
- Makes constellation clean again

**10. Demodulator** (📊 Read symbols):
- Like: Reading semaphore flags
- Converts received symbols back to bits
- Makes "soft decisions" (probabilities)

**11. FEC Decoder** (🤓 Fix errors):
- Like: Piecing together damaged package
- Uses redundancy to detect and correct errors
- Output: Clean data bits (hopefully!)

**12. Recovered Data** (✅ Success!):
- Your message: "Hello!" 
- Delivered successfully (if BER low enough)

**Real-world example - Sending emoji via WiFi**:

1. You type 😀 (1 byte = 8 bits)
2. **FEC**: Encoded to 12 bits (50% overhead)
3. **Modulator**: 12 bits → 6 QPSK symbols
4. **Upconverter**: Shifted to 2.4 GHz
5. **TX antenna**: Radiated at 100 mW
6. **Channel**: Signal travels 10 meters, weakened to 10 µW
7. **RX antenna**: Laptop antenna collects 1 µW
8. **Downconverter**: Back to baseband
9. **Synchronize**: Align timing (4 µs correction)
10. **Equalize**: Undo multipath from walls
11. **Demodulate**: Recover 12 bits (2 errors!)
12. **FEC decode**: Fix 2 errors → perfect 8 bits
13. You see: 😀

All in 0.00024 seconds! ⚡

**Why understanding the chain matters**:
- **Weak signal?** Check: Antenna, link budget, path loss
- **Lots of errors?** Check: Noise, FEC, modulation
- **Intermittent?** Check: Fading, synchronization
- **Each block** can be optimized independently!

**Fun fact**: A typical WiFi packet goes through this entire signal chain (11+ processing blocks) in under 1 millisecond. Your laptop's WiFi chip performs billions of calculations per second to execute these steps in real-time!

---

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

**Purpose**: Add [[Forward-Error-Correction-(FEC)]] for error correction

```
Information: [1 0 1 0] (4 bits)
      ↓
[[LDPC-Codes]] Encoder (rate 1/2)
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

**Purpose**: Map bits to [[QPSK-Modulation]]

```
Bits: [0 0] → Symbol: (-1, -1) → 225°
Bits: [0 1] → Symbol: (-1, +1) → 135°
Bits: [1 0] → Symbol: (+1, -1) → 315°
Bits: [1 1] → Symbol: (+1, +1) →  45°
```

**For QPSK**: 2 bits per symbol

**[[Constellation-Diagrams]] (TX side)**:
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

See: [[Link-Loss-vs-Noise]]

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

**[[Signal-to-Noise-Ratio-(SNR)]]** determines noise power:
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

See: [[Additive-White-Gaussian-Noise-(AWGN)]]

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

**Purpose**: Maximize [[Signal-to-Noise-Ratio-(SNR)]] before sampling

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

**Purpose**: Fix errors using [[Forward-Error-Correction-(FEC)]]

**[[LDPC-Codes]] Decoding**:
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
- [[Bit-Error-Rate-(BER)]]: Errors before decoding
- [[Bit-Error-Rate-(BER)]]: Residual errors after decoding
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
- [[Link-Loss-vs-Noise]] (dB)
- [[Signal-to-Noise-Ratio-(SNR)]] (dB)
- [[Energy-Ratios-(Es-N0-and-Eb-N0)]] (dB per symbol)
- [[Energy-Ratios-(Es-N0-and-Eb-N0)]] (dB per bit)

### Receiver Metrics
- Pre-FEC errors
- Post-FEC errors
- [[Bit-Error-Rate-(BER)]] (pre and post)
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
- **Tool**: [[Constellation-Diagrams]]

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

- [[Forward-Error-Correction-(FEC)]] - Encoding/decoding details
- [[QPSK-Modulation]] - Modulation scheme
- [[Constellation-Diagrams]] - Symbol visualization
- [[Signal-to-Noise-Ratio-(SNR)]] - Channel quality metric
- [[Link-Loss-vs-Noise]] - Channel impairments
- [[Bit-Error-Rate-(BER)]] - Performance metric
<!-- - Reading the Constellation - Practical guide to interpreting plots (Coming Soon) -->
