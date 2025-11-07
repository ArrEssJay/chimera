# Frequency-Shift Keying (FSK)

## 🎵 For Non-Technical Readers

**FSK is like morse code with two different musical notes—high note = 1, low note = 0. Simple, robust, and still used everywhere!**

**The idea**:
- Want to send a **1**? Transmit at **high frequency** (e.g., 1200 Hz)
- Want to send a **0**? Transmit at **low frequency** (e.g., 1000 Hz)
- Receiver listens for which tone is present

**Musical analogy**:
- Playing piano: **C note** = 0, **E note** = 1
- Song: "C C E C E E C" = data: "0 0 1 0 1 1 0"
- Your ear (receiver) easily distinguishes C from E!
- FSK receiver does the same with radio frequencies

**Why it's great**:
- **Super robust**: Noise changes amplitude, but frequency stays clear!
- **Simple**: Just detect which frequency is present
- **Immune to fading**: Signal can get weaker, but frequency doesn't change
- **Works in harsh environments**: Industrial, underwater, long-range

**Where you encounter FSK**:
- **Caller ID**: Your phone uses FSK to send caller info between rings!
- **Old dial-up modems**: 1980s modems used FSK (remember the screeching sound?)
- **Bluetooth Low Energy**: Uses GFSK (Gaussian FSK) for low power
- **RFID tags**: Many use FSK for simplicity
- **Weather balloons**: FSK survives atmospheric interference
- **Pagers**: Remember pagers? FSK!

**Real-world sounds**:
- **Fax machine**: That squawking noise is FSK! Listen carefully—you can hear the two tones alternating
- **Dial-up internet**: BEEEE-doo-BEEEE-doo = FSK handshake
- **Emergency broadcast tones**: Two-tone alert = FSK

**Variants**:
- **BFSK**: Binary (2 frequencies) = 1 bit/symbol
- **MFSK**: Multiple frequencies (4, 8, 16, etc.) = more bits/symbol
- **GFSK**: Gaussian FSK (smooth transitions) = used in Bluetooth

**Trade-off**:
- **Advantage**: Extremely robust, immune to amplitude variations
- **Disadvantage**: Slow compared to QAM (lower spectral efficiency)
- Best for: Low-power, long-range, harsh environments

**Fun fact**: Old telegraph operators could "read" morse code by EAR at 40+ words/minute. FSK is the same idea—humans can literally hear binary data if you slow it down!

---

**Frequency-Shift Keying (FSK)** is a digital modulation scheme where binary data is represented by discrete changes in carrier frequency.

---

## 📐 Basic Principle

**Binary FSK (BFSK)**:
```
Bit "1": s₁(t) = A·cos(2πf₁·t)
Bit "0": s₀(t) = A·cos(2πf₀·t)

where:
- A = constant amplitude
- f₁ = "mark" frequency (higher)
- f₀ = "space" frequency (lower)
- Δf = f₁ - f₀ = frequency separation
```

**Time-domain representation**:
```
Frequency
   ↑
f₁ |───|   |───|       |───|    ← Bit "1"
   |   |   |   |       |   |
f₀ |   |---|   |-------|   |--- ← Bit "0"
   └──────────────────────────→ Time
       1   0   1   0       1
```

---

## 🔢 Mathematical Description

**Transmitted signal**:
```
s(t) = A·cos[2π(f_c + b_k·Δf/2)·t]     for kT_b ≤ t < (k+1)T_b

where:
- f_c = carrier frequency (center)
- b_k ∈ {-1, +1} (or {0, 1})
- Δf = frequency deviation
- T_b = bit duration
```

**Modulation index**:
```
h = 2Δf·T_b

Common values:
- h = 0.5 (Minimum Shift Keying - MSK)
- h = 1.0 (Sunde's FSK)
- h > 1 (Wideband FSK)
```

---

## 📊 Spectral Characteristics

