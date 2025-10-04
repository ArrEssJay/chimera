# Weather Effects: Rain Fade & Fog Attenuation

[[Home]] | **RF Propagation** | [[Free-Space Path Loss (FSPL)]] | [[Atmospheric Effects (Ionospheric, Tropospheric)]]

---

## Overview

**Weather significantly impacts RF propagation**, especially at frequencies above 10 GHz. Rain, fog, snow, and clouds introduce **frequency-dependent attenuation** that must be accounted for in link budgets.

**Key principle**: Attenuation increases with:
1. **Frequency** (higher frequencies = more attenuation)
2. **Precipitation rate** (heavier rain = more loss)
3. **Path length through weather** (longer distance = more cumulative loss)

**Critical for**: Satellite communications (Ku/Ka/V-band), 5G mmWave (28/39 GHz), point-to-point microwave links

---

## Rain Attenuation

### Physical Mechanism

**Raindrops act as lossy dielectric spheres**:

1. **Absorption**: EM energy heats water molecules (dielectric loss)
2. **Scattering**: Raindrops redirect energy out of main beam (Mie scattering when droplet size ≈ λ)

**Frequency dependence**:
- **< 10 GHz**: Rain effects negligible (λ >> raindrop size)
- **10-100 GHz**: Strong attenuation (λ ≈ raindrop size, 1-5 mm)
- **> 100 GHz**: Extreme attenuation (THz communications impossible in rain)

---

### ITU-R Rain Attenuation Model

**Standard method**: ITU-R P.838 and P.618

**Specific attenuation** (dB/km):

$$
\gamma_R = k \cdot R^\alpha
$$

Where:
- $\gamma_R$ = Specific attenuation (dB/km)
- $R$ = Rain rate (mm/hr)
- $k, \alpha$ = Frequency-dependent coefficients (from ITU tables)

---

#### Coefficients by Frequency

**Selected values** (horizontal polarization):

| Frequency | $k$ | $\alpha$ | Attenuation @ 25 mm/hr rain |
|-----------|-----|----------|----------------------------|
| 1 GHz | 0.0000387 | 0.912 | 0.0005 dB/km |
| 4 GHz | 0.00065 | 1.121 | 0.025 dB/km |
| 10 GHz | 0.0101 | 1.276 | 0.50 dB/km |
| 12 GHz (Ku) | 0.0188 | 1.310 | 1.02 dB/km |
| 20 GHz (Ka) | 0.0751 | 1.099 | 3.26 dB/km |
| 30 GHz | 0.187 | 1.021 | 7.14 dB/km |
| 40 GHz | 0.350 | 0.939 | 12.2 dB/km |
| 50 GHz | 0.536 | 0.873 | 16.8 dB/km |
| 60 GHz | 0.707 | 0.826 | 20.3 dB/km |
| 80 GHz | 0.999 | 0.784 | 26.4 dB/km |
| 100 GHz | 1.187 | 0.751 | 29.8 dB/km |

**Note**: Vertical polarization has slightly different coefficients (typically ~10-20% more attenuation)

---

### Rain Rate Classifications

**ITU rain zones** (global climate regions):

| Zone | Climate | Rain rate exceeded 0.01% of year | Example locations |
|------|---------|----------------------------------|-------------------|
| A | Polar | 8 mm/hr | Arctic, Antarctic |
| B | Temperate | 12 mm/hr | Northern Europe, Canada |
| C | Subtropical | 22 mm/hr | Southern US, Mediterranean |
| D | Moderate tropical | 32 mm/hr | Southeast Asia, India |
| E | Equatorial | 42 mm/hr | Central Africa, Indonesia |
| F | Tropical maritime | 53 mm/hr | Amazon, Congo Basin |
| G | Monsoon | 63 mm/hr | Bangladesh, Myanmar |
| H | Intense tropical | 95 mm/hr | Extreme storms |

**Design criterion**: Typically design for 99.9% availability (0.01% outage time)
- Temperate: 12 mm/hr
- Tropical: 42-63 mm/hr

---

### Link Budget Impact: Satellite Examples

#### Example 1: Ku-Band Satellite (12 GHz Downlink)

**Scenario**: GEO satellite → Home receiver, temperate climate

**Path geometry**:
- Elevation angle: 30°
- Slant path through rain: ~6 km effective length
- Rain rate (0.01% time): 12 mm/hr

**Calculation**:

$$
\gamma_R = 0.0188 \times 12^{1.310} = 0.50\ \text{dB/km}
$$

$$
A_{\text{rain}} = \gamma_R \times d_{\text{eff}} = 0.50 \times 6 = 3\ \text{dB}
$$

**Impact**: 3 dB margin needed for 99.9% availability

