# Baseband vs Passband Signals

[[Home]] | **Digital Modulation** | [[QPSK Modulation]] | [[IQ Representation]]

---

## Overview

**Baseband signal**: Information signal at **original frequency range** (near DC, ~0 Hz)

**Passband signal**: Information signal **shifted to carrier frequency** $f_c$ (RF)

**Why we need both**:
- **Baseband**: Digital signal processing, modulation/demodulation, algorithm development
- **Passband**: Radio transmission (antennas need RF, spectrum allocation, propagation)

**Key operation**: **Upconversion** (baseband → passband) and **downconversion** (passband → baseband)

---

## Baseband Signal

**Definition**: Signal with frequency content centered **around DC (0 Hz)**

**Spectrum**: Extends from ~0 Hz to $B$ Hz (bandwidth)

### Examples

**Digital baseband**:
- NRZ (Non-Return-to-Zero): Rectangular pulses, ±1
- Manchester encoding: Phase transitions
- Pulse-shaped symbols: Raised cosine, RRC

**Analog baseband**:
- Voice: 300-3400 Hz
- Audio: 20 Hz - 20 kHz
- Video: DC - 6 MHz (NTSC)

---

### Complex Baseband Representation

**For bandpass systems**, represent signal as **complex envelope**:

$$
s(t) = s_I(t) + j s_Q(t)
$$

Where:
- $s_I(t)$ = In-phase component
- $s_Q(t)$ = Quadrature component

**Advantages**:
- Simplifies DSP (single complex signal vs two real signals)
- Natural representation for IQ modulation
- Halves sampling rate requirement (no negative frequencies)

**See**: [[IQ Representation]]

---

### Baseband Bandwidth

**Occupied bandwidth** depends on symbol rate $R_s$ and pulse shaping:

**Ideal rectangular pulses**: 

$$
B = R_s \quad (\text{Hz})
$$

**Raised cosine pulse shaping** (roll-off $\alpha$):

$$
B = R_s (1 + \alpha) \quad (\text{Hz})
$$

**Example**: QPSK @ 1 Msps, α = 0.35
- Bandwidth: 1 × (1 + 0.35) = 1.35 MHz (baseband)

---

## Passband Signal

**Definition**: Signal with frequency content centered **around carrier $f_c$**

**Spectrum**: Extends from $f_c - B/2$ to $f_c + B/2$

### Why Passband?

1. **Antenna efficiency**: Antenna size ~ λ/4, need high frequency for practical size
   - 100 Hz baseband: λ = 3000 km → 750 km antenna (infeasible!)
   - 2.4 GHz RF: λ = 12.5 cm → 3 cm antenna (WiFi)

2. **Spectrum allocation**: Different services assigned different frequency bands (AM 540-1600 kHz, FM 88-108 MHz, WiFi 2.4/5 GHz)

3. **Propagation characteristics**: HF skips ionosphere, VHF line-of-sight, UHF penetrates buildings

4. **Multiplexing**: Multiple baseband signals upconverted to different carriers (FDM)

---

### Passband Representation

**Real passband signal** from complex baseband:

$$
s_{\text{RF}}(t) = \text{Re}\{s(t) e^{j2\pi f_c t}\}
$$

$$
= s_I(t) \cos(2\pi f_c t) - s_Q(t) \sin(2\pi f_c t)
$$

**Interpretation**: **IQ modulation**
- I channel modulates cosine (0° phase)
- Q channel modulates sine (90° phase)

**Example**: QPSK
- $s(t) = A e^{j\phi}$ where $\phi \in \{45°, 135°, 225°, 315°\}$
- $s_I(t) = A\cos\phi$, $s_Q(t) = A\sin\phi$
- $s_{\text{RF}}(t) = A\cos\phi \cos(2\pi f_c t) - A\sin\phi \sin(2\pi f_c t) = A\cos(2\pi f_c t + \phi)$

---

## Upconversion (Modulation)

**Process**: Shift baseband signal to carrier frequency

### IQ Modulator (Quadrature Modulator)

