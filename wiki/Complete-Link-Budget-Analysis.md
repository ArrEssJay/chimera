# Complete Link Budget Analysis

[[Home]] | **Link Budget & System Performance** | [[Free-Space Path Loss (FSPL)]] | [[Signal-to-Noise Ratio (SNR)]]

---

## Overview

**Link budget** is a comprehensive accounting of **all gains and losses** from transmitter to receiver, determining if a communication link will work.

**Purpose**: Answer the critical question:
> "Will the receiver get enough signal power to achieve the required data rate and error rate?"

**Bottom line**: 

$$
P_r = P_t + G_t - L_{\text{total}} + G_r \quad (\text{dBm or dBW})
$$

Where all gains/losses are in dB

**Link closes** if: $P_r > P_{\text{min}}$ (receiver sensitivity)

**Margin**: $M = P_r - P_{\text{min}}$ (dB of safety buffer)

---

## Link Budget Components

### 1. Transmitter Side

#### Transmitted Power (P_t)

**RF power delivered to antenna** (after all TX losses):

$$
P_t = P_{\text{amp}} - L_{\text{TX}} \quad (\text{dB})
$$

Where:
- $P_{\text{amp}}$ = Power amplifier output (dBm)
- $L_{\text{TX}}$ = TX losses (cables, filters, circulators)

**Example**: WiFi router
- PA output: 20 dBm (100 mW)
- Cable/connector loss: 0.5 dB
- $P_t = 20 - 0.5 = 19.5$ dBm

---

#### Transmit Antenna Gain (G_t)

**Gain relative to isotropic radiator** (dBi):

**EIRP** (Effective Isotropic Radiated Power):

$$
\text{EIRP} = P_t + G_t \quad (\text{dBm or dBW})
$$

**Example**: 
- $P_t = 19.5$ dBm
- $G_t = 2$ dBi (WiFi router)
- EIRP = 19.5 + 2 = 21.5 dBm

**Regulatory limits**: FCC limits EIRP (e.g., 36 dBm for 2.4 GHz WiFi)

---

### 2. Propagation Path

#### Free-Space Path Loss (FSPL)

**Loss due to spherical spreading**:

$$
L_{\text{FSPL}} = 20\log_{10}(d) + 20\log_{10}(f) + 20\log_{10}\left(\frac{4\pi}{c}\right)
$$

**Simplified**:

$$
L_{\text{FSPL}} = 32.45 + 20\log_{10}(d_{\text{km}}) + 20\log_{10}(f_{\text{MHz}}) \quad (\text{dB})
$$

**Example**: WiFi @ 2.4 GHz, 100 m
- $f = 2400$ MHz, $d = 0.1$ km

$$
L_{\text{FSPL}} = 32.45 + 20\log_{10}(0.1) + 20\log_{10}(2400)
$$

$$
= 32.45 - 20 + 67.6 = 80\ \text{dB}
$$

**See**: [[Free-Space Path Loss (FSPL)]]

---

#### Atmospheric Absorption

**Oxygen and water vapor absorption** (significant > 10 GHz):

**Zenith attenuation** (at sea level):

| Frequency | Attenuation (dB/km) | Notes |
|-----------|---------------------|-------|
| < 10 GHz | < 0.01 | Negligible |
| 22.2 GHz | 0.2 | H₂O resonance |
| 60 GHz | 15 | O₂ resonance (peak) |
| 120 GHz | 2 | Secondary O₂ line |
| 183 GHz | 5 | H₂O line |

**Example**: Ka-band satellite @ 20 GHz, 5° elevation (path length ~ 11 km through atmosphere)
- Attenuation: ~0.05 dB/km × 11 km = **0.55 dB**

**See**: [[Atmospheric Effects (Ionospheric, Tropospheric)]]

---

#### Rain Attenuation

**Dominant impairment for satellite Ku/Ka/V-band**:

**ITU-R model**: $\gamma_R = k \cdot R^{\alpha}$ (dB/km)

**Example**: Ku-band @ 12 GHz, heavy rain (25 mm/hr), 4 km path
- k = 0.0188, α = 1.217
- $\gamma_R = 0.0188 \times 25^{1.217} = 1.2$ dB/km
- **Total loss**: 1.2 × 4 = 4.8 dB

