# Synchronization (Carrier, Timing, Frame)

[[Home]] | **System Implementation** | [[Signal Chain (End-to-End Processing)]] | [[Channel Equalization]]

---

## Overview

**Synchronization** is critical for coherent demodulation—receiver must align with transmitter.

**Three types**:
1. **Carrier frequency synchronization**: Match local oscillator frequency
2. **Carrier phase synchronization**: Align carrier phase (0° reference)
3. **Symbol timing synchronization**: Sample at correct instants
4. **Frame synchronization**: Identify packet/frame boundaries

**Failure modes**:
- **CFO** (Carrier Frequency Offset) → Constellation rotation, ICI
- **Phase error** → Constellation rotation
- **Timing error** → ISI, wrong sample points
- **Frame misalignment** → Lost packets

---

## Carrier Frequency Synchronization

### Carrier Frequency Offset (CFO)

**Cause**: LO frequency mismatch between TX and RX

$$
f_{\text{error}} = f_{\text{LO,RX}} - f_{\text{LO,TX}}
$$

**Effect on received signal**:

$$
r(t) = s(t) e^{j 2\pi \Delta f t} + n(t)
$$

Where $\Delta f$ = CFO (Hz)

---

### CFO Impact

**Normalized CFO**: $\epsilon = \Delta f \cdot T_s$ (fraction of symbol rate)

**Effects**:

1. **Constellation rotation**: Phase rotates $2\pi \epsilon$ per symbol
2. **ICI** (Inter-Carrier Interference in OFDM): Subcarriers leak into neighbors
3. **SNR degradation**: Effective SNR loss ~$10 \log_{10}(1 + \text{SNR} \cdot (2\pi\epsilon)^2)$ dB

**Example**: 2.4 GHz WiFi, ±20 ppm crystal
- CFO: ±48 kHz
- Symbol rate: 250 ksps → $\epsilon = \pm 0.192$ (19%!)
- **Must correct** before demodulation

---

### CFO Estimation Methods

#### 1. Data-Aided (Preamble-Based)

**Transmit known symbols** (preamble, pilot tones)

**Correlate** received with expected:

$$
\hat{\Delta f} = \frac{1}{2\pi T} \angle \left(\sum_{k} r_k \cdot s_k^*\right)
$$

Where:
- $r_k$ = Received preamble symbol $k$
- $s_k$ = Known preamble symbol $k$
- $T$ = Symbol period

**Range**: $|\Delta f| < 1/(2T)$ (ambiguity)

**Accuracy**: $\sim 10^{-4}$ of symbol rate

---

#### 2. Blind (Non-Data-Aided)

**No preamble**, use signal properties

**Method**: Power spectral density peak, cyclostationary features

**Example (MPSK)**:

$$
\hat{\Delta f} = \frac{1}{2\pi M T} \angle \left(\sum_{k} r_k^M\right)
$$

For M-PSK (raise to M-th power removes modulation)

**Range**: $|\Delta f| < 1/(2MT)$ (reduced ambiguity)

---

#### 3. Two-Stage Acquisition

**Coarse acquisition**: ±50% symbol rate (preamble autocorrelation)

**Fine tracking**: ±0.1% (PLL, decision-directed)

**Example (WiFi 802.11a)**:
- Short preamble: Coarse CFO (±100 kHz range)
- Long preamble: Fine CFO (±1 kHz accuracy)

---

### CFO Correction

**Digital correction** (after downconversion):

$$
r_{\text{corrected}}[n] = r[n] \cdot e^{-j 2\pi \hat{\epsilon} n}
$$

**Analog correction**: Adjust VCO frequency (AFC, Automatic Frequency Control)

**Hybrid**: Coarse analog, fine digital

---

## Carrier Phase Synchronization

### Phase Offset

**After CFO correction**, residual phase error $\theta$:

$$
r(t) = s(t) e^{j\theta} + n(t)
$$

**Effect**: Constellation rotates by $\theta$

**Tolerance**:
- **BPSK**: ±90° (ambiguity, differential coding helps)
- **QPSK**: ±45°
- **16-QAM**: ±22.5°
- **256-QAM**: ±2.8° (very sensitive!)