**Bandwidth** (Carson's rule):
```
B = 2(Δf + R_b)

where R_b = 1/T_b = bit rate
```

**Examples**:
- Narrowband FSK (h = 0.5): B ≈ 1.5 R_b
- Wideband FSK (h = 2): B ≈ 5 R_b

**Power spectral density**: Two main lobes centered at f₀ and f₁

---

## 📡 Demodulation Methods

### 1. Non-Coherent Detection (Envelope Detector)

**Simple and practical** - no carrier phase recovery!

```
Architecture:

         ┌─────────┐
    r(t)─┤ BPF @ f₁├──┐
         └─────────┘  │
                      ├─→ Envelope → Compare → Decision
         ┌─────────┐  │   Detectors
    r(t)─┤ BPF @ f₀├──┘
         └─────────┘

Decision:
If |output of f₁ filter| > |output of f₀ filter|: bit = 1
Else: bit = 0
```

**Advantages**: Simple, no synchronization
**Disadvantages**: ~1 dB worse than coherent

---

### 2. Coherent Detection (Correlation)

**Optimal performance** but requires carrier synchronization:

```
Correlators:

         ┌──────────┐
    r(t)─┤ × cos(2πf₁t) ├──→ ∫ → z₁
         └──────────┘      0 to Tb

         ┌──────────┐
    r(t)─┤ × cos(2πf₀t) ├──→ ∫ → z₀
         └──────────┘      0 to Tb

Decision:
If z₁ > z₀: bit = 1
Else: bit = 0
```

---

### 3. Frequency Discriminator

**Classic FM receiver approach**:

```
r(t) → [Limiter] → [Frequency Discriminator] → [LPF] → Decision
```

**Converts frequency deviation to voltage**, then samples at bit boundaries.

---

## 📈 Performance Analysis

### Bit Error Rate (BER)

**With non-coherent detection** (AWGN channel):
```
BER = (1/2)exp(-E_b/2N₀)      for orthogonal FSK

where:
- E_b = bit energy = (A²T_b)/2
- N₀ = noise power spectral density
```

**With coherent detection**:
```
BER = Q(√(E_b/N₀))            (1 dB better!)
```

**For orthogonal FSK**: Frequencies f₀ and f₁ must satisfy:
```
(f₁ - f₀)·T_b = n/2    (n = integer)

Minimum: Δf = 1/(2T_b)  → h = 1 (Sunde's FSK)
```

---

## ⚖️ Advantages & Disadvantages

### Advantages
✅ **Constant envelope** - efficient power amplifiers (Class C)
✅ **Non-coherent detection** - simple receivers
✅ **Robust to fading** - amplitude variations don't affect frequency
✅ **Good for noisy channels** - frequency easier to detect than phase
✅ **Legacy compatibility** - used in many older systems

### Disadvantages
❌ **Poor spectral efficiency** - wider bandwidth than PSK
❌ **Moderate power efficiency** - 1-2 dB worse than [[BPSK]]
❌ **Frequency stability** - requires accurate oscillators
❌ **Doppler sensitivity** - frequency shifts problematic

---

## 🎯 Applications

### Historical & Current
- **Telephone modems** (Bell 103: 1962, 300 baud, f₀=1070 Hz, f₁=1270 Hz)
- **Radio teletype** (RTTY, 1930s-)
- **Caller ID** (Bell 202: 1200 bps, f₀=2200 Hz, f₁=1200 Hz)
- **Pagers** (POCSAG, FLEX protocols)

### Modern
- **LoRa** (sub-GHz IoT, chirp spread spectrum FSK)
- **Bluetooth Low Energy** (GFSK - Gaussian FSK)
- **Wireless sensor networks** - low power, simple receivers
- **Optical fiber** (frequency-shifted laser)
- **[[AID-Protocol-Case-Study]]** - 12 kHz FSK sub-carrier (11,999/12,001 Hz)

---

## 🔀 FSK Variants

### 1. Minimum Shift Keying (MSK)

**Special case**: h = 0.5 (minimum for orthogonality)

```
Properties:
- Continuous phase (no discontinuities)
- Constant envelope
- Bandwidth = 1.5 R_b (narrowest FSK)
- Equivalent to offset QPSK with sinusoidal pulse shaping
```

**Used in**: GSM cellular (GMSK - Gaussian MSK)

---

### 2. Gaussian FSK (GFSK)

**MSK + Gaussian pre-modulation filter**

```
Purpose: Further reduce spectral sidelobes
Bandwidth: ~1.2-1.5 R_b (depending on BT product)
BT product: Bandwidth × T_b (typical: 0.3-0.5)
```

**Used in**: Bluetooth, Zigbee

---

### 3. Continuous Phase FSK (CPFSK)

**Phase is continuous** across bit boundaries:

```
φ(t) = 2π[f_c·t + (hΔf/2)·∫₀ᵗ b(τ)dτ]

Benefits:
- No spectral splatter
- Better spectral efficiency
- Smoother power envelope
```

---

### 4. Multi-Frequency FSK (MFSK)

**M > 2 frequencies** for higher data rates:

```
M symbols → log₂(M) bits per symbol

Example (4-FSK):
- f₀: bits 00
- f₁: bits 01
- f₂: bits 10
- f₃: bits 11

Bandwidth: B = M·R_b (wider!)
Power efficiency: Better than BFSK for high M
```

**Used in**: HF radio (MT63, Olivia modes)

---

## 📐 Constellation Diagram

**BFSK in frequency space**:
```
Frequency
   ↑
f₁ |     • Symbol "1"
   |
f_c|       (carrier)
   |
f₀ |     • Symbol "0"
   └────────────→ Time
```

**Not a traditional I/Q constellation** (frequency, not amplitude/phase).

**Equivalent I/Q representation** (for coherent detection):
```
      Q
      ↑
      |
  0 • | • 1  ← On real axis, separated
      |
──────┼──────→ I
```

**Distance between points**: d = √(2E_b) (for orthogonal FSK)

---

## 🔬 Comparison Table

| Modulation | Bits/Symbol | Bandwidth | E_b/N₀ @ BER=10⁻⁶ | Envelope | Detection |
|------------|-------------|-----------|-------------------|----------|-----------|
| [[On-Off-Keying-(OOK)|OOK]] | 1 | 2R_b | 13.5 dB | Variable | Non-coherent |
| **FSK** | 1 | 2R_b | 12.5 dB | Constant | Non-coherent |
| **MSK** | 1 | 1.5R_b | 10.5 dB | Constant | Coherent |
| [[BPSK]] | 1 | R_b | 10.5 dB | Constant | Coherent |
| [[QPSK-Modulation|QPSK]] | 2 | R_b | 10.5 dB | Constant | Coherent |

**Key insight**: FSK trades bandwidth for simplicity. [[BPSK]]/[[QPSK-Modulation|QPSK]] are more efficient but require phase synchronization.

---

## 🎓 Key Takeaways

1. **Frequency switching**: Binary data → two different frequencies
2. **Constant envelope**: Good for non-linear amplifiers
3. **Non-coherent detection**: Simple receivers, still good performance
4. **Bandwidth penalty**: ~2× wider than PSK
5. **Robust**: Good for noisy, fading channels
6. **Still widely used**: Bluetooth, LoRa, pagers, caller ID
7. **Gateway to chirp spread spectrum**: LoRa uses frequency chirps

---

## 🔗 See Also

- [[On-Off-Keying-(OOK)]] - Simpler (amplitude modulation)
- [[Binary-Phase-Shift-Keying-(BPSK)]] - Alternative (phase modulation)
- [[QPSK-Modulation]] - More bits per symbol (phase)
- [[Constellation-Diagrams]] - Visualizing modulation schemes
- [[AID-Protocol-Case-Study]] - Uses 1 bps FSK sub-carrier (11,999/12,001 Hz)

---

## 📚 References

1. **Sunde, E.D.** (1946) "Ideal binary pulse transmission by AM and FM" *Bell Syst. Tech. J.* 25, 1067-1093
2. **de Jager, F. & Dekker, C.B.** (1978) "Tamed Frequency Modulation" *IEEE Trans. Comm.* COM-26, 534-542
3. **Proakis, J.G. & Salehi, M.** (2008) *Digital Communications* 5th ed. (McGraw-Hill)
4. **Sklar, B.** (2001) *Digital Communications* 2nd ed. (Prentice Hall)
