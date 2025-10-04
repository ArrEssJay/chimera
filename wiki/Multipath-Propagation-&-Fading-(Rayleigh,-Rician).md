# Multipath Propagation & Fading: Rayleigh & Rician

[[Home]] | **RF Propagation** | [[Propagation Modes (Ground Wave, Sky Wave, Line-of-Sight)]] | [[Atmospheric Effects (Ionospheric, Tropospheric)]]

---

## Overview

**Multipath propagation** occurs when RF signals reach the receiver via **multiple paths** simultaneously, each with different:
- **Delay** (arrival time)
- **Amplitude** (path loss)
- **Phase** (due to different path lengths)

**Result**: Signals combine **constructively** or **destructively**, causing **fading** (rapid signal strength variations).

**Critical for**: Cellular networks, WiFi, mobile satellite, any NLOS (non-line-of-sight) communication

---

## Physical Mechanisms

### Reflection

**EM waves bounce off surfaces**:
- **Ground** (two-ray model)
- **Buildings** (urban canyons)
- **Water** (maritime comms)
- **Ionosphere** (HF skywave)

**Reflection coefficient** depends on:
- Polarization (horizontal vs vertical)
- Angle of incidence
- Surface material (conductivity, permittivity)

**Example**: Concrete wall at 2.4 GHz → ~0.3-0.5 reflection coefficient (~3-6 dB loss per bounce)

---

### Diffraction

**Bending around obstacles** (Fresnel diffraction):
- Building edges
- Hills/terrain
- Trees

**Knife-edge diffraction loss**:

$$
L_d \approx 6 + 20\log\left(\sqrt{(v-0.1)^2 + 1} + v - 0.1\right) \quad (\text{dB})
$$

Where $v$ = Fresnel-Kirchhoff diffraction parameter

**Implication**: Signals can "bend" into shadowed regions (NLOS coverage possible)

---

### Scattering

**Interaction with rough surfaces or small objects**:
- Rough terrain (vegetation, rocks)
- Lamp posts, signs
- Rain/fog droplets (at high frequencies)

**Rayleigh scattering** (object size ≪ λ):

$$
P_{\text{scattered}} \propto \frac{1}{\lambda^4}
$$

**Example**: Blue sky (visible light Rayleigh scattering from air molecules)

---

## Time-Domain Effects

### Delay Spread

**Multipath components arrive at different times**:

$$
\tau_{\text{rms}} = \sqrt{\frac{\sum P_i (\tau_i - \bar{\tau})^2}{\sum P_i}}
$$

Where:
- $P_i$ = Power of path $i$
- $\tau_i$ = Delay of path $i$
- $\bar{\tau}$ = Mean delay

**Typical values**:
- **Rural/suburban**: 0.1-1 μs
- **Urban**: 1-5 μs
- **Indoor**: 10-100 ns

---

### Coherence Bandwidth

**Frequency range over which channel response is flat**:

$$
B_c \approx \frac{1}{5\tau_{\text{rms}}}
$$

**Implication**: If signal bandwidth $B > B_c$ → **Frequency-selective fading** (different frequencies fade independently)

**Example**: Urban ($\tau_{\text{rms}} = 1\ \mu$s):

$$
B_c \approx \frac{1}{5 \times 10^{-6}} = 200\ \text{kHz}
$$

- **Narrowband signal** (< 200 kHz): Flat fading
- **Wideband signal** (> 200 kHz): Frequency-selective fading (ISI)

---

### Intersymbol Interference (ISI)

**Delayed multipath overlaps with next symbol**:

**Condition for ISI**:

$$
\tau_{\text{rms}} > T_s
$$

Where $T_s$ = Symbol period

**Example**: 1 Mbps data ($T_s = 1\ \mu$s), urban channel ($\tau_{\text{rms}} = 3\ \mu$s):
- ISI severe (3 symbols overlap!)
- **Mitigation**: Equalization, OFDM (see [[OFDM & Multicarrier Modulation]])

---

## Frequency-Domain Effects

### Doppler Shift

**Relative motion between TX/RX causes frequency shift**:

$$
f_d = \frac{v}{\lambda} \cos(\theta)
$$

Where:
- $v$ = Velocity (m/s)
- $\theta$ = Angle between velocity and signal direction
- $f_d$ = Doppler shift (Hz)

**Example**: Car at 100 km/h (27.8 m/s), 2 GHz signal:

$$
f_d = \frac{27.8}{0.15} \cos(0°) = 185\ \text{Hz}
$$

---

### Doppler Spread

**Multiple paths with different Doppler shifts**:

$$
B_d = 2f_{d,\text{max}} = \frac{2v}{\lambda}
$$

