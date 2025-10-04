# 8PSK & Higher-Order PSK

[[Home]] | **Digital Modulation** | [[QPSK Modulation]] | [[Binary Phase-Shift Keying (BPSK)]]

---

## Overview

**8PSK (8-ary Phase-Shift Keying)** encodes data using **8 phase states**, transmitting **3 bits per symbol**.

**Higher-order PSK** (M-PSK): M phase states, $\log_2(M)$ bits per symbol

**Trade-off**: Higher spectral efficiency but increased SNR requirement

**Applications**: Satellite (DVB-S2), military (MILSTAR), microwave backhaul

---

## 8PSK Modulation

### Constellation

**8 equally-spaced phases** around unit circle:

$$
\phi_m = \frac{2\pi m}{8} = \frac{\pi m}{4}, \quad m = 0, 1, \ldots, 7
$$

**Symbol** $m$:

$$
s_m(t) = A\cos(2\pi f_c t + \phi_m)
$$

**Complex baseband**:

$$
s_m = A e^{j\phi_m} = A e^{j\pi m/4}
$$

---

### Constellation Diagram

```
          Q
          ↑
     010  |  011
       •  |  •
         \|/
  001 •---+---• 100
         /|\
       •  |  •
     000  |  111
          ↓
          I
```

**Phases**: 0°, 45°, 90°, 135°, 180°, 225°, 270°, 315°

**Gray coding** (adjacent symbols differ by 1 bit):

| Symbol | Bits | Phase (°) | I | Q |
|--------|------|-----------|---|---|
| 0 | 000 | 0 | 1 | 0 |
| 1 | 001 | 45 | 0.707 | 0.707 |
| 2 | 010 | 90 | 0 | 1 |
| 3 | 011 | 135 | -0.707 | 0.707 |
| 4 | 100 | 180 | -1 | 0 |
| 5 | 101 | 225 | -0.707 | -0.707 |
| 6 | 110 | 270 | 0 | -1 |
| 7 | 111 | 315 | 0.707 | -0.707 |

---

## Signal Characteristics

### Constant Envelope

**All symbols same amplitude** A:

$$
|s_m| = A, \quad \forall m
$$

**Advantage**: Power amplifier can operate at saturation (maximum efficiency)

**PAPR** (Peak-to-Average Power Ratio): 0 dB (constant)

---

### Symbol Energy

$$
E_s = \int_0^{T_s} |s_m(t)|^2 dt = A^2 T_s = A^2
$$

**Energy per bit**:

$$
E_b = \frac{E_s}{\log_2(8)} = \frac{E_s}{3}
$$

---

### Minimum Distance

**Euclidean distance** between adjacent symbols:

$$
d_{\min} = 2A\sin\left(\frac{\pi}{8}\right) = 2A \times 0.383 = 0.765A
$$

**Normalized** (A=1): $d_{\min} = 0.765$

**Comparison**:
- **QPSK**: $d_{\min} = \sqrt{2}A = 1.414A$ (same energy)
- **8PSK**: $d_{\min} = 0.765A$
- **Ratio**: 8PSK is 1.85× worse (5.3 dB)

---

## Modulation & Demodulation

### IQ Modulator

**Baseband I/Q** for symbol $m$:

$$
I_m = A\cos(\phi_m), \quad Q_m = A\sin(\phi_m)
$$

**Modulated signal**:

$$
s_{\text{RF}}(t) = I_m \cos(2\pi f_c t) - Q_m \sin(2\pi f_c t)
$$

**Implementation**: Standard IQ modulator (same as QPSK)

---

### Coherent Demodulation

**Receiver**:
1. **IQ demodulation**: Recover I and Q components
2. **Phase calculation**: $\hat{\phi} = \arctan(Q/I)$
3. **Decision**: Find closest constellation point

**Decision regions**: 8 pie-slice wedges, each 45° wide

**Hard decision**:

$$
\hat{m} = \left\lfloor \frac{\hat{\phi} + \pi/8}{2\pi/8} \right\rfloor \mod 8
$$

---

### Differential 8PSK (D8PSK)

**Differential encoding** avoids phase ambiguity:

**Transmitted phase**:

$$
\phi_k = \phi_{k-1} + \Delta\phi_k \mod 2\pi
$$

Where $\Delta\phi_k$ encodes 3 bits

**Demodulation**: Compute phase difference between consecutive symbols

$$
\Delta\hat{\phi}_k = \hat{\phi}_k - \hat{\phi}_{k-1}
$$

**Advantage**: No carrier phase recovery needed (only frequency sync)

