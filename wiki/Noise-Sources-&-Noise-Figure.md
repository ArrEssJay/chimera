# Noise Sources & Noise Figure

[[Home]] | **Link Budget & System Performance** | [[Signal-to-Noise-Ratio-(SNR)]] | [[Complete-Link-Budget-Analysis]]

---

## 🎓 For Non-Technical Readers

**Think of radio communication like having a conversation in a crowded restaurant:**

- **Signal** = Your friend's voice trying to reach you
- **Noise** = All the background chatter, kitchen sounds, and air conditioning
- **Noise Figure** = How much your hearing aids (or bad acoustics) make it harder to understand

**Why noise matters:**
If the background noise is too loud, you can't hear your friend—even if they're shouting. Same with radio: if noise is too high, the receiver can't "hear" the signal, no matter how powerful the transmitter.

**Key insights in plain English:**

1. **Thermal noise is everywhere**: Just like atoms vibrating creates heat, electrons vibrating creates random electrical "static" in every wire, antenna, and amplifier. This sets a fundamental limit—you can't eliminate it, only work around it.

2. **The -174 dBm magic number**: This is the "noise floor" at room temperature for a 1 Hz bandwidth. Think of it as the quietest possible "background hum" in radio. Everything adds noise on top of this baseline.

3. **Amplifiers make noise worse**: Every amplifier adds its own noise (like a hearing aid with poor quality that adds hiss). The **noise figure** tells you how much worse the amplifier makes the signal-to-noise ratio.

4. **First stage is critical**: Just like you want your hearing aid right at your ear (not connected by a long cable), you want the first amplifier (Low-Noise Amplifier, or LNA) as close to the antenna as possible. Once noise is added early, you can't remove it later.

5. **Wider bandwidth = more noise**: Like opening more windows lets in more outside noise, using a wider radio bandwidth lets in more thermal noise. This is why high-speed data links (wide bandwidth) need stronger signals than voice links (narrow bandwidth).

**Real-world impact:**
- **Satellite TV**: Premium receivers have better (lower) noise figures, letting them work with smaller dishes
- **GPS**: Your phone's GPS can detect signals 1,000× weaker than WiFi because it fights noise cleverly (spread spectrum)
- **Deep space missions**: NASA uses cryogenically-cooled amplifiers (like refrigerating your hearing aid!) to reduce noise and hear probes billions of miles away

**Bottom line**: If you want to receive weak signals (long range, small antenna, low power), you must minimize noise. That's why the first few inches of cable and the first amplifier matter more than anything else in the receiver chain.

---

## Overview

**Noise** is unwanted random signal that degrades communication system performance.

**Key metrics**:
- **Noise power** (N): Total noise at receiver input (dBm, watts)
- **Noise figure** (NF): How much a component degrades SNR (dB)
- **Noise temperature** (T_e): Equivalent thermal noise (Kelvin)

**Why it matters**:
- Determines **receiver sensitivity** (minimum detectable signal)
- Sets **SNR** at demodulator input
- Dominates link performance in low-signal scenarios (satellite, deep space)

**Bottom line**: Lower noise = Better sensitivity = Longer range

---

## Thermal Noise

**Fundamental noise source**: Random motion of charge carriers due to thermal agitation

### Johnson-Nyquist Noise

**Noise power** in resistor at temperature T:

$$
N = k T B \quad (\text{watts})
$$

Where:
- $k = 1.38 \times 10^{-23}$ J/K (Boltzmann constant)
- $T$ = Absolute temperature (Kelvin)
- $B$ = Bandwidth (Hz)

**Standard reference**: $T_0 = 290$ K (room temperature, ~17°C)

---

### Noise Power Spectral Density

**Noise power per Hz**:

$$
N_0 = k T \quad (\text{W/Hz})
$$

**At 290 K**:

$$
N_0 = 1.38 \times 10^{-23} \times 290 = 4 \times 10^{-21}\ \text{W/Hz}
$$

**In dBm/Hz**:

$$
N_0 = 10\log_{10}\left(\frac{4 \times 10^{-21}}{10^{-3}}\right) = -174\ \text{dBm/Hz}
$$

**This is the famous -174 dBm/Hz thermal noise floor!**

---

### Noise Power in Bandwidth B

$$
N = N_0 \times B = kTB \quad (\text{watts})
$$

**In dB**:

$$
N_{\text{dBm}} = -174 + 10\log_{10}(B) \quad (\text{dBm})
$$

**Example**: 1 MHz bandwidth @ 290 K

$$
N = -174 + 10\log_{10}(10^6) = -174 + 60 = -114\ \text{dBm}
$$

---