---

### Phase-Locked Loop (PLL)

**Classic analog/digital feedback loop**:

```
Input --> [Phase      --> [Loop    --> [VCO/NCO] --> Local
           Detector]      Filter]                    Carrier
              ↑                                         |
              +--------------------<--------------------+
```

**Components**:
1. **Phase detector**: Measure phase error (mixer + LPF)
2. **Loop filter**: 2nd-order (PI controller)
3. **VCO/NCO**: Voltage/Numerically Controlled Oscillator

---

#### Loop Dynamics

**2nd-order PLL**:

$$
H(s) = \frac{2\zeta\omega_n s + \omega_n^2}{s^2 + 2\zeta\omega_n s + \omega_n^2}
$$

Where:
- $\omega_n$ = Natural frequency (rad/s)
- $\zeta$ = Damping factor (0.707 critical)

**Loop bandwidth** $B_L \approx \omega_n$ (for $\zeta \approx 0.7$)

**Trade-off**:
- **Narrow BW** ($B_L < 0.01 R_s$): Better noise rejection, slower acquisition
- **Wide BW** ($B_L > 0.1 R_s$): Faster acquisition, more noise

---

### Costas Loop

**For suppressed-carrier modulation** (BPSK, QPSK)

**Structure**:

```
           cos(ωt+θ)
                |
Input --> [×] --> [LPF] --> I(t) --\
          [×] --> [LPF] --> Q(t) ---+--> [Phase  --> [Loop   --> [VCO]
           |                         |    Detector]     Filter]      |
      -sin(ωt+θ)                     |                               |
           ^                         |                               |
           +-------------------------+-------------------------------+
```

**Phase detector**: $e = I \cdot Q$ (for BPSK)

**QPSK**: $e = \text{sign}(I) \cdot Q - I \cdot \text{sign}(Q)$

**Advantage**: Tracks phase without pilot carrier (bandwidth efficient)

---

### Decision-Directed Phase Tracking

**Use decoded symbols** as phase reference:

$$
e[n] = \angle(r[n] \cdot \hat{s}[n]^*)
$$

Where $\hat{s}[n]$ = Decoded symbol (nearest constellation point)

**Update**:

$$
\hat{\theta}[n+1] = \hat{\theta}[n] + \mu \cdot e[n]
$$

**Step size** $\mu$: Small (~0.01) for stability

**Works after initial acquisition** (preamble-based)

---

## Symbol Timing Synchronization

### Timing Offset

**Sample at wrong instant** → ISI, degraded SNR

**Optimal sampling**: Peak of matched filter output (zero ISI point)

**Timing error** $\tau$: Offset from optimal (fraction of symbol period)

---

### Early-Late Gate

**Classic timing recovery** (Mueller & Müller algorithm):

**Sample at 3 points**: Early, On-time ($t_k$), Late

$$
e_k = \text{sign}(r[t_k - T/2]) \cdot r[t_k] - \text{sign}(r[t_k + T/2]) \cdot r[t_k]
$$

**Update**:

$$
\hat{\tau}[k+1] = \hat{\tau}[k] - \mu \cdot e_k
$$

**Advantage**: Works with any signal (blind)

---

### Gardner Timing Error Detector

**Improved early-late** for bandlimited signals:

$$
e_k = (r[t_k] - r[t_{k-1}]) \cdot r[t_k - T/2]
$$

**Where**:
- $r[t_k]$ = Current sample
- $r[t_{k-1}]$ = Previous symbol sample
- $r[t_k - T/2]$ = Mid-point sample

**Advantage**: Better performance, still blind

---

### Maximum Likelihood (ML) Timing

**Data-aided** (preamble):

**Find** $\hat{\tau}$ that maximizes:

$$
\Lambda(\tau) = \left|\sum_{k} r[t_k + \tau] \cdot s_k^*\right|^2
$$

**Implementation**: Interpolate and search (±0.5 symbol period)

**Accuracy**: Sub-sample (~0.01 symbol period)

---

### Timing Loop

**Similar to PLL**:

