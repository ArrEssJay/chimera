# Channel Models: Rayleigh & Rician Implementation

[[Home]] | **Link Budget & System Performance** | [[Multipath Propagation & Fading (Rayleigh, Rician)]] | [[Signal-to-Noise Ratio (SNR)]]

---

## 🎭 For Non-Technical Readers

**Channel models are like flight simulators for radio engineers—they let you test communication systems in virtual cities, tunnels, and open fields before building real hardware!**

**The problem**:
- Can't test every scenario: urban, suburban, highway, indoor, etc.
- Real-world testing is expensive (need hardware, locations, permits)
- Need to test in bad conditions (rain, crowds, interference)
- Can't test satellites or Mars missions easily!

**The solution - Mathematical simulation**:
- Create computer models of radio environments
- Run communication system in simulation
- See how well it performs
- Fix problems BEFORE building hardware!

**The two main models**:

**1. Rayleigh Fading** (no line-of-sight):
- **Environment**: Dense urban (downtown), indoors, tunnels
- **Characteristic**: Signal bounces everywhere, no direct path
- **Result**: Wild signal fluctuations (30+ dB swings!)
- **Example**: Walking through city, WiFi in building with walls

**2. Rician Fading** (strong line-of-sight):
- **Environment**: Suburban, rural, highways, open areas
- **Characteristic**: One strong direct path + weaker echoes
- **Result**: More stable signal, less severe fading
- **Example**: Highway cell tower, rural WiFi

**How engineers use models**:

**Step 1**: Pick scenario
- "Designing WiFi for dense apartment building" → use Rayleigh
- "Designing highway cell system" → use Rician

**Step 2**: Run simulation
- Send 1 million test bits through model
- Model adds realistic fading, multipath, noise

**Step 3**: Measure performance
- How many errors? (Bit Error Rate)
- How fast can it go? (Data rate)
- Does it meet requirements?

**Step 4**: Iterate
- Try different modulations (QPSK vs 64-QAM)
- Add error correction (FEC)
- Optimize until it works!

**Real-world impact**:
- **5G standard**: Tested in standardized channel models before deployment
- **Your WiFi**: Manufacturers test with Rayleigh/Rician models
- **Satellite systems**: Simulated before launching $500M satellite!
- **Military radios**: Tested in tactical channel models

**Why simulation beats real testing**:
- **Reproducible**: Same conditions every test
- **Extreme scenarios**: Test 99.9th percentile bad cases
- **Fast**: Test 10,000 scenarios in hours
- **Cheap**: No hardware, no permits, no travel
- **Safe**: Can test failure modes without consequences

**Standards bodies define models**:
- **3GPP**: Defines channel models for 4G/5G (TDL-A, TDL-B, TDL-C)
- **ITU**: Defines models for satellite, fixed wireless
- **WiFi**: IEEE 802.11 working groups define indoor/outdoor models

**Fun fact**: When engineers designed the LTE standard (4G), they ran over 1 million simulations using standardized channel models. This is why 4G "just worked" globally from day one—they'd already tested every conceivable environment virtually!

---

## Overview

**Channel models** simulate propagation effects for communication system design and testing.

**Purpose**:
- **System simulation**: Test modulation/coding without real-world deployment
- **Performance prediction**: Estimate BER vs SNR for different environments
- **Algorithm development**: Design equalizers, synchronizers without hardware
- **Standards compliance**: 3GPP, ITU specify reference channel models

**Key models**:
1. **AWGN**: Ideal (additive white Gaussian noise only)
2. **Rayleigh fading**: NLOS multipath (no dominant path)
3. **Rician fading**: LOS + multipath (K-factor parameterizes LOS strength)
4. **Frequency-selective**: Wideband channels with delay spread (ISI)

---

## AWGN Channel

**Simplest model**: Received signal = transmitted signal + Gaussian noise

$$
r(t) = s(t) + n(t)
$$

Where:
- $s(t)$ = Transmitted signal
- $n(t)$ = White Gaussian noise, variance $\sigma^2 = N_0 B$

### Implementation (MATLAB/Python)