**With 95 mm/hr extreme storm** (H zone):

$$
\gamma_R = 0.0188 \times 95^{1.310} = 6.3\ \text{dB/km}
$$

$$
A_{\text{rain}} = 6.3 \times 6 = 38\ \text{dB}
$$

**Result**: Complete outage (exceeds typical 10-15 dB link margin)

---

#### Example 2: Ka-Band Satellite (20 GHz Downlink)

**Same scenario** as Ku-band:

**Temperate (12 mm/hr)**:

$$
\gamma_R = 0.0751 \times 12^{1.099} = 1.16\ \text{dB/km}
$$

$$
A_{\text{rain}} = 1.16 \times 6 = 7\ \text{dB}
$$

**Tropical (42 mm/hr)**:

$$
\gamma_R = 0.0751 \times 42^{1.099} = 4.4\ \text{dB/km}
$$

$$
A_{\text{rain}} = 4.4 \times 6 = 26\ \text{dB}
$$

**Comparison**: Ka-band suffers **2-3× more rain fade** than Ku-band!

**Mitigation**: 
- Adaptive coding/modulation (ACM) → Lower data rate in rain
- Site diversity → Multiple ground stations (rain cells are localized)
- Higher TX power margin

---

#### Example 3: V-Band Satellite (40 GHz)

**Next-gen satellite comms** (e.g., OneWeb, Starlink inter-satellite links):

**Temperate (12 mm/hr)**:

$$
\gamma_R = 0.350 \times 12^{0.939} = 3.6\ \text{dB/km}
$$

$$
A_{\text{rain}} = 3.6 \times 6 = 22\ \text{dB}
$$

**Result**: **Severe rain fade**, requires 25+ dB margin or advanced mitigation

---

### Terrestrial Path: 5G mmWave

#### Example 4: 28 GHz 5G Link (Urban Microcell)

**Scenario**: Base station → UE, 200 m range, light rain (5 mm/hr)

**Calculation**:

$$
\gamma_R = 0.187 \times 5^{1.021} = 0.98\ \text{dB/km}
$$

$$
A_{\text{rain}} = 0.98 \times 0.2 = 0.2\ \text{dB}
$$

**Impact**: Minimal (short path length)

**Heavy rain (25 mm/hr)**:

$$
\gamma_R = 0.187 \times 25^{1.021} = 5.2\ \text{dB/km}
$$

$$
A_{\text{rain}} = 5.2 \times 0.2 = 1\ \text{dB}
$$

**Conclusion**: 5G mmWave is **relatively rain-tolerant for short ranges** (< 500 m)

---

#### Example 5: 60 GHz Point-to-Point Link

**Scenario**: Building-to-building backhaul, 1 km, moderate rain (15 mm/hr)

**Calculation**:

$$
\gamma_R = 0.707 \times 15^{0.826} = 6.4\ \text{dB/km}
$$

$$
A_{\text{rain}} = 6.4 \times 1 = 6.4\ \text{dB}
$$

**Plus oxygen absorption**: ~15 dB/km at 60 GHz (clear air)

$$
A_{\text{total}} = 15 + 6.4 = 21.4\ \text{dB}
$$

**Result**: **60 GHz is impractical for >1 km in rain** (used for indoor/short-range only)

---

## Fog & Cloud Attenuation

**Fog = suspended water droplets** (smaller than rain, ~10-100 μm diameter)

**Attenuation mechanism**: Primarily absorption (droplets ≪ λ for most RF bands)

---

### Fog Attenuation Model

**Specific attenuation**:

$$
\gamma_{\text{fog}} = K_l \cdot M \quad (\text{dB/km})
$$

Where:
- $M$ = Liquid water content (g/m³)
- $K_l$ = Frequency-dependent coefficient

**Typical fog**: $M = 0.05$ g/m³ (light fog) to $M = 0.5$ g/m³ (dense fog)

---

### Coefficients by Frequency

| Frequency | $K_l$ (dB/km per g/m³) | Attenuation (dense fog, 0.5 g/m³) |
|-----------|------------------------|-----------------------------------|
| 10 GHz | 0.01 | 0.005 dB/km |
| 20 GHz | 0.07 | 0.035 dB/km |
| 30 GHz | 0.20 | 0.10 dB/km |
| 60 GHz | 1.0 | 0.50 dB/km |
| 100 GHz | 2.5 | 1.25 dB/km |
| 300 GHz | 15 | 7.5 dB/km |

**Key insight**: Fog is **negligible below 30 GHz**, but significant at THz frequencies.

---

### Comparison: Rain vs Fog

**At 30 GHz, 1 km path**:

