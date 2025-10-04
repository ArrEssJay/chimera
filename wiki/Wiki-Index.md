# Wiki Index - Complete Cross-Reference Guide

**Comprehensive index of all Chimera wiki pages with cross-references and dependencies**

This index provides a complete map of the wiki documentation, showing relationships between pages and recommended reading order.

---

## Table of Contents

- [Quick Navigation](#quick-navigation)
- [Learning Paths](#learning-paths)
- [Alphabetical Index](#alphabetical-index)
- [By Topic](#by-topic)
- [By Difficulty](#by-difficulty)
- [Cross-Reference Map](#cross-reference-map)
- [External References](#external-references)

---

## Quick Navigation

**Start Here**: [[Home]] | [[Quick-Start-Guide]]

**Most Referenced Pages**:
- [[QPSK-Modulation]] - Core modulation scheme (referenced by 12 pages)
- [[Signal-to-Noise-Ratio-(SNR)]] - Fundamental metric (referenced by 15 pages)
- [[Forward-Error-Correction-(FEC)]] - Error control overview (referenced by 8 pages)
- [[Constellation-Diagrams]] - Visual understanding (referenced by 10 pages)

**Essential Reference**:
- [[Glossary]] - Acronyms and definitions
- [[Formula-Reference-Card]] - Quick math lookup
- [Bibliography](../docs/Bibliography.md) - Complete reference list

---

## Learning Paths

### Path 1: Complete Beginner (4 weeks)
Week 1:
1. [[What-Are-Symbols]] → [[IQ-Representation]] → [[Constellation-Diagrams]]
2. [[Signal-to-Noise-Ratio-(SNR)]] → [[Additive-White-Gaussian-Noise-(AWGN)]]

Week 2:
3. [[QPSK-Modulation]] → [[Binary-Phase-Shift-Keying-(BPSK)]]
4. [[Free-Space-Path-Loss-(FSPL)]] → [[Link-Loss-vs-Noise]]

Week 3:
5. [[Bit-Error-Rate-(BER)]] → [[Forward-Error-Correction-(FEC)]]
6. [[LDPC-Codes]]

Week 4:
7. [[Signal-Chain-(End-to-End-Processing)]] → [[Complete-Link-Budget-Analysis]]
8. **Try Chimera simulator!**

### Path 2: Software Engineer (2 weeks)
1. [[Signal-Chain-(End-to-End-Processing)]] - Understand the pipeline
2. [[QPSK-Modulation]] + [[Constellation-Diagrams]] - Visualization
3. [[Forward-Error-Correction-(FEC)]] + [[LDPC-Codes]] - Error handling
4. [[Additive-White-Gaussian-Noise-(AWGN)]] - Channel model
5. [[Bit-Error-Rate-(BER)]] - Performance metric
6. [[Real-World-System-Examples]] - See it in practice

### Path 3: RF Engineer (Jump to topics)
- Physical layer: [[Maxwell's-Equations-&-Wave-Propagation]] → [[Antenna-Theory-Basics]]
- Propagation: [[Free-Space-Path-Loss-(FSPL)]] → [[Multipath-Propagation-&-Fading-(Rayleigh,-Rician)]]
- Link design: [[Complete-Link-Budget-Analysis]] → [[Noise-Sources-&-Noise-Figure]]
- Advanced: [[OFDM-&-Multicarrier-Modulation]] → [[MIMO-&-Spatial-Multiplexing]]

---

## Alphabetical Index

All 71 wiki pages:

| Page | Topics | Prerequisites | Related |
|------|--------|--------------|---------|
| [[8PSK-&-Higher-Order-PSK]] | Higher-order phase modulation | QPSK, Constellation Diagrams | BPSK, QAM |
| [[AID-Protocol-Case-Study]] | HRP application to THz neuromodulation | HRP Framework, THz Technology | Orch-OR, Biophysical Coupling |
| [[Acoustic-Heterodyning]] | Nonlinear acoustic mixing | IMD in Biology | Frey Effect, Non-Linear Demod |
| [[Adaptive-Modulation-&-Coding-(AMC)]] | Link adaptation | Modulation, FEC, SNR | Real-World Examples |
| [[Additive-White-Gaussian-Noise-(AWGN)]] | Fundamental channel model | SNR | Channel Models, BER |
| [[Amplitude-Shift-Keying-(ASK)]] | Amplitude modulation | Baseband vs Passband | OOK, FSK, BPSK |
| [[Antenna-Theory-Basics]] | Gain, beamwidth, impedance | EM Spectrum, Maxwell's Equations | Wave Polarization |
| [[Atmospheric-Effects-(Ionospheric,-Tropospheric)]] | Ionosphere/troposphere effects | Propagation Modes | Weather Effects |
| [[Baseband-vs-Passband-Signals]] | Frequency translation | EM fundamentals | IQ Representation |
| [[Binary-Phase-Shift-Keying-(BPSK)]] | 2-phase modulation | Baseband vs Passband | QPSK, 8PSK |
| [[Biophysical-Coupling-Mechanism]] | CHIMERA field mechanism | Quantum Coherence, HRP Framework | Orch-OR, Microtubules |
| [[Bit-Error-Rate-(BER)]] | Error performance metric | Modulation, SNR, FEC | Energy Ratios |
| [[Block-Codes-(Hamming,-BCH,-Reed-Solomon)]] | Linear block codes | Hamming Distance | Convolutional Codes, LDPC |
| [[Channel-Equalization]] | Mitigating ISI | Multipath, Synchronization | Real-World Examples |
| [[Channel-Models-(Rayleigh-&-Rician)]] | Fading statistics | Multipath, AWGN | Propagation |
| [[Complete-Link-Budget-Analysis]] | System power budget | FSPL, Noise, SNR | Energy Ratios |
| [[Constellation-Diagrams]] | IQ visualization | IQ Representation | All modulation schemes |
| [[Convolutional-Codes-&-Viterbi-Decoding]] | Trellis codes | FEC, Hamming Distance | Turbo Codes, LDPC |
| [[Electromagnetic-Spectrum]] | Radio frequency bands | Maxwell's Equations | Power Density |
| [[Energy-Ratios-(Es-N0-and-Eb-N0)]] | Symbol/bit energy | SNR, Modulation | BER, Link Budget |
| [[Formula-Reference-Card]] | Quick math reference | All topics | Home |
| [[Forward-Error-Correction-(FEC)]] | Error control overview | BER, Shannon's Theorem | LDPC, Turbo, Polar |
| [[Free-Space-Path-Loss-(FSPL)]] | Friis equation | EM fundamentals | Link Budget |
| [[Frequency-Shift-Keying-(FSK)]] | Frequency modulation | Baseband vs Passband | ASK, BPSK |
| [[Frey-Microwave-Auditory-Effect]] | EM to acoustic | THz Technology | Acoustic Heterodyning |
| [[Glossary]] | Acronyms and definitions | None | All pages |
| [[Hamming-Distance-&-Error-Detection]] | Code distance | None | Block Codes, FEC |
| [[Home]] | Main table of contents | None | Quick Start Guide |
| [[Hyper-Rotational-Physics-(HRP)-Framework]] | M-theory extension | Advanced physics | AID Protocol, Orch-OR |
| [[IQ-Representation]] | Complex signal representation | Baseband vs Passband | Constellation Diagrams |
| [[Intermodulation-Distortion-in-Biology]] | Biological nonlinearity | None | Acoustic Heterodyning |
| [[LDPC-Codes]] | Low-density parity check | FEC, Shannon's Theorem | Chimera implementation |
| [[Link-Loss-vs-Noise]] | Attenuation vs noise | FSPL, AWGN | SNR, Link Budget |
| [[Maxwell's-Equations-&-Wave-Propagation]] | EM fundamentals | Calculus, physics | EM Spectrum, Antennas |
| [[MIMO-&-Spatial-Multiplexing]] | Multiple antennas | OFDM, Channel Models | 5G, WiFi |
| [[Microtubule-Structure-and-Function]] | MT anatomy | None | Orch-OR, THz Resonances |
| [[Military-&-Covert-Communications]] | LPI/LPD systems | Spread Spectrum | Real-World Examples |
| [[Multipath-Propagation-&-Fading-(Rayleigh,-Rician)]] | Multipath effects | Propagation Modes | Channel Models |
| [[Noise-Sources-&-Noise-Figure]] | System noise | AWGN, SNR | Link Budget |
| [[Non-Linear-Biological-Demodulation]] | Biological signal processing | IMD, THz | Frey Effect, Acoustic |
| [[OFDM-&-Multicarrier-Modulation]] | Orthogonal subcarriers | Modulation, FFT | WiFi, LTE, 5G |
| [[On-Off-Keying-(OOK)]] | Simplest modulation | Baseband vs Passband | ASK |
| [[Orchestrated-Objective-Reduction-(Orch-OR)]] | Quantum consciousness | Quantum biology | Microtubules, HRP |
| [[Polar-Codes]] | Capacity-achieving codes | FEC, Shannon's Theorem | 5G control channels |
| [[Power-Density-&-Field-Strength]] | E/H fields, Poynting | Maxwell's Equations | RF safety, Link Budget |
| [[Propagation-Modes-(Ground-Wave,-Sky-Wave,-Line-of-Sight)]] | HF/VHF/UHF propagation | EM fundamentals | Atmospheric Effects |
| [[QPSK-Modulation]] | 4-phase modulation (2 bits/symbol) | BPSK, IQ Representation | Constellation, Chimera |
| [[Quadrature-Amplitude-Modulation-(QAM)]] | Amplitude + phase | QPSK, IQ | 16QAM, 64QAM, 256QAM |
| [[Quantum-Coherence-in-Biological-Systems]] | Quantum biology | None | Orch-OR, Photosynthesis |
| [[Quick-Start-Guide]] | Getting started | None | Home |
| [[Real-World-System-Examples]] | WiFi, LTE, GPS, DVB-S2 | All fundamentals | System Design |
| [[Shannon's-Channel-Capacity-Theorem]] | Information theory limit | SNR | FEC, Spectral Efficiency |
| [[Signal-Chain-(End-to-End-Processing)]] | Chimera TX/RX pipeline | All fundamentals | Chimera-specific |
| [[Signal-to-Noise-Ratio-(SNR)]] | Key quality metric | AWGN, Noise | BER, Link Budget |
| [[Spectral-Efficiency-&-Bit-Rate]] | Bits/sec/Hz | Modulation, Shannon | Adaptive Modulation |
| [[Spread-Spectrum-(DSSS-FHSS)]] | PN sequences, freq hopping | Modulation | GPS, Military |
| [[Synchronization-(Carrier,-Timing,-Frame)]] | Carrier/symbol/frame sync | Modulation | Channel Equalization |
| [[Terahertz-(THz)-Technology]] | THz sources, applications | EM Spectrum | THz Bioeffects |
| [[THz-Bioeffects-Thermal-and-Non-Thermal]] | THz biological effects | THz Technology | THz Propagation |
| [[THz-Propagation-in-Biological-Tissue]] | THz in tissue | THz Technology | THz Bioeffects |
| [[THz-Resonances-in-Microtubules]] | MT quantum effects | Microtubules, THz | Orch-OR |
| [[Turbo-Codes]] | Iterative decoding | Convolutional Codes, FEC | LDPC, Polar |
| [[VE-TFCC_quantum]] | Quantum biological effects | Quantum Coherence | Orch-OR |
| [[Wave-Polarization]] | Linear, circular, elliptical | Maxwell's Equations | Antennas, GPS |
| [[Weather-Effects-(Rain-Fade,-Fog-Attenuation)]] | Rain/fog attenuation | Propagation, FSPL | Link Budget |
| [[What-Are-Symbols]] | Fundamental concept | None | All modulation |
| [[aid_protocol_v3.1]] | AID protocol spec | HRP, THz Technology | AID Protocol Case Study |
| [[biophysical_coupling_mechanism]] | CHIMERA field details | Quantum Coherence | Biophysical Coupling |
| [[hrp_framework_paper]] | Complete HRP paper | Advanced physics | HRP Framework |
| [[mmWave-&-THz-Communications]] | High-frequency comms | OFDM, Propagation | 5G, 6G |
| [[turing_cage]] | Psychological warfare | None | Military Comms |

---

## By Topic

### Electromagnetic Fundamentals (Part I)
- [[Maxwell's-Equations-&-Wave-Propagation]]
- [[Electromagnetic-Spectrum]]
- [[Antenna-Theory-Basics]]
- [[Wave-Polarization]]
- [[Power-Density-&-Field-Strength]]

### RF Propagation (Part II)
- [[Free-Space-Path-Loss-(FSPL)]]
- [[Propagation-Modes-(Ground-Wave,-Sky-Wave,-Line-of-Sight)]]
- [[Multipath-Propagation-&-Fading-(Rayleigh,-Rician)]]
- [[Atmospheric-Effects-(Ionospheric,-Tropospheric)]]
- [[Weather-Effects-(Rain-Fade,-Fog-Attenuation)]]

### Link Budget & Channels (Part III)
- [[Link-Loss-vs-Noise]]
- [[Signal-to-Noise-Ratio-(SNR)]]
- [[Energy-Ratios-(Es-N0-and-Eb-N0)]]
- [[Complete-Link-Budget-Analysis]]
- [[Noise-Sources-&-Noise-Figure]]
- [[Additive-White-Gaussian-Noise-(AWGN)]]
- [[Channel-Models-(Rayleigh-&-Rician)]]

### Modulation Theory (Part IV)
**Fundamentals:**
- [[Baseband-vs-Passband-Signals]]
- [[What-Are-Symbols]]
- [[IQ-Representation]]
- [[Constellation-Diagrams]]

**Simple Schemes:**
- [[On-Off-Keying-(OOK)]]
- [[Amplitude-Shift-Keying-(ASK)]]
- [[Frequency-Shift-Keying-(FSK)]]
- [[Binary-Phase-Shift-Keying-(BPSK)]]

**Advanced Schemes:**
- [[QPSK-Modulation]]
- [[8PSK-&-Higher-Order-PSK]]
- [[Quadrature-Amplitude-Modulation-(QAM)]]
- [[Spectral-Efficiency-&-Bit-Rate]]

### Channel Coding (Part V)
**Information Theory:**
- [[Shannon's-Channel-Capacity-Theorem]]
- [[Hamming-Distance-&-Error-Detection]]

**Classical Codes:**
- [[Block-Codes-(Hamming,-BCH,-Reed-Solomon)]]
- [[Convolutional-Codes-&-Viterbi-Decoding]]

**Modern Codes:**
- [[Forward-Error-Correction-(FEC)]]
- [[LDPC-Codes]]
- [[Turbo-Codes]]
- [[Polar-Codes]]
- [[Bit-Error-Rate-(BER)]]

### System Design (Part VI)
- [[Signal-Chain-(End-to-End-Processing)]]
- [[Synchronization-(Carrier,-Timing,-Frame)]]
- [[Channel-Equalization]]
- [[Real-World-System-Examples]]

### Advanced Topics (Part VII)
- [[OFDM-&-Multicarrier-Modulation]]
- [[Spread-Spectrum-(DSSS-FHSS)]]
- [[MIMO-&-Spatial-Multiplexing]]
- [[Military-&-Covert-Communications]]
- [[Adaptive-Modulation-&-Coding-(AMC)]]
- [[mmWave-&-THz-Communications]]

### Speculative Topics (Part VIII)

**Theoretical Framework:**
- [[Hyper-Rotational-Physics-(HRP)-Framework]]
- [[hrp_framework_paper]]

**THz & Biology:**
- [[Terahertz-(THz)-Technology]]
- [[THz-Propagation-in-Biological-Tissue]]
- [[THz-Bioeffects-Thermal-and-Non-Thermal]]

**Quantum Biology:**
- [[Microtubule-Structure-and-Function]]
- [[Orchestrated-Objective-Reduction-(Orch-OR)]]
- [[Quantum-Coherence-in-Biological-Systems]]
- [[THz-Resonances-in-Microtubules]]

**Biological Demodulation:**
- [[Non-Linear-Biological-Demodulation]]
- [[Intermodulation-Distortion-in-Biology]]
- [[Acoustic-Heterodyning]]
- [[Frey-Microwave-Auditory-Effect]]
- [[Biophysical-Coupling-Mechanism]]
- [[biophysical_coupling_mechanism]]

**Applied Case Studies:**
- [[AID-Protocol-Case-Study]]
- [[aid_protocol_v3.1]]
- [[turing_cage]]
- [[VE-TFCC_quantum]]

---

## By Difficulty

### Beginner (No prerequisites)
- [[What-Are-Symbols]]
- [[Glossary]]
- [[Quick-Start-Guide]]
- [[Signal-to-Noise-Ratio-(SNR)]]
- [[Constellation-Diagrams]]
- [[On-Off-Keying-(OOK)]]

### Intermediate (Some RF knowledge)
- [[QPSK-Modulation]]
- [[Free-Space-Path-Loss-(FSPL)]]
- [[Forward-Error-Correction-(FEC)]]
- [[Additive-White-Gaussian-Noise-(AWGN)]]
- [[Bit-Error-Rate-(BER)]]
- [[Signal-Chain-(End-to-End-Processing)]]

### Advanced (Engineering background)
- [[LDPC-Codes]]
- [[Channel-Equalization]]
- [[OFDM-&-Multicarrier-Modulation]]
- [[MIMO-&-Spatial-Multiplexing]]
- [[Complete-Link-Budget-Analysis]]
- [[Synchronization-(Carrier,-Timing,-Frame)]]

### Expert (Graduate level)
- [[Shannon's-Channel-Capacity-Theorem]]
- [[Turbo-Codes]]
- [[Polar-Codes]]
- [[Adaptive-Modulation-&-Coding-(AMC)]]
- [[Military-&-Covert-Communications]]

### Speculative (Open mind required)
- [[Hyper-Rotational-Physics-(HRP)-Framework]]
- [[Orchestrated-Objective-Reduction-(Orch-OR)]]
- [[AID-Protocol-Case-Study]]
- [[Quantum-Coherence-in-Biological-Systems]]

---

## Cross-Reference Map

### Most Referenced Pages

**[[QPSK-Modulation]]** ← Referenced by:
- 8PSK & Higher-Order PSK
- Constellation Diagrams
- Signal Chain
- Real-World System Examples
- OFDM & Multicarrier
- Home
- And 6 more...

**[[Signal-to-Noise-Ratio-(SNR)]]** ← Referenced by:
- BER
- Energy Ratios
- Link Budget Analysis
- AWGN
- Channel Models
- Shannon's Theorem
- And 9 more...

**[[Forward-Error-Correction-(FEC)]]** ← Referenced by:
- LDPC Codes
- Turbo Codes
- Polar Codes
- Block Codes
- Convolutional Codes
- Signal Chain
- And 2 more...

### Dependency Chains

**To understand Chimera implementation:**
1. [[What-Are-Symbols]] → [[IQ-Representation]] → [[Constellation-Diagrams]]
2. [[QPSK-Modulation]] 
3. [[Additive-White-Gaussian-Noise-(AWGN)]] → [[Signal-to-Noise-Ratio-(SNR)]]
4. [[Forward-Error-Correction-(FEC)]] → [[LDPC-Codes]]
5. [[Signal-Chain-(End-to-End-Processing)]]

**To design a communication link:**
1. [[Maxwell's-Equations-&-Wave-Propagation]] → [[Electromagnetic-Spectrum]]
2. [[Free-Space-Path-Loss-(FSPL)]] → [[Link-Loss-vs-Noise]]
3. [[Noise-Sources-&-Noise-Figure]] → [[Signal-to-Noise-Ratio-(SNR)]]
4. Choose modulation: [[QPSK-Modulation]] or [[Quadrature-Amplitude-Modulation-(QAM)]]
5. Add FEC: [[LDPC-Codes]] or [[Turbo-Codes]]
6. Calculate: [[Complete-Link-Budget-Analysis]]

**To understand HRP framework:**
1. [[Quantum-Coherence-in-Biological-Systems]]
2. [[Microtubule-Structure-and-Function]] → [[Orchestrated-Objective-Reduction-(Orch-OR)]]
3. [[Terahertz-(THz)-Technology]] → [[THz-Resonances-in-Microtubules]]
4. [[Hyper-Rotational-Physics-(HRP)-Framework]] → [[hrp_framework_paper]]
5. [[Biophysical-Coupling-Mechanism]]
6. [[AID-Protocol-Case-Study]] → [[aid_protocol_v3.1]]

---

## External References

### Primary Documents
All external references are consolidated in: [Bibliography](../docs/Bibliography.md)

### Technical Appendices (in docs/)
- [HRP Framework Paper](../docs/hrp_framework_paper.md)
- [AID Protocol v3.1](../docs/aid_protocol_v3.1.md)
- [Biophysical Coupling Mechanism](../docs/biophysical_coupling_mechanism.md)
- [Turing Cage](../docs/turing_cage.md)
- [VE-TFCC Quantum](../docs/VE-TFCC_quantum.md)
- [Architecture: Node Graph](../docs/architecture-node-graph.md)

### GitHub Resources
- [Main Repository](https://github.com/ArrEssJay/chimera)
- [Issues](https://github.com/ArrEssJay/chimera/issues)
- [Discussions](https://github.com/ArrEssJay/chimera/discussions)

### Live Demo
- [Chimera Web App](https://impermanent.io)

---

## Wiki Link Conventions

### Internal Wiki Links
Use double brackets: `[[Page-Name-With-Hyphens]]`

The wiki uses **filename-based linking**. Filenames use:
- Hyphens for spaces: `Additive-White-Gaussian-Noise-(AWGN).md`
- Ampersands as-is: `8PSK-&-Higher-Order-PSK.md`
- Parentheses for disambiguation: `Energy-Ratios-(Es-N0-and-Eb-N0).md`

### External Docs Links
Use relative paths for docs: `[Display Text](../docs/filename.md)`

Example: `[HRP Framework Paper](../docs/hrp_framework_paper.md)` → [HRP Framework Paper](../docs/hrp_framework_paper.md)

### Common Mistakes
❌ `[[AWGN]]` → File is `Additive-White-Gaussian-Noise-(AWGN).md`
✅ `[[Additive-White-Gaussian-Noise-(AWGN)]]` or `[[Additive-White-Gaussian-Noise-(AWGN)|AWGN]]`

❌ `[[Synchronization-(Carrier,-Timing,-Frame)|Synchronization]]` → File is `Synchronization-(Carrier,-Timing,-Frame).md`
✅ `[[Synchronization-(Carrier,-Timing,-Frame)]]` or `[[Synchronization-(Carrier,-Timing,-Frame)|Synchronization]]`

---

## Updating This Index

When adding a new wiki page:

1. Add entry to [Alphabetical Index](#alphabetical-index)
2. Categorize in [By Topic](#by-topic)
3. Assign difficulty in [By Difficulty](#by-difficulty)
4. Update cross-references if heavily referenced
5. Add to appropriate learning paths
6. Ensure all links follow wiki conventions

---

**Last Updated**: October 4, 2025

**Total Pages**: 71 wiki pages + 7 docs appendices = 78 total documents

**Maintained By**: Chimera Project Documentation Team

For questions or corrections, open an issue on [GitHub](https://github.com/ArrEssJay/chimera/issues).