```python
import numpy as np

def awgn_channel(signal, snr_db):
    """
    Add AWGN to signal for target SNR
    
    Args:
        signal: Complex baseband signal (numpy array)
        snr_db: Target SNR in dB
        
    Returns:
        Noisy signal
    """
    # Signal power
    signal_power = np.mean(np.abs(signal)**2)
    
    # Noise power for target SNR
    snr_linear = 10**(snr_db/10)
    noise_power = signal_power / snr_linear
    
    # Generate complex Gaussian noise
    noise = np.sqrt(noise_power/2) * (np.random.randn(len(signal)) + 
                                       1j*np.random.randn(len(signal)))
    
    return signal + noise
```

**Usage**:
```python
tx_signal = np.array([1+0j, -1+0j, 1+1j, -1-1j])  # QPSK symbols
rx_signal = awgn_channel(tx_signal, snr_db=10)
```

---

## Flat Fading Channel

**Narrowband model**: Single complex gain + AWGN

$$
r(t) = h(t) \cdot s(t) + n(t)
$$

Where:
- $h(t)$ = Complex channel gain (time-varying)
- $|h(t)|$ = Amplitude (Rayleigh or Rician distributed)
- $\angle h(t)$ = Phase (uniformly distributed)

**Flat fading applies when**: Signal bandwidth $\ll$ coherence bandwidth

---

## Rayleigh Fading Channel

**Model**: No LOS, many scattered paths with equal power

### Statistical Properties

**Envelope** $r = |h(t)|$ follows **Rayleigh distribution**:

$$
p(r) = \frac{r}{\sigma^2} \exp\left(-\frac{r^2}{2\sigma^2}\right), \quad r \geq 0
$$

**Mean**: $\bar{r} = \sigma\sqrt{\pi/2}$

**Variance**: $\sigma_r^2 = \sigma^2(2 - \pi/2)$

**Normalized** (average power = 1): $\sigma^2 = 1/2$

---

### Clarke's Model (Isotropic Scattering)

**Assumption**: Infinite scatterers uniformly distributed in azimuth

**Doppler spectrum** (U-shaped):

$$
S(f) = \frac{1}{\pi f_d \sqrt{1 - (f/f_d)^2}}, \quad |f| < f_d
$$

Where $f_d = v/\lambda$ = Maximum Doppler frequency

**Autocorrelation**:

$$
R(\tau) = J_0(2\pi f_d \tau)
$$

$J_0$ = Bessel function of first kind, order 0

---

### Jakes' Model (Sum of Sinusoids)

**Efficient implementation** using sum of sinusoids:

**In-phase component**:

$$
h_I(t) = \frac{1}{\sqrt{M}} \sum_{m=1}^{M} \cos(2\pi f_d t \cos\theta_m + \phi_m)
$$

**Quadrature component**:

$$
h_Q(t) = \frac{1}{\sqrt{M}} \sum_{m=1}^{M} \sin(2\pi f_d t \cos\theta_m + \phi_m)
$$

Where:
- $M$ = Number of scatterers (typically 8-20)
- $\theta_m = \frac{2\pi m}{M}$ (equally spaced angles)
- $\phi_m$ = Random phase, uniform [0, 2π]

**Complex channel gain**:

$$
h(t) = h_I(t) + j h_Q(t)
$$

---

### Implementation (Jakes' Model)

```python
def rayleigh_channel_jakes(N_samples, fd, fs, M=8):
    """
    Generate Rayleigh fading channel using Jakes' model
    
    Args:
        N_samples: Number of time samples
        fd: Maximum Doppler frequency (Hz)
        fs: Sampling frequency (Hz)
        M: Number of scatterers (default 8)
        
    Returns:
        Complex channel gains h(t)
    """
    t = np.arange(N_samples) / fs
    h_I = np.zeros(N_samples)
    h_Q = np.zeros(N_samples)
    
    for m in range(1, M+1):
        theta_m = 2*np.pi*m / M
        phi_m = np.random.uniform(0, 2*np.pi)
        
        h_I += np.cos(2*np.pi*fd*t*np.cos(theta_m) + phi_m)
        h_Q += np.sin(2*np.pi*fd*t*np.cos(theta_m) + phi_m)
    
    h_I /= np.sqrt(M)
    h_Q /= np.sqrt(M)
    
    h = (h_I + 1j*h_Q) / np.sqrt(2)  # Normalize to unit power
    
    return h
```

