# Free-Space Path Loss (FSPL)

## 🛰️ For Non-Technical Readers

**Like shouting across a field—the farther away, the quieter. Radio waves spread out and weaken with distance.**

**Key insights**:
- **Double the distance** = signal becomes **4× weaker**
- **Higher frequency** (5G) = weaker than lower frequency (4G) over same distance
- **Satellites 36,000 km away**: Signal weakens by 10 trillion trillion times! (That's why dishes are big)

**Real examples**: WiFi weakens 10,000× over 50 meters. Cell towers need to be closer for 5G than 4G.

---

**Free-Space Path Loss (FSPL)** quantifies how much signal power is lost as an electromagnetic wave propagates through free space.

---

## 📐 The Friis Transmission Equation

**Fundamental equation** linking transmitter and receiver:

```
P_R = P_T · G_T · G_R · (λ/4πd)²

where:
- P_R = received power (W)
- P_T = transmitted power (W)
- G_T = transmit antenna gain (linear, dimensionless)
- G_R = receive antenna gain (linear, dimensionless)
- λ = wavelength (m)
- d = distance between antennas (m)
```

**Assumptions**:
- Free space (no obstacles, no atmosphere)
- Far-field (d >> antenna dimensions)
- Polarization matched
- Antennas aligned

---

## 📊 Path Loss Definition

**Path Loss (L)** is the ratio of transmitted to received power:

```
L = P_T/P_R

In dB:
L_dB = 10 log₁₀(L) = 10 log₁₀(P_T/P_R)
```

From Friis equation (assuming isotropic antennas, G_T = G_R = 1):

```
L = (4πd/λ)²

In dB:
FSPL_dB = 20 log₁₀(4πd/λ)
        = 20 log₁₀(d) + 20 log₁₀(f) + 20 log₁₀(4π/c)
        = 20 log₁₀(d) + 20 log₁₀(f) - 147.55
```

**More practical form** (f in Hz, d in m):

```
FSPL_dB = 20 log₁₀(d) + 20 log₁₀(f) + 92.45
```

**Or** (f in MHz, d in km):

```
FSPL_dB = 20 log₁₀(d_km) + 20 log₁₀(f_MHz) + 32.45
```

---

## 🧮 Example Calculations

### Example 1: WiFi (2.4 GHz, 10 m)

```
f = 2.4 GHz = 2.4×10⁹ Hz
d = 10 m

FSPL = 20 log₁₀(10) + 20 log₁₀(2.4×10⁹) + 92.45
     = 20 + 187.6 + 92.45
     = 100 dB
```

**Interpretation**: Signal power drops by factor of 10¹⁰ (10 billion) over 10 m!

---

### Example 2: Cell Phone (900 MHz, 1 km)

```
f = 900 MHz
d = 1 km = 1000 m

FSPL = 20 log₁₀(1000) + 20 log₁₀(900×10⁶) + 92.45
     = 60 + 179 + 92.45
     = 131.5 dB
```

---

### Example 3: Satellite (12 GHz, 36,000 km - GEO)

```
f = 12 GHz
d = 36,000 km = 3.6×10⁷ m

FSPL = 20 log₁₀(3.6×10⁷) + 20 log₁₀(12×10⁹) + 92.45
     = 151 + 201.6 + 92.45
     = 205 dB
```

**Massive loss!** Requires high TX power + high-gain antennas.

---

### Example 4: THz Link (1 THz, 10 m) - For [[AID Protocol Case Study]]

```
f = 1 THz = 1×10¹² Hz
d = 10 m

FSPL = 20 log₁₀(10) + 20 log₁₀(1×10¹²) + 92.45
     = 20 + 240 + 92.45
     = 352.5 dB
```

**Extreme loss!** This is why [[Terahertz (THz) Technology|THz communications]] are short-range only.

---

## 📈 Scaling Laws

### Distance Dependence

```
FSPL ∝ d²  (power law)

In dB: FSPL_dB increases by 20 dB per decade of distance

Examples:
- 1 m → 10 m: +20 dB loss
- 10 m → 100 m: +20 dB loss
- 100 m → 1 km: +20 dB loss
```

**Doubling distance**: +6 dB loss (power drops to 1/4)

---

### Frequency Dependence

```
FSPL ∝ f²  (power law)

In dB: FSPL_dB increases by 20 dB per decade of frequency

Examples:
- 100 MHz → 1 GHz: +20 dB loss
- 1 GHz → 10 GHz: +20 dB loss
- 10 GHz → 100 GHz: +20 dB loss
```

**Doubling frequency**: +6 dB loss (higher frequencies lose more power!)

**Why?** Effective aperture of receiving antenna ∝ λ² (smaller at higher f)

---

## 🎯 Physical Interpretation

### Not True "Loss"

FSPL is **NOT** energy dissipation (free space is lossless!). It's **geometric spreading**:

```
Transmit antenna radiates P_T into sphere
Surface area: A = 4πd²
Power density at distance d:

S = P_T/(4πd²)  (W/m²)

Received power:
P_R = S · A_eff

where A_eff = G_R λ²/4π (effective area of RX antenna)

Result:
P_R = P_T · G_T · G_R · (λ/4πd)²  (Friis equation!)
```

**Analogy**: Flashlight beam spreads out → same total power, but lower intensity at greater distance.

---

## 📡 Link Budget Analysis

**Link Budget** accounts for all gains and losses:

```
P_R [dBm] = P_T [dBm] + G_T [dBi] + G_R [dBi] - FSPL [dB] - L_other [dB]

where:
- P_T = transmit power (dBm, referenced to 1 mW)
- G_T, G_R = antenna gains (dBi, referenced to isotropic)
- FSPL = free-space path loss (dB)
- L_other = other losses (cables, connectors, atmosphere, etc.)
```

**Goal**: Ensure P_R >> P_noise (receiver noise floor) for reliable communication.

---

### Example: WiFi Link Budget (2.4 GHz, 50 m)

```
Transmitter:
- TX power: +20 dBm (100 mW, typical WiFi)
- TX antenna gain: +2 dBi (dipole)
- EIRP: 22 dBm

Channel:
- Distance: 50 m
- FSPL: 20log(50) + 20log(2400) + 32.45 = 34 + 67.6 + 32.45 = 134 dB
- Indoor losses (walls, furniture): ~15 dB
- Total loss: 149 dB

Receiver:
- RX antenna gain: +2 dBi
- Cable loss: -1 dB
- Net RX gain: +1 dB

Received power:
P_R = 22 + 1 - 149 = -126 dBm

Noise floor (10 MHz bandwidth, 300K):
N = -174 + 10log(10^7) = -174 + 70 = -104 dBm

SNR = P_R - N = -126 - (-104) = -22 dB
```

**Too low!** WiFi needs ~-65 dBm minimum. This link would fail.

**Solution**: Reduce distance, add amplifiers, or use directional antennas.

---

## 🌍 Real-World Deviations

### FSPL Assumes Free Space

**Reality**:
- Atmosphere absorbs (especially water vapor at mmWave/THz)
- Obstacles block (buildings, trees, terrain)
- Ground reflections create multipath
- Weather attenuates (rain, fog)

**Actual path loss** > FSPL

---

### Frequency-Specific Effects

**Low Frequencies (< 30 MHz)**:
- Ground wave propagation
- Ionospheric reflection
- **Can exceed FSPL predictions** (longer range!)

**Mid Frequencies (30 MHz - 3 GHz)**:
- Mostly line-of-sight (LOS)
- FSPL + diffraction
- Close to FSPL predictions

**High Frequencies (> 3 GHz)**:
- Atmospheric absorption becomes significant
- Rain fade (especially > 10 GHz)
- **Path loss > FSPL**

**THz (> 300 GHz)**:
- Extreme atmospheric absorption
- Water vapor resonances
- **Path loss >> FSPL** (can be +100 dB extra!)

---

## 🔬 Measurement vs. Prediction

### Received Power Measurement

```
Measured:  P_R,meas
Predicted: P_R,pred (from Friis equation)

Path loss exponent n:
P_R ∝ d^(-n)

Free space: n = 2
Urban: n = 3-4
Indoor: n = 4-6
```

**Empirical models** (e.g., Okumura-Hata, COST 231) fit measured data to more complex formulas.

---

## 🎓 Key Takeaways

1. **FSPL ∝ d² · f²**: Geometric spreading, worse at high frequencies
2. **20 dB per decade**: Doubling d or f adds 6 dB loss
3. **Not energy loss**: Power spreads out, doesn't vanish
4. **Baseline for link budgets**: Real losses are usually higher
5. **Frequency trade-off**: Higher f → more bandwidth but more path loss
6. **THz communications**: FSPL alone is ~350 dB at 10 m, 1 THz!

---

## 🔗 See Also

- [[Maxwell's Equations & Wave Propagation]] - Theoretical foundation
- [[Antenna Theory Basics]] - Antenna gain (G_T, G_R)
- [[Link Loss vs Noise]] - FSPL vs additive noise
- [[Atmospheric Effects]] - Additional losses beyond FSPL *(coming soon)*
- [[Multipath Propagation & Fading]] - Deviations from FSPL *(coming soon)*
- [[Terahertz (THz) Technology]] - Extreme FSPL regime

---

## 📚 References

1. **Friis, H.T.** (1946) "A note on a simple transmission formula" *Proc. IRE* 34, 254-256
2. **Rappaport, T.S.** (2002) *Wireless Communications: Principles and Practice* 2nd ed. (Prentice Hall)
3. **Goldsmith, A.** (2005) *Wireless Communications* (Cambridge UP)
4. **ITU-R P.525** (2019) "Calculation of free-space attenuation"