```
Input --> [Timing Error --> [Loop    --> [NCO/    --> Sample
           Detector]         Filter]      Interp]    Clock
              ↑                               |
              +---------------<---------------+
```

**NCO**: Numerically Controlled Oscillator (adjusts sampling phase)

**Interpolator**: Polyphase filter (fractional delay)

---

## Frame Synchronization

### Purpose

**Identify** packet/frame start in continuous bit stream

**Required for**:
- Header decoding (length, MCS)
- Payload extraction
- Retransmission (ARQ)

---

### Preamble Detection

**Transmit known pattern** at start of frame

**Receiver correlates**:

$$
C[n] = \sum_{k=0}^{L-1} r[n+k] \cdot p[k]^*
$$

Where:
- $p[k]$ = Preamble (length $L$)
- $r[n]$ = Received signal

**Threshold**:

$$
|C[n]| > \gamma \quad \Rightarrow \quad \text{Frame detected at } n
$$

**Threshold** $\gamma$: Balance false alarm vs missed detection

---

### Auto-Correlation (WiFi Example)

**Short preamble**: 16-sample pattern, repeated 10 times

**Auto-correlate** with delayed version:

$$
R[n] = \sum_{k=0}^{15} r[n+k] \cdot r[n+k-16]^*
$$

**Peak** when aligned → Frame start

**Advantage**: No stored template (self-synchronizing)

---

### Barker Code

**Binary sequence** with ideal autocorrelation:

**11-bit Barker**: +1 +1 +1 −1 −1 −1 +1 −1 −1 +1 −1

**Autocorrelation peak**: 11 (sidelobes ≤1)

**Used in**: 802.11b (1-2 Mbps DSSS)

---

### Frame Structure Example (WiFi 802.11a)

```
[Short Preamble (8 μs)] [Long Preamble (8 μs)] [SIGNAL (4 μs)] [DATA]
        |                        |                      |
   CFO coarse              CFO fine, channel est    Rate, length
   AGC setting              Timing sync
```

**Short preamble**: 10× repetition (1.6 μs pattern)
- AGC settling
- Coarse CFO (±100 kHz)
- Frame detection

**Long preamble**: 2× known symbols (3.2 μs each)
- Fine CFO (±1 kHz)
- Channel estimation (64 subcarriers)
- Symbol timing

---

## Practical System Examples

### 1. GPS L1 C/A Acquisition

**Challenge**: Signal below noise floor (−130 dBm)

**C/A code**: 1023-chip Gold code, 1.023 MHz (1 ms period)

**Acquisition**:
1. **Coarse search**: ±5 kHz Doppler, 0.5-chip spacing
2. **FFT-based**: Correlate in frequency domain (fast)
3. **Threshold**: SNR > 10 dB after integration

**Time**: 0.1-1 second (cold start), 1-10 ms (hot start)

---

### 2. LTE Cell Search

**PSS** (Primary Synchronization Signal): Detect slot timing, cell ID (mod 3)

**SSS** (Secondary Synchronization Signal): Frame timing, cell ID (504 total)

**Steps**:
1. **PSS detection**: 3 Zadoff-Chu sequences, correlate every 0.5 ms
2. **Coarse CFO**: From PSS phase
3. **SSS detection**: 168 sequences (2 per cell group)
4. **Fine CFO**: From SSS
5. **PBCH decode**: Master Info Block (bandwidth, frame number)

**Time**: ~100 ms (initial), ~10 ms (handover)

---

### 3. DVB-S2 Satellite Receiver

**Coarse CFO**: ±500 kHz (Doppler, LNB drift)

**Timing offset**: ±100 ppm

**Acquisition**:
1. **PLHEADER**: 90-symbol pilot block (start of frame)
2. **Coarse timing**: Sliding correlation
3. **Fine CFO/phase**: Pilot symbols every 16 data symbols
4. **Tracking**: Decision-directed on pilots

**Time**: ~1 second (blind search), ~100 ms (known frequency)

---

## Synchronization Errors

### CFO Impact

**Small CFO** ($\epsilon = 0.01$):
- Constellation rotates 3.6° per symbol (10 symbols → 36°)
- QPSK: OK (±45° tolerance)
- 256-QAM: **Fails** (±2.8° tolerance)