### Typical Bandwidths and Noise Power

| System | Bandwidth | Noise Power @ 290 K |
|--------|-----------|---------------------|
| **GPS C/A code** | 2 MHz | -111 dBm |
| **WiFi 20 MHz** | 20 MHz | -101 dBm |
| **LTE 10 MHz** | 10 MHz | -104 dBm |
| **DVB-S2 36 MHz** | 36 MHz | -98.4 dBm |
| **Radar (1 GHz pulse)** | 1 GHz | -84 dBm |

**Key insight**: Wider bandwidth = More noise power

---

## Noise Figure (NF)

**Definition**: **Degradation of SNR** through a component or system

$$
\text{NF} = \frac{\text{SNR}_{\text{in}}}{\text{SNR}_{\text{out}}} \quad (\text{linear ratio})
$$

**In dB**:

$$
\text{NF}_{\text{dB}} = 10\log_{10}(\text{NF}) = \text{SNR}_{\text{in,dB}} - \text{SNR}_{\text{out,dB}}
$$

**Interpretation**:
- **NF = 1 (0 dB)**: Ideal (no noise added)
- **NF = 2 (3 dB)**: SNR halved (doubles noise power)
- **NF = 10 (10 dB)**: SNR reduced by 10× (10× noise power)

---

### Noise Figure vs Noise Factor

**Noise factor** (F): Linear ratio

**Noise figure** (NF): Logarithmic (dB)

$$
\text{NF}_{\text{dB}} = 10\log_{10}(F)
$$

**Example**: F = 2 → NF = 3 dB

---

### Typical Noise Figures

| Component | Noise Figure (dB) | Notes |
|-----------|-------------------|-------|
| **Passive cable** | Loss in dB | Lossy line: NF = loss |
| **Ideal amplifier** | 0 | Theoretical only |
| **Cryogenic LNA** | 0.3-0.8 | Cooled to 20-80 K |
| **Premium LNA** | 0.8-1.5 | GaAs HEMT, room temp |
| **Good LNA** | 1.5-3 | Typical satellite ground |
| **WiFi/cellular front-end** | 5-9 | Consumer devices |
| **Mixer (passive)** | 6-10 | Diode mixer |
| **Mixer (active)** | 10-15 | Gilbert cell |

---

## Noise Temperature

**Alternative to noise figure**: Equivalent input noise temperature

$$
T_e = T_0 (F - 1) \quad (\text{K})
$$

Where $T_0 = 290$ K (reference)

**Relationship**:

$$
F = 1 + \frac{T_e}{T_0}
$$

$$
\text{NF}_{\text{dB}} = 10\log_{10}\left(1 + \frac{T_e}{290}\right)
$$

---

### Noise Figure ↔ Noise Temperature

| NF (dB) | Noise Factor F | $T_e$ (K) |
|---------|----------------|-----------|
| 0 | 1 | 0 |
| 0.5 | 1.12 | 35 |
| 1 | 1.26 | 75 |
| 2 | 1.58 | 169 |
| 3 | 2 | 290 |
| 6 | 4 | 870 |
| 10 | 10 | 2610 |

**Usage**: Satellite/radio astronomy communities prefer $T_e$, RF engineers prefer NF

---

## Cascaded Noise Figure (Friis Formula)

**Multi-stage system**: Amplifiers, mixers, filters in series

**Total noise factor**:

$$
F_{\text{total}} = F_1 + \frac{F_2 - 1}{G_1} + \frac{F_3 - 1}{G_1 G_2} + \frac{F_4 - 1}{G_1 G_2 G_3} + \ldots
$$

Where:
- $F_i$ = Noise factor of stage $i$ (linear)
- $G_i$ = Gain of stage $i$ (linear)

**In dB**:

$$
\text{NF}_{\text{total}} = 10\log_{10}(F_{\text{total}})
$$

---

### Key Insights from Friis Formula

1. **First stage dominates**: $F_1$ appears without division → **LNA critical!**
2. **High gain helps**: Later stages divided by $G_1 G_2 \ldots$ → Less impact
3. **Avoid loss before LNA**: Cable loss before LNA directly adds to NF

---

### Example 1: Simple Receiver Chain

**Components**:
1. **Cable**: Loss 2 dB (NF = 2 dB, F = 1.58, G = 0.63 = -2 dB)
2. **LNA**: NF = 1 dB, F = 1.26, G = 20 dB (100×)
3. **Mixer**: NF = 10 dB, F = 10, G = -6 dB (0.25×)

**Total NF**:

$$
F_{\text{total}} = 1.58 + \frac{1.26 - 1}{0.63} + \frac{10 - 1}{0.63 \times 100}
$$

