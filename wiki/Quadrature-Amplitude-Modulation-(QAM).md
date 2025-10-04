# Quadrature Amplitude Modulation (QAM)

[[Home]] | **Digital Modulation** | [[QPSK Modulation]] | [[8PSK & Higher-Order PSK]]

---

## 🚀 For Non-Technical Readers

**QAM is like having a grid of mailboxes—the more boxes, the more messages you can send at once. Your WiFi/phone picks bigger grids when signal is strong!**

**The idea - Vary BOTH brightness and angle**:
- **PSK** (like QPSK): Only varies angle (4 or 8 positions)
- **QAM**: Varies **both** angle AND distance from center!
- Result: Many more possible positions = much faster data!

**Real QAM sizes**:
- **16-QAM**: 4×4 grid = 16 positions = 4 bits/symbol
- **64-QAM**: 8×8 grid = 64 positions = 6 bits/symbol
- **256-QAM**: 16×16 grid = 256 positions = 8 bits/symbol
- **1024-QAM** (WiFi 6): 32×32 grid = 1024 positions = 10 bits/symbol!

**Why you care - Speed differences**:
- **QPSK**: 2 bits/symbol (baseline)
- **16-QAM**: 4 bits/symbol = **2× faster**
- **64-QAM**: 6 bits/symbol = **3× faster**  
- **256-QAM**: 8 bits/symbol = **4× faster**
- **1024-QAM**: 10 bits/symbol = **5× faster**!

**The trade-off**:
- **More positions** = faster BUT positions are closer together
- **Closer positions** = easier to confuse when signal is noisy
- Strong signal (close to router): Use 1024-QAM = blazing fast!
- Weak signal (far from router): Use QPSK = slower but reliable

**Where you see it**:
- **Your WiFi stats**: "MCS 9, 256-QAM" = using 256-position grid
- **4G/5G**: "Modulation: 64-QAM" = using 64-position grid
- **Cable modem**: DOCSIS 3.1 uses 4096-QAM (12 bits/symbol!)
- **Phone signal bars**: Full bars = can use high QAM, low bars = must use simple modulation

**Real experience**:
- Walk toward router: Speed increases as phone switches QPSK → 16-QAM → 64-QAM → 256-QAM
- Walk away: Speed decreases as phone steps back down
- This happens automatically hundreds of times per second!

**Fun fact**: Modern WiFi 6E can use 1024-QAM, but ONLY at close range with zero interference—it's like threading a needle with radio waves!

---

## Overview

**Quadrature Amplitude Modulation (QAM)** encodes data by modulating **both amplitude and phase** of a carrier wave.

**Key insight**: Combine **ASK** (amplitude-shift keying) and **PSK** (phase-shift keying) in **2D constellation** (I/Q plane)

**Advantage**: **Best spectral efficiency** for given SNR (optimal use of 2D signal space)

**Applications**: WiFi, LTE/5G, cable modems (DOCSIS), DSL, digital TV (DVB-C), microwave backhaul

---

## QAM Fundamentals

### Complex Baseband Representation

**QAM symbol**:

$$
s_m = I_m + jQ_m
$$

Where:
- $I_m$ = In-phase amplitude (real axis)
- $Q_m$ = Quadrature amplitude (imaginary axis)
- $m$ = Symbol index (0 to M-1)

**Passband signal**:

$$
s_{\text{RF}}(t) = I_m \cos(2\pi f_c t) - Q_m \sin(2\pi f_c t)
$$

---

### M-ary QAM

**M constellation points**: $\sqrt{M} \times \sqrt{M}$ grid (for square QAM)

**Bits per symbol**: $\log_2(M)$

**Common sizes**: 16-QAM, 64-QAM, 256-QAM, 1024-QAM, 4096-QAM

---

## 16-QAM

### Constellation

**4×4 grid** in I/Q plane:

```
          Q
          ↑
    •  •  •  •   (I=±3d, Q=±3d)
    •  •  •  •   (I=±d, Q=±3d)
    •  •  •  •   (I=±3d, Q=±d)
    •  •  •  •   (I=±d, Q=±d)
          ↓
          I
```

**Amplitude levels**: $I, Q \in \{-3d, -d, +d, +3d\}$

Where $d$ = Unit spacing (normalized distance)

---

### Bit Mapping (Gray Coding)

**4 bits per symbol**: $b_3 b_2 b_1 b_0$

**Typical mapping**:
- $b_3 b_2$ → I component (00=-3d, 01=-d, 11=+d, 10=+3d)
- $b_1 b_0$ → Q component (00=-3d, 01=-d, 11=+d, 10=+3d)

