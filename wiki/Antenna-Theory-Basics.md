# Antenna Theory Basics

[[Home]] | **Foundation** | [[Electromagnetic-Spectrum]] | [[Maxwell's-Equations-&-Wave-Propagation]]

---

## 📡 For Non-Technical Readers

**An antenna is like a funnel for radio waves—it concentrates energy in one direction (transmit) or collects it from many directions (receive).**

**Simple analogies**:
- **Flashlight vs. bare bulb**: A flashlight (directional antenna) focuses light. A bare bulb (omnidirectional) lights up everything.
- **Satellite dish**: Curved shape collects weak space signals and focuses them onto a tiny receiver
- **Your cell phone**: Has multiple tiny antennas inside—cellular, WiFi, GPS, Bluetooth (each tuned to different frequencies)

**Key insights**:
- **Bigger = stronger**: 10-meter dish collects 100× more energy than 1-meter dish
- **Shape matters**: Long wire for AM radio, small stub for WiFi, dish for satellites
- **Trade-off**: Omnidirectional (WiFi router) covers whole area but weak. Directional (satellite dish) is strong but must point exactly right.

---

## Overview

An **antenna** is a transducer that converts **electrical signals into electromagnetic waves** (transmit) and vice versa (receive). Antennas are governed by **reciprocity**: their transmit and receive properties are identical.

**Fundamental principle**: Accelerating charges radiate EM energy ([[Maxwell's-Equations-&-Wave-Propagation]]).

---

## Key Antenna Parameters

### 1. Radiation Pattern

**The spatial distribution of radiated power**.

**Coordinate system**:
- **Azimuth (φ)**: Horizontal angle (0° - 360°)
- **Elevation (θ)**: Vertical angle from zenith (0° = straight up)

**Typical patterns**:

#### Isotropic Radiator (Theoretical)
- Radiates **equally in all directions** (sphere)
- **Power density** at distance $r$:

$$
S = \frac{P_t}{4\pi r^2}
$$

- **Does not exist in reality** (used as reference for gain)

---

#### Dipole (λ/2)

**Classic antenna**: Half-wavelength wire

**Pattern**:
- **Omnidirectional in azimuth** (φ)
- **Figure-8 in elevation** (θ): Nulls along wire axis

**3D pattern**: Donut-shaped (toroid)

**Radiation resistance**: $R_r = 73\ \Omega$ (lossless)

---

#### Directional Antennas

**Yagi-Uda** (TV antenna):
- Single driven element (dipole)
- Parasitic elements (directors + reflector)
- **Gain**: 10-15 dBi
- **Beamwidth**: ~30-60°

**Parabolic Dish**:
- Large aperture (diameter $D \gg \lambda$)
- **Gain**: 30-60 dBi (satellite comms)
- **Beamwidth**: $\theta \approx 70 \lambda / D$ degrees

**Phased Array**:
- Multiple elements with controllable phase
- **Electronically steerable** beam (no mechanical movement)
- Used in: Radar, 5G base stations, [[AID Protocol|AID Protocol]] (THz)

---

### 2. Antenna Gain (G)

**Ratio of power density in preferred direction vs isotropic radiator**.

$$
G = \frac{S(\theta, \phi)}{S_{\text{iso}}}
$$

**Units**: dBi (dB relative to isotropic)

**Typical gains**:

| Antenna Type | Gain (dBi) | Beamwidth |
|--------------|------------|-----------|
| Isotropic (reference) | 0 dBi | 360° (all directions) |
| Dipole (λ/2) | 2.15 dBi | ~78° (elevation) |
| Monopole (λ/4) | 5.15 dBi | ~30° (over ground plane) |
| Patch (microstrip) | 6-9 dBi | ~70-90° |
| Yagi (10 elements) | 12-15 dBi | ~30° |
| Parabolic dish (1 m @ 10 GHz) | ~40 dBi | ~2° |
| Phased array (64 elements) | 18-24 dBi | Steerable |

**Relationship to directivity**:

$$
G = \eta_{\text{ant}} \cdot D
$$

Where:
- $D$ = Directivity (concentrates power)
- $\eta_{\text{ant}}$ = Antenna efficiency (0.5-0.95 typical, accounts for ohmic losses)

---

### 3. Directivity (D)

**Power concentration factor** (independent of losses):

$$
D = \frac{4\pi}{\Omega_A}
$$

Where $\Omega_A$ is the **solid angle** of the main lobe (steradians).

**Approximation** for narrow beams:

$$
D \approx \frac{41,253}{\theta_E \cdot \theta_H}
$$

Where:
- $\theta_E$ = Elevation beamwidth (degrees)
- $\theta_H$ = Azimuth beamwidth (degrees)

**Example**: Beamwidth 10° × 10° → $D = 41,253 / (10 \times 10) = 412.53 \approx 26.2$ dBi

---

### 4. Beamwidth

**Angular width where power drops to half (-3 dB) of peak**.

**Half-power beamwidth (HPBW)**:

$$
\theta_{\text{HPBW}} \approx \frac{k \lambda}{D}
$$

Where:
- $D$ = Antenna diameter (aperture antennas)
- $k$ = Constant (~70° for parabolic dishes)

**Example**: 1 m dish at 10 GHz ($\lambda = 3$ cm):

$$
\theta_{\text{HPBW}} = \frac{70 \times 0.03}{1} = 2.1°
$$

**Implication**: Narrow beams require **precise pointing** (satellites, radar)

---

### 5. Polarization

**Orientation of electric field vector**.

#### Linear Polarization
- **Vertical**: E-field parallel to ground (monopole, vertical dipole)
- **Horizontal**: E-field perpendicular to ground (horizontal dipole)

**Cross-polarization loss**: 20-30 dB if TX and RX polarizations are perpendicular

---

#### Circular Polarization
- **E-field rotates** as wave propagates
- **Right-hand circular (RHCP)**: Clockwise (looking at source)
- **Left-hand circular (LHCP)**: Counter-clockwise

**Applications**: GPS, satellite comms (immune to Faraday rotation in ionosphere)

**Axial ratio**: Measure of circularity (0 dB = perfect circular, >3 dB = elliptical)

---

#### Elliptical Polarization
- General case (between linear and circular)
- Common when reflection/scattering depolarizes signal

---

### 6. Impedance & Matching

**Antenna input impedance**:

$$
Z_{\text{ant}} = R_{\text{rad}} + R_{\text{loss}} + jX
$$

Where:
- $R_{\text{rad}}$ = Radiation resistance (power radiated)
- $R_{\text{loss}}$ = Loss resistance (heat in conductors/dielectrics)
- $X$ = Reactance (energy storage in near-field)

**Goal**: Match to transmission line (typically 50Ω or 75Ω)

---

#### Standing Wave Ratio (SWR)

**Mismatch metric**:

$$
\text{SWR} = \frac{1 + |\Gamma|}{1 - |\Gamma|}
$$

Where $\Gamma = \frac{Z_{\text{ant}} - Z_0}{Z_{\text{ant}} + Z_0}$ (reflection coefficient)

**Acceptable values**:
- SWR < 1.5:1 → Good match (< 4% power reflected)
- SWR = 2:1 → Marginal (11% reflected)
- SWR > 3:1 → Poor (25% reflected, may damage TX)

**Measurement**: Antenna analyzer, network analyzer, SWR meter

---

### 7. Bandwidth

**Frequency range where antenna performs adequately**.

**Criteria**:
- SWR < 2:1
- Gain variation < 3 dB
- Pattern distortion minimal

**Narrowband antennas**: Dipole (2-5%), loop (1-2%)
**Wideband antennas**: Log-periodic (10:1 ratio), biconical (octave), spiral (decade+)

**Example**: WiFi 2.4 GHz (2.4-2.5 GHz = 4% bandwidth) → Simple patch works
**Example**: UWB radar (3-10 GHz = 107% fractional BW) → Needs spiral or horn

---

### 8. Effective Aperture ($A_e$)

**Equivalent capture area** for receiving antennas:

$$
A_e = \frac{G \lambda^2}{4\pi}
$$

**Physical interpretation**: Power received = Incident power density × $A_e$

$$
P_r = S \cdot A_e
$$

**Example**: Dipole ($G = 2.15$ dBi = 1.64 linear) at 1 GHz ($\lambda = 0.3$ m):

$$
A_e = \frac{1.64 \times (0.3)^2}{4\pi} = 0.0125\ \text{m}^2
$$

**Aperture efficiency**: $\eta_{\text{ap}} = A_e / A_{\text{phys}}$ (0.5-0.7 for dishes)

---

## Antenna Types by Application

### 1. Communication Antennas

#### Dipole (VHF/UHF)
- **Simple, cheap, omnidirectional**
- **Use**: FM broadcast, amateur radio, WiFi (2.4 GHz diversity antennas)

#### Patch (Microstrip)
- **Flat, low-profile, easy to integrate**
- **Use**: GPS, cellular, WiFi (5 GHz), IoT devices

#### Yagi-Uda
- **Directional, moderate gain**
- **Use**: TV reception, point-to-point links, amateur radio

#### Parabolic Dish
- **High gain, narrow beam**
- **Use**: Satellite TV (12 GHz), deep-space comms (Ka-band), radio astronomy

---

### 2. Mobile/Wearable Antennas

#### Monopole (λ/4)
- **Requires ground plane** (vehicle roof, PCB)
- **Use**: Car antennas, handheld radios

#### PIFA (Planar Inverted-F Antenna)
- **Compact, dual-band**
- **Use**: Smartphones (cellular + WiFi)

#### Loop Antenna
- **Small, magnetic field dominant**
- **Use**: RFID tags, NFC, AM radio (ferrite bar)

---

### 3. Phased Arrays

**Multiple elements with controllable phase/amplitude**:

**Advantages**:
- **Electronic beam steering** (no moving parts)
- **Adaptive nulling** (cancel interference)
- **MIMO** (spatial multiplexing)

**Beam steering**:

$$
\theta = \sin^{-1}\left(\frac{\phi \lambda}{2\pi d}\right)
$$

Where:
- $\phi$ = Phase shift between elements
- $d$ = Element spacing

**Applications**:
- Radar (military, automotive 77 GHz)
- 5G base stations (massive MIMO, 64-256 elements)
- [[AID Protocol|AID Protocol]] (THz phased array for coherent combining)

---

## Friis Transmission Equation

**Link budget fundamental** (connects antennas to [[Free-Space-Path-Loss-(FSPL)]]):

$$
P_r = P_t + G_t + G_r - L_{\text{FSPL}}
$$

(in dB)

Or in linear form:

$$
P_r = P_t \cdot G_t \cdot G_r \cdot \left(\frac{\lambda}{4\pi d}\right)^2
$$

**Derivation**:

1. TX power $P_t$ radiated isotropically → Power density at distance $d$:

$$
S_{\text{iso}} = \frac{P_t}{4\pi d^2}
$$

2. TX antenna gain $G_t$ concentrates power:

$$
S = \frac{P_t G_t}{4\pi d^2}
$$

3. RX antenna effective aperture $A_e = G_r \lambda^2 / 4\pi$ captures power:

$$
P_r = S \cdot A_e = \frac{P_t G_t}{4\pi d^2} \cdot \frac{G_r \lambda^2}{4\pi}
$$

4. Simplify:

$$
P_r = P_t G_t G_r \left(\frac{\lambda}{4\pi d}\right)^2
$$

**Key insight**: Antenna gain **adds** to link budget (in dB), compensating for path loss.

---

## Antenna Design by Frequency

### VLF/LF (< 300 kHz)

**Challenge**: Wavelength >> practical antenna size

**Solution**:
- **Electrically small antennas** (length $\ll \lambda$)
- **Low efficiency** (most power lost in ohmic resistance)
- **Loading coils** to resonate (match reactance)

**Example**: 100 kHz (λ = 3000 m), 10 m vertical monopole:
- Radiation resistance: ~0.1 Ω
- Loss resistance: ~10 Ω
- Efficiency: ~1%

---

### HF/VHF (3-300 MHz)

**Sweet spot**: Antennas are practical size

**Common types**:
- Dipole (λ/2): 50 m @ 3 MHz, 1 m @ 150 MHz
- Monopole (λ/4): 25 m @ 3 MHz (vertical tower)
- Yagi-Uda: TV reception (VHF channels)

**Efficiency**: 50-90% (good conductors, minimal loss)

---

### UHF/SHF (300 MHz - 30 GHz)

**Miniaturization**: Antennas fit on PCBs

**Common types**:
- Patch (microstrip): 3 cm × 3 cm @ 2.4 GHz
- Slot: Waveguide-based (radar, satellite)
- Horn: Wideband, calibration standard

**Phased arrays become feasible**: Element spacing $d \sim \lambda/2$

**Example**: 10 GHz, $\lambda = 3$ cm → 1.5 cm spacing → 100 elements in 15 cm × 15 cm

---

### EHF/THz (30 GHz - 10 THz)

**Challenges**:
- **Fabrication tolerance** (μm precision required)
- **Surface roughness losses** (skin depth at THz ~ nm)
- **Impedance matching** difficult (high frequencies)

**Solutions**:
- **On-chip antennas** (silicon, III-V semiconductors)
- **Photolithography** (THz: <100 μm features)
- **Lens-coupled antennas** (match impedance to free space)

**Example**: 1.875 THz (AID protocol), $\lambda = 160$ μm:
- Dipole: 80 μm (fabricated via e-beam lithography)
- Phased array: 40 μm spacing, 1024 elements in 40 mm × 40 mm

---

## Antenna Measurements

### Anechoic Chamber

**Facility for measuring radiation patterns**:

- **Absorber walls**: Eliminate reflections (simulate free space)
- **Turntable**: Rotate antenna under test (AUT)
- **Reference antenna**: Known gain/pattern
- **Network analyzer**: Measure S₂₁ (transmission) vs angle

**Far-field distance**: $d > 2D^2/\lambda$ (Fraunhofer region)

**Example**: 1 m dish @ 10 GHz → $d > 2 \times 1^2 / 0.03 = 67$ m (large chamber!)

---

### Near-Field Scanning

**For electrically large antennas** (where far-field distance is impractical):

1. **Scan E/H fields** on planar/cylindrical/spherical surface near antenna
2. **FFT transform** to compute far-field pattern
3. **Smaller chamber** required (1-2 m)

---

### Gain Measurement (Comparison Method)

1. Measure received power with **standard gain horn** (calibrated)
2. Replace with **antenna under test** (AUT)
3. Compare powers:

$$
G_{\text{AUT}} = G_{\text{std}} + (P_{\text{AUT}} - P_{\text{std}})
$$

(in dB)

---

## Practical Design Considerations

### 1. Matching Network

**Goal**: Transform antenna impedance to 50Ω

**Techniques**:
- **LC network**: Series/shunt inductors/capacitors
- **Quarter-wave transformer**: $Z_{\lambda/4} = \sqrt{Z_0 Z_{\text{ant}}}$
- **Stub matching**: Open/short-circuited transmission line stubs

**Example**: Dipole ($Z = 73 + j42.5\ \Omega$) to 50Ω:
- Add series capacitor to cancel reactance (j42.5Ω)
- Use transformer to match 73Ω to 50Ω

---

### 2. Balun (Balanced-Unbalanced Transformer)

**Problem**: Coaxial cable (unbalanced) feeding dipole (balanced) → Common-mode currents on outer shield (pattern distortion)

**Solution**: Balun isolates antenna from feedline

**Types**:
- **Choke balun**: Coil of coax (high impedance to common-mode)
- **Sleeve balun**: λ/4 sleeve over coax
- **Transformer balun**: 1:1 or 4:1 turns ratio (ferrite core)

---

### 3. Environmental Effects

#### Ground Plane
- **Monopole requires ground plane** (acts as mirror image)
- **Poor ground** (dry soil, concrete) → Reduced efficiency
- **Elevated radials** (4-8 wires, λ/4 length) improve performance

#### Nearby Objects
- **Metal structures**: Detune antenna (shift resonance), reflect energy
- **Human body**: Lossy dielectric (especially at cellular frequencies) → Detuning, absorption
- **Solution**: Antenna placement away from body (smartphones: top/bottom), adaptive matching

---

## Summary: Key Antenna Formulas

| Parameter | Formula | Units |
|-----------|---------|-------|
| **Gain** | $G = \eta_{\text{ant}} \cdot D$ | Linear or dBi |
| **Effective aperture** | $A_e = \frac{G\lambda^2}{4\pi}$ | m² |
| **Beamwidth (aperture)** | $\theta \approx 70\lambda/D$ | Degrees |
| **Directivity (narrow beam)** | $D \approx 41253/(\theta_E \theta_H)$ | Linear |
| **Friis equation** | $P_r = P_t G_t G_r (\lambda/4\pi d)^2$ | Watts |
| **FSPL** | $L = 20\log(d) + 20\log(f) + 92.45$ | dB |
| **Radiation resistance (dipole)** | $R_r = 73\ \Omega$ | Ohms |
| **SWR** | $\text{SWR} = (1+|\Gamma|)/(1-|\Gamma|)$ | Ratio |

---

## Related Topics

- **[[Free-Space-Path-Loss-(FSPL)]]**: Quantifies distance-dependent loss (uses antenna gains)
- **[[Electromagnetic-Spectrum]]**: Frequency-dependent antenna design
- **[[Maxwell's-Equations-&-Wave-Propagation]]**: Radiation mechanism
- **[[Signal-to-Noise-Ratio-(SNR)]]**: Antenna gain improves SNR
- **[[AID-Protocol-Case-Study]]**: THz phased array example (1.875 THz, 40 dB gain)
- **Propagation Modes**: How antennas couple to environment (TBD)
- **Multipath & Fading**: Antenna diversity, MIMO (TBD)

---

**Next**: **Binary Phase-Shift Keying (BPSK)** (TBD) - Simplest phase modulation, bridge to [[QPSK-Modulation]]

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
