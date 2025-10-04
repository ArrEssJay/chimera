# mmWave & THz Communications

**Millimeter-wave (mmWave, 24-300 GHz)** and **Terahertz (THz, 0.3-10 THz)** communications exploit ultra-high-frequency spectrum for multi-gigabit wireless links. 5G NR FR2 (24-52 GHz) delivers 20+ Gbps, while future 6G targets sub-THz (100-300 GHz) for 100+ Gbps. These bands offer massive bandwidth but face severe propagation challenges requiring advanced beamforming and novel system architectures.

---

## 🎯 Why mmWave & THz?

### The Spectrum Crunch

**Sub-6 GHz problem**:
```
Available spectrum: ~1 GHz (fragmented across bands)
Demand: Exponential growth (video, AR/VR, IoT)
Result: Spectrum scarcity → congestion

Shannon capacity:
C = B · log₂(1 + SNR)

To increase C:
- Increase B (bandwidth) → Move to higher frequencies ✅
- Increase SNR → Limited by power, interference
```

**mmWave/THz solution**:
```
mmWave (24-52 GHz): 28 GHz bandwidth available (5G FR2)
Sub-THz (100-300 GHz): 200 GHz bandwidth potential (6G)
THz (1-10 THz): Multi-THz bandwidths (research)

Example (100 GHz carrier, 10 GHz BW, SNR = 20 dB):
C = 10 GHz · log₂(1 + 100) = 66 Gbps

Compare to sub-6 GHz (100 MHz BW):
C = 100 MHz · log₂(1 + 100) = 660 Mbps

100× more bandwidth → 100× higher capacity!
```

---

## 📐 Propagation Characteristics

### Path Loss: The Main Challenge

**Free-space path loss** (FSPL):
```
FSPL(dB) = 32.4 + 20·log₁₀(f_MHz) + 20·log₁₀(d_km)

Example comparisons (d = 100 m):

2.4 GHz (WiFi):
FSPL = 32.4 + 20·log₁₀(2400) + 20·log₁₀(0.1) = 80 dB

28 GHz (5G mmWave):
FSPL = 32.4 + 20·log₁₀(28000) + 20·log₁₀(0.1) = 101 dB

300 GHz (sub-THz):
FSPL = 32.4 + 20·log₁₀(300000) + 20·log₁₀(0.1) = 122 dB

Relative loss:
28 GHz: +21 dB worse than 2.4 GHz
300 GHz: +42 dB worse than 2.4 GHz

Implication: Higher frequency → much shorter range (or need much higher antenna gain)
```

---

### Atmospheric Absorption

**Oxygen (O₂) and water vapor (H₂O)** absorb mmWave/THz strongly.

**Absorption peaks**:
```
Frequency (GHz) | Attenuation (dB/km at sea level) | Cause
----------------|-----------------------------------|-------
60              | 15 dB/km                         | O₂ resonance
120             | 2 dB/km                          | O₂ 2nd harmonic
183             | 2 dB/km                          | H₂O resonance
325             | 1 dB/km                          | H₂O
380-750         | 0.1-1 dB/km                      | Windows (low absorption)
>1 THz          | 10-100 dB/km                     | Multiple molecular resonances

Transmission windows:
- 71-76 GHz, 81-86 GHz (5G FR2 upper band)
- 94 GHz (radar, imaging)
- 130-175 GHz (low absorption)
- 220-325 GHz (6G candidate)
```

**Distance implications**:
```
Example (100 m link):

28 GHz: 0.1 dB/km × 0.1 km = 0.01 dB (negligible)
60 GHz: 15 dB/km × 0.1 km = 1.5 dB (moderate)
300 GHz: 1 dB/km × 0.1 km = 0.1 dB (low, in window)
1 THz: 50 dB/km × 0.1 km = 5 dB (significant)

Indoor/short-range: Absorption manageable
Outdoor/long-range: Limits reach to <1 km
```

**Weather effects**:
```
Rain attenuation (ITU-R model):
α = k · R^β  dB/km

where R = rain rate (mm/h)

At 28 GHz (heavy rain, 50 mm/h):
α ≈ 5 dB/km → 100 m link: 0.5 dB

At 300 GHz (same rain):
α ≈ 15 dB/km → 100 m link: 1.5 dB

THz: Extremely sensitive to humidity, fog, rain
→ Indoor/short-range only in adverse weather
```