**Usage**:
```python
# Mobile @ 100 km/h (27.8 m/s), 2.4 GHz (λ = 0.125 m)
fd = 27.8 / 0.125  # 222 Hz
fs = 10000  # 10 kHz sampling
N = 10000  # 1 second

h = rayleigh_channel_jakes(N, fd, fs)

# Apply to signal
tx_signal = np.ones(N)  # Constant amplitude
rx_signal = h * tx_signal + awgn_channel(h * tx_signal, snr_db=10)
```

---

### Verification

**Check statistics**:
```python
import matplotlib.pyplot as plt

# Generate long realization
h = rayleigh_channel_jakes(100000, fd=100, fs=10000)
envelope = np.abs(h)

# Plot histogram vs theoretical Rayleigh PDF
plt.hist(envelope, bins=50, density=True, alpha=0.7, label='Simulated')

r = np.linspace(0, 3, 100)
sigma = 1/np.sqrt(2)  # Normalized
pdf_rayleigh = (r/sigma**2) * np.exp(-r**2/(2*sigma**2))
plt.plot(r, pdf_rayleigh, 'r-', linewidth=2, label='Theoretical')

plt.xlabel('Envelope |h|')
plt.ylabel('PDF')
plt.legend()
plt.title('Rayleigh Fading Envelope Distribution')
plt.show()

# Check average power
print(f"Average power: {np.mean(np.abs(h)**2):.3f} (should be ~1.0)")
```

---

## Rician Fading Channel

**Model**: Dominant LOS + scattered components

### Statistical Properties

**Envelope** follows **Rician distribution**:

$$
p(r) = \frac{r}{\sigma^2} \exp\left(-\frac{r^2 + A^2}{2\sigma^2}\right) I_0\left(\frac{Ar}{\sigma^2}\right)
$$

Where:
- $A$ = Amplitude of LOS component
- $I_0$ = Modified Bessel function of first kind, order 0

**K-factor** (ratio of LOS to scattered power):

$$
K = \frac{A^2}{2\sigma^2}
$$

**In dB**: $K_{\text{dB}} = 10\log_{10}(K)$

**Special cases**:
- $K = 0$ (K = -∞ dB): Pure Rayleigh (no LOS)
- $K \to \infty$: Pure LOS (AWGN-like)

---

### Implementation (LOS + Rayleigh)

```python
def rician_channel(N_samples, K_dB, fd, fs, M=8):
    """
    Generate Rician fading channel
    
    Args:
        N_samples: Number of time samples
        K_dB: Rician K-factor in dB
        fd: Maximum Doppler frequency (Hz)
        fs: Sampling frequency (Hz)
        M: Number of scatterers
        
    Returns:
        Complex channel gains h(t)
    """
    K = 10**(K_dB/10)  # Convert to linear
    
    # LOS component (constant, unit phase)
    h_los = np.sqrt(K / (K+1)) * np.ones(N_samples)
    
    # Scattered component (Rayleigh fading)
    h_scatter = rayleigh_channel_jakes(N_samples, fd, fs, M)
    h_scatter *= np.sqrt(1 / (K+1))  # Scale for Rician
    
    return h_los + h_scatter
```

**Usage**:
```python
# Suburban environment, K = 6 dB
h_rician = rician_channel(10000, K_dB=6, fd=100, fs=10000)

# Verify K-factor
los_power = np.mean(np.abs(np.sqrt(6/(6+1)) * np.ones(10000))**2)
scatter_power = np.mean(np.abs(h_rician - np.sqrt(6/(6+1)))**2)
K_estimated = 10*np.log10(los_power / scatter_power)
print(f"Estimated K-factor: {K_estimated:.1f} dB (target: 6.0 dB)")
```

---

### Verification

```python
# Generate Rician channel
h = rician_channel(100000, K_dB=6, fd=100, fs=10000)
envelope = np.abs(h)

# Plot histogram
plt.hist(envelope, bins=50, density=True, alpha=0.7, label='Simulated')

# Theoretical Rician PDF
from scipy.special import i0  # Modified Bessel I0
K = 10**(6/10)  # 6 dB in linear
A = np.sqrt(K / (K+1))
sigma = np.sqrt(1 / (2*(K+1)))

r = np.linspace(0, 3, 100)
pdf_rician = (r/sigma**2) * np.exp(-(r**2 + A**2)/(2*sigma**2)) * i0(A*r/sigma**2)
plt.plot(r, pdf_rician, 'r-', linewidth=2, label='Theoretical K=6dB')

plt.xlabel('Envelope |h|')
plt.ylabel('PDF')
plt.legend()
plt.title('Rician Fading Envelope Distribution (K=6 dB)')
plt.show()
```