| Condition | Attenuation |
|-----------|-------------|
| Clear air | ~0.1 dB |
| Dense fog (0.5 g/m³) | 0.10 dB |
| Light rain (5 mm/hr) | 3.7 dB |
| Moderate rain (12 mm/hr) | 7.1 dB |
| Heavy rain (25 mm/hr) | 12.5 dB |

**Rain dominates** at microwave/mmWave frequencies.

**Fog becomes important** at THz (> 100 GHz):

**At 300 GHz (THz), 100 m path**:

| Condition | Attenuation |
|-----------|-------------|
| Clear air | ~5 dB (water vapor) |
| Dense fog | 0.75 dB |
| Light rain (5 mm/hr) | **300+ dB** (complete blockage) |

---

## Snow & Ice Attenuation

**Dry snow**: Very low attenuation (air + ice crystals, low loss)

$$
\gamma_{\text{dry snow}} \approx 0.0005 \times f^2 \times S \quad (\text{dB/km})
$$

Where:
- $f$ = Frequency (GHz)
- $S$ = Snowfall rate (mm/hr liquid equivalent)

**At 20 GHz, 10 mm/hr dry snow**: $\gamma \approx 0.2$ dB/km (negligible)

---

**Wet snow** (melting): Much higher attenuation (comparable to rain)

**Ice crystals** (cirrus clouds): Minimal attenuation (< 0.1 dB even at 100 GHz)

**Practical implication**: Snow is **far less problematic** than rain for RF links.

---

## Hail Attenuation

**Hailstones**: Large (5-50 mm), but mostly ice (low loss tangent)

**Attenuation**: Typically **less than rain of equivalent water content**

**Why?**: Ice has lower dielectric loss than liquid water ($\tan \delta_{\text{ice}} \ll \tan \delta_{\text{water}}$)

**Concern**: **Depolarization** (hailstones tumble, scatter energy to cross-pol)

---

## Frequency-Specific Considerations

### Bands Most Affected by Rain

| Band | Frequency | Primary Use | Rain Sensitivity |
|------|-----------|-------------|------------------|
| C-band | 4-8 GHz | Satellite TV, radar | **Low** (0.05 dB/km @ 25 mm/hr) |
| X-band | 8-12 GHz | Military, radar | **Moderate** (0.5 dB/km) |
| Ku-band | 12-18 GHz | Satellite TV/broadband | **Moderate-High** (1-2 dB/km) |
| Ka-band | 26.5-40 GHz | Satellite, 5G backhaul | **High** (3-12 dB/km) |
| V-band | 40-75 GHz | Next-gen satellite | **Very High** (12-20 dB/km) |
| W-band | 75-110 GHz | Automotive radar, imaging | **Extreme** (20-30 dB/km) |

**C-band advantage**: Widely used for tropical regions (low rain fade)

**Ka-band challenge**: High data rates, but needs ACM and large margins

---

## Mitigation Techniques

### 1. Link Margin

**Add extra dB to link budget** for rain:

- Temperate climate, Ku-band: **+3-5 dB**
- Tropical climate, Ka-band: **+8-15 dB**
- mmWave terrestrial (< 1 km): **+2-3 dB**

**Tradeoff**: Higher TX power or larger antennas (more expensive)

---

### 2. Adaptive Coding & Modulation (ACM)

**Dynamically adjust modulation** based on link quality:

- Clear sky: 16-APSK (4 bits/symbol)
- Light rain: QPSK (2 bits/symbol)
- Heavy rain: BPSK + strong FEC (0.5 bits/symbol effective)

**Result**: **Graceful degradation** (lower data rate instead of outage)

**Used in**: DVB-S2, 5G NR, satellite modems

---

### 3. Site Diversity

**Multiple ground stations** separated by 5-20 km:

**Principle**: Rain cells are **localized** (~5-10 km diameter)
- Probability both sites in heavy rain is low
- Switch to non-rainy site

**Diversity gain**: 5-10 dB improvement in availability

**Example**: Satellite gateways often have 2-3 sites for 99.99% uptime

---

### 4. Frequency Diversity

**Backup link at lower frequency**:

- Primary: Ka-band (high data rate, rain-sensitive)
- Backup: Ku-band (lower rate, rain-tolerant)

**Switchover**: Automatic when Ka-band SNR drops

---

### 5. Uplink Power Control (UPC)

**Increase TX power during rain** to compensate for attenuation:

- Monitor beacon signal from satellite
- Detect fade, boost uplink power (up to ~10 dB)
- Avoid saturating satellite transponder