---

### Blockage & Diffraction

**Non-Line-of-Sight (NLOS) problem**:
```
mmWave/THz wavelengths:
λ = c/f

28 GHz: λ = 10.7 mm (1 cm)
300 GHz: λ = 1 mm

Diffraction scales with λ:
- Lower frequencies: Diffract around obstacles (wavelength ~ building size)
- mmWave: Minimal diffraction (wavelength << human body)
- THz: No practical diffraction (wavelength ~ grain of sand)

Blockage:
- Human body: 20-40 dB attenuation (28 GHz)
- Hand: 10-20 dB
- Wall: 30-80 dB (depends on material)
- Foliage: 10-30 dB

Result: Highly directional, LOS-dependent propagation
```

**Multipath in mmWave/THz**:
```
Sparse multipath environment:
- Few reflections reach receiver (high absorption, blockage)
- Reflections off smooth surfaces (specular, not diffuse)
- Delay spread: Shorter than sub-6 GHz (fewer paths)

Advantage: Simpler channel model (ray-tracing accurate)
Disadvantage: No diversity from multipath → beamforming essential
```

---

## 📡 Beamforming: The Enabling Technology

**Why beamforming is mandatory**:
```
Path loss compensation:
- 28 GHz: 21 dB more loss than 2.4 GHz
- Need: 21 dB+ antenna gain to match range

Beamforming gain:
G(dB) = 10·log₁₀(N)  (for N-element array)

Example (64-element array):
G = 10·log₁₀(64) = 18 dB

With 256 elements:
G = 10·log₁₀(256) = 24 dB

Overcomes path loss + provides spatial selectivity
```

---

### Analog Beamforming

**Architecture**:
```
Single RF chain → Phase shifters on each antenna element

TX: Data → DAC → Mixer → Power Divider → [Phase Shifters] → Antenna Array
                                           ↓
                                    All elements see same data
                                    Phase shifts steer beam

Advantages:
- Low power (1 RF chain)
- Simple, cost-effective
- High gain (all power focused)

Disadvantages:
- Single beam at a time
- Cannot do MIMO spatial multiplexing
- Fixed beam (hard to adapt dynamically)
```

**Phase shift calculation**:
```
Desired beam direction: θ
Element spacing: d (typically λ/2)

Phase shift for element n:
φₙ = (2π/λ) · n·d · sin(θ)

Example (28 GHz, θ = 30°, d = λ/2):
λ = 10.7 mm
φₙ = π · n · sin(30°) = π/2 · n

Element 0: 0°
Element 1: 90°
Element 2: 180°
Element 3: 270°
```

---

### Hybrid Beamforming

**Compromise**: Analog beamforming per subarray + digital baseband processing.

```
Architecture:
Data streams → [Digital Precoder] → DACs (Nᵣꜰ chains) → Mixers →
               [Analog Phase Shifters per subarray] → Antenna Array (Nₐₙₜ elements)

Where Nᵣꜰ << Nₐₙₜ

Example:
- Total antennas: 256
- RF chains: 16
- Digital precoding: 16 streams (MIMO)
- Analog beamforming: 256/16 = 16 elements per subarray

Benefits:
- Multi-beam capability (Nᵣꜰ simultaneous beams)
- MIMO spatial multiplexing (up to Nᵣꜰ streams)
- Moderate power/cost (Nᵣꜰ RF chains)
```

**Precoding**:
```
Transmit signal: x = F_analog · F_digital · s

where:
- s: Data streams (Nₛ × 1, Nₛ ≤ Nᵣꜰ)
- F_digital: Digital precoder (Nᵣꜰ × Nₛ)
- F_analog: Analog beamformer (Nₐₙₜ × Nᵣꜰ, phase-only)

Optimization:
Maximize: ||H · F_analog · F_digital||²
Subject to: F_analog has constant-modulus entries (phase-only)

Algorithms: Orthogonal Matching Pursuit (OMP), alternating minimization
```

---

### Beam Management

**Challenge**: Narrow beams must be steered to track users.

**Beam sweeping (initial access)**:
```
1. BS transmits sync signals on multiple beam directions
2. UE measures RSRP (Reference Signal Received Power) per beam
3. UE reports best beam index to BS
4. BS selects beam for data transmission

Example (5G NR):
- BS: 64 beam directions (8×8 azimuth/elevation grid)
- Sweep time: 5 ms (one beam per SSB - SS/PBCH Block)
- UE selects best beam (e.g., beam 23)
- Data transmission on beam 23

Beamwidth: ~10° (64-element array at 28 GHz)
```