**Example symbols**:

| Bits | I | Q | Position |
|------|---|---|----------|
| 0000 | -3d | -3d | Bottom-left corner |
| 0001 | -3d | -d | |
| 0011 | -3d | +d | |
| 0010 | -3d | +3d | Top-left corner |
| 1010 | +3d | +3d | Top-right corner |

**Gray coding**: Adjacent symbols differ by 1 bit (minimizes BER)

---

### Signal Characteristics

**Average symbol energy**:

$$
\bar{E}_s = \frac{1}{16}\sum_{m=0}^{15} (I_m^2 + Q_m^2) = \frac{1}{16} \times 16 \times 10d^2 = 10d^2
$$

**Normalization**: Set $d^2 = 1/10$ → $\bar{E}_s = 1$

**Minimum distance**: $d_{\min} = 2d$

**With normalization**: $d_{\min} = 2/\sqrt{10} = 0.632$

---

## 64-QAM

### Constellation

**8×8 grid**: 64 points

**Amplitude levels**: $I, Q \in \{-7d, -5d, -3d, -d, +d, +3d, +5d, +7d\}$

**Bits per symbol**: 6

**Average energy**:

$$
\bar{E}_s = \frac{1}{64}\sum (I_m^2 + Q_m^2) = 42d^2
$$

**Normalized**: $d = 1/\sqrt{42}$ → $\bar{E}_s = 1$

**Minimum distance**: $d_{\min} = 2d = 0.309$

---

## 256-QAM

### Constellation

**16×16 grid**: 256 points

**Bits per symbol**: 8

**Average energy**: $\bar{E}_s = 170d^2$

**Normalized**: $d = 1/\sqrt{170}$

**Minimum distance**: $d_{\min} = 2d = 0.153$

---

### High-Order QAM

**1024-QAM**: 32×32 grid, 10 bits/symbol

**4096-QAM**: 64×64 grid, 12 bits/symbol

**Practical limit**: ~4096-QAM (802.11ax WiFi 6, cable modems)

**Challenge**: Requires very high SNR (>40 dB) and excellent linearity

---

## Performance Analysis

### Symbol Error Rate (SER)

**Square M-QAM in AWGN** (approximate, high SNR):

$$
P_s \approx 4\left(1 - \frac{1}{\sqrt{M}}\right) Q\left(\sqrt{\frac{3}{M-1} \cdot \frac{E_s}{N_0}}\right)
$$

**Where**: $Q(x) = \frac{1}{\sqrt{2\pi}} \int_x^\infty e^{-t^2/2} dt$

---

### Bit Error Rate (BER)

**With Gray coding**:

$$
\text{BER} \approx \frac{P_s}{\log_2(M)}
$$

**In terms of Eb/N0**:

$$
\text{BER} \approx \frac{4}{\log_2(M)}\left(1 - \frac{1}{\sqrt{M}}\right) Q\left(\sqrt{\frac{3\log_2(M)}{M-1} \cdot \frac{E_b}{N_0}}\right)
$$

---

### Required Eb/N0 for BER = 10⁻⁶

| Modulation | Bits/symbol | Required Eb/N0 (dB) | SNR Penalty vs QPSK |
|------------|-------------|---------------------|---------------------|
| **QPSK** | 2 | 10.5 | 0 dB (baseline) |
| **16-QAM** | 4 | 14.5 | +4 dB |
| **64-QAM** | 6 | 18.5 | +8 dB |
| **256-QAM** | 8 | 23 | +12.5 dB |
| **1024-QAM** | 10 | 27.5 | +17 dB |
| **4096-QAM** | 12 | 32 | +21.5 dB |

**Pattern**: Each 4× increase in M adds ~4 dB

---

### BER Comparison Table

| Eb/N0 (dB) | QPSK | 16-QAM | 64-QAM | 256-QAM |
|------------|------|--------|--------|---------|
| 10 | 3.9×10⁻⁶ | 2×10⁻³ | 0.1 | 0.3 |
| 15 | 7×10⁻¹⁰ | 5×10⁻⁶ | 5×10⁻³ | 0.08 |
| 20 | <10⁻¹² | 1×10⁻⁹ | 1×10⁻⁵ | 3×10⁻³ |
| 25 | <10⁻¹² | <10⁻¹² | 1×10⁻⁸ | 2×10⁻⁵ |
| 30 | <10⁻¹² | <10⁻¹² | <10⁻¹² | 2×10⁻⁸ |

---

## Bandwidth Efficiency

**Occupied bandwidth** (raised cosine pulse shaping):

