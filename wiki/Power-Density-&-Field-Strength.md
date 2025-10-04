# Power Density & Field Strength

[[Home]] | **EM Fundamentals** | [[Maxwell's Equations & Wave Propagation]] | [[Wave Polarization]]

---

## Overview

**Power density** and **field strength** quantify the **intensity of electromagnetic radiation** at a given point in space.

**Key relationships**:
- **Field strength** (E, H) → Measured in V/m, A/m
- **Power density** (S) → Measured in W/m²
- **Relationship**: Power density proportional to E²

**Why it matters**:
- **Link budget calculations**: Determine received signal strength
- **Safety standards**: RF exposure limits (FCC, ICNIRP)
- **Antenna performance**: Radiated power distribution
- **Radar range equation**: Detection capability vs distance

---

## Electric Field Strength (E)

**Electric field** $\vec{E}$ describes the **force per unit charge** exerted on a test charge:

$$
\vec{E} = \frac{\vec{F}}{q} \quad (\text{V/m or N/C})
$$

**In electromagnetic wave** (plane wave, propagating in +z):

$$
E(z,t) = E_0 \cos(\omega t - kz + \phi)
$$

Where:
- $E_0$ = Peak electric field amplitude (V/m)
- Often use **RMS value**: $E_{\text{rms}} = E_0 / \sqrt{2}$

---

### Typical Values

| Source | Distance | E-field (V/m) |
|--------|----------|---------------|
| **AM broadcast** (50 kW) | 1 km | ~0.1 |
| **FM broadcast** (100 kW) | 1 km | ~0.2 |
| **Cell tower** (40 W ERP) | 100 m | ~1-2 |
| **WiFi router** (100 mW) | 1 m | ~3 |
| **Microwave oven** leak | 5 cm | ~10-50 (max allowed) |
| **Lightning** | Near channel | ~10⁶ |

---

## Magnetic Field Strength (H)

**Magnetic field** $\vec{H}$ describes the **magnetizing force**:

$$
\vec{H} = \frac{\vec{B}}{\mu} \quad (\text{A/m})
$$

Where:
- $\vec{B}$ = Magnetic flux density (Tesla)
- $\mu$ = Permeability (H/m)

**In free space**: $\mu = \mu_0 = 4\pi \times 10^{-7}$ H/m

---

### Relationship Between E and H (Far Field)

**In plane wave** (far from source), E and H are related by **wave impedance**:

$$
\frac{E}{H} = \eta_0 = \sqrt{\frac{\mu_0}{\epsilon_0}} \approx 377\ \Omega
$$

Where:
- $\eta_0$ = Impedance of free space ≈ 120π Ω ≈ 377 Ω
- $\epsilon_0$ = Permittivity of free space

**Practical form**:

$$
H = \frac{E}{377} \quad (\text{A/m})
$$

**Example**: E = 10 V/m → H = 10/377 ≈ 0.0265 A/m

---

### Near Field vs Far Field

#### Near Field (Reactive Near Field)

**Distance from antenna**: $r < 0.62\sqrt{D^3/\lambda}$ (for large antennas)

Or simpler: $r < \lambda/(2\pi)$ (for small antennas)

**Characteristics**:
- E and H not in simple ratio (reactive energy dominates)
- Energy oscillates between E-field and H-field storage
- Fields decay faster than $1/r$ (typically $1/r^2$ or $1/r^3$)

**Example**: HF antenna (3 MHz, λ = 100 m) at 10 m distance
- Near field: E/H ≠ 377 Ω
- Inductive or capacitive coupling dominates

---

#### Far Field (Radiating Far Field)

**Distance from antenna**: $r > 2D^2/\lambda$ (Fraunhofer distance)

Where D = Largest antenna dimension

**Characteristics**:
- E/H = 377 Ω (plane wave approximation valid)
- Radiation pattern independent of distance (shape constant)
- Fields decay as $1/r$ (power density as $1/r^2$)

**Example**: WiFi 2.4 GHz (λ = 12.5 cm), antenna size D = 5 cm

$$
r_{\text{far}} = \frac{2 \times (0.05)^2}{0.125} = 0.04\ \text{m} = 4\ \text{cm}
$$

**Far field begins at 4 cm** (very close for WiFi!)

---

## Power Density (Poynting Vector)

**Poynting vector** $\vec{S}$ represents **power flow per unit area**:

$$
\vec{S} = \vec{E} \times \vec{H} \quad (\text{W/m}^2)
$$

**Magnitude** (for plane wave with E ⊥ H):

$$
S = E \cdot H = \frac{E^2}{\eta_0} = \frac{E^2}{377}
$$

Or in terms of H:

$$
S = \eta_0 H^2 = 377 H^2
$$

---

### Time-Averaged Power Density

**For sinusoidal wave**, instantaneous power oscillates at 2f. Use **time-average**:

$$
\langle S \rangle = \frac{1}{2} \frac{E_0^2}{\eta_0} = \frac{E_{\text{rms}}^2}{\eta_0} = \frac{E_{\text{rms}}^2}{377}
$$

**Example**: E_rms = 10 V/m

$$
\langle S \rangle = \frac{100}{377} \approx 0.265\ \text{W/m}^2
$$

---

## Power Density from Isotropic Source

**Isotropic radiator** distributes power uniformly over sphere:

$$
S = \frac{P_t}{4\pi r^2}
$$

Where:
- $P_t$ = Transmitted power (W)
- $r$ = Distance from source (m)
- $4\pi r^2$ = Surface area of sphere

**Inverse square law**: Power density decreases as $1/r^2$

**Example**: 100 W isotropic source at 10 m

$$
S = \frac{100}{4\pi (10)^2} = \frac{100}{1257} \approx 0.0796\ \text{W/m}^2
$$

---

## Power Density from Directional Antenna

**Antenna with gain** G concentrates power:

$$
S = \frac{P_t \cdot G}{4\pi r^2}
$$

**Effective Isotropic Radiated Power (EIRP)**:

$$
\text{EIRP} = P_t \cdot G
$$

**Power density becomes**:

$$
S = \frac{\text{EIRP}}{4\pi r^2}
$$

---

### Example: WiFi Router

**Specs**:
- Transmit power: 100 mW = 0.1 W
- Antenna gain: 2 dBi (linear gain ≈ 1.58)
- Distance: 10 m

**EIRP**:

$$
\text{EIRP} = 0.1 \times 1.58 = 0.158\ \text{W}
$$

**Power density at 10 m**:

$$
S = \frac{0.158}{4\pi (10)^2} = \frac{0.158}{1257} \approx 0.000126\ \text{W/m}^2 = 0.126\ \text{mW/m}^2
$$

**Convert to E-field**:

$$
E_{\text{rms}} = \sqrt{S \times 377} = \sqrt{0.000126 \times 377} \approx 0.218\ \text{V/m}
$$

---

## Relationship Between Power Density and E-field

**Summary formulas** (far field, plane wave):

$$
S = \frac{E_{\text{rms}}^2}{377} \quad (\text{W/m}^2)
$$

$$
E_{\text{rms}} = \sqrt{377 \times S} \approx 19.4\sqrt{S} \quad (\text{V/m})
$$

$$
E_0 = \sqrt{2} \times E_{\text{rms}} = \sqrt{2 \times 377 \times S} \approx 27.5\sqrt{S}
$$

---

### Quick Conversion Table

| Power Density (W/m²) | E_rms (V/m) | E_peak (V/m) |
|----------------------|-------------|--------------|
| 0.001 (1 mW/m²) | 0.61 | 0.87 |
| 0.01 (10 mW/m²) | 1.94 | 2.75 |
| 0.1 | 6.14 | 8.68 |
| 1 | 19.4 | 27.5 |
| 10 | 61.4 | 86.8 |
| 100 | 194 | 275 |

---

## Power Delivered to Receiving Antenna

**Effective aperture** $A_e$ captures power from incident wave:

$$
P_r = S \cdot A_e
$$

Where:

$$
A_e = \frac{G_r \lambda^2}{4\pi}
$$

- $G_r$ = Receive antenna gain (linear)
- $\lambda$ = Wavelength

**Combining**:

$$
P_r = \frac{P_t G_t G_r \lambda^2}{(4\pi r)^2}
$$

**This is the Friis transmission equation** (see [[Free-Space Path Loss (FSPL)]])

---

### Example: Satellite Downlink

**Specs**:
- Satellite EIRP: 50 dBW = 100 kW
- Frequency: 12 GHz (λ = 0.025 m)
- Distance: 36,000 km (GEO)
- RX antenna gain: 40 dBi (10,000 linear)

**Power density at ground**:

$$
S = \frac{10^5}{4\pi (3.6 \times 10^7)^2} = \frac{10^5}{1.63 \times 10^{16}} \approx 6.1 \times 10^{-12}\ \text{W/m}^2
$$

**E-field**:

$$
E_{\text{rms}} = \sqrt{377 \times 6.1 \times 10^{-12}} \approx 1.5 \times 10^{-3}\ \text{V/m} = 1.5\ \text{mV/m}
$$

**Received power** (1 m² dish, A_e ≈ 0.5 m²):

$$
P_r = 6.1 \times 10^{-12} \times 0.5 \approx 3 \times 10^{-12}\ \text{W} = 3\ \text{pW}
$$

**In dBm**: $10\log_{10}(3 \times 10^{-12} / 10^{-3}) = -115$ dBm

**Using Friis equation**:

$$
P_r = \frac{100,000 \times 10,000 \times (0.025)^2}{(4\pi \times 3.6 \times 10^7)^2} \approx 3 \times 10^{-12}\ \text{W}
$$

**Consistent!**

---

## RF Safety Standards

**Exposure limits** protect against thermal and non-thermal effects:

### FCC Limits (USA)

**Occupational/Controlled Exposure** (aware workers):

| Frequency | E-field (V/m) | H-field (A/m) | Power Density (W/m²) |
|-----------|---------------|---------------|----------------------|
| 0.3-3 MHz | 614 | 1.63 | - |
| 3-30 MHz | 1842/f | 4.89/f | - |
| 30-300 MHz | 61.4 | 0.163 | 1.0 |
| 300-1500 MHz | - | - | f/300 |
| 1500-100,000 MHz | - | - | 5.0 |

Where f is in MHz

**General Population/Uncontrolled Exposure** (public):

Limits are **5× lower** (e.g., 0.2 W/m² @ 30-300 MHz)

---

### ICNIRP Limits (International)

**General Public** (6-minute average):

| Frequency | E-field (V/m) | Power Density (W/m²) |
|-----------|---------------|----------------------|
| 10-400 MHz | 28 | 2 |
| 400-2000 MHz | 1.375√f | f/200 |
| 2-300 GHz | 61 | 10 |

Where f is in MHz

---

### Example: WiFi Router Compliance

**WiFi 2.4 GHz, 100 mW, gain 2 dBi**

**At 20 cm** (typical human distance):

$$
S = \frac{0.1 \times 1.58}{4\pi (0.2)^2} = \frac{0.158}{0.503} \approx 0.314\ \text{W/m}^2
$$

**FCC limit @ 2.4 GHz**: 5 W/m² (controlled), 1 W/m² (uncontrolled)

**ICNIRP limit**: f/200 = 2400/200 = 12 W/m²

**Result**: WiFi at 20 cm = 0.314 W/m² **< 1 W/m²** (OK for public exposure, but close!)

**At 1 m**: $S = 0.0126$ W/m² (much safer)

---

## Radar Power Budget

**Radar equation** relates transmitted power to received echo:

$$
P_r = \frac{P_t G^2 \lambda^2 \sigma}{(4\pi)^3 R^4}
$$

Where:
- $\sigma$ = Target radar cross-section (m²)
- R = Range to target (m)

**Power density at target**:

$$
S_{\text{target}} = \frac{P_t G}{4\pi R^2}
$$

**Reflected power density back at radar**:

$$
S_{\text{return}} = \frac{S_{\text{target}} \cdot \sigma}{4\pi R^2} = \frac{P_t G \sigma}{(4\pi)^2 R^4}
$$

**Notice**: $1/R^4$ dependence (power travels to target and back)

---

### Example: Weather Radar

**Specs**:
- Transmit power: 1 MW (peak)
- Antenna gain: 45 dBi (≈ 31,600 linear)
- Frequency: 3 GHz (λ = 0.1 m)
- Target: Raindrop, σ = 10⁻⁶ m² (light rain)
- Range: 100 km

**Power density at raindrop**:

$$
S_{\text{target}} = \frac{10^6 \times 31,600}{4\pi (10^5)^2} = \frac{3.16 \times 10^{10}}{1.26 \times 10^{11}} \approx 0.25\ \text{W/m}^2
$$

**Received power**:

$$
P_r = \frac{10^6 \times (31,600)^2 \times (0.1)^2 \times 10^{-6}}{(4\pi)^3 (10^5)^4} \approx 1.6 \times 10^{-13}\ \text{W} = -98\ \text{dBm}
$$

**Weak but detectable** with sensitive receiver (noise floor ~ -110 dBm)

---

## Electromagnetic Interference (EMI)

**Field strength limits** for conducted and radiated emissions:

### FCC Part 15 Radiated Emission Limits

**Class B** (residential):

| Frequency | E-field @ 3 m (μV/m) | dBμV/m |
|-----------|----------------------|--------|
| 30-88 MHz | 100 | 40 |
| 88-216 MHz | 150 | 43.5 |
| 216-960 MHz | 200 | 46 |
| Above 960 MHz | 500 | 54 |

**Measurement**: Use calibrated antenna + spectrum analyzer

---

### Example: Spurious Emission Check

**Digital device @ 300 MHz, measured 180 μV/m @ 3 m**

**Limit @ 300 MHz**: 200 μV/m

**Result**: 180 < 200 → **Pass**

**Margin**: $20\log_{10}(200/180) = 0.9$ dB

---

## Field Strength in Different Media

**In dielectric medium** (not free space):

$$
\eta = \sqrt{\frac{\mu}{\epsilon}} = \frac{\eta_0}{\sqrt{\epsilon_r}}
$$

Where:
- $\epsilon_r$ = Relative permittivity
- $\eta_0 = 377$ Ω (free space)

**Example**: Water ($\epsilon_r \approx 80$ @ low freq)

$$
\eta_{\text{water}} = \frac{377}{\sqrt{80}} \approx 42\ \Omega
$$

**Power density for same E-field**:

$$
S = \frac{E^2}{42}
$$

**9× higher power density** than free space (for same E-field)

**Implication**: Underwater communications have different impedance matching requirements

---

## Antenna Gain and Directivity

**Gain** increases power density in preferred direction:

$$
G = \eta_{\text{ant}} \cdot D
$$

Where:
- $\eta_{\text{ant}}$ = Antenna efficiency (0-1)
- D = Directivity (ratio of max to average power density)

**Directivity**:

$$
D = \frac{S_{\text{max}}}{S_{\text{avg}}} = \frac{4\pi S_{\text{max}} r^2}{P_t}
$$

**Example**: Isotropic antenna
- $D = 1$ (0 dBi)
- Power uniformly distributed

**Half-wave dipole**:
- $D = 1.64$ (2.15 dBi)
- Power concentrated in broadside direction

**Parabolic dish** (diameter D, wavelength λ):

$$
G \approx \eta_{\text{ant}} \left(\frac{\pi D}{\lambda}\right)^2
$$

With $\eta_{\text{ant}} \approx 0.5-0.7$ (typical)

---

## Skin Depth and Field Penetration

**In conductors**, field decays exponentially:

$$
E(z) = E_0 e^{-z/\delta}
$$

**Skin depth** $\delta$:

$$
\delta = \sqrt{\frac{2}{\omega \mu \sigma}} = \sqrt{\frac{1}{\pi f \mu \sigma}}
$$

Where:
- $\sigma$ = Conductivity (S/m)
- Copper: $\sigma = 5.8 \times 10^7$ S/m

**Example**: Copper @ 1 GHz

$$
\delta = \sqrt{\frac{1}{\pi \times 10^9 \times 4\pi \times 10^{-7} \times 5.8 \times 10^7}} \approx 2.1\ \mu\text{m}
$$

**Implication**: At microwave frequencies, current flows in thin surface layer (< 2 μm)

---

## Summary Table

| Quantity | Symbol | Units | Typical Range | Relationship |
|----------|--------|-------|---------------|--------------|
| **Electric field** | E | V/m | 0.01-1000 | $E = \sqrt{377 \times S}$ |
| **Magnetic field** | H | A/m | 0.00003-3 | $H = E/377$ |
| **Power density** | S | W/m² | 10⁻⁶ - 10 | $S = E^2/377$ |
| **Transmitted power** | $P_t$ | W | 0.001-100,000 | $S = P_t G / (4\pi r^2)$ |
| **Distance** | r | m | 0.01-10⁸ | $S \propto 1/r^2$ |
| **Antenna gain** | G | - | 1-10⁶ | $S \propto G$ |

---

## Related Topics

- **[[Free-Space Path Loss (FSPL)]]**: Uses power density to derive path loss
- **[[Antenna Theory Basics]]**: Gain, effective aperture, directivity
- **[[Maxwell's Equations & Wave Propagation]]**: E and H field derivation
- **[[Signal-to-Noise Ratio (SNR)]]**: Received power from power density
- **[[Weather Effects (Rain Fade, Fog Attenuation)]]**: Power density reduction mechanisms

---

**Key takeaway**: **Power density S = E²/377 in far field**. Follows inverse square law ($1/r^2$) from isotropic source. Directional antennas concentrate power (multiply by gain). E-field strength and power density determine link performance and safety compliance. Far field (E/H = 377 Ω) begins at $2D^2/\lambda$ from antenna. Safety limits typically 0.2-10 W/m² depending on frequency and exposure type.

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