**Beam tracking**:
```
Problem: User moves → beam misalignment → link failure

Solutions:
1. Periodic re-sweeping (every 20-100 ms)
2. Predictive tracking:
   - Estimate velocity from Doppler
   - Adjust beam direction proactively
3. Multi-beam transmission:
   - Transmit on 2-3 adjacent beams
   - Handover smoothly as user moves

5G NR: Beam Failure Recovery (BFR)
- UE monitors beam quality (RSRP)
- If below threshold: Trigger beam switch
- Latency: <10 ms for recovery
```

---

## 🚀 5G NR FR2 (mmWave)

**Frequency Range 2**: 24.25-52.6 GHz

### Frequency Bands

```
n257: 26.5-29.5 GHz (3 GHz BW)
n258: 24.25-27.5 GHz
n260: 37-40 GHz
n261: 27.5-28.35 GHz

Typical deployment:
- n257 (28 GHz): US carriers (Verizon, AT&T)
- n258 (26 GHz): Europe, Asia
- n260 (39 GHz): US (fixed wireless access)
```

---

### 5G NR mmWave Specifications

```
Bandwidth: 50-400 MHz per carrier
- Typical: 100 MHz (lower latency, easier beam management)
- Maximum: 400 MHz (peak throughput)

Numerology:
- SCS (Subcarrier Spacing): 120 kHz (fast Doppler tolerance)
- Symbol duration: 8.33 μs (short, good for mobility)
- Slot: 0.125 ms (8× faster than sub-6 GHz)

Modulation: Up to 256-QAM (spectral efficiency: 7.4 bits/s/Hz)

MIMO: Up to 4 layers (spatial multiplexing with hybrid beamforming)

Peak data rate:
R = BW × Spectral_Eff × MIMO_layers × Aggregation
  = 400 MHz × 7.4 × 4 × 1 = 11.8 Gbps (single carrier)
  
With carrier aggregation (8 carriers):
R = 11.8 × 8 = 94 Gbps (theoretical)

Practical: 2-5 Gbps (typical deployment, moderate SINR)
```

---

### Applications

**Enhanced Mobile Broadband (eMBB)**:
```
Use case: Stadiums, airports, malls (high user density)
- 1000+ users per cell
- Aggregate: 20-50 Gbps per gNB
- Per-user: 20-50 Mbps (shared capacity)

Deployment: Small cells (50-200 m range)
- Dense urban: 1 cell per block
- Outdoor-to-indoor: Penetration challenges (require indoor cells)
```

**Fixed Wireless Access (FWA)**:
```
Use case: Home/business internet (alternative to fiber/cable)
- CPE (Customer Premises Equipment) on roof/window
- LOS to nearby gNB (200-500 m)
- Throughput: 1-3 Gbps (comparable to gigabit fiber)
- Latency: 10-20 ms

Advantage: Rapid deployment (no trenching)
Disadvantage: Weather-sensitive, requires LOS or near-LOS
```

**Industrial IoT / URLLC**:
```
Use case: Factory automation, robotics
- Latency: 1-5 ms (mini-slot transmission)
- Reliability: 99.999% (5 nines)
- Capacity: 10-100 Mbps per device

Private 5G networks:
- Dedicated spectrum (CBRS, local licensing)
- On-premises gNB (security, low latency)
```

---

## 🔮 Beyond 5G: Sub-THz (6G)

**6G target frequencies**: 100-300 GHz (D-band, G-band)

### Why Sub-THz for 6G?

```
Bandwidth availability:
- 92-114.25 GHz (WRC-19): 22 GHz continuous
- 130-174.8 GHz: 44 GHz
- 200-260 GHz: 60 GHz (being considered)

Total: 100+ GHz spectrum (vs. 5 GHz for all cellular below 6 GHz!)

Peak data rate (conservative estimate):
BW = 10 GHz, SE = 5 bits/s/Hz, MIMO = 8
R = 10 × 5 × 8 = 400 Gbps

Target: 100 Gbps-1 Tbps (100× faster than 5G)
```

---

### Sub-THz Challenges