**Block diagram**:
```
           cos(2πf_c t)
                |
    s_I(t) --> [×] ----\
                        [+] --> s_RF(t)
    s_Q(t) --> [×] ----/
                |
          -sin(2πf_c t)
```

**Output**:

$$
s_{\text{RF}}(t) = s_I(t) \cos(2\pi f_c t) - s_Q(t) \sin(2\pi f_c t)
$$

---

### Single-Sideband (SSB) Upconversion

**Complex multiplication**:

$$
s_{\text{RF}}(t) = \text{Re}\{s(t) e^{j2\pi f_c t}\}
$$

**In frequency domain**:

$$
S_{\text{RF}}(f) = \frac{1}{2}[S(f - f_c) + S^*(-f - f_c)]
$$

**Result**: Positive frequencies shifted to $f_c$, negative frequencies to $-f_c$ (conjugate)

**Since $s(t)$ real RF signal**: Spectrum symmetric around 0, so both sidebands present

---

### Image Rejection

**Problem**: Real mixer produces both $f_c + f_{\text{BB}}$ and $f_c - f_{\text{BB}}$ (USB and LSB)

**IQ modulator advantage**: Can select **one sideband** by controlling I/Q phase
- USB only: I/Q phase = +90°
- LSB only: I/Q phase = -90°
- DSB: I only (Q = 0)

---

### Example: WiFi 2.4 GHz

**Baseband**:
- Symbol rate: 20 Msps (20 MHz OFDM)
- Complex baseband: -10 MHz to +10 MHz

**Upconversion**:
- Carrier: 2.412 GHz (channel 1)
- RF spectrum: 2.402-2.422 GHz (20 MHz)

**Transmit chain**:
1. Generate OFDM baseband (I/Q symbols)
2. DAC @ 40 Msps (2× oversampling)
3. IQ modulator @ 2.412 GHz
4. PA → antenna

---

## Downconversion (Demodulation)

**Process**: Shift RF signal back to baseband

### IQ Demodulator (Quadrature Demodulator)

**Block diagram**:
```
               cos(2πf_c t)
                    |
s_RF(t) --> [×] --> [LPF] --> s_I(t)
         |
         |  -sin(2πf_c t)
         |      |
         └--> [×] --> [LPF] --> s_Q(t)
```

**I channel**:

$$
s_I(t) = \text{LPF}\{s_{\text{RF}}(t) \cos(2\pi f_c t)\}
$$

**Q channel**:

$$
s_Q(t) = \text{LPF}\{s_{\text{RF}}(t) \cdot [-\sin(2\pi f_c t)]\}
$$

---

### Mathematical Derivation

**Input**:

$$
s_{\text{RF}}(t) = s_I^{\text{TX}}(t) \cos(2\pi f_c t) - s_Q^{\text{TX}}(t) \sin(2\pi f_c t)
$$

**I channel after mixing**:

$$
s_I^{\text{mix}}(t) = [s_I^{\text{TX}} \cos(2\pi f_c t) - s_Q^{\text{TX}} \sin(2\pi f_c t)] \cos(2\pi f_c t)
$$

$$
= s_I^{\text{TX}} \cos^2(2\pi f_c t) - s_Q^{\text{TX}} \sin(2\pi f_c t)\cos(2\pi f_c t)
$$

**Using trig identities**:
- $\cos^2\theta = \frac{1 + \cos(2\theta)}{2}$
- $\sin\theta\cos\theta = \frac{\sin(2\theta)}{2}$

$$
s_I^{\text{mix}}(t) = s_I^{\text{TX}} \frac{1 + \cos(4\pi f_c t)}{2} - s_Q^{\text{TX}} \frac{\sin(4\pi f_c t)}{2}
$$

**After LPF** (removes $2f_c$ terms):

$$
s_I(t) = \frac{1}{2} s_I^{\text{TX}}(t)
$$

**Similarly for Q channel**:

$$
s_Q(t) = \frac{1}{2} s_Q^{\text{TX}}(t)
$$