**Disadvantage**: ~3 dB penalty vs coherent

---

## Performance Analysis

### Symbol Error Rate (SER)

**8PSK in AWGN** (approximate, high SNR):

$$
P_s \approx 2Q\left(2\sin\left(\frac{\pi}{8}\right)\sqrt{\frac{E_s}{N_0}}\right) = 2Q\left(0.765\sqrt{\frac{E_s}{N_0}}\right)
$$

**Where**: $Q(x) = \frac{1}{\sqrt{2\pi}} \int_x^\infty e^{-t^2/2} dt$

---

### Bit Error Rate (BER)

**With Gray coding**:

$$
\text{BER} \approx \frac{P_s}{\log_2(8)} = \frac{P_s}{3}
$$

**In terms of Eb/N0**:

$$
\text{BER} \approx \frac{2}{3}Q\left(0.765\sqrt{\frac{3E_b}{N_0}}\right) = \frac{2}{3}Q\left(1.325\sqrt{\frac{E_b}{N_0}}\right)
$$

---

### Required Eb/N0 for BER = 10⁻⁶

**8PSK**: 14 dB (coherent detection)

**Comparison**:
- **BPSK**: 10.5 dB
- **QPSK**: 10.5 dB (same as BPSK)
- **8PSK**: 14 dB (+3.5 dB penalty vs QPSK)
- **16-PSK**: 18 dB (+7.5 dB penalty vs QPSK)

**Pattern**: Each doubling of M adds ~3.5-4 dB penalty

---

### BER vs SNR Curves

| Eb/N0 (dB) | BPSK | QPSK | 8PSK | 16-PSK |
|------------|------|------|------|--------|
| 6 | 1.9×10⁻³ | 1.9×10⁻³ | 0.04 | 0.15 |
| 8 | 5.6×10⁻⁵ | 5.6×10⁻⁵ | 8×10⁻³ | 0.08 |
| 10 | 3.9×10⁻⁶ | 3.9×10⁻⁶ | 7×10⁻⁴ | 0.03 |
| 12 | 7.8×10⁻⁸ | 7.8×10⁻⁸ | 4×10⁻⁵ | 8×10⁻³ |
| 14 | 7.7×10⁻¹⁰ | 7.7×10⁻¹⁰ | 1×10⁻⁶ | 7×10⁻⁴ |

**Observation**: Higher-order PSK needs significantly more SNR for same BER

---

## Bandwidth Efficiency

**Symbol rate** $R_s$ (symbols/sec):

$$
R_s = \frac{R_b}{\log_2(M)}
$$

**Occupied bandwidth** (with pulse shaping):

$$
B = (1 + \alpha) R_s = (1 + \alpha) \frac{R_b}{\log_2(M)} \quad (\text{Hz})
$$

**Spectral efficiency**:

$$
\eta = \frac{R_b}{B} = \frac{\log_2(M)}{1 + \alpha} \quad (\text{bits/sec/Hz})
$$

---

### Comparison (α = 0.35)

| Modulation | Bits/symbol | Spectral Efficiency | Required Eb/N0 (10⁻⁶) |
|------------|-------------|---------------------|------------------------|
| **BPSK** | 1 | 0.74 | 10.5 dB |
| **QPSK** | 2 | 1.48 | 10.5 dB |
| **8PSK** | 3 | 2.22 | 14 dB |
| **16-PSK** | 4 | 2.96 | 18 dB |
| **32-PSK** | 5 | 3.70 | 22 dB |

**Trade-off**: Higher spectral efficiency requires higher SNR

---

## Higher-Order PSK

### 16-PSK

**16 phase states**: 22.5° spacing

**Bits per symbol**: 4

**Minimum distance**: $d_{\min} = 2A\sin(\pi/16) = 0.39A$

**Performance**: ~4 dB worse than 8PSK (at same BER)

**Problem**: Very sensitive to phase noise (small angular separation)

---

### 32-PSK and Beyond

**32-PSK**: 11.25° spacing, 5 bits/symbol

**64-PSK**: 5.625° spacing, 6 bits/symbol

**Practical limit**: M > 16 rarely used
- Phase noise becomes limiting factor
- QAM more efficient for M > 8

---

## 8PSK vs Other Modulations

### 8PSK vs 16-QAM

**Same spectral efficiency** (≈2.2 bits/sec/Hz with α=0.35):
- **8PSK**: 3 bits/symbol
- **16-QAM**: 4 bits/symbol @ 1.33× symbol rate