**At 99% availability**: Design for rain rate exceeded 1% of time (temperate: 12 mm/hr, tropical: 42 mm/hr)

**See**: [[Weather Effects (Rain Fade, Fog Attenuation)]]

---

#### Other Propagation Effects

| Effect | Typical Loss | When Applicable |
|--------|--------------|-----------------|
| **Ionospheric scintillation** | 1-20 dB | L-band satellite, equatorial, solar max |
| **Tropospheric scintillation** | 0.5-2 dB | Low elevation, > 10 GHz |
| **Polarization mismatch** | 0-∞ dB | Antenna misalignment, Faraday rotation |
| **Multipath fading** | 10-30 dB | Mobile, urban NLOS |
| **Foliage loss** | 0.3-1 dB/m | Trees, vegetation (VHF/UHF) |
| **Building penetration** | 5-20 dB | Indoor (depends on freq, materials) |

**See**: [[Multipath Propagation & Fading (Rayleigh, Rician)]], [[Wave Polarization]]

---

### 3. Receiver Side

#### Receive Antenna Gain (G_r)

**Same concept as TX antenna** (reciprocity):

**Example**: WiFi laptop
- $G_r = 0$ dBi (omnidirectional)

**Directional antenna**: 
- Parabolic dish: 30-60 dBi (satellite)
- Yagi: 10-15 dBi (TV, point-to-point)

---

#### Receiver Losses (L_RX)

**Losses between antenna and receiver input**:

- Cable loss: 0.5-3 dB (depends on length, freq, cable type)
- Connector loss: 0.1-0.5 dB per connector
- Filter loss: 0.5-2 dB (bandpass filters)
- Circulator/duplexer loss: 0.5-1 dB

**Example**: Satellite ground station
- Cable: 2 dB (long run from dish to equipment room)
- Connectors: 0.3 dB
- LNA inline: -40 dB (gain, not loss!)
- **Net**: 2 + 0.3 - 40 = -37.7 dB (LNA provides gain)

---

#### Receiver Sensitivity (P_min)

**Minimum signal power for acceptable performance**:

$$
P_{\text{min}} = -174 + 10\log_{10}(B) + \text{NF} + \text{SNR}_{\text{req}} + L_{\text{impl}} \quad (\text{dBm})
$$

Where:
- **-174 dBm/Hz**: Thermal noise floor at 290 K
- **B**: Bandwidth (Hz)
- **NF**: Noise figure (dB)
- **SNR_req**: Required SNR for demodulation (dB)
- **L_impl**: Implementation loss (quantization, imperfect sync, etc.) ~1-3 dB

**Example**: WiFi 802.11n, 20 MHz channel, QPSK 1/2
- Bandwidth: 20 MHz = 73 dBHz
- NF: 6 dB (typical WiFi chipset)
- SNR_req: 5 dB (QPSK with robust FEC)
- L_impl: 2 dB
- $P_{\text{min}} = -174 + 73 + 6 + 5 + 2 = -88$ dBm

**See**: [[Noise Sources & Noise Figure]]

---

## Complete Link Budget Equation

$$
P_r = \text{EIRP} - L_{\text{FSPL}} - L_{\text{atm}} - L_{\text{rain}} - L_{\text{other}} + G_r - L_{\text{RX}}
$$

**Expanded**:

$$
P_r = [P_t + G_t] - L_{\text{FSPL}} - L_{\text{atm}} - L_{\text{rain}} - L_{\text{multipath}} - L_{\text{misc}} + [G_r - L_{\text{cable}}]
$$

**Link margin**:

$$
M = P_r - P_{\text{min}}
$$

**Design guideline**: $M \geq 10$ dB (provides fade margin, interference tolerance)

---

## Example 1: WiFi Indoor Link

**Scenario**: 2.4 GHz WiFi, 802.11n, 20 MHz, QPSK 1/2, 50 m indoor

### Transmitter
- PA output: 20 dBm
- Cable loss: 0.5 dB
- **P_t**: 19.5 dBm
- Antenna gain: 2 dBi
- **EIRP**: 21.5 dBm