**Recovered baseband** (with 1/2 amplitude, easily corrected):

$$
s(t) = s_I(t) + j s_Q(t) = \frac{1}{2}[s_I^{\text{TX}}(t) + j s_Q^{\text{TX}}(t)]
$$

---

### Image Frequency

**Problem**: Mixer sensitive to both $f_c + f$ and $f_c - f$

**Image frequency**: $f_{\text{image}} = 2f_c - f_{\text{desired}}$

**Example**: Desired signal @ 2.45 GHz, LO @ 2.4 GHz
- Downconverted to: 2.45 - 2.4 = 50 MHz
- Image @ 2.4 - 0.05 = 2.35 GHz also downconverts to 50 MHz!

**Mitigation**:
- **Image-reject filter** before mixer (RF bandpass filter)
- **IQ demodulator** (natural image rejection if I/Q balanced)
- **Superheterodyne** (multiple conversion stages with filtering)

---

## Superheterodyne Receiver

**Classic architecture**: RF → IF → Baseband

**Stages**:
1. **RF stage**: LNA, RF bandpass filter
2. **First mixer**: RF → IF (intermediate frequency, e.g., 10.7 MHz for FM radio)
3. **IF stage**: IF filter (high selectivity), IF amplifier
4. **Second mixer**: IF → Baseband (or direct demodulation at IF)

**Advantages**:
- **Image rejection**: IF filter very selective
- **Fixed IF**: Optimized filters regardless of RF tuning
- **Gain distribution**: Spread gain across stages (avoid instability)

**Example**: FM radio receiver
- RF: 88-108 MHz (tunable)
- LO: 98.7-118.7 MHz (tracks RF + 10.7 MHz)
- IF: 10.7 MHz (fixed)
- Crystal filter @ IF: 150 kHz bandwidth (adjacent channel rejection)

---

## Zero-IF (Direct Conversion) Receiver

**Modern SDR approach**: RF → Baseband (no IF)

**Advantages**:
- Fewer components (no IF filters, single LO)
- Compact, low power (mobile devices)
- Flexible (software-defined bandwidth)

**Challenges**:
- **DC offset**: LO leakage self-mixes to DC (corrupts baseband)
- **Flicker noise**: 1/f noise near DC
- **I/Q imbalance**: Gain/phase mismatch between I/Q paths

**Mitigation**:
- AC coupling (removes DC)
- High-pass filtering (kills flicker noise)
- Digital calibration (I/Q imbalance correction)

---

## Sampling Considerations

### Nyquist for Passband Signals

**Real passband signal** $s_{\text{RF}}(t)$ centered at $f_c$, bandwidth $B$:

**Bandpass sampling theorem**: Can sample at $f_s < 2f_c$ if:

$$
f_s \geq 2B
$$

**Condition**: $f_c = n \frac{f_s}{4}$ (integer $n$) for easy downconversion

**Example**: WiFi @ 2.4 GHz, 20 MHz BW
- Minimum $f_s = 2 \times 20 = 40$ MHz (bandpass sampling)
- Typical $f_s$ = 80-100 MHz (allows filtering roll-off)

---

### Complex Baseband Sampling

**Complex baseband** $s(t) = s_I(t) + j s_Q(t)$:

**Sampling rate**:

$$
f_s \geq B \quad (\text{Hz})
$$

**Why lower?** Negative frequencies meaningful (complex signal asymmetric)

**Example**: QPSK @ 1 MHz baseband bandwidth
- Real passband @ 2.4 GHz: Need $f_s \geq 2$ MHz (bandpass sampling)
- Complex baseband: Need $f_s \geq 1$ MHz (but typically 2× for pulse shaping)

---

## Practical Impairments

### 1. Carrier Frequency Offset (CFO)

**TX and RX oscillators not perfectly matched**:

$$
\Delta f = f_{\text{TX}} - f_{\text{RX}}
$$

**Effect on baseband**:

$$
s_{\text{RX}}(t) = s(t) e^{j2\pi \Delta f t}
$$

**Consequence**: Constellation rotates over time

