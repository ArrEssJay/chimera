# Amplitude-Shift Keying (ASK)

[[Home]] | **Digital Modulation** | [[On-Off-Keying-(OOK)]] | [[QPSK-Modulation]]

---

## 💡 For Non-Technical Readers

**ASK is like using a dimmer switch on a flashlight—bright = 1, dim = 0. Simple to understand, but noise makes it hard to tell bright from dim!**

**The idea - Vary the volume**:
- Want to send **1**? Transmit at **full power**
- Want to send **0**? Transmit at **low power** (or off)
- Receiver measures: "How loud is the signal right now?"

**Real-world analogy - Theater lights**:
- **Full brightness** = data bit "1"
- **Dim/off** = data bit "0"
- Audience (receiver) can see which state you're in
- But if room is smoky (noise), hard to tell bright from dim!

**Why it's problematic**:
- **Noise affects amplitude**: Interference makes signal stronger/weaker
- **Fading affects amplitude**: Obstacles change signal strength
- Hard to tell: "Is this a dim signal, or a faded bright signal?"
- This is why ASK isn't used much in modern systems!

**Where ASK is still used**:
- **RFID tags**: Extremely simple, low power (backscatter modulation)
- **Fiber optic**: Optical fiber has low noise, so ASK works well
- **Historical modems**: Old 300 baud modems used ASK
- **Combined with PSK**: QAM = ASK + PSK together (best of both!)

**Advanced: M-ary ASK**:
- Instead of 2 levels (on/off), use 4, 8, or 16 levels
- **4-ASK**: Four brightness levels = 2 bits/symbol
- **8-ASK**: Eight brightness levels = 3 bits/symbol
- Even more sensitive to noise!

**Why phase (PSK) or frequency (FSK) is better**:
- **PSK**: Noise changes amplitude, phase stays stable
- **FSK**: Noise changes amplitude, frequency stays stable
- **ASK**: Noise directly corrupts the information! ☹️

**Fun fact**: Your TV remote uses a form of ASK—infrared LED blinks on/off to send button codes. It works because the path from remote to TV is short and clean (low noise)!

---

## Overview

**Amplitude-Shift Keying (ASK)** encodes digital data by varying the **amplitude** of a carrier wave.

**Principle**: Different symbols represented by different amplitude levels

**M-ary ASK**: M distinct amplitude levels encode $\log_2(M)$ bits per symbol

**Relationship to OOK**: OOK is binary ASK (M=2, one amplitude is zero)

---

## Binary ASK (2-ASK)

**Two amplitude levels**: $A_1$ and $A_2$ (typically 0 and A)

**Signal**:

$$
s(t) = \begin{cases}
A \cos(2\pi f_c t) & \text{bit = 1} \\
0 & \text{bit = 0}
\end{cases}
$$

**This is On-Off Keying (OOK)**

**See**: [[On-Off-Keying-(OOK)]]

---

## M-ary ASK

**M amplitude levels**: $A_m$ for $m = 0, 1, \ldots, M-1$

**Signal for symbol** $m$:

$$
s_m(t) = A_m \cos(2\pi f_c t), \quad 0 \leq t < T_s
$$

**Amplitude levels** (equally spaced):

$$
A_m = A_0 + m \cdot \Delta A
$$

Where:
- $A_0$ = Minimum amplitude (often 0)
- $\Delta A$ = Amplitude spacing
- $m$ = Symbol index (0 to M-1)

---

### 4-ASK Example

**4 amplitude levels**: 0, A, 2A, 3A

**Bits per symbol**: $\log_2(4) = 2$ bits

**Mapping**:

| Bits | Symbol | Amplitude |
|------|--------|-----------|
| 00 | 0 | 0 |
| 01 | 1 | A |
| 10 | 2 | 2A |
| 11 | 3 | 3A |

**Constellation diagram** (1D):
```
      0   A   2A  3A
      |   |   |   |
     [•] [•] [•] [•]
```

---

### 8-ASK Example

**8 amplitude levels**: 0, A, 2A, ..., 7A

**Bits per symbol**: $\log_2(8) = 3$ bits

**Spectral efficiency**: 3 bits/symbol (3× binary ASK for same bandwidth)

---

## Modulation & Demodulation

### ASK Modulator

**Block diagram**:
```
Data bits --> [Serial to     --> [DAC] --> [×] --> ASK signal
               Parallel (log2M)]              |
                                          cos(2πf_c t)
```

**Steps**:
1. Group bits into symbols ($\log_2(M)$ bits per symbol)
2. Map symbol to amplitude level $A_m$
3. Multiply amplitude by carrier

**Baseband equivalent**:

$$
s(t) = A_m(t) \cos(2\pi f_c t)
$$

