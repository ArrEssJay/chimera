# Channel Equalization

[[Home]] | **System Implementation** | [[Synchronization (Carrier, Timing, Frame)]] | [[Multipath Propagation & Fading (Rayleigh, Rician)]]

---

## 🎚️ For Non-Technical Readers

**Channel equalization is like using an audio equalizer to undo the distortion from a bad microphone—it reverses the damage the radio channel causes to your signal!**

**The problem**: 
- Radio signals bounce off buildings/walls (multipath)
- Echoes arrive at different times and smear together → **inter-symbol interference (ISI)**
- It's like someone talking in a cave—echoes make words blur together

**The solution - Equalization**:
1. Receiver analyzes how the channel distorts known training signals
2. Calculates inverse filter: "Channel made signal quieter at 5 kHz? Let's boost 5 kHz!"
3. Applies correction to all received data
4. Result: Clean, sharp signal restored!

**Real-world analogy - Audio equalizer**:
- Cheap headphones boost bass (distortion)
- Audio app detects this and reduces bass to compensate
- Result: Flat, accurate sound
- Channel equalizer does the same for radio signals!

**Types you encounter**:
- **Linear equalizers** (simple, fast): WiFi, basic cellular
- **Decision feedback equalizers** (smarter): High-speed data links
- **Adaptive equalizers** (learns channel in real-time): Your phone constantly adjusts as you move!

**When you see it**:
- **4G/5G handoff**: Brief pause = phone learning new tower's channel
- **Fast internet over long phone lines**: DSL equalizers undo cable distortion
- **Underwater communications**: Extreme multipath requires heavy equalization

**Fun fact**: Modern equalizers update hundreds of times per second as you walk—they track the changing radio environment in real-time!

---

## Overview

**Channel equalization** compensates for **Inter-Symbol Interference (ISI)** caused by multipath propagation.

**Problem**: Delayed signal copies overlap with current symbol → ISI

**Solution**: Apply inverse channel filter to restore original signal

**Types**:
1. **Linear equalizers**: ZF, MMSE
2. **Nonlinear equalizers**: DFE (Decision Feedback)
3. **Adaptive equalizers**: LMS, RLS
4. **Frequency-domain**: OFDM per-subcarrier

---

## Inter-Symbol Interference (ISI)

### Cause

**Multipath channel**:

$$
h(t) = \sum_{l=0}^{L-1} h_l \delta(t - \tau_l)
$$

**Received signal**:

$$
r(t) = \sum_{l=0}^{L-1} h_l \cdot s(t - \tau_l) + n(t)
$$

**Effect**: Current symbol affected by $L-1$ previous symbols

---

### ISI Illustration

**Transmit**: 1 0 1 1

**Channel**: 2-tap ($h_0 = 1$, $h_1 = 0.5$, delay = 1 symbol)

**Received**:
- Symbol 0: $1 \cdot h_0 = 1.0$ ✓
- Symbol 1: $0 \cdot h_0 + 1 \cdot h_1 = 0.5$ ✗ (ISI from symbol 0)
- Symbol 2: $1 \cdot h_0 + 0 \cdot h_1 = 1.0$ ✓
- Symbol 3: $1 \cdot h_0 + 1 \cdot h_1 = 1.5$ ✗ (ISI from symbol 2)

**Equalizer goal**: Remove $h_1$ contribution

---

### Delay Spread

**RMS delay spread** $\tau_{\text{RMS}}$: Channel memory duration

**Coherence bandwidth**:

$$
B_c \approx \frac{1}{5 \tau_{\text{RMS}}}
$$

**Flat fading**: $B_{\text{signal}} < B_c$ (no ISI)

**Frequency-selective fading**: $B_{\text{signal}} > B_c$ (ISI present, equalization needed)

---

## Zero-Forcing (ZF) Equalizer

**Idea**: Perfect inversion of channel (force ISI to zero)

**Frequency domain**:

$$
W(f) = \frac{1}{H(f)}
$$

**Time domain** (FIR filter, $N$ taps):

$$
y[n] = \sum_{k=0}^{N-1} w_k \cdot r[n-k]
$$