**Coherence time** (how long channel stays constant):

$$
T_c \approx \frac{0.423}{B_d}
$$

**Example**: 100 km/h at 2 GHz:

$$
B_d = 2 \times 185 = 370\ \text{Hz}
$$

$$
T_c = \frac{0.423}{370} = 1.14\ \text{ms}
$$

**Implication**: Channel changes every ~1 ms (fast fading for stationary systems, slow fading for fast data rates)

---

## Fading Classifications

### Flat vs Frequency-Selective

| Type | Condition | Effect | Mitigation |
|------|-----------|--------|------------|
| **Flat** | $B \ll B_c$ | All frequencies fade together | Diversity, FEC |
| **Frequency-selective** | $B \gg B_c$ | Different frequencies fade independently | Equalization, OFDM |

---

### Fast vs Slow Fading

| Type | Condition | Effect | Mitigation |
|------|-----------|--------|------------|
| **Slow** | $T_c \gg T_s$ | Channel constant over many symbols | Interleaving, FEC |
| **Fast** | $T_c \ll T_s$ | Channel changes within symbol | Pilot-aided estimation |

---

## Rayleigh Fading

**Occurs when**: **No dominant LOS path**, many scattered components with random phases

**Statistical model**: Envelope follows **Rayleigh distribution**

### PDF (Probability Density Function)

$$
p(r) = \frac{r}{\sigma^2} \exp\left(-\frac{r^2}{2\sigma^2}\right), \quad r \geq 0
$$

Where:
- $r$ = Signal envelope
- $\sigma^2$ = Average power

**Mean**: $\bar{r} = \sigma\sqrt{\pi/2}$

**Variance**: $\text{Var}(r) = \sigma^2(2 - \pi/2)$

---

### CDF (Cumulative Distribution Function)

**Probability that signal < threshold**:

$$
P(r < R) = 1 - \exp\left(-\frac{R^2}{2\sigma^2}\right)
$$

**Example**: Probability signal drops below 10 dB below average:

$$
P(r < 0.316\bar{r}) = 1 - \exp(-0.05) \approx 5\%
$$

---

### Impact on BER

**Rayleigh fading severely degrades performance**:

**BPSK in Rayleigh fading**:

$$
\text{BER} = \frac{1}{2}\left(1 - \sqrt{\frac{\bar{\gamma}}{1 + \bar{\gamma}}}\right)
$$

Where $\bar{\gamma} = E_b/N_0$ (average SNR)

**Comparison** (at 10 dB average SNR):
- **AWGN**: BER = 3.9 × 10⁻⁶
- **Rayleigh fading**: BER = 0.005 (1000× worse!)

**Key insight**: Deep fades cause **error bursts** (when signal drops into noise floor)

---

### Applications

**Typical Rayleigh environments**:
- Dense urban (no LOS, many reflections)
- Indoor (office, corridors)
- Suburban/rural (obstructed by trees/buildings)

---

## Rician Fading

**Occurs when**: **Dominant LOS path + scattered components**

**Statistical model**: Envelope follows **Rician distribution**

### PDF

$$
p(r) = \frac{r}{\sigma^2} \exp\left(-\frac{r^2 + A^2}{2\sigma^2}\right) I_0\left(\frac{Ar}{\sigma^2}\right)
$$

Where:
- $A$ = Amplitude of LOS component
- $I_0$ = Modified Bessel function of first kind, order 0

**Rician K-factor**:

$$
K = \frac{A^2}{2\sigma^2} = \frac{\text{LOS power}}{\text{Scattered power}}
$$

**K in dB**: $K_{\text{dB}} = 10\log_{10}(K)$

---

### Interpretation of K-factor

| K (dB) | Environment | Fading Severity |
|--------|-------------|-----------------|
| **-∞ (K=0)** | No LOS (pure Rayleigh) | **Severe** (deep fades) |
| **0 dB (K=1)** | Equal LOS/scatter | Moderate |
| **6 dB (K=4)** | Strong LOS | Mild |
| **10 dB (K=10)** | Dominant LOS | Negligible fading |
| **+∞** | Pure LOS (AWGN-like) | None |

**Special case**: $K = 0$ → Rayleigh fading (Rician generalizes Rayleigh)

---

### Impact on BER

**Rician fading is less severe than Rayleigh**:

**BPSK in Rician fading**:

$$
\text{BER} = Q\left(\sqrt{\frac{2K\bar{\gamma}}{K+1}}\right) \exp\left(-\frac{K\bar{\gamma}}{K+1}\right) \times ...
$$

(Complex expression, see textbooks)

