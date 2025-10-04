# On-Off Keying (OOK)

## 🔦 For Non-Technical Readers

**OOK is literally just turning a signal ON and OFF—it's the simplest possible way to send data, like morse code with a flashlight!**

**The idea**:
- **ON** (signal present) = binary **1**
- **OFF** (no signal) = binary **0**
- That's it! Simplest modulation possible.

**Flashlight analogy**:
- Shine flashlight = 1
- Turn off flashlight = 0
- Sequence: ON-ON-OFF-ON = "1101"
- Morse code uses the same principle!

**Why it's everywhere (despite being old)**:
- **Dead simple**: Easiest to transmit and receive
- **Lowest power**: No signal = no power consumption for 0s!
- **Cheap hardware**: Basic transistor switch = complete transmitter
- **Good enough**: For short-range, low-speed, it just works

**Where you see OOK every day**:
- **Car key fobs**: Unlock button uses OOK!
- **Garage door openers**: Yep, OOK
- **Wireless doorbells**: OOK at ~315/433 MHz
- **Cheap weather sensors**: Temperature transmitter → receiver
- **RC toys**: Simple remote controls
- **Old telegraph**: On/off keying of electrical circuit!

**Why it's not used for high-speed**:
- **Bandwidth inefficient**: Need wide frequency band for sharp on/off transitions
- **Noise sensitive**: Hard to tell weak signal from noise
- **No error detection**: Unlike PSK/QAM, can't detect phase errors
- **Synchronization issues**: Receiver must guess when bits start/end

**Modern variant - ASK**:
- OOK is binary ASK (Amplitude-Shift Keying)
- Instead of on/off, use multiple power levels
- Still simple, slightly more efficient

**The ultimate simplicity**:
- **Transmitter**: Microcontroller + transistor + antenna
- **Receiver**: Antenna + diode + microcontroller
- Total cost: <$2 for both sides!
- This is why every wireless doorbell uses OOK

**Fun fact**: The first wireless telegraph (Marconi, 1895) used OOK—literally just turning a spark-gap transmitter on and off to send morse code. 130 years later, your car keys still use the same basic principle!

---

**On-Off Keying (OOK)** is the simplest form of digital modulation, where the presence or absence of a carrier wave represents binary data.

---

## 📐 Basic Principle

```
Bit "1": Carrier ON  → s₁(t) = A·cos(2πf_c·t)
Bit "0": Carrier OFF → s₀(t) = 0

where:
- A = carrier amplitude
- f_c = carrier frequency
- T_b = bit duration
```

**Time-domain representation**:
```
       ___     ___         ___
      |   |   |   |       |   |
  ____|   |___|   |_______|   |___
      1   0   1       0       1     (data bits)
```

---

## 🔬 Mathematical Description

**Transmitted signal**:
```
s(t) = Σ b_k · A·cos(2πf_c·t)     for kT_b ≤ t < (k+1)T_b
       k

where b_k ∈ {0, 1}
```

**Modulation index**: m = 1 (100% modulation depth)

---

## 📊 Spectral Characteristics

**Bandwidth** (null-to-null):
```
B = 2/T_b = 2R_b

where R_b = bit rate (bps)
```

**Power spectral density**: Sinc² function centered at f_c

**Example**: 1 kbps data rate → 2 kHz bandwidth

---

## 📡 Demodulation

### Non-Coherent Detection (Envelope Detector)

**Simplest receiver** - no carrier phase recovery needed!

```
Received signal:
r(t) = s(t) + n(t)

Envelope detector:
e(t) = |r(t)| = √[I²(t) + Q²(t)]

Decision:
If e(t) > threshold: bit = 1
If e(t) < threshold: bit = 0
```

**Advantage**: Very simple hardware (diode + capacitor)
**Disadvantage**: 3 dB worse performance than coherent detection

---

### Coherent Detection (Correlation)

**Better performance** but requires carrier synchronization:

```
Correlator output:
z = ∫₀^Tb r(t)·cos(2πf_c·t) dt

Decision:
If z > 0: bit = 1
If z < 0: bit = 0
```

---

## 📈 Performance Analysis

### Bit Error Rate (BER)

**With coherent detection** (AWGN channel):
```
BER = Q(√(E_b/N₀))

where:
- E_b = bit energy = (A²T_b)/2
- N₀ = noise power spectral density
- Q(x) = (1/√2π) ∫_x^∞ e^(-t²/2) dt  (tail probability of Gaussian)
```