### Path
- Free-space loss @ 50 m, 2.4 GHz:

$$
L_{\text{FSPL}} = 32.45 + 20\log_{10}(0.05) + 20\log_{10}(2400) = 32.45 - 26 + 67.6 = 74\ \text{dB}
$$

- Wall penetration (2 walls × 5 dB): 10 dB
- **Total path loss**: 74 + 10 = 84 dB

### Receiver
- Antenna gain: 0 dBi (laptop internal)
- Cable loss: 0 dB (integrated)
- **Received power**:

$$
P_r = 21.5 - 84 + 0 = -62.5\ \text{dBm}
$$

### Sensitivity
- Thermal noise: -174 dBm/Hz + 73 dBHz = -101 dBm
- NF: 6 dB
- SNR_req: 5 dB (QPSK 1/2)
- Impl loss: 2 dB
- **P_min**: -101 + 6 + 5 + 2 = -88 dBm

### Margin

$$
M = -62.5 - (-88) = 25.5\ \text{dB}
$$

**Result**: Link **closes comfortably** with 25.5 dB margin (can tolerate fading, interference)

---

## Example 2: GEO Satellite Ku-band Downlink

**Scenario**: 12 GHz downlink, 36,000 km slant range, 1 m dish RX, clear sky

### Transmitter (Satellite)
- Satellite PA: 100 W = 50 dBm
- TX antenna gain: 30 dBi (spot beam)
- **EIRP**: 80 dBm = 80 dBW

### Path
- Distance: 36,000 km
- Frequency: 12 GHz

$$
L_{\text{FSPL}} = 32.45 + 20\log_{10}(36,000) + 20\log_{10}(12,000)
$$

$$
= 32.45 + 91.1 + 81.6 = 205\ \text{dB}
$$

- Atmospheric absorption (5° elevation): 0.5 dB
- Clear-sky rain (0.01 mm/hr): 0.01 dB (negligible)
- Ionospheric scintillation margin: 2 dB
- **Total path loss**: 205 + 0.5 + 0 + 2 = 207.5 dB

### Receiver (Ground Station)
- Dish diameter: 1 m
- Antenna gain (eff 60%):

$$
G_r = 10\log_{10}\left(0.6 \times \left(\frac{\pi \times 1}{0.025}\right)^2\right) = 10\log_{10}(0.6 \times 1580) = 37.8\ \text{dBi}
$$

- LNA noise figure: 0.8 dB (cryogenic)
- Cable loss: 1 dB
- **Net RX gain**: 37.8 - 1 = 36.8 dB

**Received power**:

$$
P_r = 80 - 207.5 + 36.8 = -90.7\ \text{dBm}
$$

### Sensitivity (DVB-S2, QPSK 3/4, 36 MHz bandwidth)
- Bandwidth: 36 MHz = 75.6 dBHz
- Thermal noise: -174 + 75.6 = -98.4 dBm
- NF: 0.8 dB (LNA at antenna)
- SNR_req: 6.5 dB (QPSK 3/4 with LDPC)
- Impl loss: 1.5 dB
- **P_min**: -98.4 + 0.8 + 6.5 + 1.5 = -89.6 dBm

### Margin (Clear Sky)

$$
M = -90.7 - (-89.6) = -1.1\ \text{dB}
$$

**Uh oh!** Link **fails** in clear sky (need higher gain or more TX power)

**Fix**: Increase dish to 1.8 m
- New gain: 37.8 + 20log(1.8) = 42.9 dBi
- New $P_r$: 80 - 207.5 + 42.9 = -84.6 dBm
- **New margin**: -84.6 - (-89.6) = **5 dB** (marginal)

**With 99% rain margin** (add 5 dB rain attenuation):
- $P_r$ in rain: -84.6 - 5 = -89.6 dBm
- **Rain margin**: 0 dB (link at threshold!)

**Better fix**: Use 2.4 m dish
- Gain: 47.5 dBi
- Clear sky: -80 dBm, margin 10 dB
- 99% rain: -85 dBm, margin 5 dB

---