**Typical**: ±10 ppm (parts per million)
- @ 2.4 GHz: ±24 kHz offset
- @ 28 GHz (5G mmWave): ±280 kHz offset

**Mitigation**: Frequency synchronization (see [[Synchronization]])

---

### 2. Phase Noise

**Oscillator jitter** causes random phase variations:

$$
s_{\text{RF}}(t) = s_I(t) \cos(2\pi f_c t + \phi_n(t))
$$

Where $\phi_n(t)$ = Random phase noise process

**Effect**: Constellation spreading, ICI (inter-carrier interference in OFDM)

**Spec**: $\mathcal{L}(f_m)$ (phase noise PSD at offset $f_m$ from carrier, dBc/Hz)

**Example**: Good TCXO @ 10 kHz offset
- Phase noise: -110 dBc/Hz
- Integrated phase error: ~1° RMS (acceptable for QPSK)

---

### 3. I/Q Imbalance

**Gain mismatch**: $G_I \neq G_Q$

**Phase mismatch**: 90° shifter imperfect (e.g., 88° or 92°)

**Effect**: Image sideband leakage, constellation distortion

**Model**:

$$
s_{\text{imb}}(t) = G_I s_I(t) + G_Q e^{j(\pi/2 + \epsilon)} s_Q(t)
$$

**Typical**: ±0.5 dB gain, ±2° phase (good hardware)

**Mitigation**: Digital pre-distortion, calibration using known pilots

---

### 4. LO Leakage (DC Offset)

**TX LO leaks** into RF path → Self-mixing at RX → DC component

**Effect**: DC spike in baseband spectrum

**Mitigation**:
- AC coupling (blocks DC)
- Blank center subcarrier (OFDM)
- Digital DC offset estimation/cancellation

---

## Spectral Efficiency Comparison

| Architecture | Bandwidth Used | Spectral Efficiency | Example |
|--------------|----------------|---------------------|---------|
| **Baseband (DSB)** | $2B$ (USB + LSB) | N/A (not RF) | Ethernet |
| **SSB (analog)** | $B$ | 1× | HAM radio |
| **DSB-SC** | $2B$ | 0.5× | AM radio (suppressed carrier) |
| **VSB** | $1.25B$ | 0.8× | Analog TV |
| **IQ modulation** | $B$ | 1× | QPSK, QAM (most digital) |

---

## Summary Table

| Aspect | Baseband | Passband |
|--------|----------|----------|
| **Frequency range** | ~0 to $B$ Hz | $f_c - B/2$ to $f_c + B/2$ |
| **Signal type** | Complex or real | Real only |
| **Sampling rate** | $\geq B$ (complex) or $\geq 2B$ (real) | $\geq 2B$ (bandpass) |
| **Processing** | Digital (DSP) | Analog (RF) or digital (SDR) |
| **Transmission** | Wired (Ethernet) | Wireless (antenna) |
| **Representation** | $s(t) = s_I + js_Q$ | $s_{\text{RF}} = s_I\cos\omega t - s_Q\sin\omega t$ |

---

## Related Topics

- **[[IQ Representation]]**: Complex baseband I/Q signals
- **[[QPSK Modulation]]**: Example of IQ modulation
- **[[Constellation Diagrams]]**: Visualizing baseband IQ symbols
- **[[Synchronization]]**: Carrier frequency/phase recovery
- **[[OFDM & Multicarrier Modulation]]**: Uses IQ modulation per subcarrier
- **[[Free-Space Path Loss (FSPL)]]**: Why we need RF (antenna efficiency)

---

**Key takeaway**: **Baseband = information at low frequency, passband = shifted to RF carrier.** IQ modulation (quadrature upconversion) shifts complex baseband to RF without image. Downconversion reverses process. Complex baseband simplifies DSP, halves sample rate. Passband required for wireless (antenna, propagation, spectrum). Practical impairments: CFO, phase noise, I/Q imbalance, LO leakage. Superheterodyne = RF→IF→BB (classic), zero-IF = RF→BB (modern SDR).

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