$$
B = (1 + \alpha) R_s = (1 + \alpha) \frac{R_b}{\log_2(M)} \quad (\text{Hz})
$$

**Spectral efficiency**:

$$
\eta = \frac{R_b}{B} = \frac{\log_2(M)}{1 + \alpha} \quad (\text{bits/sec/Hz})
$$

---

### Comparison (α = 0.35)

| Modulation | Bits/symbol | Spectral Efficiency | Practical Limit |
|------------|-------------|---------------------|-----------------|
| **QPSK** | 2 | 1.48 | Good SNR (10 dB) |
| **16-QAM** | 4 | 2.96 | Moderate SNR (15 dB) |
| **64-QAM** | 6 | 4.44 | High SNR (20 dB) |
| **256-QAM** | 8 | 5.93 | Very high SNR (25 dB) |
| **1024-QAM** | 10 | 7.41 | Excellent SNR (30 dB), wired only |
| **4096-QAM** | 12 | 8.89 | Exceptional SNR (35 dB), cable/DSL |

---

## Modulation & Demodulation

### IQ Modulator

**Standard quadrature modulator**:

```
           cos(2πf_c t)
                |
    I(t) ----> [×] ----\
                        [+] --> s_RF(t)
    Q(t) ----> [×] ----/
                |
          -sin(2πf_c t)
```

**Same hardware as QPSK**, different symbol mapping

---

### Coherent Demodulation

**IQ demodulator**:

```
               cos(2πf_c t)
                    |
s_RF(t) --> [×] --> [LPF] --> [Sample] --> I(t)
         |
         |  -sin(2πf_c t)
         |      |
         └--> [×] --> [LPF] --> [Sample] --> Q(t)
```

**Decision**:
1. Sample I and Q at symbol rate
2. Find nearest constellation point (minimum Euclidean distance)
3. Map constellation point to bits

---

### Soft-Decision Decoding

**Hard decision**: Nearest neighbor → Bits

**Soft decision**: Pass I/Q values (or LLRs) to decoder

**Log-Likelihood Ratio (LLR)** for bit $b_k$:

$$
\text{LLR}(b_k) = \log\frac{P(b_k=0 | r)}{P(b_k=1 | r)}
$$

**Benefit**: ~2 dB coding gain (LDPC, Turbo codes use soft decisions)

---

## Power Efficiency

### Peak-to-Average Power Ratio (PAPR)

**QAM has varying envelope**:

$$
|s_m| = \sqrt{I_m^2 + Q_m^2}
$$

**PAPR**:

$$
\text{PAPR} = \frac{P_{\max}}{P_{\text{avg}}} = \frac{|s_{\max}|^2}{\bar{E}_s}
$$

---

### PAPR Values

| Modulation | PAPR (linear) | PAPR (dB) | Notes |
|------------|---------------|-----------|-------|
| **QPSK** | 1 | 0 dB | Constant envelope |
| **16-QAM** | 2.55 | 4.1 dB | Corner points 2.55× average |
| **64-QAM** | 3.68 | 5.7 dB | |
| **256-QAM** | 4.80 | 6.8 dB | |
| **1024-QAM** | 5.93 | 7.7 dB | |

**Impact**: High PAPR requires PA backoff (reduces efficiency)

**Example**: 64-QAM with 5.7 dB PAPR
- PA must back off 5.7 dB from saturation
- Efficiency drops from 50% to ~13% (4× penalty)

---

## Practical Impairments

### 1. I/Q Imbalance

**Gain mismatch**: $G_I \neq G_Q$

**Phase error**: 90° hybrid imperfect (e.g., 88° or 92°)

**Effect**: Constellation distortion, image leakage

**Model**:

$$
r = (1 + \alpha_G) I + j(1 - \alpha_G) e^{j\epsilon} Q + n
$$

Where:
- $\alpha_G$ = Gain imbalance
- $\epsilon$ = Phase error

**Typical**: ±0.5 dB gain, ±2° phase (degrades 256-QAM significantly)

**Mitigation**: Digital calibration (pilot-aided estimation)

---

### 2. Nonlinear PA Distortion

**AM-AM conversion**: Gain compression at high amplitudes

**AM-PM conversion**: Phase shift varies with amplitude

**Effect**: Constellation warping, especially outer points

**Example**: 64-QAM, corner points compress 1 dB
- Minimum distance reduced → BER increases
- Spectral regrowth (adjacent channel interference)

**Mitigation**:
- **Backoff**: 6-10 dB (kills efficiency)
- **Predistortion**: Digital (DPD) or analog
- **Crest factor reduction (CFR)**: Clip peaks, re-generate signal