Where $A_m(t)$ = Pulse-shaped amplitude sequence

---

### Coherent Demodulation

**Optimal detector** (requires carrier phase reference):

**Receiver**:
```
ASK signal --> [×] --> [LPF] --> [Sample] --> [Threshold] --> Bits
                |                   ↓          Detector
           cos(2πf_c t)          t = kT_s
```

**Steps**:
1. Multiply by synchronized carrier (coherent mixing)
2. Low-pass filter → Baseband amplitude
3. Sample at symbol rate
4. Threshold detection (compare to M-1 thresholds)

**Decision thresholds**:

$$
\text{Threshold}_k = \frac{A_k + A_{k+1}}{2}, \quad k = 0, 1, \ldots, M-2
$$

**Example (4-ASK)**: Thresholds at A/2, 3A/2, 5A/2

---

### Non-Coherent Demodulation

**Envelope detector** (no phase reference needed):

**Block diagram**:
```
ASK signal --> [Envelope] --> [Sample] --> [Threshold] --> Bits
               Detector         ↓          Detector
                             t = kT_s
```

**Envelope detector**: Rectifier + low-pass filter (extracts |s(t)|)

**Advantage**: Simple, no carrier recovery

**Disadvantage**: ~3 dB worse SNR than coherent

**Used in**: AM radio, OOK (IR remote, RFID)

---

## Signal Space Representation

**1-dimensional constellation** (real axis only):

$$
s_m(t) = A_m \phi(t)
$$

Where $\phi(t) = \sqrt{\frac{2}{T_s}} \cos(2\pi f_c t)$ (orthonormal basis)

**Energy per symbol**:

$$
E_m = \int_0^{T_s} s_m^2(t) dt = A_m^2
$$

**Average symbol energy** (assuming equal probability):

$$
\bar{E}_s = \frac{1}{M} \sum_{m=0}^{M-1} A_m^2
$$

---

## Performance Analysis

### Bit Error Rate (BER) - Coherent Detection

**Symbol error rate** (SER) for M-ASK in AWGN:

$$
P_s \approx 2\left(1 - \frac{1}{M}\right) Q\left(\sqrt{\frac{6\log_2(M)}{M^2 - 1} \cdot \frac{E_b}{N_0}}\right)
$$

**Gray coding assumed** (SER ≈ BER / $\log_2(M)$):

$$
\text{BER} \approx \frac{2}{M\log_2(M)}\left(1 - \frac{1}{M}\right) Q\left(\sqrt{\frac{6\log_2(M)}{M^2 - 1} \cdot \frac{E_b}{N_0}}\right)
$$

**Where**: $Q(x) = \frac{1}{\sqrt{2\pi}} \int_x^\infty e^{-t^2/2} dt$

---

### BER for Specific M

**2-ASK (OOK)** @ 10 dB Eb/N0:

$$
\text{BER} = Q\left(\sqrt{2 \cdot 10}\right) \approx 3.9 \times 10^{-6}
$$

**4-ASK** @ 10 dB Eb/N0:

$$
\text{BER} \approx \frac{3}{4} Q\left(\sqrt{\frac{12}{15} \cdot 10}\right) = 0.75 \times Q(2.83) \approx 1.8 \times 10^{-3}
$$

**Observation**: Higher M → Worse BER (smaller amplitude spacing → more noise sensitivity)

---

### Required Eb/N0 for BER = 10⁻⁶

| Modulation | Required Eb/N0 (dB) | Notes |
|------------|---------------------|-------|
| **2-ASK (OOK)** | 10.5 | Baseline |
| **4-ASK** | 14 | +3.5 dB penalty |
| **8-ASK** | 18 | +7.5 dB penalty |
| **16-ASK** | 22 | +11.5 dB penalty |

**Pattern**: Each doubling of M adds ~3.5-4 dB penalty

---

## Power Efficiency

**Average power** for M-ASK:

$$
\bar{P} = \frac{1}{M} \sum_{m=0}^{M-1} \frac{A_m^2}{2}
$$

**Peak-to-average power ratio (PAPR)**:

$$
\text{PAPR} = \frac{A_{\max}^2}{\bar{P}}
$$

**For M-ASK with amplitudes 0, A, 2A, ..., (M-1)A**:

$$
\text{PAPR} = \frac{(M-1)^2}{\frac{1}{M}\sum_{m=0}^{M-1} m^2} = \frac{3(M-1)^2}{M(M-1)(2M-1)/3} = \frac{3(M-1)}{2M-1}
$$

**Example**:
- 2-ASK: PAPR = 3 (4.8 dB)
- 4-ASK: PAPR = 9/7 ≈ 1.1 dB
- 8-ASK: PAPR = 21/15 = 1.4 (1.5 dB)