**Comparison** (10 dB average SNR):
- **AWGN**: BER = 3.9 × 10⁻⁶
- **Rician K=6 dB**: BER ≈ 10⁻⁵ (better than Rayleigh, worse than AWGN)
- **Rayleigh (K=0)**: BER = 0.005

---

### Applications

**Typical Rician environments**:
- Suburban with partial LOS
- Elevated antennas (rooftops)
- Satellite-to-handheld (weak LOS + ground reflections)
- Indoor near windows (outdoor LOS + indoor scatter)

---

## Fade Depth & Duration

### Fade Margin

**Link budget must include fade margin** to maintain target availability:

$$
P_r(\text{min}) = P_r(\text{average}) - M_{\text{fade}}
$$

**Example**: Target 99% availability (1% outage)

**Rayleigh fading**: 10% of time, signal < -10 dB below average
- Need **10 dB margin** for 90% availability
- Need **20 dB margin** for 99% availability

**Rician K=6 dB**: Fades less severe
- ~5 dB margin for 90%
- ~10 dB margin for 99%

---

### Level Crossing Rate (LCR)

**How often signal crosses threshold** (in fades/sec):

$$
N_R = \sqrt{2\pi} f_d \rho \exp(-\rho^2)
$$

Where:
- $f_d$ = Maximum Doppler frequency
- $\rho = R/R_{\text{rms}}$ (normalized threshold)

**Example**: Mobile at 100 km/h, 2 GHz ($f_d = 185$ Hz), threshold = average power ($\rho = 1$):

$$
N_R = \sqrt{2\pi} \times 185 \times 1 \times \exp(-1) \approx 85\ \text{fades/sec}
$$

---

### Average Fade Duration

$$
\bar{t} = \frac{\exp(\rho^2) - 1}{\rho f_d \sqrt{2\pi}}
$$

**Example**: Same scenario, threshold = average:

$$
\bar{t} = \frac{e - 1}{1 \times 185 \times \sqrt{2\pi}} \approx 3.7\ \text{ms}
$$

**Implication**: Fast fading (85 fades/sec), short fades (~4 ms) → Interleaving effective

---

## Mitigation Techniques

### 1. Diversity

**Combine multiple independent fading signals**:

#### Spatial Diversity (Antenna Diversity)

**Separate antennas by** $d > \lambda/2$:

**Diversity gain**: ~10 dB improvement with 2 antennas (selection combining)

**Example**: WiFi access point with 2 antennas (2.4 GHz, $\lambda = 12.5$ cm):
- Antenna spacing: 6 cm minimum
- Result: Probability both antennas in deep fade is very low

---

#### Frequency Diversity

**Transmit same data on multiple frequencies** separated by $> B_c$:

**Application**: Frequency hopping spread spectrum (FHSS)

---

#### Time Diversity

**Transmit same data at different times** separated by $> T_c$:

**Implementation**: Interleaving + FEC (spread coded bits over time)

---

### 2. Equalization

**Compensate for frequency-selective fading** (ISI):

#### Linear Equalization (LE)

**Zero-forcing (ZF)**: Invert channel response

$$
H_{\text{eq}}(f) = \frac{1}{H_{\text{channel}}(f)}
$$

**Problem**: Noise amplification at deep fades

---

#### Decision-Feedback Equalization (DFE)

**Use past decisions** to cancel ISI:

**Better than ZF**: Doesn't amplify noise as much

---

#### Adaptive Equalization

**Track time-varying channel**:

**Algorithms**: LMS (Least Mean Squares), RLS (Recursive Least Squares)

**Training**: Periodic pilot symbols to update equalizer coefficients

---

### 3. OFDM

**Divide wideband signal into many narrowband subcarriers**:

**Each subcarrier < $B_c$** → Flat fading per subcarrier

**Per-subcarrier equalization**: Simple single-tap equalizer

**See**: [[OFDM & Multicarrier Modulation]]

---

### 4. Spread Spectrum

**Spread signal over wide bandwidth**:

**Frequency diversity**: Different frequency components fade independently

**See**: [[Spread Spectrum (DSSS, FHSS)]]

---

### 5. Error Correction Coding

**FEC protects against error bursts**:

**Interleaving**: Spread coded bits across time/frequency

**Example**: Convolutional code + interleaver
- Error burst of 10 bits → Spread across 100+ bit positions
- Decoder sees isolated errors (easier to correct)

**See**: [[Forward Error Correction (FEC)]], [[LDPC Codes]]

---

## Channel Models

### Rayleigh Fading Channel (MATLAB/Python)