---

### 3. Phase Noise

**Oscillator jitter** causes constellation rotation/spread:

$$
r(t) = s(t) e^{j\phi_n(t)} + n(t)
$$

**Effect**: Common phase error (CPE) + inter-carrier interference (OFDM)

**Sensitivity**: Higher-order QAM more sensitive

**Example**: 256-QAM
- Tolerable phase noise: ~1° RMS
- Requires high-quality oscillator (PLL, TCXO, or OCXO)

---

### 4. Timing Jitter

**Symbol clock error** causes sampling offset:

**Effect**: ISI, constellation blurring

**Requirement**: Timing error < 0.1 symbol period

**Example**: 64-QAM @ 10 Msps
- Symbol period: 100 ns
- Tolerable jitter: < 10 ns RMS

---

## Practical Applications

### 1. WiFi (802.11a/n/ac/ax)

**OFDM subcarriers** use QAM:

| Standard | Max QAM | Max Rate | Notes |
|----------|---------|----------|-------|
| **802.11a** | 64-QAM | 54 Mbps | 20 MHz channel |
| **802.11n** | 64-QAM | 600 Mbps | 4×4 MIMO, 40 MHz |
| **802.11ac** | 256-QAM | 6.9 Gbps | 8×8 MIMO, 160 MHz |
| **802.11ax** (WiFi 6) | 1024-QAM | 9.6 Gbps | OFDMA, MU-MIMO |

**Adaptive modulation**: Switch QPSK → 16/64/256/1024-QAM based on SNR

---

### 2. LTE/5G NR

**LTE downlink**: Up to 256-QAM (Cat 9+)

**5G NR**: Up to 256-QAM (mmWave can use 1024-QAM in some scenarios)

**Example**: LTE Cat 16 (1 Gbps downlink)
- 4×4 MIMO, 256-QAM, 20 MHz carrier aggregation
- Per-carrier: 4 layers × 8 bits/symbol × 75k symbols/sec = 2.4 Gbps (theoretical)

**Adaptive MCS** (Modulation & Coding Scheme):
- Poor channel: QPSK 1/4 (0.5 bits/symbol effective)
- Good channel: 256-QAM 3/4 (6 bits/symbol effective)

---

### 3. Cable Modems (DOCSIS)

**DOCSIS 3.0**: 256-QAM (8 bits/symbol)

**DOCSIS 3.1**: 4096-QAM (12 bits/symbol)
- Requires SNR > 40 dB (excellent cable plant)
- OFDM with 4096-QAM subcarriers → 10 Gbps downstream

**Key**: Wired channel (no fading), high SNR possible

---

### 4. Digital TV

**DVB-C (Cable)**: 256-QAM standard

**DVB-T2 (Terrestrial)**: Up to 256-QAM (typically 64-QAM)

**ATSC 3.0 (US)**: 256-QAM, 1024-QAM, 4096-QAM (OFDM)

---

### 5. Microwave Backhaul

**Point-to-point links**:
- **Clear weather**: 2048-QAM, 4096-QAM (≥30 dB SNR)
- **Light rain**: 256-QAM
- **Heavy rain**: Adaptive down to 16-QAM or QPSK

**Frequency**: 6-42 GHz (E-band: 70-80 GHz)

**Example**: 28 GHz link, 56 MHz channel
- 4096-QAM: 12 bits/symbol → 672 Mbps (no coding)
- With FEC 3/4: 504 Mbps net

---

## QAM vs PSK

**Same spectral efficiency**:

| M-PSK | M-QAM | Comparison |
|-------|-------|------------|
| 4-PSK (QPSK) | 4-QAM (identical) | Same constellation |
| 8-PSK | 8-QAM (rare) | 8-PSK used (const envelope) |
| 16-PSK | 16-QAM | **16-QAM 4 dB better** |
| 32-PSK | 32-QAM | **32-QAM much better** |
| 64-PSK | 64-QAM | **64-QAM far superior** |

**General rule**: For M > 8, QAM always better than M-PSK

**Reason**: 2D rectangular grid (QAM) uses signal space more efficiently than circle (PSK)

---

## Non-Square QAM

**Cross QAM**: Non-square constellations (e.g., 32-QAM, 128-QAM)

**32-QAM**: 5 bits/symbol
- Constellation: 4 inner points + 12 middle + 16 outer (hexagonal-like)
- Used in some proprietary systems

**128-QAM**: 7 bits/symbol
- Between 64-QAM and 256-QAM

**Trade-off**: Slightly worse performance than square QAM, but allows finer granularity

---