**With non-coherent detection**:
```
BER = (1/2)exp(-E_b/2N₀)    (3 dB worse!)
```

**Example**: For BER = 10⁻⁶
- Coherent OOK: E_b/N₀ ≈ 13.5 dB
- Non-coherent OOK: E_b/N₀ ≈ 16.5 dB
- [[QPSK Modulation|QPSK]]: E_b/N₀ ≈ 10.5 dB (better!)

---

## ⚖️ Advantages & Disadvantages

### Advantages
✅ **Simplest modulation** - minimal transmitter complexity
✅ **No phase synchronization** (non-coherent detection)
✅ **Power efficient when off** - ideal for low duty cycle
✅ **Easy to implement** - analog/digital

### Disadvantages
❌ **Poor spectral efficiency** - 0.5 bits/s/Hz (twice bandwidth of BPSK)
❌ **Poor power efficiency** - needs 3 dB more power than BPSK for same BER
❌ **Susceptible to fading** - deep fades completely eliminate signal
❌ **No use of "0" transmission** - wastes half the signal space

---

## 🎯 Applications

### Historical
- **Morse code** (telegraphy, 1840s)
- **Early radio** (spark-gap transmitters)
- **Infrared remote controls** (TV remotes, 1980s)

### Modern
- **Optical fiber** (on-off of laser)
- **RFID tags** (passive, backscatter modulation)
- **Low-power IoT** (e.g., LoRa preamble)
- **Visible light communication** (LED on-off)

**Why still used?** Simplicity trumps efficiency for low-cost, low-power devices.

---

## 🔀 Variants

### Amplitude-Shift Keying (ASK)

**Generalization of OOK** with non-zero "off" level:
```
Bit "1": s₁(t) = A₁·cos(2πf_c·t)
Bit "0": s₀(t) = A₀·cos(2πf_c·t)    (A₀ > 0)
```

**OOK is special case**: A₀ = 0

---

### Pulse-Position Modulation (PPM)

**Used in optical communications**:
```
Bit "1": Pulse at t = 0
Bit "0": Pulse at t = T_b/2
```

**More power-efficient** than OOK for optical systems.

---

## 📐 Constellation Diagram

```
      Q
      ↑
      |
  0   |   • 1  ← Only two points!
      |   (A, 0)
──────┼──────→ I
      |
```

**Single dimension** (amplitude only, no phase modulation).

**Distance between points**: d = A

**Compare to [[QPSK Modulation|QPSK]]**: Four points, better use of signal space.

---

## 🔬 Comparison to Other Modulations

| Modulation | Bits/Symbol | Bandwidth | Power (for BER=10⁻⁶) | Complexity |
|------------|-------------|-----------|----------------------|------------|
| **OOK** | 1 | 2R_b | 16.5 dB (non-coh) | Lowest |
| [[BPSK]] | 1 | R_b | 10.5 dB | Low |
| [[QPSK Modulation\|QPSK]] | 2 | R_b | 10.5 dB | Medium |
| [[16-QAM]] | 4 | R_b | 18.5 dB | High |

**Key insight**: OOK is simple but inefficient. [[BPSK]] is better in almost every way (except hardware complexity).

---

## 🎓 Key Takeaways

1. **Simplest modulation**: Just turn carrier on/off
2. **Non-coherent detection possible**: No carrier recovery needed
3. **Poor efficiency**: Both spectral and power
4. **Historical importance**: First digital modulation
5. **Still used**: Low-cost, low-power applications (optical, RFID)
6. **Gateway to understanding**: Good starting point before [[FSK]], [[BPSK]]

---

## 🔗 See Also

- [[Amplitude-Shift Keying (ASK)]] - Generalization of OOK *(coming soon)*
- [[Frequency-Shift Keying (FSK)]] - Next step in modulation complexity
- [[Binary Phase-Shift Keying (BPSK)]] - Better alternative (same complexity, better performance)
- [[QPSK Modulation]] - Even more efficient
- [[Constellation Diagrams]] - Visual representation of modulations

---

## 📚 References

1. **Morse, S.** (1840) - First practical OOK system (telegraph)
2. **Proakis, J.G. & Salehi, M.** (2008) *Digital Communications* 5th ed. (McGraw-Hill)
3. **Sklar, B.** (2001) *Digital Communications: Fundamentals and Applications* 2nd ed. (Prentice Hall)