**1. Path Loss**:
```
300 GHz FSPL (100 m): 122 dB
Compensation:
- Ultra-massive MIMO: 1024+ elements → 30 dB gain
- Dense deployment: 10-50 m cell radius (pico/femto cells)
- Relay/RIS: Intelligent reflecting surfaces
```

**2. Hardware Limitations**:
```
PA (Power Amplifier):
- 28 GHz: 20-30 dBm per element (mature GaN technology)
- 300 GHz: 5-10 dBm per element (InP, SiGe limited)

Phase shifters:
- 28 GHz: 4-6 bit resolution, low loss
- 300 GHz: 2-3 bit (lossy, expensive)

ADC/DAC:
- Nyquist rate: 2× bandwidth
- 10 GHz BW → 20 Gsps ADC/DAC
- Power: 10-100 W per RF chain (prohibitive for mobile)

Solution: Ultra-low-power circuits (sub-threshold, approximate computing)
```

**3. Beam Alignment**:
```
Beamwidth (1024-element array at 300 GHz):
θ ≈ λ / (N·d) ≈ 1 mm / (32 × 0.5 mm) = 0.06 rad ≈ 3.5°

Challenge: <4° beam → precise alignment required
- Rotation/motion: 10°/s movement → beam misalignment in 0.35 s
- Solution: 100+ Hz beam tracking

Beam switching latency:
- Analog: <1 μs (phase shifter settling)
- Digital: 10-100 μs (baseband processing)
- Requirement: <1 ms for mobility
```

---

### 6G Candidate Technologies

**Reconfigurable Intelligent Surface (RIS)**:
```
Concept: Passive reflector with electronically tunable elements

Application:
- Coverage extension: Reflect signal around obstacles
- Virtual LOS: Create alternative paths in NLOS
- Energy efficiency: Passive (no power amplifier)

Example:
- RIS: 1024 elements (1m × 1m panel)
- Placement: Building wall
- Reflect 300 GHz signal from BS to blocked UE
- Gain: 20-30 dB (overcome blockage loss)

Status: Research prototypes, not yet standardized
```

**Wireless Fiber (WF)**:
```
Concept: Short-range (1-10 m), fiber-like data rates

Use case: Wireless backhaul, kiosk downloads, data center links
- Frequency: 300 GHz
- Bandwidth: 20-50 GHz (entire band)
- Data rate: 100-200 Gbps
- Range: <10 m (LOS required)

Advantage: 100× faster than WiFi, no fiber installation
Disadvantage: Ultra-short range, perfect alignment needed
```

**OAM (Orbital Angular Momentum) Multiplexing**:
```
Concept: Use twisted EM waves (vortex beams) as additional dimension

Orthogonal OAM modes: l = 0, ±1, ±2, ...
- Each mode carries independent data stream
- Separation by phase profile (not frequency)

Capacity:
C = N_OAM × N_MIMO × B × SE

Example (N_OAM = 4, N_MIMO = 8, B = 10 GHz, SE = 5):
C = 4 × 8 × 10 × 5 = 1.6 Tbps

Status: Lab demonstrations, far from practical (alignment critical)
```

---

## 📡 Automotive Radar (mmWave)

**77-81 GHz radar** for autonomous vehicles.

### System Parameters

```
Frequency: 76-81 GHz (5 GHz bandwidth allocated)
Modulation: FMCW (Frequency-Modulated Continuous Wave)
Range resolution: Δr = c / (2·BW) = 3 cm (for 5 GHz BW)
Velocity resolution: Doppler shift
Angular resolution: Beamforming (MIMO radar)

Performance:
- Detection range: 200+ m (long-range radar)
- Velocity: ±70 m/s (Doppler)
- Angle: ±60° (wide FoV for short-range, ±10° for long-range)
- Update rate: 10-20 Hz

Applications:
- Adaptive Cruise Control (ACC)
- Collision avoidance
- Blind-spot detection
- Parking assistance
```

**MIMO radar**:
```
Virtual array: N_TX × N_RX elements
- Physical: 3 TX, 4 RX = 12 elements
- Virtual: 3 × 4 = 12 unique TX-RX pairs (phase centers)
- Angular resolution: Equivalent to 12-element receive array

Imaging:
- Range-Doppler map (2D)
- Range-Angle map (2D)
- 3D point cloud (range-azimuth-elevation)

Example (Bosch 5th gen):
- TX: 3 antennas
- RX: 4 antennas
- Virtual: 12 elements
- Angular resolution: 1° (azimuth)
```