**Problem**: High PAPR stresses power amplifier (requires large backoff)

---

## Bandwidth Efficiency

**Occupied bandwidth** (with pulse shaping):

$$
B = (1 + \alpha) R_s \quad (\text{Hz})
$$

Where:
- $R_s$ = Symbol rate (symbols/sec)
- $\alpha$ = Roll-off factor (0-1, typically 0.35)

**Bit rate**:

$$
R_b = R_s \log_2(M) \quad (\text{bits/sec})
$$

**Spectral efficiency**:

$$
\eta = \frac{R_b}{B} = \frac{\log_2(M)}{1 + \alpha} \quad (\text{bits/sec/Hz})
$$

---

### Comparison

| Modulation | Bits/symbol | η (α=0.35) | Notes |
|------------|-------------|------------|-------|
| **2-ASK** | 1 | 0.74 | Same as BPSK |
| **4-ASK** | 2 | 1.48 | 2× bandwidth efficiency |
| **8-ASK** | 3 | 2.22 | 3× bandwidth efficiency |
| **16-ASK** | 4 | 2.96 | 4× bandwidth efficiency |

**Trade-off**: Higher η but worse BER (need higher SNR)

---

## ASK vs PSK vs QAM

**At same bit rate**:

| Modulation | Bits/symbol | Constellation | BER (@ 10 dB) | Notes |
|------------|-------------|---------------|---------------|-------|
| **BPSK** | 1 | 2 points (1D) | 3.9 × 10⁻⁶ | Best BER |
| **2-ASK** | 1 | 2 points (1D) | 3.9 × 10⁻⁶ | Same as BPSK |
| **QPSK** | 2 | 4 points (2D) | 3.9 × 10⁻⁶ | 2× efficiency, same BER |
| **4-ASK** | 2 | 4 points (1D) | 1.8 × 10⁻³ | Worse BER (1D) |
| **16-QAM** | 4 | 16 points (2D) | ~10⁻⁴ | Better than 16-ASK |
| **16-ASK** | 4 | 16 points (1D) | ~10⁻² | Worst BER (1D) |

**Key insight**: **QAM (2D) outperforms ASK (1D)** for M > 2

**Why QAM better**: Spreads points in 2D → Larger minimum distance for same average power

---

## Practical Applications

### 1. Optical Communications (OOK)

**Binary ASK (OOK)** dominates fiber optics:
- **10 Gbps Ethernet**: OOK with direct detection
- **PON (Passive Optical Networks)**: OOK upstream
- **Simple**: LED/laser ON/OFF, photodiode detection

**Higher-order**: 4-ASK (PAM-4) emerging for 100G+ (50 Gbaud PAM-4 = 100 Gbps)

---

### 2. RFID (OOK/2-ASK)

**Passive RFID tags**: OOK modulation
- **Reader → Tag**: Continuous carrier (powers tag)
- **Tag → Reader**: Load modulation (OOK)
- **Frequency**: 125 kHz (LF), 13.56 MHz (HF), 900 MHz (UHF)

**Advantage**: Non-coherent detection (simple tag circuit)

---

### 3. Infrared (IR) Remote Controls

**Consumer IR**: OOK at 38 kHz carrier
- **Protocol**: Manchester-encoded OOK
- **Range**: 5-10 meters
- **Power**: <10 mW (eye safety)

---

### 4. DSL (Discrete Multi-Tone with ASK)

**ADSL/VDSL**: Multi-carrier system with per-tone QAM/ASK
- Each subcarrier uses 2-15 bit QAM (includes ASK as subset)
- Adaptive bit loading per tone (waterfilling)

---

### 5. Visible Light Communication (VLC)

**LED-based VLC**: OOK or M-ASK
- **OOK**: Simple, high speed (100+ Mbps)
- **Dimming**: Adjust average amplitude (DC level)
- **Application**: Indoor positioning, LiFi

---

## Pulse Shaping

**Rectangular pulses** cause excessive ISI and spectral regrowth

**Raised cosine (RC)** pulse:

$$
p(t) = \frac{\sin(\pi t/T_s)}{\pi t/T_s} \cdot \frac{\cos(\alpha \pi t/T_s)}{1 - (2\alpha t/T_s)^2}
$$

**Properties**:
- **α = 0**: Sinc pulse (infinite time, brick-wall spectrum)
- **α = 0.35**: Common (35% excess bandwidth)
- **α = 1**: Smoother time domain, 2× bandwidth

**Root raised cosine (RRC)**: Split filtering between TX and RX (matched filter)

---

## Noise Analysis

**Additive noise** $n(t)$ with power $\sigma^2 = N_0/2$ (single-sided PSD)