**Optimal taps**: $\mathbf{w} = (\mathbf{H}^H \mathbf{H})^{-1} \mathbf{H}^H \mathbf{e}_0$

Where $\mathbf{e}_0 = [1, 0, \ldots, 0]^T$ (force zero ISI)

---

### ZF Performance

**Advantage**: **Perfect ISI cancellation** (if channel known)

**Disadvantage**: **Noise enhancement** at frequency nulls

**Example**: If $H(f_0) \approx 0$ (deep fade), $W(f_0) \to \infty$ → Amplifies noise

**Result**: ZF poor at low SNR

---

## Minimum Mean Square Error (MMSE) Equalizer

**Idea**: Minimize **combined** ISI + noise (trade-off)

**Cost function**:

$$
\text{MSE} = E[|s[n] - y[n]|^2]
$$

**Optimal taps** (Wiener solution):

$$
\mathbf{w}_{\text{MMSE}} = (\mathbf{H}^H \mathbf{H} + \sigma^2 \mathbf{I})^{-1} \mathbf{H}^H \mathbf{e}_0
$$

Where $\sigma^2 = N_0/E_s$ (normalized noise)

---

### MMSE vs ZF

**Frequency domain**:

$$
W_{\text{MMSE}}(f) = \frac{H^*(f)}{|H(f)|^2 + \sigma^2}
$$