$$
= 1.58 + 0.41 + 0.14 = 2.13
$$

$$
\text{NF}_{\text{total}} = 10\log_{10}(2.13) = 3.3\ \text{dB}
$$

**Dominated by cable loss!**

---

### Example 2: Cable After LNA (Best Practice)

**Components**:
1. **LNA**: NF = 1 dB, F = 1.26, G = 20 dB (100×)
2. **Cable**: Loss 2 dB, F = 1.58, G = 0.63
3. **Mixer**: NF = 10 dB, F = 10, G = -6 dB (0.25×)

**Total NF**:

$$
F_{\text{total}} = 1.26 + \frac{1.58 - 1}{100} + \frac{10 - 1}{100 \times 0.63}
$$

$$
= 1.26 + 0.0058 + 0.14 = 1.41
$$

$$
\text{NF}_{\text{total}} = 10\log_{10}(1.41) = 1.5\ \text{dB}
$$

**Much better! LNA at antenna isolates from cable loss.**

---

### Example 3: Two-Stage LNA

**Components**:
1. **LNA1**: NF = 0.8 dB, F = 1.2, G = 15 dB (31.6×)
2. **LNA2**: NF = 1.5 dB, F = 1.41, G = 20 dB (100×)
3. **Mixer**: NF = 10 dB, F = 10, G = -6 dB (0.25×)

**Total NF**:

$$
F_{\text{total}} = 1.2 + \frac{1.41 - 1}{31.6} + \frac{10 - 1}{31.6 \times 100}
$$

$$
= 1.2 + 0.013 + 0.0028 = 1.216
$$

$$
\text{NF}_{\text{total}} = 10\log_{10}(1.216) = 0.85\ \text{dB}
$$

**Excellent! High gain LNA1 suppresses later stages.**

---

## System Noise Temperature

**Total noise temperature** of cascaded system:

$$
T_{\text{sys}} = T_{\text{ant}} + T_e
$$

Where:
- $T_{\text{ant}}$ = Antenna noise temperature (K)
- $T_e$ = Receiver equivalent noise temperature (K)

---

### Antenna Noise Temperature

**Antenna picks up thermal radiation** from environment:

**Sources**:
- **Sky**: 3-300 K (depends on frequency, elevation)
- **Ground**: 290 K (room temperature)
- **Sun**: ~10,000 K (if pointed directly)
- **Cosmic background**: 2.7 K (everywhere)

**Typical values**:

| Scenario | Frequency | Elevation | $T_{\text{ant}}$ (K) |
|----------|-----------|-----------|----------------------|
| **Deep space** | Any | - | 3-5 |
| **Satellite (clear sky)** | 1-10 GHz | 30° | 20-50 |
| **Satellite (rain)** | 12 GHz | 30° | 100-200 |
| **Terrestrial** | Any | Horizon | 290 |

---

### G/T Ratio (Figure of Merit)

**System performance metric** for satellite ground stations:

$$
\frac{G}{T} = G_r - 10\log_{10}(T_{\text{sys}}) \quad (\text{dB/K})
$$

Where:
- $G_r$ = RX antenna gain (dBi)
- $T_{\text{sys}}$ = System noise temperature (K)

**Interpretation**: Higher G/T = Better sensitivity

**Example**: 3 m Ku-band dish, LNA at feed
- Antenna gain: 48 dBi
- Antenna temp: 30 K (clear sky, 30° elevation)
- LNA NF: 0.8 dB → $T_e = 55$ K
- $T_{\text{sys}} = 30 + 55 = 85$ K

$$
\frac{G}{T} = 48 - 10\log_{10}(85) = 48 - 19.3 = 28.7\ \text{dB/K}
$$

**Typical G/T**:
- **VSAT terminals** (0.6-1.2 m): 10-20 dB/K
- **Professional earth stations** (3-9 m): 25-35 dB/K
- **Large observatories** (25+ m): 40-60 dB/K

---

## Other Noise Sources

### 1. Shot Noise

**Due to discrete nature of charge carriers**:

$$
i_n^2 = 2 q I_{\text{DC}} B
$$

Where:
- $q = 1.6 \times 10^{-19}$ C (electron charge)
- $I_{\text{DC}}$ = DC current (A)
- $B$ = Bandwidth (Hz)

**Significant in**: Photodetectors, diodes, low-current circuits

**Example**: Photodiode @ 1 mA DC, 1 MHz BW

$$
i_n = \sqrt{2 \times 1.6 \times 10^{-19} \times 10^{-3} \times 10^6} = 5.7 \times 10^{-10}\ \text{A}_{\text{rms}}
$$

