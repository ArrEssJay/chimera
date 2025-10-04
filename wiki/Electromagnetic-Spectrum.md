# Electromagnetic Spectrum

[[Home]] | **Foundation** | [[Maxwell's Equations & Wave Propagation]]

---

## Overview

The **electromagnetic (EM) spectrum** encompasses all frequencies of electromagnetic radiation, from extremely low frequency (ELF) radio waves to ultra-high energy gamma rays. **All EM waves travel at the speed of light** ($c \approx 3 \times 10^8$ m/s in vacuum) and obey [[Maxwell's Equations & Wave Propagation|Maxwell's equations]].

**Key relationship**:

$$
c = \lambda f
$$

Where:
- $c$ = Speed of light (299,792,458 m/s)
- $\lambda$ = Wavelength (meters)
- $f$ = Frequency (Hz)

**Energy per photon** (quantum perspective):

$$
E = h f
$$

Where $h = 6.626 \times 10^{-34}$ J·s (Planck's constant)

---

## Spectrum Bands & Applications

### Radio Frequencies (RF): 3 kHz - 300 GHz

#### **ELF (Extremely Low Frequency): 3 Hz - 3 kHz**
- **Wavelength**: 100,000 km - 100 km
- **Applications**: Submarine communication (penetrates seawater), geophysical surveys
- **Propagation**: Earth-ionosphere waveguide, minimal attenuation
- **Example**: 76 Hz US Navy submarine comms

---

#### **VLF (Very Low Frequency): 3 kHz - 30 kHz**
- **Wavelength**: 100 km - 10 km
- **Applications**: Navigation (LORAN), time signals, lightning detection
- **Propagation**: Ground wave, ionospheric reflection
- **Example**: 24 kHz VLF navigation beacon

---

#### **LF (Low Frequency): 30 kHz - 300 kHz**
- **Wavelength**: 10 km - 1 km
- **Applications**: AM radio (longwave), RFID, aviation beacons
- **Propagation**: Ground wave (stable day/night), ionospheric at night
- **Example**: 153 kHz longwave broadcast

---

#### **MF (Medium Frequency): 300 kHz - 3 MHz**
- **Wavelength**: 1 km - 100 m
- **Applications**: AM radio (broadcast), maritime communication
- **Propagation**: Ground wave (daytime), skywave (nighttime)
- **Example**: 540-1600 kHz AM broadcast band

---

#### **HF (High Frequency): 3 MHz - 30 MHz**
- **Wavelength**: 100 m - 10 m
- **Applications**: Shortwave radio, amateur radio, over-the-horizon radar
- **Propagation**: Ionospheric refraction (skywave), global reach
- **Example**: 14.2 MHz amateur band, intercontinental comms

---

#### **VHF (Very High Frequency): 30 MHz - 300 MHz**
- **Wavelength**: 10 m - 1 m
- **Applications**: FM radio (88-108 MHz), TV broadcast, aviation, marine
- **Propagation**: Line-of-sight (LOS), occasional tropospheric ducting
- **Example**: 146 MHz amateur band, 120 MHz air traffic control

---

#### **UHF (Ultra High Frequency): 300 MHz - 3 GHz**
- **Wavelength**: 1 m - 10 cm
- **Applications**: TV, cellular (GSM/LTE), GPS, WiFi (2.4 GHz), Bluetooth
- **Propagation**: LOS, building penetration moderate, rain attenuation minimal
- **Example**: 1.575 GHz GPS L1, 2.4 GHz ISM band

---

#### **SHF (Super High Frequency): 3 GHz - 30 GHz**
- **Wavelength**: 10 cm - 1 cm
- **Applications**: Satellite comms, radar, 5G (3.5 GHz), WiFi (5-6 GHz), point-to-point links
- **Propagation**: LOS required, rain fade significant, atmospheric absorption
- **Example**: 5.8 GHz WiFi, 12 GHz satellite downlink (Ku-band)

---

#### **EHF (Extremely High Frequency): 30 GHz - 300 GHz**
- **Wavelength**: 1 cm - 1 mm
- **Applications**: mmWave 5G (28/39 GHz), automotive radar (77 GHz), radio astronomy
- **Propagation**: Severe rain/foliage attenuation, oxygen absorption peak @ 60 GHz
- **Example**: 39 GHz 5G, 94 GHz cloud radar

**60 GHz oxygen absorption**: 15 dB/km (used for secure short-range comms)

---

### Terahertz (THz) Gap: 300 GHz - 10 THz

- **Wavelength**: 1 mm - 30 μm
- **Applications**: Security imaging, spectroscopy, biomedical sensing, **[[AID Protocol|AID Protocol]]** (1.875 THz)
- **Propagation**: Atmospheric absorption severe (H₂O lines), limited range
- **Technology**: Quantum cascade lasers (QCLs), photoconductive switches
- **Status**: "THz gap" (historically difficult to generate/detect)

**Key THz features**:
- Non-ionizing (safe for biological tissue, unlike X-rays)
- Penetrates clothing, paper, plastics (not metal)
- High spatial resolution (sub-mm)
- Strong water absorption (limits biomedical depth)

**See**: [[Terahertz (THz) Technology]] for detailed discussion

---

### Infrared (IR): 300 GHz - 430 THz

#### **Far-IR (FIR): 300 GHz - 20 THz**
- **Wavelength**: 1 mm - 15 μm
- **Applications**: Thermal imaging, astronomy, spectroscopy
- **Source**: Blackbody radiation (room temperature objects peak ~10 μm)

#### **Mid-IR (MIR): 20 THz - 120 THz**
- **Wavelength**: 15 μm - 2.5 μm
- **Applications**: Night vision, chemical sensing (molecular fingerprints), CO₂ lasers

#### **Near-IR (NIR): 120 THz - 430 THz**
- **Wavelength**: 2.5 μm - 700 nm
- **Applications**: Fiber optic comms (1550 nm), remote controls, biomedical imaging
- **Atmospheric window**: 1.3-1.55 μm (low loss in silica fiber)

---

### Visible Light: 430 THz - 750 THz

- **Wavelength**: 700 nm (red) - 400 nm (violet)
- **Frequencies**:
  - Red: ~430 THz (700 nm)
  - Yellow: ~510 THz (590 nm)
  - Green: ~560 THz (535 nm)
  - Blue: ~670 THz (450 nm)
  - Violet: ~750 THz (400 nm)
- **Applications**: Human vision, optical comms (free-space), LiDAR, photovoltaics
- **Energy**: 1.6-3.1 eV per photon (non-ionizing)

**Solar spectrum**: Peaks at ~550 nm (green), corresponds to peak sensitivity of human eye (photopic vision)

---

### Ultraviolet (UV): 750 THz - 30 PHz

#### **Near-UV (NUV): 750 THz - 1.5 PHz**
- **Wavelength**: 400 nm - 200 nm
- **Applications**: Sterilization, fluorescence microscopy, photolithography
- **Biological effects**: Tanning, vitamin D synthesis, DNA damage (UVB)

#### **Far-UV (FUV): 1.5 PHz - 30 PHz**
- **Wavelength**: 200 nm - 10 nm
- **Applications**: Extreme sterilization, plasma diagnostics
- **Absorption**: Strongly absorbed by atmosphere (ozone layer blocks < 290 nm)

**UVC (< 280 nm)**: Germicidal (destroys DNA/RNA), used in air/water purification

---

### X-Rays: 30 PHz - 30 EHz

- **Wavelength**: 10 nm - 0.01 nm
- **Energy**: 100 eV - 100 keV
- **Applications**: Medical imaging, crystallography, security screening, astronomy
- **Generation**: Bremsstrahlung (electron deceleration), synchrotron radiation
- **Biological effects**: **Ionizing** (breaks chemical bonds, causes mutations)

**Soft X-rays** (0.1-10 keV): Water window imaging, biological samples
**Hard X-rays** (10-100 keV): Penetrates tissue, bone imaging (radiography)

---

### Gamma Rays: > 30 EHz

- **Wavelength**: < 0.01 nm
- **Energy**: > 100 keV
- **Sources**: Radioactive decay, nuclear reactions, cosmic rays, pulsars
- **Applications**: Cancer therapy (radiotherapy), sterilization, astrophysics
- **Detection**: Scintillation detectors, Compton scattering
- **Biological effects**: **Highly ionizing** (severe DNA damage, cell death)

**Cosmic gamma rays**: Up to TeV energies (10¹² eV), from supernovae, black holes

---

## Atmospheric Transmission Windows

**Earth's atmosphere is opaque to most EM spectrum**. Only certain "windows" allow propagation:

| Band | Frequency/Wavelength | Transmission | Absorbers |
|------|----------------------|--------------|-----------|
| **RF (< 30 GHz)** | All RF below mmWave | Excellent | Ionosphere (HF reflection) |
| **mmWave (30-300 GHz)** | 1-10 mm | Poor | Water vapor, oxygen (60 GHz) |
| **THz (0.3-10 THz)** | 30 μm - 1 mm | Very poor | Water vapor, CO₂ |
| **Far-IR** | 15-300 μm | Poor | H₂O, CO₂, O₃ |
| **Mid-IR** | 2.5-15 μm | Moderate | H₂O (many lines), CO₂ (15 μm) |
| **Near-IR** | 0.7-2.5 μm | Good | H₂O (weak bands) |
| **Visible** | 400-700 nm | Excellent | Rayleigh scattering (sky is blue) |
| **Near-UV** | 300-400 nm | Good | Ozone (< 320 nm) |
| **UVC / X-ray / Gamma** | < 280 nm | Blocked | Ozone, O₂, N₂ |

**Implications**:
- **Ground-to-satellite comms**: Use RF (microwaves) or optical (laser comms)
- **THz security imaging**: Indoor only (outdoor = severe H₂O absorption)
- **Radio astronomy**: "Radio window" (few MHz - 30 GHz) and "optical window" (visible/NIR)

---

## Ionizing vs Non-Ionizing Radiation

**Critical distinction**:

### Non-Ionizing (< 3.1 eV, < 1 PHz)

**Photon energy insufficient to ionize atoms**:

- **RF/Microwave/IR/Visible**: Causes heating (dielectric loss), molecular vibration/rotation
- **Biological effects**: Thermal (tissue heating), non-thermal (debated, e.g., RF-EMF effects)
- **Safety**: Exposure limits based on specific absorption rate (SAR, W/kg)

**Example**: WiFi (2.4 GHz, $E = hf = 10^{-5}$ eV) → Pure heating, no ionization

---

### Ionizing (> 10 eV, > 2.4 PHz)

**Photon energy sufficient to eject electrons from atoms**:

- **UV (high-energy), X-rays, Gamma rays**: Breaks chemical bonds, damages DNA
- **Biological effects**: Mutations, cancer, acute radiation syndrome (high dose)
- **Safety**: Exposure limits based on dose (Sieverts, Sv)

**Ionization threshold**: ~10 eV for biological molecules (double-strand DNA breaks at ~20 eV)

**Example**: X-ray (30 keV) → Ejects inner-shell electrons, Compton scattering, DNA damage

---

## Frequency Allocation & Regulation

**International Telecommunication Union (ITU)** allocates spectrum globally:

### Key Allocated Bands

| Service | Frequency | Regulation |
|---------|-----------|------------|
| AM Radio | 530-1710 kHz | Licensed broadcast |
| FM Radio | 88-108 MHz | Licensed broadcast |
| TV (VHF) | 54-216 MHz | Licensed broadcast (analog legacy) |
| Aviation | 108-137 MHz | Regulated (safety of life) |
| Marine VHF | 156-162 MHz | Regulated |
| Cellular (US) | 600-6000 MHz | Licensed (carriers) |
| GPS | 1.176-1.575 GHz | Protected (military/civilian) |
| WiFi (2.4 GHz) | 2.400-2.4835 GHz | **ISM band** (unlicensed) |
| WiFi (5 GHz) | 5.150-5.850 GHz | **U-NII** (unlicensed, indoor/outdoor rules) |
| 5G mmWave | 24-47 GHz | Licensed (auction) |
| Satellite (Ka) | 26.5-40 GHz | Licensed |

**ISM bands** (Industrial, Scientific, Medical): Unlicensed, shared use, higher interference
- 902-928 MHz (US), 2.4-2.5 GHz (global), 5.725-5.875 GHz

---

## Wavelength vs Antenna Size

**Rule of thumb**: Efficient antennas are typically $\lambda/2$ or $\lambda/4$ in size.

**Examples**:

| Frequency | Wavelength | Typical Antenna |
|-----------|------------|-----------------|
| 150 kHz (LF) | 2000 m | 500 m tower (impractical!) |
| 1 MHz (AM) | 300 m | 75 m vertical mast |
| 100 MHz (FM) | 3 m | 1.5 m whip (λ/2 dipole) |
| 1 GHz (cellular) | 30 cm | 7.5 cm patch (λ/4) |
| 10 GHz (satellite) | 3 cm | 1.5 cm patch array |
| 300 GHz (mmWave) | 1 mm | 0.25 mm array element |
| 1.875 THz (AID) | 160 μm | 40 μm aperture (phased array) |

**Implication**: Higher frequencies enable **smaller antennas and phased arrays**, but propagation is poorer.

---

## Spectrum Utilization Trends

### Historical Progression

**1900s**: MF/HF (AM radio, maritime)
**1950s**: VHF/UHF (FM, TV, early mobile)
**1990s**: SHF (cellular 2G/3G, WiFi, GPS)
**2010s**: EHF (5G mmWave, 60 GHz WiGig)
**2020s**: THz (security, 6G research, biomedical)

**Driver**: **Spectrum congestion** → Move to higher frequencies for bandwidth
- VHF/UHF: Crowded (licensed, competitive)
- mmWave: Abundant spectrum (GHz of bandwidth available)
- THz: Virtually unlimited (atmospheric absorption limits range, but OK for short-range)

---

## Propagation Characteristics by Band

### Long Wavelengths (LF/MF/HF)

**Advantages**:
- Ground wave propagation (stable, follows Earth curvature)
- Ionospheric reflection (HF skywave → global reach)
- Penetrates buildings, foliage, water

**Disadvantages**:
- Large antennas required
- Low bandwidth (kHz)
- Crowded spectrum

---

### Medium Wavelengths (VHF/UHF)

**Advantages**:
- Moderate antenna size
- Good building penetration (lower UHF)
- Balanced range vs bandwidth

**Disadvantages**:
- Line-of-sight propagation (VHF)
- Spectrum congestion

---

### Short Wavelengths (SHF/EHF/THz)

**Advantages**:
- Huge bandwidth (GHz available)
- Small antennas (phased arrays feasible)
- Narrow beams (spatial reuse, security)

**Disadvantages**:
- Severe atmospheric attenuation (rain, oxygen, water vapor)
- No building penetration
- Requires line-of-sight

**Oxygen absorption**: 60 GHz (15 dB/km) → Secure short-range comms (signals don't travel far)
**Water vapor**: THz (>100 dB/km) → Indoor/short-range only

---

## Summary Table: Spectrum at a Glance

| Band | Frequency | Wavelength | Key Applications | Propagation | Ionizing? |
|------|-----------|------------|------------------|-------------|-----------|
| ELF | 3 Hz - 3 kHz | 100,000 km - 100 km | Submarine comms | Earth-ionosphere waveguide | No |
| VLF | 3-30 kHz | 100-10 km | Navigation, time signals | Ground wave | No |
| LF | 30-300 kHz | 10-1 km | Longwave radio, RFID | Ground wave, ionosphere | No |
| MF | 300 kHz - 3 MHz | 1 km - 100 m | AM broadcast | Ground/skywave | No |
| HF | 3-30 MHz | 100-10 m | Shortwave, amateur | Ionospheric refraction | No |
| VHF | 30-300 MHz | 10-1 m | FM, TV, aviation | Line-of-sight | No |
| UHF | 300 MHz - 3 GHz | 1 m - 10 cm | Cellular, WiFi, GPS | LOS, some penetration | No |
| SHF | 3-30 GHz | 10-1 cm | Satellite, 5G, radar | LOS, rain fade | No |
| EHF | 30-300 GHz | 1 cm - 1 mm | mmWave 5G, radar | Severe attenuation | No |
| THz | 0.3-10 THz | 1 mm - 30 μm | Imaging, spectroscopy, AID | Very limited (H₂O) | No |
| Far-IR | 10-120 THz | 30-2.5 μm | Thermal imaging | Atmospheric windows | No |
| Near-IR | 120-430 THz | 2.5-0.7 μm | Fiber optics, night vision | Good (1.55 μm window) | No |
| Visible | 430-750 THz | 700-400 nm | Vision, optical comms | Excellent | No |
| UV | 750 THz - 30 PHz | 400-10 nm | Sterilization, lithography | Absorbed (ozone) | **Yes (high-energy UV)** |
| X-ray | 30 PHz - 30 EHz | 10-0.01 nm | Medical imaging, crystallography | Blocked by atmosphere | **Yes** |
| Gamma | > 30 EHz | < 0.01 nm | Radiotherapy, astrophysics | Blocked by atmosphere | **Yes** |

---

## Related Topics

- **[[Maxwell's Equations & Wave Propagation]]**: Mathematical foundation of EM waves
- **[[Free-Space Path Loss (FSPL)]]**: Frequency-dependent propagation loss
- **[[Terahertz (THz) Technology]]**: Applications and challenges in THz band
- **[[AID Protocol Case Study]]**: 1.875 THz carrier for neural modulation
- **Antenna Theory**: Design principles for frequency-specific antennas (TBD)
- **Atmospheric Propagation**: Absorption, refraction, ducting effects (TBD)

---

**Next**: [[Antenna Theory Basics]] (TBD) - How to design antennas for different spectrum bands

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