**At deep fade** ($|H(f)| \ll 1$): $W \approx H^*/\sigma^2$ (doesn't blow up)

**At high SNR** ($\sigma^2 \to 0$): $W \to H^*/|H|^2 = 1/H$ (converges to ZF)

**Result**: MMSE better than ZF at low-moderate SNR

---

### Performance Comparison

| SNR (dB) | ZF BER | MMSE BER | Improvement |
|----------|--------|----------|-------------|
| 5 | 0.05 | 0.02 | 2.5× better |
| 10 | 0.01 | 0.005 | 2× better |
| 20 | 10⁻⁴ | 10⁻⁴ | Same |
| 30 | 10⁻⁶ | 10⁻⁶ | Same |

**Pattern**: MMSE wins at low SNR, converge at high SNR

---

## Decision Feedback Equalizer (DFE)

**Idea**: Use **past decisions** to cancel ISI from previous symbols

**Structure**:

```
           [Feedforward Filter]
                    |
Input ---------> [Σ] -----> [Slicer] --> Output
                 ↑            |
                 |            v
           [Feedback Filter] <-
```

**Feedforward** (FF): Linear filter (like MMSE)

**Feedback** (FB): Use previous decisions to cancel post-cursor ISI

---

### DFE Equations

**Feedforward**:

$$
z[n] = \sum_{k=0}^{N_f-1} w_k \cdot r[n-k]
$$

**Feedback**:

$$
y[n] = z[n] - \sum_{k=1}^{N_b} b_k \cdot \hat{s}[n-k]
$$

**Decision**: $\hat{s}[n] = \text{slicer}(y[n])$ (nearest constellation point)

---

### DFE Advantages

1. **No noise enhancement**: Feedback uses clean decisions (no noise amplification from channel inversion)
2. **Better than linear**: Handles severe ISI
3. **Practical**: Moderate complexity

---

### DFE Disadvantages

1. **Error propagation**: Wrong decision → Future decisions corrupted
2. **Training needed**: Requires channel estimate
3. **Latency**: Sequential decisions (can't parallelize easily)

---

### Error Propagation

**Example**: 2-tap feedback, BER = 10⁻³

**Error probability** (1 wrong decision in past 2):

$$
P_{\text{error}} \approx 2 \times 10^{-3} = 2 \times 10^{-3}
$$

**If decision wrong**: Feedback adds wrong ISI → Higher BER

**Mitigation**: Use coding (corrects burst errors from propagation)

---

## Adaptive Equalization

**Problem**: Channel unknown or time-varying (mobile, fading)

**Solution**: **Adaptive algorithms** adjust equalizer taps in real-time

---

### Least Mean Squares (LMS)

**Stochastic gradient descent**:

$$
\mathbf{w}[n+1] = \mathbf{w}[n] + \mu \cdot e^*[n] \cdot \mathbf{r}[n]
$$

Where:
- $e[n] = d[n] - y[n]$ (error)
- $d[n]$ = Desired output (training symbol or decision)
- $\mu$ = Step size (0.01-0.1)

**Advantages**:
- Simple (~$2N$ operations)
- Low memory
- Stable

**Disadvantages**:
- Slow convergence (~1000+ symbols)
- Step size trade-off (fast vs stable)

---

### Recursive Least Squares (RLS)

**Minimize weighted sum** of all past errors:

$$
\min_{\mathbf{w}} \sum_{i=1}^{n} \lambda^{n-i} |d[i] - \mathbf{w}^H \mathbf{r}[i]|^2
$$

**Update** (Kalman gain):

$$
\mathbf{w}[n] = \mathbf{w}[n-1] + \mathbf{k}[n] \cdot e^*[n]
$$

**Advantages**:
- Fast convergence (~$2N$ symbols)
- Better tracking

**Disadvantages**:
- High complexity ($O(N^2)$)
- Numerical instability

---

### LMS vs RLS

| Aspect | LMS | RLS |
|--------|-----|-----|
| **Complexity** | $O(N)$ | $O(N^2)$ |
| **Convergence** | Slow (1000+) | Fast (10-100) |
| **Tracking** | Poor | Excellent |
| **Stability** | Robust | Can diverge |
| **Use case** | Slow channels | Fast fading |

---

## Training vs Blind Equalization

### Training Mode

**Transmit known symbols** (preamble, midamble)

**Receiver**: Compare $y[n]$ to $d[n]$, adjust taps

**Duration**: 50-500 symbols (depends on channel)

**Example**: WiFi long preamble (64 OFDM symbols for channel estimation)

---

### Decision-Directed Mode

**After training**, use **decisions** as reference:

$$
d[n] = \hat{s}[n] \quad (\text{slicer output})
$$

**Works if**: BER low enough (~10⁻² after training)

**Tracks slowly varying channel**

---

### Blind Equalization

**No training sequence** (constant modulus, higher-order statistics)

**Constant Modulus Algorithm (CMA)**:

$$
e[n] = |y[n]|^2 - R_2
$$

Where $R_2 = E[|s|^4] / E[|s|^2]$ (modulus)

**For QPSK**: $R_2 = 1$ (all symbols same magnitude)

**Update**: Same as LMS with $e[n]$ above

**Advantage**: No preamble overhead

**Disadvantage**: Slower convergence, phase ambiguity

---

## Fractionally-Spaced Equalizer (FSE)

**Problem**: Symbol-rate sampling misses information (timing-dependent)

**Solution**: Sample at **T/2** (twice symbol rate) or faster

**Structure**: $2N$ taps at T/2 spacing

**Advantages**:
1. **Timing-independent**: Works at any sampling phase
2. **Better performance**: Exploits oversampled signal
3. **Joint timing + equalization**

**Complexity**: 2× taps, but worth it

---

## Frequency-Domain Equalization

**For OFDM**: Equalize each subcarrier independently

**Per-subcarrier**:

$$
\hat{S}_k = \frac{R_k}{H_k}
$$

**Where**:
- $R_k$ = Received symbol on subcarrier $k$
- $H_k$ = Channel frequency response at subcarrier $k$
- $\hat{S}_k$ = Equalized symbol

**Equivalent to**: ZF equalizer per tone

**MMSE variant**:

$$
\hat{S}_k = \frac{H_k^*}{|H_k|^2 + \sigma^2} R_k
$$

---

### OFDM Advantage

**Flat fading per subcarrier**:
- Wideband channel → Frequency-selective
- Each subcarrier → Narrow (< $B_c$) → Flat

**Simple equalization**: Single complex multiply per subcarrier

**Example**: WiFi 802.11a
- 64 subcarriers (52 used)
- 20 MHz channel (312.5 kHz per subcarrier)
- Delay spread ~200 ns → $B_c \approx 1$ MHz
- Each subcarrier flat → 1-tap equalizer ✓

---

## Channel Estimation

**Equalizer needs** $H[k]$ or $\mathbf{h}$

---

### Pilot-Based Estimation

**Known symbols** (pilots) at indices $\mathcal{P}$:

$$
\hat{H}_k = \frac{R_k}{S_k}, \quad k \in \mathcal{P}
$$

**Interpolation** (for data subcarriers):

$$
\hat{H}_k = \sum_{p \in \mathcal{P}} H_p \cdot \text{sinc}(k - p), \quad k \notin \mathcal{P}
$$

**Or**: Wiener interpolation (MMSE), spline

**Example**: LTE
- 4 pilots per 12 subcarriers (every 3rd subcarrier)
- Linear interpolation (frequency)
- Averaging (time, multiple OFDM symbols)

---

### Least-Squares (LS) Estimation

**Training sequence** $\mathbf{S}$ (length $N$):

$$
\hat{\mathbf{h}} = (\mathbf{S}^H \mathbf{S})^{-1} \mathbf{S}^H \mathbf{r}
$$

**For pilots**: $\hat{H}_k = R_k / S_k$ (same as above)

**Noise**: Not suppressed (LS unbiased but noisy)

---

### MMSE Channel Estimation

**Incorporate statistics**:

$$
\hat{\mathbf{h}} = \mathbf{R}_{hh} \mathbf{S}^H (\mathbf{S} \mathbf{R}_{hh} \mathbf{S}^H + \sigma^2 \mathbf{I})^{-1} \mathbf{r}
$$

**Requires**: Channel correlation $\mathbf{R}_{hh}$ (from delay profile)

**Advantage**: Noise suppression (~3 dB gain over LS)

**Disadvantage**: Complexity, needs statistics

---

## Practical Examples

### 1. WiFi 802.11n (MIMO)

**Channel estimation**: Long preamble (HT-LTF)
- 2 OFDM symbols per spatial stream
- LS estimation
- Linear interpolation (frequency)

**Equalization**: MMSE per subcarrier
- $\hat{\mathbf{S}} = (\mathbf{H}^H \mathbf{H} + \sigma^2 \mathbf{I})^{-1} \mathbf{H}^H \mathbf{R}$
- Per-subcarrier 2×2 or 4×4 matrix inversion

**Tracking**: Pilot tones (4 per 56 subcarriers)

---

### 2. LTE Downlink

**Channel estimation**: Cell-Specific Reference Signals (CRS)
- 4 pilots per 12 subcarriers per OFDM symbol
- MMSE estimation (Wiener filtering)

**Equalization**: MMSE (frequency domain)
- Per-subcarrier, per-antenna

**Interference**: MRC (Maximum Ratio Combining) across antennas

**Result**: Supports 300 km/h (high Doppler)

---

### 3. DVB-T (Terrestrial TV)

**Channel estimation**: Scattered pilots (8%)
- Wiener interpolation (time + frequency)
- Handles long delay spread (SFN networks, 200 μs)

**Equalization**: Per-subcarrier ZF or MMSE

**Guard interval**: 1/4, 1/8, 1/16, 1/32 of symbol (user-selectable)

---

### 4. GSM (Legacy Cellular)

**Training sequence**: 26-bit midamble

**Equalization**: Viterbi (MLSE, Maximum Likelihood Sequence Estimation)
- 5-tap channel → 16 states
- Optimal for short bursts

**ISI**: ~5-15 symbols (urban, hilly)

**Result**: Works up to 10 μs delay spread

---

## Advanced Techniques

### 1. Turbo Equalization

**Iterative**: Equalizer ↔ Decoder exchange soft information

**Structure**:

```
Received --> [SISO      <---> [Deinterleaver] <---> [SISO
              Equalizer]                              Decoder]
                ↓                                       ↓
           (extrinsic LLRs)                        (decoded bits)
```

**Iterations**: 3-5

**Gain**: ~2-3 dB over separate equalization + decoding

**Used in**: Deep space, underwater acoustics

---

### 2. Precoding (Transmit Equalization)

**Pre-invert channel at transmitter** (if channel known via feedback):

**Transmit**: $\mathbf{x} = \mathbf{W} \mathbf{s}$

**Where**: $\mathbf{W} = \mathbf{H}^{-1}$ or MMSE variant

**Advantage**: Simple receiver (no equalization)

**Disadvantage**: Requires CSI at TX (feedback latency)

**Used in**: TDD systems (reciprocity), MU-MIMO downlink

---

### 3. Dirty Paper Coding (DPC)

**Theoretical**: Pre-cancel interference without power penalty

**Practical approximation**: Tomlinson-Harashima Precoding (THP)

**Gain**: Approaches capacity (multi-user downlink)

**Complexity**: High (not widely deployed)

---

## Equalization Complexity

| Method | Complexity (per symbol) | Notes |
|--------|-------------------------|-------|
| **ZF (freq domain)** | $O(\log N)$ | FFT + per-tone multiply |
| **MMSE (freq domain)** | $O(\log N)$ | FFT + per-tone multiply |
| **Linear (time domain)** | $O(N_{\text{taps}})$ | FIR filter |
| **DFE** | $O(N_f + N_b)$ | FF + FB filters |
| **LMS** | $O(N)$ | Simple update |
| **RLS** | $O(N^2)$ | Matrix operations |
| **MLSE (Viterbi)** | $O(M^L)$ | $M$ = constellation, $L$ = ISI length |

---

## Design Guidelines

### 1. Choose Equalizer Type

**Flat fading** (delay spread < 0.1 symbol period):
- No equalizer needed (or 1-tap phase correction)

**Mild ISI** (delay spread 0.1-1 symbol period):
- Linear MMSE (5-15 taps)
- Fractionally-spaced

**Severe ISI** (delay spread > 1 symbol period):
- DFE (15+ feedforward, 5-10 feedback)
- Or OFDM (avoid time-domain equalization)

**Very severe ISI** (delay spread > 5 symbols):
- OFDM with guard interval
- Or MLSE (if short burst)

---

### 2. Select Adaptation Algorithm

**Slow channel** (< 1 Hz Doppler):
- LMS ($\mu = 0.01$)
- Low complexity

**Moderate channel** (1-100 Hz):
- LMS ($\mu = 0.05$) or RLS
- Update every symbol

**Fast channel** (100+ Hz):
- RLS or decision-directed
- Pilot-aided tracking

---

### 3. Training Overhead

**Packet systems** (WiFi, 5G):
- Training per packet (10-20% overhead)
- Decision-directed within packet

**Continuous** (TV, broadcast):
- Sparse pilots (1-5% overhead)
- Continuous tracking

**Burst** (GSM, satellite TDMA):
- Midamble (10-15% overhead)
- Per-burst estimation

---

## Equalization vs Coding

**Equalization**: Removes ISI (deterministic distortion)

**Coding**: Corrects random errors (noise)

**Combined**: Achieves near-capacity
- Coding gain: 5-10 dB
- Equalization: Enables coding to work (removes ISI)

**Without equalization**: Coding fails (BER floor from ISI)

---

## Related Topics

- **[[Multipath Propagation & Fading (Rayleigh, Rician)]]**: Cause of ISI
- **[[Channel Models (Rayleigh & Rician)]]**: Simulation models
- **[[OFDM & Multicarrier Modulation]]**: Frequency-domain approach
- **[[Synchronization (Carrier, Timing, Frame)]]**: Complements equalization
- **[[MIMO & Spatial Multiplexing]]**: Multi-antenna equalization

---

**Key takeaway**: **Channel equalization removes ISI from multipath.** ZF inverts channel perfectly but amplifies noise (poor low SNR). MMSE trades ISI vs noise ($W = H^*/(|H|^2 + \sigma^2)$), optimal at moderate SNR. DFE uses past decisions (feedback) to avoid noise enhancement—better than linear but error propagation risk. Adaptive: LMS simple ($O(N)$, slow), RLS fast ($O(N^2)$, complex). Fractionally-spaced (T/2 sampling) is timing-independent. OFDM: Per-subcarrier 1-tap equalizer (frequency-domain, flat fading per tone). Channel estimation: Pilots (LS noisy, MMSE better). WiFi: Long preamble + pilots. LTE: CRS pilots, Wiener filtering. Turbo equalization: Iterative with decoder (+2-3 dB). Delay spread > 0.1T needs equalization. Severe ISI (>1T) → Use OFDM or DFE. Coding + equalization = near-capacity.

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