---

### 2. Flicker Noise (1/f Noise)

**Low-frequency noise**, power inversely proportional to frequency:

$$
S(f) \propto \frac{1}{f}
$$

**Significant**: < 1 kHz (audio, low-IF systems)

**Mitigation**: Use higher IF, differential circuits, chopper stabilization

---

### 3. Phase Noise

**Oscillator noise** causes frequency jitter:

**Specified as** $\mathcal{L}(f_m)$ (dBc/Hz at offset $f_m$ from carrier)

**Example**: Satellite LO @ 10 GHz
- Phase noise: -90 dBc/Hz @ 10 kHz offset
- Degrades SNR in adjacent channels

**See**: [[Synchronization-(Carrier,-Timing,-Frame)]] for impact on coherent demodulation

---

### 4. Quantization Noise

**Analog-to-digital conversion** introduces rounding error:

$$
\text{SNR}_{\text{quant}} = 6.02n + 1.76 \quad (\text{dB})
$$

Where $n$ = Number of bits

**Example**: 12-bit ADC
- SNR = 6.02 × 12 + 1.76 = 74 dB

**Implication**: Need enough ADC bits to avoid degrading RF SNR

---

### 5. Intermodulation Distortion (IMD)

**Non-linear components** create spurious products:

**Two-tone test**: Inputs at $f_1$ and $f_2$ → Products at $2f_1 - f_2$, $2f_2 - f_1$ (3rd order)

**IP3** (Third-order intercept point):

$$
\text{IMD3}_{\text{dBc}} = 2(P_{\text{IP3}} - P_{\text{in}})
$$

**Example**: Mixer with IP3 = +10 dBm, input = -20 dBm
- IMD3 = 2(10 - (-20)) = 60 dBc below carrier

**Implication**: Strong interferers create in-band noise

---

### 6. Atmospheric Noise

**Natural sources**:
- **Lightning**: Dominates < 30 MHz (HF, VHF)
- **Cosmic noise**: Galactic background (0.1-3 GHz)
- **Solar noise**: Sun radiation (all frequencies)

**External noise temperature** $T_{\text{ext}}$:

| Frequency | $T_{\text{ext}}$ (K) | Dominant Source |
|-----------|----------------------|-----------------|
| 10 MHz | 10,000-100,000 | Lightning (HF) |
| 100 MHz | 1,000-10,000 | Galactic noise |
| 1 GHz | 10-100 | Cosmic background |
| 10 GHz | 3-30 | Sky temp (clear) |
| 60 GHz | 100-300 | Atmospheric O₂ |

**Antenna noise temp**:

$$
T_{\text{ant}} = T_{\text{ext}} \eta + T_0 (1 - \eta)
$$

Where $\eta$ = Antenna efficiency

---

### 7. Man-Made Noise

**Interference from**: Power lines, electric motors, computers, switching power supplies

**Impulsive noise**: Short bursts (microseconds) → High peak power

**Mitigation**: Filtering, shielding, time diversity (retransmission)

---

## Receiver Sensitivity Calculation

**Minimum detectable signal** for target SNR:

$$
P_{\text{min}} = N + \text{NF} + \text{SNR}_{\text{req}} + L_{\text{impl}}
$$

Where (all in dB):
- $N = -174 + 10\log_{10}(B)$ (thermal noise in bandwidth B)
- NF = Receiver noise figure (dB)
- SNR_req = Required SNR for demodulation (dB)
- $L_{\text{impl}}$ = Implementation loss (1-3 dB typical)

---

### Example: GPS Receiver

**Specs**:
- Bandwidth: 2 MHz (C/A code)
- NF: 3 dB (typical GPS front-end)
- SNR_req: -20 dB (spread spectrum processing gain 43 dB, need C/N₀ = 33 dB-Hz)
- Impl loss: 2 dB

**Thermal noise**:

$$
N = -174 + 10\log_{10}(2 \times 10^6) = -174 + 63 = -111\ \text{dBm}
$$

**Sensitivity**:

$$
P_{\text{min}} = -111 + 3 + (-20) + 2 = -126\ \text{dBm}
$$

**But wait!** GPS uses C/N₀ metric (per Hz):

**C/N₀** requirement: 33 dB-Hz (acquisition), 28 dB-Hz (tracking)

**Sensitivity** (alternate method):

$$
P_{\text{min}} = -174 + 33 + 3 + 2 = -136\ \text{dBm}
$$

**Typical GPS signal**: -130 dBm (open sky) → 6 dB margin

---

### Example: Satellite DVB-S2 Receiver