---

## 🧮 Link Budget Example (28 GHz)

```
System: 5G FR2 mmWave (28 GHz, 100 MHz BW)

Transmitter (gNB):
- TX power per element: 23 dBm (200 mW)
- Number of elements: 64
- Total TX power: 23 + 10·log₁₀(64) = 41 dBm
- Analog beamforming gain: 18 dB (64 elements, single beam)
- EIRP: 41 + 18 = 59 dBm

Path:
- Distance: 100 m
- FSPL: 32.4 + 20·log₁₀(28000) + 20·log₁₀(0.1) = 101 dB
- Atmospheric absorption: 0.01 dB (negligible)
- Blockage margin: 10 dB (foliage, wall)
- Total loss: 111 dB

Receiver (UE):
- RX antenna gain: 10 dB (16-element array)
- Noise figure: 7 dB
- Thermal noise: -174 + 10·log₁₀(100 MHz) + 7 = -87 dBm

Received signal:
RX power = 59 - 111 + 10 = -42 dBm

SNR:
SNR = -42 - (-87) = 45 dB

Throughput (Shannon):
C = 100 MHz × log₂(1 + 10^(45/10)) = 100 MHz × 15 = 1.5 Gbps

Practical (256-QAM, rate-5/6, 75% efficiency):
R = 100 MHz × 7.4 × 0.75 = 555 Mbps

Margin: 45 - 20 (required SNR for 256-QAM) = 25 dB ✅
```

---

## 🐍 Python Example: mmWave Path Loss Calculator

```python
import numpy as np

def mmwave_path_loss(freq_ghz, distance_m, rain_rate_mm_h=0):
    """
    Calculate mmWave path loss including atmospheric effects.
    
    Args:
        freq_ghz: Frequency (GHz)
        distance_m: Distance (meters)
        rain_rate_mm_h: Rain rate (mm/h, optional)
    
    Returns:
        Total path loss (dB)
    """
    # Free-space path loss
    fspl = 32.4 + 20*np.log10(freq_ghz*1000) + 20*np.log10(distance_m/1000)
    
    # Atmospheric absorption (simplified model)
    if freq_ghz < 30:
        attenuation_db_km = 0.1
    elif freq_ghz < 100:
        attenuation_db_km = 0.5 + 0.05 * (freq_ghz - 30)
    else:
        attenuation_db_km = 4 + 0.02 * (freq_ghz - 100)
    
    atmospheric_loss = attenuation_db_km * (distance_m / 1000)
    
    # Rain attenuation (ITU-R model)
    if rain_rate_mm_h > 0:
        k = 0.0001 * freq_ghz**2
        alpha = 1.0
        rain_loss = k * rain_rate_mm_h**alpha * (distance_m / 1000)
    else:
        rain_loss = 0
    
    total_loss = fspl + atmospheric_loss + rain_loss
    
    print(f"Frequency: {freq_ghz} GHz, Distance: {distance_m} m")
    print(f"  FSPL: {fspl:.1f} dB")
    print(f"  Atmospheric: {atmospheric_loss:.2f} dB")
    print(f"  Rain: {rain_loss:.2f} dB")
    print(f"  Total: {total_loss:.1f} dB")
    
    return total_loss

def beamforming_gain(n_elements, beamwidth_deg=None):
    """
    Calculate antenna array gain.
    
    Args:
        n_elements: Number of antenna elements
        beamwidth_deg: Optional 3dB beamwidth (degrees)
    
    Returns:
        Gain (dB)
    """
    gain_db = 10 * np.log10(n_elements)
    
    if beamwidth_deg:
        # Approximate directivity from beamwidth
        directivity = 41253 / (beamwidth_deg**2)
        gain_from_bw = 10 * np.log10(directivity)
        print(f"Array gain (element count): {gain_db:.1f} dB")
        print(f"Gain from beamwidth: {gain_from_bw:.1f} dB")
        gain_db = max(gain_db, gain_from_bw)
    
    return gain_db

# Example: 5G mmWave link budget
print("=== 5G mmWave Link Budget ===\n")

freq = 28  # GHz
distance = 100  # meters
tx_power_dbm = 23  # dBm per element
n_tx_elements = 64
n_rx_elements = 16

path_loss = mmwave_path_loss(freq, distance, rain_rate_mm_h=0)
tx_gain = beamforming_gain(n_tx_elements)
rx_gain = beamforming_gain(n_rx_elements)

eirp = tx_power_dbm + tx_gain
rx_power = eirp - path_loss + rx_gain

noise_figure = 7  # dB
bandwidth_mhz = 100
thermal_noise = -174 + 10*np.log10(bandwidth_mhz * 1e6) + noise_figure

snr = rx_power - thermal_noise

print(f"\nLink Budget:")
print(f"  EIRP: {eirp:.1f} dBm")
print(f"  RX power: {rx_power:.1f} dBm")
print(f"  Noise: {thermal_noise:.1f} dBm")
print(f"  SNR: {snr:.1f} dB")

capacity_gbps = (bandwidth_mhz * np.log2(1 + 10**(snr/10))) / 1000
print(f"  Shannon capacity: {capacity_gbps:.2f} Gbps")
```