**BER comparison** @ BER = 10⁻⁶:
- **8PSK**: 14 dB Eb/N0
- **16-QAM**: 14.5 dB Eb/N0

**Advantage 8PSK**: Constant envelope (PA efficiency)

**Advantage 16-QAM**: Slightly better BER, more flexible coding rates

---

### 8PSK vs OFDM with QPSK

**Wideband system** (20 MHz):

**8PSK single carrier**:
- 6.67 Msps, 20 Mbps
- Requires equalization (frequency-selective fading)
- Constant envelope

**OFDM with QPSK** (64 subcarriers):
- 312.5 kHz per subcarrier (flat fading)
- 20 Mbps total
- Varying envelope (PAPR ~10 dB)

**Trade-off**: OFDM handles multipath better, 8PSK more PA-efficient

---

## Phase Noise Sensitivity

**Oscillator phase noise** $\phi_n(t)$ rotates constellation:

$$
r_m(t) = A e^{j(\phi_m + \phi_n(t))} + n(t)
$$

**Phase error** $\phi_n$ causes:
- **Rotation**: All symbols rotate equally
- **Spreading**: Random jitter → Constellation blur

**Sensitivity** (angular spacing):
- **QPSK**: 90° spacing (robust)
- **8PSK**: 45° spacing (moderate)
- **16-PSK**: 22.5° spacing (sensitive)
- **32-PSK**: 11.25° spacing (very sensitive)

**Rule of thumb**: Phase noise RMS should be < 1/10 of angular spacing

**Example**: 8PSK with 45° spacing
- Tolerable phase noise: ~4.5° RMS
- Equivalent phase noise: ~-25 dBc integrated (tight spec!)

---

## Practical Implementations

### 1. DVB-S2 (Satellite TV)

**8PSK** used for high data rates:
- **QPSK**: Low C/N (rain fade conditions)
- **8PSK**: Clear sky, high throughput
- **Adaptive Coding & Modulation (ACM)**: Switch based on link quality

**Example**:
- QPSK 1/2: 1 bit/symbol effective → 0.74 bits/sec/Hz
- 8PSK 3/4: 2.25 bits/symbol effective → 1.67 bits/sec/Hz
- **2.25× throughput** when SNR permits

---

### 2. Military SATCOM (MILSTAR)

**Differential 8PSK**:
- Robust against jamming
- Low-probability-of-intercept (LPI)
- Spread spectrum combined with D8PSK

---

### 3. Microwave Backhaul

**Point-to-point links** (cellular backhaul):
- **Clear weather**: 256-QAM (8 bits/symbol)
- **Rain fade**: Adaptive down to 8PSK or QPSK
- **Example**: 6-11 GHz bands, 28/56 MHz channels

---

### 4. Deep Space Communications

**NASA/ESA**: Primarily BPSK/QPSK (maximize link margin)

**Emerging**: 8PSK for high-rate science data return
- **Mars orbiters**: 8PSK @ Ka-band (32 GHz)
- **Trade-off**: 3× data rate vs 3.5 dB link margin

---

## Implementation Challenges

### 1. Carrier Phase Recovery

**8PSK phase ambiguity**: 8-fold (every 45°)

**Pilot-aided sync**:
- Insert known pilot symbols
- Estimate phase offset
- Correct data symbols

**Blind sync**:
- 8th-power loop (remove modulation)
- Costas loop (feedback)
- Decision-directed (after initial acquisition)

**See**: [[Synchronization]]

---

### 2. Timing Recovery

**Symbol clock** must be accurate:

**Timing jitter** causes:
- Sampling offset → ISI
- Increased BER

**Early-late gate** detector:
- Sample early, on-time, late
- Adjust clock based on correlation

---

### 3. Nonlinear PA Distortion

**8PSK constant envelope**: Tolerates PA saturation

**BUT**: Pulse shaping filter creates envelope variations
- Raised cosine filter → 3-4 dB PAPR
- PA must back off → Reduced efficiency

**Mitigation**:
- **Constant envelope pulse shaping**: MSK, GMSK (no overshoot)
- **Predistortion**: Digital or analog linearization

---

### 4. Frequency Offset

**Carrier frequency offset** $\Delta f$ rotates constellation:

$$
r(t) = s(t) e^{j2\pi \Delta f t}
$$

**Tolerable offset** (rule of thumb): $|\Delta f| < 0.01 \times R_s$

**Example**: 8PSK @ 1 Msps
- Tolerable offset: < 10 kHz
- Oscillator spec: < 10 ppm @ 1 GHz carrier (= 10 kHz)

---