---

## Frequency-Selective Fading (Tapped Delay Line)

**Wideband model**: Multiple delayed copies (taps)

$$
h(t, \tau) = \sum_{l=0}^{L-1} h_l(t) \delta(\tau - \tau_l)
$$

Where:
- $L$ = Number of paths (taps)
- $h_l(t)$ = Complex gain of path $l$ (Rayleigh or Rician)
- $\tau_l$ = Delay of path $l$

**Received signal**:

$$
r(t) = \sum_{l=0}^{L-1} h_l(t) s(t - \tau_l) + n(t)
$$

---

### Implementation (Tapped Delay Line)

```python
def frequency_selective_channel(signal, fs, taps, delays_us, fd):
    """
    Frequency-selective fading channel (Rayleigh taps)
    
    Args:
        signal: Input signal (numpy array)
        fs: Sampling frequency (Hz)
        taps: List of tap powers (linear, sums to 1)
        delays_us: List of tap delays (microseconds)
        fd: Maximum Doppler frequency (Hz)
        
    Returns:
        Output signal
    """
    N = len(signal)
    output = np.zeros(N, dtype=complex)
    
    for tap_power, delay_us in zip(taps, delays_us):
        # Generate Rayleigh fading for this tap
        h_tap = rayleigh_channel_jakes(N, fd, fs)
        h_tap *= np.sqrt(tap_power)  # Scale by tap power
        
        # Delay signal
        delay_samples = int(delay_us * 1e-6 * fs)
        signal_delayed = np.concatenate([np.zeros(delay_samples), 
                                          signal[:N-delay_samples]])
        
        # Apply fading and accumulate
        output += h_tap * signal_delayed
    
    return output
```

**Usage (Urban channel)**:
```python
# 3GPP Urban Macro (UMa) model simplified
taps = [0.5, 0.3, 0.15, 0.05]  # Power profile (exponential decay)
delays_us = [0, 0.5, 1.0, 2.0]  # Delays in microseconds
fd = 50  # Hz (pedestrian)

tx_signal = np.random.randn(10000) + 1j*np.random.randn(10000)
rx_signal = frequency_selective_channel(tx_signal, fs=10e6, 
                                         taps=taps, delays_us=delays_us, fd=fd)

# Add AWGN
rx_signal = awgn_channel(rx_signal, snr_db=10)
```

---

## Standard Channel Models

### 3GPP Spatial Channel Model (SCM)

**LTE/5G NR channel models**:

| Model | Environment | Delay Spread | Doppler | K-factor |
|-------|-------------|--------------|---------|----------|
| **EPA** | Extended Pedestrian A | 0.41 μs | Low (3 km/h) | - |
| **EVA** | Extended Vehicular A | 2.51 μs | Medium (30 km/h) | - |
| **ETU** | Extended Typical Urban | 5.0 μs | High (120 km/h) | - |
| **CDL-A** | Clustered Delay Line A | NLOS (varies) | Configurable | Rayleigh |
| **CDL-B** | Clustered Delay Line B | NLOS | Configurable | Rayleigh |
| **CDL-C** | Clustered Delay Line C | LOS | Configurable | Rician (K=13 dB) |

---

### ITU-R Pedestrian/Vehicular Models

**Pedestrian A** (low delay spread):

| Tap | Delay (ns) | Power (dB) |
|-----|------------|------------|
| 1 | 0 | 0 |
| 2 | 110 | -9.7 |
| 3 | 190 | -19.2 |
| 4 | 410 | -22.8 |

**Vehicular A** (moderate delay spread):

| Tap | Delay (ns) | Power (dB) |
|-----|------------|------------|
| 1 | 0 | 0 |
| 2 | 310 | -1 |
| 3 | 710 | -9 |
| 4 | 1090 | -10 |
| 5 | 1730 | -15 |
| 6 | 2510 | -20 |