---

## 🎯 Summary Comparison

| Aspect | Sub-6 GHz | mmWave (24-52 GHz) | Sub-THz (100-300 GHz) |
|--------|-----------|--------------------|-----------------------|
| **Bandwidth** | 100 MHz | 400 MHz-2 GHz | 10-50 GHz |
| **Peak Rate** | 1 Gbps | 10 Gbps | 100+ Gbps |
| **Range** | 1-5 km | 100-500 m | 10-100 m |
| **Propagation** | NLOS-friendly | LOS-preferred | LOS-only |
| **Mobility** | Excellent | Good | Limited |
| **Beamforming** | Optional | Mandatory | Ultra-massive |
| **Applications** | Wide-area | Dense urban, FWA | Indoor, backhaul |

---

## 📚 Further Reading

### Textbooks
- **Rappaport et al.**, *Millimeter Wave Wireless Communications* - Comprehensive mmWave treatment
- **Akyildiz et al.**, *Terahertz Band Communication* - THz fundamentals
- **Rangan et al.**, *Millimeter-Wave Cellular Wireless Networks* - 5G mmWave

### Key Papers
- **Rappaport et al.** (2013): "Millimeter Wave Mobile Communications for 5G" - Seminal 5G mmWave paper
- **Alsharif et al.** (2020): "Sixth Generation (6G) Wireless Networks" - 6G vision including THz
- **ITU-R P.676**: Atmospheric attenuation models (O₂, H₂O)

### Standards
- **3GPP TS 38.104**: 5G NR Base Station radio transmission/reception (FR2 specs)
- **IEEE 802.11ad/ay**: WiGig 60 GHz mmWave WiFi
- **IEEE 802.15.3d**: 100 Gbps WPAN (THz band)

### Related Topics
- [[MIMO & Spatial Multiplexing]] - Beamforming foundations
- [[OFDM & Multicarrier Modulation]] - mmWave uses OFDM
- [[Adaptive Modulation & Coding (AMC)]] - Critical for variable mmWave channels
- [[Atmospheric Effects (Ionospheric, Tropospheric)]] - Propagation background
- [[Terahertz (THz) Technology]] - THz-specific content (quantum cascade lasers, imaging)
- [[Real-World System Examples]] - 5G NR deployments

---

**Summary**: mmWave (24-300 GHz) and THz (0.3-10 THz) offer massive bandwidth (100× more than sub-6 GHz) enabling multi-gigabit to terabit wireless. 5G NR FR2 (24-52 GHz) delivers 2-10 Gbps with 100-500 m range using massive MIMO beamforming (64-256 elements). Path loss increases 20-40 dB vs. sub-6 GHz, requiring directional antennas and dense small-cell deployment. Atmospheric absorption (O₂ at 60 GHz, H₂O at 183 GHz) and rain attenuation limit range. Blockage (human body 20-40 dB, walls 30-80 dB) makes LOS critical. Beamforming is mandatory (analog or hybrid) for coverage. Applications: urban hotspots, fixed wireless access, industrial IoT. 6G targets sub-THz (100-300 GHz) for 100 Gbps-1 Tbps with ultra-massive MIMO (1024+ elements), intelligent surfaces (RIS), and 10-50 m cell radius. Automotive radar (77-81 GHz) uses FMCW for 3 cm range resolution. mmWave/THz = ultra-high bandwidth, ultra-short range, ultra-directional—requires paradigm shift in network architecture.