## Adaptive Modulation & Coding (AMC)

**Dynamically select modulation** based on channel quality:

**Link adaptation table**:

| C/N (dB) | Modulation | Code Rate | Spectral Eff. | Target BER |
|----------|------------|-----------|---------------|------------|
| 2-5 | QPSK | 1/4 | 0.5 | 10⁻⁷ |
| 5-7 | QPSK | 1/2 | 1.0 | 10⁻⁷ |
| 7-9 | QPSK | 3/4 | 1.5 | 10⁻⁷ |
| 9-11 | 8PSK | 2/3 | 2.0 | 10⁻⁷ |
| 11-13 | 8PSK | 3/4 | 2.25 | 10⁻⁷ |
| 13-15 | 16-QAM | 2/3 | 2.67 | 10⁻⁷ |

**Benefit**: Maximize throughput while maintaining target BER

---

## Gray Coding

**Gray code**: Adjacent symbols differ by **1 bit**

**Benefit**: Symbol error → Likely 1-bit error (not 2 or 3)

**8PSK Gray mapping**:

| Symbol | Binary | Phase (°) | Gray Code |
|--------|--------|-----------|-----------|
| 0 | 000 | 0 | 000 |
| 1 | 001 | 45 | 001 |
| 2 | 010 | 90 | 011 |
| 3 | 011 | 135 | 010 |
| 4 | 100 | 180 | 110 |
| 5 | 101 | 225 | 111 |
| 6 | 110 | 270 | 101 |
| 7 | 111 | 315 | 100 |

**Natural binary**: Symbol error → Up to 3-bit error

**Gray coding**: Symbol error → Typically 1-bit error (maybe 2)

**BER improvement**: ~2× better with Gray coding

---

## Pulse Shaping

**Rectangular pulses**: Infinite bandwidth (sinc spectrum)

**Raised cosine** (RC):

$$
P(f) = \begin{cases}
T_s & |f| \leq \frac{1-\alpha}{2T_s} \\
\frac{T_s}{2}\left[1 + \cos\left(\frac{\pi T_s}{\alpha}\left[|f| - \frac{1-\alpha}{2T_s}\right]\right)\right] & \frac{1-\alpha}{2T_s} < |f| \leq \frac{1+\alpha}{2T_s} \\
0 & |f| > \frac{1+\alpha}{2T_s}
\end{cases}
$$

**Roll-off factor** α:
- **α = 0**: Brick-wall (infinite time, impractical)
- **α = 0.35**: Common (35% excess BW, moderate time decay)
- **α = 1**: Gentle roll-off (100% excess BW, fast time decay)

**Root raised cosine** (RRC): Split between TX and RX (matched filter)

---

## Summary Table

| Modulation | Bits/sym | Min Distance | Eb/N0 (10⁻⁶) | Spectral Eff. | PAPR | Best Use Case |
|------------|----------|--------------|--------------|---------------|------|---------------|
| **BPSK** | 1 | 2A | 10.5 dB | 0.74 | 0 dB | Deep space, long range |
| **QPSK** | 2 | √2 A | 10.5 dB | 1.48 | 0 dB | Balanced (most common) |
| **8PSK** | 3 | 0.765A | 14 dB | 2.22 | 0 dB | High throughput, PA efficiency |
| **16-PSK** | 4 | 0.39A | 18 dB | 2.96 | 0 dB | Rarely (QAM better) |
| **16-QAM** | 4 | 0.63A | 14.5 dB | 2.96 | 2.6 dB | High throughput (non-const env) |

---

## Related Topics

- **[[QPSK Modulation]]**: Lower-order PSK (2 bits/symbol)
- **[[Binary Phase-Shift Keying (BPSK)]]**: Simplest PSK
- **[[Constellation Diagrams]]**: Visualizing PSK
- **[[Bit Error Rate (BER)]]**: Performance metric
- **[[Synchronization]]**: Carrier recovery for coherent detection
- **[[OFDM & Multicarrier Modulation]]**: Uses QPSK/8PSK per subcarrier

---

**Key takeaway**: **8PSK transmits 3 bits/symbol using 8 phase states.** Constant envelope = PA efficient. 50% more spectral efficiency than QPSK but needs +3.5 dB SNR. Used in satellite (DVB-S2) and backhaul. Higher-order PSK (16, 32, 64) rarely used due to phase noise sensitivity—QAM preferred for M > 8. Gray coding reduces BER by limiting bit errors per symbol error. Adaptive modulation switches between QPSK/8PSK/16-QAM based on link quality.

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
