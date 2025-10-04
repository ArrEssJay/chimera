# OFDM & Multicarrier Modulation

**Orthogonal Frequency-Division Multiplexing (OFDM)** is a multicarrier modulation technique that divides a wideband channel into many narrow, orthogonal subcarriers. It has become the foundation of modern wireless standards including WiFi (802.11a/g/n/ac/ax), LTE, 5G NR, and DVB-T.

---

## 🎯 The Core Concept

**Single-carrier problem**: High-speed data → short symbol duration → susceptible to multipath fading and intersymbol interference (ISI).

**OFDM solution**: Divide spectrum into N narrow subcarriers → each carries low-rate data → longer symbol duration → robust against multipath.

```
Single Carrier (100 Mbps):
|████████████████████████████████| ← Wide, fast, ISI-prone
     ↓ OFDM Transformation ↓
Multi-carrier (100 Mbps total):
|█|█|█|█|█|█|█|█|█|█|█|█|█|█|█|█| ← N narrow, slow, ISI-resistant
 1 2 3 4 ... N subcarriers
```

---

## 📐 Mathematical Foundation

### Orthogonality Condition

Subcarriers are **orthogonal** when their frequencies are spaced by 1/T:

```
fₖ = f₀ + k·Δf

where:
- f₀ = center frequency
- k = subcarrier index (0, 1, 2, ..., N-1)
- Δf = subcarrier spacing = 1/T_symbol
- T_symbol = OFDM symbol duration
```

**Orthogonality integral**:
```
∫₀ᵀ exp(j2π·fₖ·t) · exp(-j2π·fₘ·t) dt = { T  if k = m
                                         { 0  if k ≠ m
```

This ensures subcarriers don't interfere despite **spectral overlap**.

---

### IFFT/FFT Implementation

**Key insight**: OFDM modulation/demodulation is mathematically equivalent to Inverse Fast Fourier Transform (IFFT) and FFT.

**Transmitter (IFFT)**:
```
x[n] = (1/√N) · Σₖ₌₀^(N-1) Xₖ · exp(j2πkn/N)

where:
- Xₖ = complex data symbol on subcarrier k (from QAM/PSK constellation)
- x[n] = time-domain OFDM sample
- N = number of subcarriers (typically 64, 128, 256, 512, 1024, 2048)
```

**Receiver (FFT)**:
```
Yₖ = (1/√N) · Σₙ₌₀^(N-1) y[n] · exp(-j2πkn/N)

where:
- y[n] = received time-domain samples
- Yₖ = recovered symbol on subcarrier k
```

**Computational advantage**: FFT reduces complexity from O(N²) to O(N log N).

---

## 🔧 OFDM System Architecture

### Transmitter Block Diagram

```
Data bits
   ↓
Serial-to-Parallel Converter (splits into N streams)
   ↓
QAM/PSK Mapper (maps each stream to constellation point)
   ↓
Pilot Insertion & Subcarrier Mapping
   ↓
IFFT (N-point)
   ↓
Add Cyclic Prefix (CP)
   ↓
Parallel-to-Serial Converter
   ↓
D/A Converter & RF Upconversion
   ↓
Antenna
```

### Receiver Block Diagram

```
Antenna
   ↓
RF Downconversion & A/D Converter
   ↓
Serial-to-Parallel Converter
   ↓
Remove Cyclic Prefix
   ↓
FFT (N-point)
   ↓
Channel Estimation & Equalization (per-subcarrier)
   ↓
QAM/PSK Demapper
   ↓
Parallel-to-Serial Converter
   ↓
Data bits
```

---

## 🛡️ Cyclic Prefix (CP)

The **cyclic prefix** is OFDM's defense against multipath-induced ISI.

### What Is It?

Copy the **last L samples** of the OFDM symbol and prepend them:

```
Original OFDM symbol (N samples):
[s₀ s₁ s₂ ... s_(N-2) s_(N-1)]

With CP (N+L samples):
[s_(N-L) ... s_(N-1) | s₀ s₁ s₂ ... s_(N-2) s_(N-1)]
 └─── CP (L) ────┘     └──── Original Symbol (N) ────┘
```

### Why Does It Work?

**Problem**: Multipath creates delayed copies of the signal → samples from adjacent symbols overlap (ISI).