**Specs**:
- Bandwidth: 36 MHz
- NF: 1.5 dB (LNA at feed)
- Modulation: QPSK 3/4 (LDPC)
- SNR_req: 6.5 dB (for BER < 10⁻⁷ post-FEC)
- Impl loss: 1.5 dB

**Thermal noise**:

$$
N = -174 + 10\log_{10}(36 \times 10^6) = -174 + 75.6 = -98.4\ \text{dBm}
$$

**Sensitivity**:

$$
P_{\text{min}} = -98.4 + 1.5 + 6.5 + 1.5 = -88.9\ \text{dBm}
$$

**Link budget must deliver** > -88.9 dBm at LNB output for proper operation

---

## Noise Figure Measurement

### Y-Factor Method

**Standard technique** using hot/cold loads:

1. Measure noise power with **hot load** ($T_h = 290$ K): $P_h$
2. Measure noise power with **cold load** ($T_c = 77$ K, liquid N₂): $P_c$
3. Calculate **Y-factor**:

$$
Y = \frac{P_h}{P_c}
$$

4. **Noise figure**:

$$
\text{NF} = 10\log_{10}\left(\frac{T_h - YT_c}{290(Y-1)}\right)
$$

**Example**: $P_h = 100$ units, $P_c = 80$ units
- Y = 100/80 = 1.25
- NF = $10\log_{10}[(290 - 1.25 \times 77)/(290 \times 0.25)]$ = 1.8 dB

---

### Noise Source Method

**Use calibrated noise source** (ENR = Excess Noise Ratio in dB):

$$
\text{NF} = \text{ENR} - 10\log_{10}(Y - 1)
$$

Where Y = ratio of power with noise source ON/OFF

**Example**: ENR = 15 dB noise source, Y = 10
- NF = 15 - 10log(9) = 15 - 9.54 = 5.46 dB

---

## Design Guidelines

### Optimize Noise Figure

1. **LNA at antenna**: Minimize cable loss before LNA
2. **High LNA gain**: 15-20 dB isolates from later stages
3. **Low-loss transmission**: Use low-loss cable (LMR-400, hardline)
4. **Cool LNA**: Cryogenic cooling for satellite ground stations
5. **Avoid passive loss**: No attenuators, splitters before LNA

---

### Trade-offs

**Lower NF → Higher cost**:
- Premium LNA: 0.8 dB NF = $500+
- Standard LNA: 2 dB NF = $50
- Difference: 1.2 dB sensitivity = 1.3× range improvement

**Cryogenic cooling**:
- Cooled LNA: 0.3 dB NF @ 20 K
- Room temp LNA: 1.5 dB NF @ 290 K
- Difference: 1.2 dB (worth it for deep space, not for WiFi!)

---

## Summary Table

| Noise Source | Spectral Density | When Significant | Mitigation |
|--------------|------------------|------------------|------------|
| **Thermal** | $kT$ = -174 dBm/Hz | Always (fundamental) | Low NF, high gain |
| **Shot** | $\sqrt{2qI_{\text{DC}}B}$ | Low-light photodetectors | Increase optical power |
| **Flicker (1/f)** | $\propto 1/f$ | < 1 kHz | Higher IF, differential |
| **Phase** | $\mathcal{L}(f_m)$ | Near carrier | Better oscillator, PLL |
| **Quantization** | $-6n$ dB | Low SNR, few ADC bits | More bits, higher SNR |
| **IMD** | Nonlinear products | Strong interferers | Higher IP3, filtering |
| **Atmospheric** | Varies (10-100,000 K) | HF, low VHF | Directional antenna |
| **Man-made** | Impulsive/broadband | Urban, near power lines | Shielding, filtering |

---

## Related Topics

- **[[Signal-to-Noise-Ratio-(SNR)]]**: Determines BER performance
- **[[Complete-Link-Budget-Analysis]]**: Uses NF for sensitivity
- **[[Bit-Error-Rate-(BER)]]**: Degrades with noise
- **[[Energy-Ratios-(Es-N0-and-Eb-N0)]]**: Normalized SNR metrics
- **[[Antenna-Theory-Basics]]**: Antenna noise temperature
- **[[Free-Space-Path-Loss-(FSPL)]]**: Path loss + noise → Link budget

---

**Key takeaway**: **Noise limits receiver sensitivity.** Thermal noise floor = -174 dBm/Hz @ 290 K. Noise figure (NF) quantifies SNR degradation through receiver. Friis formula shows first stage (LNA) dominates total NF. Low-NF LNA at antenna, high gain, and minimal pre-LNA loss are critical. Sensitivity = Noise floor + NF + Required SNR. Lower noise = longer range, higher data rate, better reliability.

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