## Example 3: Cellular LTE (2.6 GHz)

**Scenario**: eNodeB to UE, 2.6 GHz, 10 MHz RB, QPSK 1/2, 5 km suburban

### Transmitter (Cell Tower)
- PA per antenna: 43 dBm (20 W)
- TX antenna: 17 dBi (sector antenna)
- Cable loss: 2 dB
- **EIRP**: 43 + 17 - 2 = 58 dBm

### Path
- FSPL @ 5 km, 2.6 GHz:

$$
L_{\text{FSPL}} = 32.45 + 20\log_{10}(5) + 20\log_{10}(2600) = 32.45 + 14 + 68.3 = 115\ \text{dB}
$$

- Shadowing margin (suburban log-normal): 8 dB (for 90% coverage)
- Building penetration: 10 dB (indoor UE)
- **Total path loss**: 115 + 8 + 10 = 133 dB

### Receiver (UE)
- Antenna gain: -2 dBi (internal, near body)
- **Received power**:

$$
P_r = 58 - 133 - 2 = -77\ \text{dBm}
$$

### Sensitivity (10 MHz, QPSK 1/2)
- Bandwidth: 10 MHz = 70 dBHz
- Thermal noise: -174 + 70 = -104 dBm
- NF: 9 dB (smartphone front-end)
- SNR_req: 4 dB (QPSK 1/2 Turbo code)
- Impl loss: 2 dB
- **P_min**: -104 + 9 + 4 + 2 = -89 dBm

### Margin

$$
M = -77 - (-89) = 12\ \text{dB}
$$

**Result**: Link **closes** with 12 dB margin (adequate for mobile fading)

**With Rayleigh fading** (10 dB fade depth @ 10% time):
- Faded $P_r$: -77 - 10 = -87 dBm
- **Faded margin**: -87 - (-89) = 2 dB (still works, but error rate increases)

**Diversity RX** (2 antennas, max ratio combining):
- Diversity gain: 5 dB (typical for 2-branch)
- Effective $P_r$ in fade: -87 + 5 = -82 dBm
- **Margin with diversity**: -82 - (-89) = 7 dB (much better!)

---

## Link Budget Table Template

| Parameter | Symbol | Value | Units | Notes |
|-----------|--------|-------|-------|-------|
| **TRANSMITTER** | | | | |
| TX power (PA) | $P_{\text{amp}}$ | | dBm | |
| TX losses | $L_{\text{TX}}$ | | dB | Cables, filters |
| Transmit power | $P_t$ | | dBm | $P_{\text{amp}} - L_{\text{TX}}$ |
| TX antenna gain | $G_t$ | | dBi | |
| **EIRP** | | | dBm | $P_t + G_t$ |
| **PROPAGATION** | | | | |
| Distance | $d$ | | km | |
| Frequency | $f$ | | GHz | |
| Free-space loss | $L_{\text{FSPL}}$ | | dB | 32.45 + 20log(d) + 20log(f) |
| Atmospheric loss | $L_{\text{atm}}$ | | dB | |
| Rain attenuation | $L_{\text{rain}}$ | | dB | ITU model |
| Other losses | $L_{\text{other}}$ | | dB | Multipath, penetration, etc. |
| **Total path loss** | $L_{\text{total}}$ | | dB | Sum |
| **RECEIVER** | | | | |
| RX antenna gain | $G_r$ | | dBi | |
| RX losses | $L_{\text{RX}}$ | | dB | Cables, connectors |
| **Received power** | $P_r$ | | dBm | EIRP - $L_{\text{total}}$ + $G_r$ - $L_{\text{RX}}$ |
| **PERFORMANCE** | | | | |
| Bandwidth | $B$ | | MHz | |
| Thermal noise | $N_0$ | -174 + 10log(B) | dBm | |
| Noise figure | NF | | dB | |
| Noise power | $N$ | $N_0$ + NF | dBm | |
| Required SNR | SNR_req | | dB | For target BER |
| Impl loss | $L_{\text{impl}}$ | | dB | Typically 1-3 dB |
| **Sensitivity** | $P_{\text{min}}$ | | dBm | $N$ + SNR_req + $L_{\text{impl}}$ |
| **MARGIN** | $M$ | | dB | $P_r - P_{\text{min}}$ |