**Solution**: CP acts as a **guard interval**:
- If delay spread < CP duration, ISI from previous symbol falls entirely within the CP
- Receiver discards CP → only clean samples remain
- CP makes **linear convolution appear as circular convolution** → simple per-subcarrier equalization

### CP Overhead

```
Overhead = L / (N + L)

Example (WiFi 802.11a):
- N = 64 subcarriers
- L = 16 samples (CP)
- Overhead = 16/80 = 20% (loss in spectral efficiency)

Tradeoff:
- Longer CP → more robust to delay spread
- Longer CP → higher overhead (lower data rate)
```

---

## 📊 OFDM Parameters

### Key Design Choices

| Parameter | Symbol | Typical Values | Impact |
|-----------|--------|----------------|--------|
| FFT Size | N | 64-2048 | Granularity, latency |
| Subcarrier Spacing | Δf | 15 kHz (LTE), 312.5 kHz (WiFi) | Doppler tolerance |
| Symbol Duration | T_symbol | 1/Δf | ISI resistance |
| CP Length | L | N/4, N/8, N/16 | Delay spread tolerance |
| Bandwidth | BW | N·Δf | Throughput |

### Example: LTE

```
Configuration:
- FFT Size: 1024 (20 MHz BW) or 512 (10 MHz)
- Subcarrier Spacing: 15 kHz
- Symbol Duration: 66.67 μs
- CP (normal): 4.69 μs (first symbol), 5.21 μs (others)
- 12 subcarriers per Resource Block (180 kHz)
- 7 OFDM symbols per slot (0.5 ms)
```

---

## 🎨 Pilot Subcarriers & Channel Estimation

Not all subcarriers carry data—some are **pilots** for channel estimation.

### Pilot Types

**1. Scattered Pilots** (time + frequency diversity):
```
Subcarrier
    ↑
    | D D D P D D D P D D D P    ← Symbol 4
    | D D P D D D P D D D P D    ← Symbol 3
    | D P D D D P D D D P D D    ← Symbol 2
    | P D D D P D D D P D D D    ← Symbol 1
    └──────────────────────────> Time
     (P = Pilot, D = Data)
```

**2. Continual Pilots** (phase/frequency tracking):
```
Fixed subcarriers (e.g., k = -21, -7, 7, 21 in WiFi) always carry pilots.
```

**3. Preamble/Training Symbols**:
```
First OFDM symbol(s) in a frame are all pilots for initial synchronization.
```

### Channel Estimation

**Per-subcarrier channel model**:
```
Yₖ = Hₖ · Xₖ + Nₖ

where:
- Hₖ = complex channel gain on subcarrier k
- Xₖ = transmitted symbol
- Yₖ = received symbol
- Nₖ = noise
```

**Estimation process**:
1. Transmitter sends known pilots Pₖ
2. Receiver measures Yₖ = Hₖ·Pₖ + Nₖ
3. Estimate: Ĥₖ = Yₖ / Pₖ
4. Interpolate Ĥₖ across data subcarriers (2D interpolation)
5. Equalize data: X̂ₖ = Yₖ / Ĥₖ

---

## 🌊 Multipath & Frequency-Selective Fading

### Why OFDM Excels in Multipath

**Single-carrier**: Entire bandwidth experiences **frequency-selective fading** → deep nulls can wipe out signal.

**OFDM**: Channel appears **flat** within each narrow subcarrier → only some subcarriers fade deeply, others remain strong.

```
Frequency Response (Multipath Channel):
Magnitude
    ↑
    |  ___       ___
    | |   |     |   |___      ← Single-carrier spans entire BW
    | |   |_____|       |        (suffers deep null)
    |_|___|_____________|___→ Frequency
       ↑   ↑   ↑   ↑   ↑
       |   |   |   |   |
     OFDM subcarriers (most unaffected, a few degraded)
```

**Per-subcarrier equalization**:
```
X̂ₖ = Yₖ / Hₖ  (simple division per subcarrier)
```

Much simpler than **time-domain equalization** (which requires complex filters).

---

## ⚡ Peak-to-Average Power Ratio (PAPR)

### The OFDM Challenge

**Problem**: When N subcarriers add constructively, instantaneous power spikes far above average.

```
PAPR = Peak Power / Average Power

Theoretical worst case: PAPR = N (e.g., 20 dB for N=100)
Typical OFDM: PAPR ≈ 10-13 dB (3-5 dB worse than single-carrier)
```