**After coherent demodulation**:

$$
r_m = A_m + n
$$

Where $n \sim \mathcal{N}(0, \sigma^2)$ (Gaussian noise)

**Symbol error** if noise pushes sample past threshold:

**Example (4-ASK, symbol 1 at amplitude A)**:
- Error if $r_1 < A/2$ (decide symbol 0) or $r_1 > 3A/2$ (decide symbol 2)

$$
P_{e|1} = Q\left(\frac{A/2}{\sigma}\right) + Q\left(\frac{A/2}{\sigma}\right) = 2Q\left(\frac{A}{2\sigma}\right)
$$

---

## Implementation Considerations

### 1. Carrier Recovery

**Coherent ASK** requires carrier phase/frequency sync:

**Methods**:
- **Squaring loop**: Square signal → 2f_c component → PLL → Divide by 2
- **Costas loop**: Feedback loop with I/Q arms
- **Pilot tone**: Transmit unmodulated carrier (reduces efficiency)

**See**: [[Synchronization-(Carrier,-Timing,-Frame)|Synchronization]]

---

### 2. Automatic Gain Control (AGC)

**Received amplitude varies** due to fading, path loss:

**AGC** adjusts receiver gain to maintain constant amplitude:

$$
\text{Gain}(t) = \frac{A_{\text{target}}}{\hat{A}_{\text{received}}(t)}
$$

**Critical for M-ASK** (M > 2) to maintain threshold accuracy

---

### 3. Nonlinear Distortion

**Power amplifier (PA) nonlinearity** compresses high amplitudes:

**Effect**: Amplitude levels not equally spaced → Increased BER

**Mitigation**:
- **Backoff**: Operate PA below saturation (reduces efficiency)
- **Predistortion**: Digital or analog linearization
- **Consider PSK/FSK**: Constant envelope (less sensitive to PA)

---

### 4. Frequency-Selective Fading

**Multipath fading** distorts amplitude:

$$
r(t) = h(t) \cdot s(t) + n(t)
$$

**Problem**: Fading gain $|h(t)|$ multiplies signal amplitude → ASK especially vulnerable

**Mitigation**:
- **Equalization**: Compensate for channel (see [[Channel-Equalization]])
- **OFDM**: Flat fading per subcarrier
- **Consider PSK**: Less sensitive to amplitude fading (phase-based)

---

## Advantages of ASK

1. **Simple modulator**: Single mixer, no phase shifter
2. **Non-coherent detection**: Envelope detector (OOK)
3. **Low cost**: Used in RFID, IR remotes
4. **Compatible with intensity modulation**: Optical, VLC (LED can't do phase)

---

## Disadvantages of ASK

1. **Poor power efficiency**: 1D constellation → Worse BER than 2D (QAM)
2. **Susceptible to fading**: Amplitude-based (fading directly affects signal)
3. **Nonlinear PA distortion**: High PAPR, AM-AM conversion
4. **Threshold sensitivity**: AGC critical for M > 2
5. **No advantage over PSK**: For M > 2, PSK/QAM preferred (except optical)

---

## Summary Table

| Aspect | 2-ASK (OOK) | 4-ASK | M-ASK |
|--------|-------------|-------|-------|
| **Bits/symbol** | 1 | 2 | log₂(M) |
| **Complexity** | Very simple | Simple | Moderate |
| **BER @ 10 dB** | 3.9 × 10⁻⁶ | 1.8 × 10⁻³ | Degrades with M |
| **Detection** | Non-coherent OK | Coherent preferred | Coherent required |
| **Applications** | RFID, IR, optical | Rarely used | Optical (PAM-4) |
| **vs QAM** | Equivalent (M=2) | 3 dB worse | Much worse |

---

## Related Topics

- **[[On-Off-Keying-(OOK)]]**: Binary ASK (M=2)
- **[[QPSK-Modulation]]**: 2D alternative (better than 4-ASK)
- **[[Frequency-Shift-Keying-(FSK)]]**: Frequency-based modulation
- **[[Constellation-Diagrams]]**: Visualizing signal space
- **[[Bit-Error-Rate-(BER)]]**: Performance metric
- **[[Synchronization-(Carrier,-Timing,-Frame)|Synchronization]]**: Carrier recovery for coherent detection

---

**Key takeaway**: **ASK encodes data in amplitude levels.** OOK (2-ASK) is simple and widely used (RFID, IR, optical). Higher-order ASK (M > 2) is power-inefficient compared to PSK/QAM due to 1D constellation. QAM dominates RF, ASK dominates optical (intensity modulation). Coherent detection needed for M > 2. Susceptible to fading, PA nonlinearity, and amplitude noise.

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