**Limitation**: Power amplifier headroom (can't boost infinitely)

---

### 6. Orbit Selection

**Low Earth Orbit (LEO)** satellites have shorter slant paths:

- GEO: ~40,000 km, slant path through rain ~6 km @ 30° elevation
- LEO: ~550 km, slant path ~2 km @ 30° elevation

**Rain attenuation**: **~3× less for LEO** (shorter path)

**Starlink/OneWeb advantage**: Better rain performance than GEO

---

## Depolarization Effects

**Rain also causes cross-polarization**:

**Mechanism**: Raindrops are **oblate** (flattened spheres)
- Horizontal and vertical polarizations experience different phase shifts
- Converts co-pol energy → cross-pol

**Impact**: Degrades dual-polarization systems (e.g., V/H reuse for 2× capacity)

**Cross-Polarization Discrimination (XPD)**:

$$
\text{XPD}_{\text{rain}} = U - V \log(A_{\text{rain}}) \quad (\text{dB})
$$

Where:
- $U, V$ = Frequency-dependent constants (~30-40 dB, ~12-20 dB typical)
- $A_{\text{rain}}$ = Co-pol attenuation (dB)

**Example**: If rain causes 10 dB attenuation → XPD degrades from 30 dB (clear) to ~20 dB

---

## Regional Considerations

### Temperate Climates (Europe, Northern US, Canada)

**Rain characteristics**:
- Moderate intensity (12-22 mm/hr, 0.01% time)
- Long-duration stratiform rain (hours)
- Lower fade durations

**Design approach**:
- Standard margins (3-5 dB for Ku, 8-12 dB for Ka)
- ACM effective (gradual fade)

---

### Tropical Climates (Southeast Asia, Equatorial Africa, Amazon)

**Rain characteristics**:
- High intensity (42-95 mm/hr, 0.01% time)
- Short-duration convective storms (minutes)
- High fade depths

**Design approach**:
- Large margins (8-15 dB for Ku, 15-25 dB for Ka)
- Site diversity essential for Ka-band
- C-band preferred for critical services

**Case study**: Indonesia (equatorial)
- Ku-band outages: ~0.5% of time (annual)
- Ka-band: ~2-5% (unacceptable without mitigation)
- C-band: < 0.01% (reliable)

---

### Coastal vs Inland

**Coastal regions**: Lower rain rates (maritime climate)
**Inland tropics**: Higher convective activity (more intense storms)

**Elevation matters**: Higher altitude → shorter path through rain layer (troposphere)

---

## Measurement & Prediction

### Radiometer Method

**Measure sky brightness temperature** $T_B$:

$$
A_{\text{rain}} = 10 \log\left(\frac{T_{\text{sky}} - T_B}{T_{\text{sky}} - T_{\text{medium}}}\right) \quad (\text{dB})
$$

**Real-time fade monitoring** for UPC systems.

---

### Weather Radar Integration

**Use ground-based weather radar** to predict rain attenuation:

1. Measure rain rate along path (3D map)
2. Apply ITU model
3. Predict fade 5-10 minutes ahead

**Proactive ACM**: Adjust modulation before fade hits (minimize disruption)

---

## Summary Table: Rain Attenuation by Band

**Path**: 6 km slant through rain (satellite, 30° elevation)

| Band | Frequency | 12 mm/hr (Temperate) | 42 mm/hr (Tropical) | 95 mm/hr (Extreme) |
|------|-----------|----------------------|---------------------|-------------------|
| C | 4 GHz | 0.15 dB | 0.7 dB | 2 dB |
| C | 6 GHz | 0.3 dB | 1.3 dB | 3.5 dB |
| X | 10 GHz | 0.5 dB | 2.5 dB | 7 dB |
| Ku | 12 GHz | **3 dB** | **9 dB** | **25 dB** |
| Ku | 14 GHz | 4 dB | 11 dB | 30 dB |
| Ka | 20 GHz | **7 dB** | **22 dB** | **55 dB** |
| Ka | 30 GHz | 13 dB | 38 dB | 90 dB |
| V | 40 GHz | 22 dB | 60 dB | 140 dB |
| V | 50 GHz | 30 dB | 80 dB | 180 dB |

**Bold**: Typical link budget fades

---

## Related Topics

- **[[Free-Space Path Loss (FSPL)]]**: Baseline propagation loss
- **[[Atmospheric Effects (Ionospheric, Tropospheric)]]**: Clear-air propagation
- **[[Multipath Propagation & Fading]]**: Rayleigh/Rician fading (different mechanism)
- **[[Signal-to-Noise Ratio (SNR)]]**: Impact of attenuation on link quality
- **[[QPSK Modulation]]** / **[[LDPC Codes]]**: ACM adapts these for rain conditions
- **Antenna Theory**: Larger antennas provide more gain margin

---

**Key takeaway**: **Rain fade increases dramatically with frequency**. C-band is robust but bandwidth-limited. Ka-band and above require sophisticated mitigation (ACM, diversity, large margins) for reliable service, especially in tropical regions.

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