---

### Implementation (3GPP EPA)

```python
def epa_channel(signal, fs, fd):
    """
    3GPP Extended Pedestrian A channel
    """
    # EPA tap profile
    delays_ns = [0, 30, 70, 90, 110, 190, 410]
    powers_db = [0, -1, -2, -3, -8, -17.2, -20.8]
    
    # Convert to linear
    powers = 10**(np.array(powers_db)/10)
    powers /= np.sum(powers)  # Normalize
    
    return frequency_selective_channel(signal, fs, 
                                        powers, delays_ns/1000, fd)
```

---

## Doppler Spectrum Visualization

**Verify Doppler spread**:

```python
def plot_doppler_spectrum(h, fs):
    """
    Plot PSD of channel to verify Doppler spectrum
    """
    from scipy import signal as sig
    
    # Compute PSD
    f, Pxx = sig.welch(h, fs=fs, nperseg=1024)
    
    plt.figure()
    plt.semilogy(f, Pxx)
    plt.xlabel('Frequency (Hz)')
    plt.ylabel('PSD')
    plt.title('Doppler Power Spectrum')
    plt.grid(True)
    plt.show()

# Generate Rayleigh channel with fd = 100 Hz
h = rayleigh_channel_jakes(100000, fd=100, fs=10000)
plot_doppler_spectrum(h, fs=10000)
# Should show U-shaped spectrum extending ±100 Hz
```

---

## BER Simulation with Fading

**Complete system simulation**:

```python
def simulate_ber_rayleigh(EbN0_dB_range, M=4, N_bits=100000):
    """
    Simulate BER for QPSK over Rayleigh fading + AWGN
    
    Args:
        EbN0_dB_range: Array of Eb/N0 values (dB)
        M: Modulation order (4 for QPSK)
        N_bits: Number of bits to simulate
        
    Returns:
        BER for each Eb/N0
    """
    import numpy as np
    
    BER = []
    
    for EbN0_dB in EbN0_dB_range:
        # Generate random bits
        bits = np.random.randint(0, 2, N_bits)
        
        # QPSK modulation (simplified)
        symbols = []
        for i in range(0, N_bits, 2):
            b = bits[i:i+2]
            if np.array_equal(b, [0,0]): symbols.append(1+1j)
            elif np.array_equal(b, [0,1]): symbols.append(-1+1j)
            elif np.array_equal(b, [1,0]): symbols.append(1-1j)
            else: symbols.append(-1-1j)
        symbols = np.array(symbols) / np.sqrt(2)  # Normalize
        
        # Rayleigh fading (flat, slow fading - one h per symbol)
        N_symbols = len(symbols)
        h = (np.random.randn(N_symbols) + 1j*np.random.randn(N_symbols)) / np.sqrt(2)
        
        # Apply fading
        rx_symbols = h * symbols
        
        # AWGN (SNR per symbol = EbN0 + 10log10(log2(M)))
        EsN0_dB = EbN0_dB + 10*np.log10(np.log2(M))
        rx_symbols = awgn_channel(rx_symbols, EsN0_dB)
        
        # Coherent demodulation (assume perfect CSI)
        rx_symbols_eq = rx_symbols / h  # Zero-forcing equalization
        
        # QPSK demodulation (hard decision)
        bits_rx = []
        for sym in rx_symbols_eq:
            if sym.real > 0 and sym.imag > 0: bits_rx.extend([0,0])
            elif sym.real < 0 and sym.imag > 0: bits_rx.extend([0,1])
            elif sym.real > 0 and sym.imag < 0: bits_rx.extend([1,0])
            else: bits_rx.extend([1,1])
        
        # Count errors
        errors = np.sum(bits[:len(bits_rx)] != np.array(bits_rx))
        BER.append(errors / len(bits_rx))
    
    return np.array(BER)

# Run simulation
EbN0_range = np.arange(0, 25, 2)
ber_rayleigh = simulate_ber_rayleigh(EbN0_range)

# Plot
plt.figure()
plt.semilogy(EbN0_range, ber_rayleigh, 'o-', label='Rayleigh fading')
plt.grid(True)
plt.xlabel('Eb/N0 (dB)')
plt.ylabel('BER')
plt.title('QPSK BER: Rayleigh Fading with Perfect CSI')
plt.legend()
plt.show()
```