### Why It Matters

- **Power Amplifier (PA) must operate in linear region** → inefficient (backed-off from saturation)
- High PAPR → PA must handle peaks → more expensive, power-hungry RF hardware
- Non-linear PA → intermodulation distortion, spectral regrowth

### PAPR Reduction Techniques

**1. Clipping & Filtering**:
```
Clip peaks at threshold → filter out-of-band distortion → slight BER degradation
```

**2. Tone Reservation**:
```
Reserve some subcarriers to generate "anti-peaks" that cancel large peaks.
```

**3. Selective Mapping (SLM)**:
```
Generate multiple OFDM symbols with different phase rotations → choose one with lowest PAPR.
```

**4. Partial Transmit Sequence (PTS)**:
```
Divide subcarriers into blocks → optimize phase per block to minimize PAPR.
```

**Tradeoff**: PAPR reduction adds complexity, may reduce spectral efficiency or increase BER.

---

## 🔄 Synchronization Challenges

OFDM is **sensitive** to timing and frequency offsets.

### Timing Offset

**Consequence**: If FFT window is misaligned:
- Within CP: No ISI, but phase rotation per subcarrier
- Beyond CP: ISI from adjacent symbols

**Solution**: Preamble correlation, CP-based timing metrics.

### Carrier Frequency Offset (CFO)

**Consequence**: Subcarriers lose orthogonality → Inter-Carrier Interference (ICI).

```
CFO = Δf / Δf_subcarrier

Example:
- 1 kHz offset on 15 kHz subcarrier spacing → CFO = 0.067
- Causes ~0.2 dB SNR loss
```

**Solution**:
1. **Coarse CFO estimation**: Preamble autocorrelation (range: ±Δf_subcarrier/2)
2. **Fine CFO tracking**: Continual pilots

### Sampling Clock Offset (SCO)

**Consequence**: Slow drift in FFT window position → phase rotation accumulates over time.

**Solution**: Track phase of continual pilots → adjust sampling clock or compensate digitally.

---

## 📡 OFDM in Real-World Standards

### WiFi 802.11a/g/n/ac/ax

**802.11a/g (54 Mbps)**:
```
- FFT Size: 64
- Used Subcarriers: 52 (48 data + 4 pilots)
- Subcarrier Spacing: 312.5 kHz
- Bandwidth: 20 MHz
- Modulation: BPSK, QPSK, 16-QAM, 64-QAM
```

**802.11n (600 Mbps)**:
```
- Up to 4×4 MIMO-OFDM
- 40 MHz channels (108 data subcarriers)
- Short Guard Interval: 400 ns (vs. 800 ns)
```

**802.11ax (WiFi 6, 9.6 Gbps)**:
```
- OFDMA (multi-user OFDM): allocate subcarriers to different users
- 1024-QAM, 160 MHz channels
- MU-MIMO (8×8)
```

---

### LTE & 5G NR

**LTE Downlink**:
```
- SC-FDMA uplink (low PAPR variant)
- 15 kHz subcarrier spacing
- 1.4, 3, 5, 10, 15, 20 MHz bandwidths
- CP-OFDM with MIMO (up to 8×8)
```

**5G NR**:
```
- Scalable numerology: Δf = 15, 30, 60, 120, 240 kHz
  (higher spacing for mmWave → shorter symbols → Doppler tolerance)
- Massive MIMO (64+ antennas)
- Flexible frame structure (dynamic TDD)
```

---

### DVB-T/T2 (Digital Video Broadcasting - Terrestrial)

**DVB-T**:
```
- FFT: 2048 or 8192
- Guard intervals: 1/4, 1/8, 1/16, 1/32
- Optimized for high-mobility (trains, cars)
- COFDM (Coded OFDM with interleaving)
```

**DVB-T2** (next-gen):
```
- Up to 256-QAM
- LDPC + BCH FEC
- Rotated constellations (diversity against deep fades)
```

---

## 🧮 Spectral Efficiency Analysis

### Calculation