**Large CFO** ($\epsilon = 0.2$):
- OFDM: ICI dominates, 10+ dB loss

---

### Phase Noise

**Oscillator jitter** causes random phase variation:

$$
\phi_n[k] \sim \mathcal{N}(0, \sigma_\phi^2)
$$

**Effect**: Constellation spreading

**Tolerance**:
- **QPSK**: ~10° RMS
- **64-QAM**: ~2° RMS
- **1024-QAM**: ~0.5° RMS

**Mitigation**: Pilot-based tracking (common phase error estimation)

---

### Timing Jitter

**Clock instability** → Sampling time variation

**Effect**: Effective SNR loss

$$
\text{SNR}_{\text{eff}} = \text{SNR} \cdot \text{sinc}^2(\pi \sigma_\tau)
$$

Where $\sigma_\tau$ = RMS timing error (fraction of symbol period)

**Example**: $\sigma_\tau = 0.1$ → 0.4 dB loss

---

## Design Guidelines

### 1. Choose Loop Bandwidth

**Narrow** ($B_L < 0.01 R_s$):
- High SNR scenarios
- Low phase noise
- Stationary channel

**Wide** ($B_L > 0.1 R_s$):
- Low SNR
- High Doppler (mobile)
- Fast acquisition needed

**Typical**: $B_L \approx 0.01 - 0.05 R_s$ (compromise)

---

### 2. Preamble Design

**Length**: Trade-off overhead vs accuracy
- **Short** (10-50 symbols): Low overhead, lower SNR threshold
- **Long** (100+ symbols): Better accuracy, higher overhead

**Pattern**: Good autocorrelation (low sidelobes)
- **Pseudorandom**: LFSR, Gold codes
- **Constant amplitude**: CAZAC (Zadoff-Chu), reduce PAPR

---

### 3. Pilot Density

**OFDM subcarrier pilots**:
- **Sparse** (1/12 subcarriers): Low overhead, slower tracking
- **Dense** (1/4 subcarriers): Fast tracking, high overhead

**Time-domain pilots** (every N symbols):
- WiFi: ~4-symbol pilot OFDM per packet
- LTE: CRS every symbol (4 subcarriers per 12)

---

## Synchronization Sequence

**Typical receiver startup**:

1. **AGC**: Adjust gain (10-100 μs)
2. **Coarse CFO**: Preamble autocorrelation (±10% symbol rate)
3. **Frame detect**: Cross-correlation with preamble
4. **Fine CFO**: Preamble phase (±0.1% symbol rate)
5. **Symbol timing**: Early-late gate or correlator peak
6. **Phase tracking**: PLL or decision-directed
7. **Channel estimation**: Known pilots/preamble
8. **Data demodulation**: Begin

**Total time**: 0.1-10 ms (packet systems), 0.1-1 s (initial cell search)

---

## Related Topics

- **[[Signal Chain (End-to-End Processing)]]**: Overall system flow
- **[[Channel Equalization]]**: Frequency-selective fading correction
- **[[OFDM & Multicarrier Modulation]]**: Pilot-based sync
- **[[QPSK Modulation]]**: Phase tracking for PSK
- **[[Bit Error Rate (BER)]]**: Performance with sync errors

---

**Key takeaway**: **Synchronization aligns receiver with transmitter in frequency, phase, timing, and frame.** CFO (±20-50 ppm typical) causes constellation rotation—correct with preamble correlation. Phase tracking uses PLL (Costas loop) or decision-directed feedback. Symbol timing recovery via early-late gate (Gardner algorithm). Frame sync via preamble correlation (Barker codes, CAZAC). WiFi: Short preamble (CFO coarse + AGC) → Long preamble (CFO fine + channel est). GPS: Gold code correlation below noise floor. LTE: PSS/SSS for cell search (~100 ms cold start). Loop bandwidth trade-off: Narrow (better noise) vs Wide (faster acquisition). Pilot symbols (OFDM) enable continuous tracking. Synchronization errors degrade BER: 256-QAM needs ±2.8° phase, <1% CFO, <0.1T timing. Critical for coherent demodulation!

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