---

## Fade Margin Design Guidelines

**Clear-sky margin** (no fading):
- **Satellite (GEO Ku/Ka)**: 5-10 dB
- **Terrestrial LOS**: 10-15 dB
- **Mobile (NLOS)**: 15-20 dB

**Rain margin** (satellite):
- **Availability**: 99% → 5-8 dB, 99.9% → 10-15 dB, 99.99% → 20-30 dB
- **Frequency**: Ku-band: 3-10 dB, Ka-band: 10-20 dB

**Multipath fading margin**:
- **Rayleigh fading**: 20-30 dB for 90% location reliability
- **Rician K=6 dB**: 10-15 dB

**Total design margin**:

$$
M_{\text{total}} = M_{\text{clear}} + M_{\text{rain}} + M_{\text{fade}}
$$

**Trade-off**: Higher margin → More expensive (bigger antennas, more power)

---

## Adaptive Techniques

**Adaptive Coding and Modulation (ACM)**:

**Concept**: Change modulation/code rate based on channel conditions

**Example**: DVB-S2X satellite
- Clear sky: 32APSK 9/10 → 3.5 bits/symbol, needs C/N = 16 dB
- Light rain: 8PSK 3/4 → 2.25 bits/symbol, needs C/N = 11 dB
- Heavy rain: QPSK 1/2 → 1 bit/symbol, needs C/N = 4 dB

**Benefit**: Maximize throughput when conditions good, maintain connectivity when conditions poor

---

## Link Availability

**Probability link meets performance requirement**:

$$
\text{Availability} = \frac{\text{Time link works}}{\text{Total time}} \times 100\%
$$

**Target availability** (depends on application):
- **Data networks**: 99.9% (8.76 hours/year downtime)
- **Voice**: 99.99% (52.6 minutes/year)
- **Mission-critical**: 99.999% (5.26 minutes/year)

**Dominated by rain** (for satellite Ku/Ka-band):

**ITU rain statistics**: 
- Temperate: 12 mm/hr exceeded 1% of time (3.65 days/year)
- Tropical: 42 mm/hr exceeded 1% of time

**Design procedure**:
1. Choose target availability (e.g., 99.9%)
2. Find rain rate exceeded 0.1% of time (e.g., 25 mm/hr temperate)
3. Calculate rain attenuation for that rain rate
4. Ensure link margin > rain attenuation

---

## Summary

**Link budget essentials**:

1. **EIRP** = TX power + TX gain (dBm)
2. **Path loss** = FSPL + atmospheric + rain + other (dB)
3. **RX power** = EIRP - path loss + RX gain - RX losses (dBm)
4. **Sensitivity** = Noise floor + NF + SNR_req + impl loss (dBm)
5. **Margin** = RX power - sensitivity (dB, must be positive!)

**Design targets**:
- Clear-sky margin: 10+ dB
- Rain margin: 5-20 dB (depends on frequency, availability)
- Total margin: 15-30 dB typical

**Adaptive techniques** (ACM, diversity) improve spectral efficiency and availability.

---

## Related Topics

- **[[Free-Space Path Loss (FSPL)]]**: Dominant loss mechanism
- **[[Signal-to-Noise Ratio (SNR)]]**: Determines required C/N
- **[[Noise Sources & Noise Figure]]**: RX sensitivity calculation
- **[[Energy Ratios (Es N0 and Eb N0)]]**: Alternative SNR metrics
- **[[Weather Effects (Rain Fade, Fog Attenuation)]]**: Rain margin design
- **[[Multipath Propagation & Fading (Rayleigh, Rician)]]**: Fade margin for mobile
- **[[Bit Error Rate (BER)]]**: Performance metric vs SNR

---

**Key takeaway**: **Link budget is systematic accounting of all gains/losses from TX to RX.** Start with EIRP, subtract path losses, add RX gain, compare to sensitivity. Margin = difference. Design for 10-30 dB total margin to handle fading, rain, interference. Adaptive techniques maximize throughput while maintaining connectivity.

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