```
Spectral Efficiency (SE) = R / BW  bits/s/Hz

where:
R = N_data · log₂(M) · (1 - CP_overhead) / T_symbol

Example (LTE 20 MHz):
- N_data = 1200 subcarriers (100 RBs × 12)
- M = 64 (64-QAM → 6 bits/symbol)
- CP overhead = 7%
- T_symbol = 66.67 μs

SE = 1200 · 6 · 0.93 / (66.67×10⁻⁶ · 20×10⁶)
   = 6696 / 1.33 = 5.0 bits/s/Hz

(Theoretical peak with MIMO: 30 bits/s/Hz for 4×4 spatial streams)
```

---

## ⚔️ OFDM vs. Single-Carrier

| Aspect | OFDM | Single-Carrier |
|--------|------|----------------|
| **ISI Robustness** | Excellent (CP + long symbols) | Requires complex equalizer |
| **Frequency-Selective Fading** | Simple per-subcarrier EQ | Time-domain EQ (adaptive filter) |
| **PAPR** | High (~10-13 dB) | Low (~3-5 dB) |
| **Spectral Efficiency** | Moderate (CP overhead) | Higher (no CP) |
| **Implementation** | FFT/IFFT (efficient) | FIR filters (complex) |
| **Doppler Sensitivity** | Moderate (ICI from CFO) | Lower |
| **Best For** | Wideband, fixed/low-mobility | Narrowband, high-mobility |

---

## 🚀 Advanced OFDM Variants

### OFDMA (Orthogonal Frequency-Division Multiple Access)

**Concept**: Assign different subcarriers to different users.

```
User 1: Subcarriers 0-15
User 2: Subcarriers 16-31
User 3: Subcarriers 32-47
...

Advantages:
- Multi-user diversity
- Flexible resource allocation
- Uplink/downlink efficiency
```

**Used in**: LTE, 5G NR, WiFi 6 (802.11ax).

---

### SC-FDMA (Single-Carrier FDMA)

**Motivation**: Lower PAPR for mobile devices (saves battery).

**Method**: DFT-spread OFDM:
```
Data → DFT → Subcarrier Mapping → IFFT → CP
```

**Effect**: Maintains OFDM benefits but with **3-5 dB lower PAPR**.

**Used in**: LTE uplink, 5G NR uplink option.

---

### Filter-Bank Multicarrier (FBMC)

**Improvement**: Replace rectangular pulse (sinc spectrum) with well-designed filters → reduced out-of-band emissions.

**Advantage**: No CP needed → higher spectral efficiency.

**Disadvantage**: More complex, incompatible with MIMO (without workarounds).

**Status**: Considered for 5G but not adopted (OFDM with windowing chosen instead).

---

## 🐍 Python Implementation Example

### Basic OFDM Transmitter

```python
import numpy as np

def ofdm_modulate(data_symbols, N=64, L_cp=16):
    """
    OFDM modulation via IFFT.
    
    Args:
        data_symbols: Array of QAM/PSK symbols (length N)
        N: FFT size
        L_cp: Cyclic prefix length
    
    Returns:
        OFDM time-domain signal (length N + L_cp)
    """
    # IFFT (convert frequency domain to time domain)
    time_domain = np.fft.ifft(data_symbols, N)
    
    # Add cyclic prefix
    cp = time_domain[-L_cp:]
    ofdm_symbol = np.concatenate([cp, time_domain])
    
    return ofdm_symbol

# Example usage
N = 64
L_cp = 16

# Generate random QPSK symbols
data_symbols = (2 * np.random.randint(0, 2, N) - 1) + \
               1j * (2 * np.random.randint(0, 2, N) - 1)
data_symbols /= np.sqrt(2)  # Normalize

# Modulate
tx_signal = ofdm_modulate(data_symbols, N, L_cp)

print(f"Input symbols: {len(data_symbols)}")
print(f"OFDM signal: {len(tx_signal)} samples (N={N} + CP={L_cp})")
print(f"PAPR: {10 * np.log10(np.max(np.abs(tx_signal)**2) / np.mean(np.abs(tx_signal)**2)):.2f} dB")
```

### Basic OFDM Receiver