---

## Channel Estimation

**Practical systems need to estimate** $h(t)$:

### Pilot-Based Estimation

**Insert known symbols (pilots) periodically**:

```python
def pilot_channel_estimate(rx_signal, pilot_positions, pilot_symbols):
    """
    Estimate channel using pilots
    
    Args:
        rx_signal: Received signal
        pilot_positions: Indices of pilot symbols
        pilot_symbols: Known pilot symbols
        
    Returns:
        Channel estimates at pilot positions
    """
    h_est = np.zeros(len(pilot_positions), dtype=complex)
    
    for i, pos in enumerate(pilot_positions):
        # h = rx / tx (assuming noiseless for simplicity)
        h_est[i] = rx_signal[pos] / pilot_symbols[i]
    
    return h_est

def interpolate_channel(h_pilots, pilot_positions, N_total):
    """
    Interpolate channel between pilots
    """
    # Linear interpolation
    h_full = np.interp(np.arange(N_total), pilot_positions, h_pilots)
    return h_full

# Example
N = 1000
pilot_spacing = 10
pilot_positions = np.arange(0, N, pilot_spacing)
pilot_symbols = np.ones(len(pilot_positions))  # BPSK pilots

# Generate channel
h_true = rayleigh_channel_jakes(N, fd=20, fs=1000)

# Simulate RX
tx_signal = np.random.randn(N) + 1j*np.random.randn(N)
tx_signal[pilot_positions] = pilot_symbols  # Insert pilots
rx_signal = h_true * tx_signal

# Estimate
h_pilots = pilot_channel_estimate(rx_signal, pilot_positions, pilot_symbols)
h_est = interpolate_channel(h_pilots, pilot_positions, N)

# Compare
mse = np.mean(np.abs(h_true - h_est)**2)
print(f"Channel estimation MSE: {10*np.log10(mse):.1f} dB")
```

---

## Summary of Channel Models

| Model | Use Case | Complexity | Realism |
|-------|----------|------------|---------|
| **AWGN** | Satellite LOS, benchmarking | Low | Idealized |
| **Rayleigh (Jakes)** | Urban NLOS, mobile | Medium | Good for NLOS |
| **Rician** | Suburban LOS+scatter | Medium | Good for partial LOS |
| **Tapped delay line** | Wideband, frequency-selective | High | Excellent |
| **3GPP CDL** | LTE/5G NR | Very high | Industry standard |

---

## Practical Implementation Tips

1. **Sampling rate**: Choose $f_s \gg 2f_d$ to avoid aliasing Doppler spectrum (typically $f_s > 50 f_d$)

2. **Number of scatterers**: M = 8-16 sufficient for Jakes' model (higher M = smoother statistics but slower)

3. **Normalization**: Always verify average channel power = 1 (so SNR definition consistent)

4. **CSI assumption**: Perfect CSI (known h) → Upper bound. Pilot-based estimation → Practical performance

5. **Long simulations**: Need many fade cycles for accurate BER (typically $> 100/\text{BER}$ bits)

6. **Tap spacing**: For frequency-selective, ensure tap delays match expected delay spread ($\tau_{\text{rms}}$)

---

## Related Topics

- **[[Multipath Propagation & Fading (Rayleigh, Rician)]]**: Theory behind channel models
- **[[Signal-to-Noise Ratio (SNR)]]**: SNR definition for fading channels
- **[[Bit Error Rate (BER)]]**: Performance metric vs fading
- **[[Complete Link Budget Analysis]]**: Using fading margin in link budget
- **[[OFDM & Multicarrier Modulation]]**: Combats frequency-selective fading
- **[[Channel Equalization]]**: Compensates for ISI in frequency-selective channels

---

**Key takeaway**: **Channel models enable realistic system simulation without hardware.** AWGN is baseline, Rayleigh for NLOS mobile, Rician for partial LOS, tapped delay line for wideband ISI. Jakes' model efficiently generates Rayleigh fading with correct Doppler spectrum. 3GPP CDL models are industry-standard for LTE/5G. Pilot-based channel estimation is practical approach. Always verify statistics (envelope PDF, average power, Doppler spectrum) match theory.

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