## Constellation Shaping

**Probabilistic shaping**: Non-uniform symbol probability

**Idea**: Transmit inner points more often (lower energy) → Reduce average power

**Benefit**: ~0.5-1 dB SNR gain (approaching Shannon limit)

**Used in**: Optical communications (400G/800G), submarine cables

---

## Adaptive QAM

**Link adaptation**: Select QAM order based on channel

**SNR thresholds** (example):

| SNR (dB) | Modulation | Code Rate | Spectral Eff. |
|----------|------------|-----------|---------------|
| 0-5 | QPSK | 1/2 | 1.0 |
| 5-10 | QPSK | 3/4 | 1.5 |
| 10-15 | 16-QAM | 1/2 | 2.0 |
| 15-20 | 16-QAM | 3/4 | 3.0 |
| 20-25 | 64-QAM | 2/3 | 4.0 |
| 25-30 | 64-QAM | 3/4 | 4.5 |
| 30-35 | 256-QAM | 3/4 | 6.0 |
| >35 | 1024-QAM | 5/6 | 8.3 |

**Used in**: All modern wireless (WiFi, LTE, 5G)

---

## Implementation Tips

### Constellation Normalization

**Normalize average power to 1**:

$$
\bar{E}_s = \frac{1}{M}\sum_{m=0}^{M-1} |s_m|^2 = 1
$$

**Example (16-QAM)**:
- Un-normalized: $I, Q \in \{-3, -1, +1, +3\}$
- Average power: 10
- Normalized: $I, Q \in \{-3, -1, +1, +3\}/\sqrt{10}$

---

### Gray Coding

**Map bits → I/Q using Gray code**:

```python
def qam_gray_mapping(bits):
    # 16-QAM Gray mapping
    gray_map = [0b00, 0b01, 0b11, 0b10]  # Gray sequence
    i_bits = bits[0:2]
    q_bits = bits[2:4]
    
    i_index = gray_map.index(i_bits)
    q_index = gray_map.index(q_bits)
    
    I = 2*i_index - 3  # Map to {-3, -1, +1, +3}
    Q = 2*q_index - 3
    
    return I + 1j*Q
```

---

### Soft-Decision LLR Calculation

**For bit $b_k$ in constellation**:

$$
\text{LLR}(b_k) = \log\frac{\sum_{s \in S_0} e^{-|r-s|^2/(2\sigma^2)}}{\sum_{s \in S_1} e^{-|r-s|^2/(2\sigma^2)}}
$$

Where:
- $S_0$ = Constellation points with $b_k = 0$
- $S_1$ = Constellation points with $b_k = 1$
- $r$ = Received symbol
- $\sigma^2$ = Noise variance

---

## Summary Table

| Modulation | Bits/sym | Min Distance | Eb/N0 (10⁻⁶) | PAPR (dB) | Applications |
|------------|----------|--------------|--------------|-----------|--------------|
| **QPSK** | 2 | 1.41 | 10.5 dB | 0 | Satellite, cellular |
| **16-QAM** | 4 | 0.63 | 14.5 dB | 4.1 | WiFi, LTE, cable |
| **64-QAM** | 6 | 0.31 | 18.5 dB | 5.7 | WiFi, LTE, backhaul |
| **256-QAM** | 8 | 0.15 | 23 dB | 6.8 | WiFi 5/6, cable, LTE+ |
| **1024-QAM** | 10 | 0.098 | 27.5 dB | 7.7 | WiFi 6, DOCSIS 3.1 |
| **4096-QAM** | 12 | 0.049 | 32 dB | 8.6 | Cable (DOCSIS 3.1) |

---

## Related Topics

- **[[QPSK Modulation]]**: Simplest QAM (4-QAM)
- **[[8PSK & Higher-Order PSK]]**: Phase-only modulation
- **[[Amplitude-Shift Keying (ASK)]]**: Amplitude-only modulation
- **[[Constellation Diagrams]]**: Visualizing QAM
- **[[Bit Error Rate (BER)]]**: Performance analysis
- **[[OFDM & Multicarrier Modulation]]**: Uses QAM per subcarrier

---

**Key takeaway**: **QAM combines amplitude and phase modulation for optimal spectral efficiency.** 2D rectangular constellation uses signal space efficiently. 16/64/256-QAM dominate modern wireless/wired systems. Higher-order QAM (1024, 4096) requires excellent SNR (>30 dB) and linearity. Trade-off: Spectral efficiency vs power efficiency (PAPR). Adaptive modulation switches QAM order based on channel quality. Gray coding + soft-decision decoding essential for good BER performance.

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