```python
def ofdm_demodulate(rx_signal, N=64, L_cp=16):
    """
    OFDM demodulation via FFT.
    
    Args:
        rx_signal: Received time-domain signal
        N: FFT size
        L_cp: Cyclic prefix length
    
    Returns:
        Recovered frequency-domain symbols
    """
    # Remove cyclic prefix
    rx_no_cp = rx_signal[L_cp:]
    
    # FFT (convert time domain to frequency domain)
    recovered_symbols = np.fft.fft(rx_no_cp, N)
    
    return recovered_symbols

# Demodulate
rx_symbols = ofdm_demodulate(tx_signal, N, L_cp)

# Compare (should be identical in ideal channel)
error = np.max(np.abs(data_symbols - rx_symbols))
print(f"Reconstruction error: {error:.2e}")
```

---

## 🔬 Performance Analysis

### BER in AWGN Channel

For OFDM with M-QAM modulation on each subcarrier:

```
BER ≈ (4/log₂(M)) · (1 - 1/√M) · Q(√(3·log₂(M)·SNR / (M-1)))

where Q(x) = Gaussian Q-function

Example (16-QAM OFDM at SNR = 20 dB):
BER ≈ 10⁻⁴ (without coding)
BER ≈ 10⁻⁶ (with rate-1/2 LDPC)
```

### Frequency-Selective Channel

```python
# Generate multipath channel
def multipath_channel(ofdm_signal, delays, gains):
    """
    Apply multipath fading.
    
    Args:
        delays: Array of tap delays (in samples)
        gains: Array of tap gains (complex)
    """
    output = np.zeros(len(ofdm_signal) + max(delays), dtype=complex)
    
    for delay, gain in zip(delays, gains):
        output[delay:delay+len(ofdm_signal)] += gain * ofdm_signal
    
    return output[:len(ofdm_signal)]

# Example: 2-tap channel
delays = [0, 8]  # Direct path + 8-sample delayed path
gains = [1.0, 0.5*np.exp(1j*np.pi/4)]  # 6 dB echo with phase

rx_signal = multipath_channel(tx_signal, delays, gains)
rx_signal += 0.01 * (np.random.randn(len(rx_signal)) + 
                     1j * np.random.randn(len(rx_signal)))  # Add noise

# Demodulate
rx_symbols = ofdm_demodulate(rx_signal, N, L_cp)

# Per-subcarrier channel estimation (if pilots known)
H_estimated = rx_symbols / data_symbols  # Assumes data_symbols are pilots
```

---

## 🎯 When to Use OFDM

### OFDM is Ideal For:

✅ **Wideband channels** (> 1 MHz) with frequency-selective fading  
✅ **Multipath-rich environments** (urban, indoor)  
✅ **Fixed or low-mobility users** (< 120 km/h)  
✅ **Multiple users** needing flexible resource allocation (OFDMA)  
✅ **High spectral efficiency** requirements  

### Avoid OFDM For:

❌ **Power-constrained devices** (high PAPR → inefficient PA)  
❌ **High-mobility** (Doppler → severe ICI)  
❌ **Narrowband channels** (CP overhead too high)  
❌ **Non-linear channels** (PAPR sensitive to distortion)  

---

## 📚 Further Reading

### Textbooks
- **Prasad**, *OFDM for Wireless Communications Systems* - Comprehensive treatment
- **Cho et al.**, *MIMO-OFDM Wireless Communications with MATLAB* - Practical implementation
- **Goldsmith**, *Wireless Communications* (Chapter 13) - Theoretical foundation

### Standards Documents
- **IEEE 802.11-2020**: WiFi OFDM/OFDMA specifications
- **3GPP TS 36.211**: LTE Physical Layer (OFDM parameters)
- **3GPP TS 38.211**: 5G NR Physical Layer (scalable OFDM)

### Related Topics
- [[MIMO & Spatial Multiplexing]] - Combining OFDM with multiple antennas
- [[Channel Equalization]] - Frequency-domain equalization in OFDM
- [[Adaptive Modulation & Coding (AMC)]] - Per-subcarrier link adaptation
- [[Synchronization (Carrier, Timing, Frame)]] - OFDM sync techniques
- [[Real-World System Examples]] - LTE, 5G, WiFi implementations

---

**Summary**: OFDM transforms wideband frequency-selective channels into many narrowband flat-fading channels, enabling simple equalization and high spectral efficiency. The FFT/IFFT makes it computationally efficient, while the cyclic prefix provides ISI immunity. Despite high PAPR and synchronization sensitivity, OFDM dominates modern wireless due to its robustness in multipath environments and natural fit for MIMO and multi-user scenarios.