```python
import numpy as np

def rayleigh_fading(N, fd, fs):
    """
    Generate Rayleigh fading samples
    N: Number of samples
    fd: Maximum Doppler frequency (Hz)
    fs: Sampling frequency (Hz)
    """
    # Jakes' model (sum of sinusoids)
    M = 8  # Number of scatterers
    t = np.arange(N) / fs
    
    h_real = np.zeros(N)
    h_imag = np.zeros(N)
    
    for m in range(M):
        theta_m = 2*np.pi*m / M
        phi_m = np.random.uniform(0, 2*np.pi)
        h_real += np.cos(2*np.pi*fd*t*np.cos(theta_m) + phi_m)
        h_imag += np.sin(2*np.pi*fd*t*np.cos(theta_m) + phi_m)
    
    h_real /= np.sqrt(M)
    h_imag /= np.sqrt(M)
    
    h = h_real + 1j*h_imag
    return h / np.sqrt(np.mean(np.abs(h)**2))  # Normalize
```

---

### Rician Fading Channel

```python
def rician_fading(N, K_dB, fd, fs):
    """
    Generate Rician fading samples
    K_dB: Rician K-factor (dB)
    """
    K = 10**(K_dB/10)  # Convert to linear
    
    # Rayleigh component (scattered)
    h_scatter = rayleigh_fading(N, fd, fs)
    
    # LOS component
    h_los = np.sqrt(K / (K+1))
    
    # Scattered component scaled
    h_scatter *= np.sqrt(1 / (K+1))
    
    return h_los + h_scatter
```

---

## Practical Examples

### Example 1: Urban Cellular (900 MHz)

**Environment**: Dense urban, NLOS

**Parameters**:
- Delay spread: $\tau_{\text{rms}} = 3\ \mu$s
- Doppler: $f_d = 50$ Hz (30 km/h)
- Fading: Rayleigh

**Coherence bandwidth**:

$$
B_c = \frac{1}{5 \times 3 \times 10^{-6}} = 67\ \text{kHz}
$$

**Implication**: GSM channel (200 kHz) experiences frequency-selective fading → Equalizer needed

**Coherence time**:

$$
T_c = \frac{0.423}{100} = 4.23\ \text{ms}
$$

**Implication**: Channel constant over ~18 GSM symbols (0.577 ms/symbol) → Slow fading

---

### Example 2: Suburban LTE (2.6 GHz)

**Environment**: Suburban, partial LOS

**Parameters**:
- Delay spread: $\tau_{\text{rms}} = 0.5\ \mu$s
- Doppler: $f_d = 240$ Hz (100 km/h)
- Fading: Rician K=5 dB

**Coherence bandwidth**:

$$
B_c = \frac{1}{5 \times 0.5 \times 10^{-6}} = 400\ \text{kHz}
$$

**Implication**: LTE RB (180 kHz) < $B_c$ → Mostly flat fading per RB

**Coherence time**:

$$
T_c = \frac{0.423}{480} = 0.88\ \text{ms}
$$

**Implication**: Channel changes over ~12 OFDM symbols (71 μs/symbol) → Moderate fading, pilot-aided tracking

---

## Summary Table

| Parameter | Rayleigh | Rician (K=6 dB) | AWGN |
|-----------|----------|-----------------|------|
| **LOS component** | None | Dominant | Pure LOS |
| **Fade depth (10% time)** | -10 dB | -5 dB | 0 dB |
| **BER penalty @ 10 dB SNR** | 1000× | 10× | 1× (baseline) |
| **Mitigation** | Diversity, FEC | Moderate FEC | Minimal FEC |
| **Typical environment** | Dense urban, indoor | Suburban, elevated | Free space, satellite |

---

## Related Topics

- **[[Propagation Modes (Ground Wave, Sky Wave, Line-of-Sight)]]**: LOS vs NLOS propagation
- **[[Atmospheric Effects (Ionospheric, Tropospheric)]]**: Clear-air effects (different from multipath)
- **[[Signal-to-Noise Ratio (SNR)]]**: Fading reduces instantaneous SNR
- **[[Bit Error Rate (BER)]]**: Fading degrades BER significantly
- **[[QPSK Modulation]]** / **[[LDPC Codes]]**: Require fade mitigation
- **[[OFDM & Multicarrier Modulation]]**: Combats frequency-selective fading
- **[[Spread Spectrum (DSSS, FHSS)]]**: Provides frequency diversity

---

**Key takeaway**: **Multipath fading is the dominant impairment** in mobile/urban wireless. Rayleigh fading (no LOS) is severe, Rician fading (with LOS) is moderate. Mitigation requires diversity, equalization, OFDM, and robust FEC. Understanding $B_c$ and $T_c$ is critical for system design.

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
