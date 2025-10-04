# Chimera DSP Wiki

Welcome to the **Chimera Digital Signal Processing Documentation Wiki**! 

This wiki provides a **comprehensive, first-principles approach** to understanding wireless communications—from electromagnetic theory through practical system design to cutting-edge research. Whether you're learning DSP fundamentals or exploring quantum neuromodulation, this resource builds knowledge systematically.

---

## 🎓 Learning Path

**New to wireless communications?** Follow the parts in order:

**Experienced engineer?** Jump to specific topics using the navigation below.

---

## 📐 Part I: Electromagnetic Fundamentals

**Build from Maxwell's equations to antenna theory**

- [[Maxwell's Equations & Wave Propagation]] - Foundation of all EM radiation
- [[Electromagnetic Spectrum]] - HF → VHF → UHF → mmWave → THz bands, applications, ionizing vs non-ionizing
- [[Antenna Theory Basics]] - Gain, directivity, impedance, beamwidth, Friis equation
- Wave Polarization *(coming soon)*
- Power Density & Field Strength *(coming soon)*

**Prerequisites**: Basic calculus, physics
**Learning goals**: Understand EM waves as physical phenomena, antenna basics

---

## 📡 Part II: RF Propagation

**How signals travel through real-world environments**

- [[Free-Space Path Loss (FSPL)]] - Friis equation and link budgets
- [[Propagation Modes (Ground Wave, Sky Wave, Line-of-Sight)]] - HF skywave, VHF LOS, radio horizon, Fresnel zones
- Multipath Propagation & Fading (Rayleigh, Rician) *(coming soon)*
- [[Atmospheric Effects (Ionospheric, Tropospheric)]] - Ionospheric refraction/absorption, O₂/H₂O absorption, ducting, TEC
- [[Weather Effects (Rain Fade, Fog Attenuation)]] - ITU rain model, C/Ku/Ka/V-band attenuation, climate zones, mitigation
- Diffraction, Reflection, Scattering *(coming soon)*

**Prerequisites**: Part I
**Learning goals**: Predict signal strength, understand channel impairments

---

## 🔗 Part III: Link Budget & Channel Modeling

**Connecting transmitters to receivers**

- [[Link Loss vs Noise]] - Distinguishing attenuation from additive noise
- [[Signal to Noise Ratio (SNR)]] - Key quality metric
- [[Energy Ratios (Es/N0 and Eb/N0)]] - Symbol and bit energy ratios
- Complete Link Budget Analysis *(coming soon)*
- Noise Sources & Noise Figure *(coming soon)*
- Channel Models (AWGN, Rayleigh, Rician) - [[Additive White Gaussian Noise (AWGN)]] *+ more coming*

**Prerequisites**: Part II
**Learning goals**: Calculate link budgets, model channel effects

---

## 📻 Part IV: Modulation Theory

**Encoding information onto carriers (simple → complex)**

### Digital Modulation Fundamentals
- Baseband vs Passband Signals *(coming soon)*
- [[On-Off Keying (OOK)]] - Simplest modulation (carrier on/off)
- Amplitude-Shift Keying (ASK) *(coming soon)*
- [[Frequency-Shift Keying (FSK)]] - Binary & M-ary frequency switching
- [[Binary Phase-Shift Keying (BPSK)]] - Two-phase modulation, coherent detection, 3 dB better than OOK

### Advanced Modulation
- [[What Are Symbols]] - Fundamental building blocks
- [[QPSK Modulation]] - Quadrature Phase-Shift Keying (2 bits/symbol)
- Higher-Order PSK (8PSK, 16PSK) *(coming soon)*
- Quadrature Amplitude Modulation (QAM) *(coming soon)*
- [[IQ Representation]] - In-phase and Quadrature components
- [[Constellation Diagrams]] - Visualizing modulation schemes
- Spectral Efficiency & Bit Rate *(coming soon)*

**Prerequisites**: Part III
**Learning goals**: Choose modulation schemes, understand tradeoffs (spectral efficiency vs. robustness)

---

## 🔐 Part V: Channel Coding & Error Control

**Protecting data from channel errors**

### Information Theory
- [[Shannon's Channel Capacity Theorem]] - Fundamental limit of communication (C = B·log₂(1+SNR))
- Hamming Distance & Error Detection *(coming soon)*
- Block Codes (Hamming, BCH, Reed-Solomon) *(coming soon)*
- Convolutional Codes & Viterbi Decoding *(coming soon)*
- Turbo Codes *(coming soon)*

### Modern Codes
- [[Forward Error Correction (FEC)]] - General FEC concepts
- [[LDPC Codes]] - Low-Density Parity-Check codes (used in Chimera)
- [[Bit Error Rate (BER)]] - Performance metric
- Polar Codes *(coming soon)*

**Prerequisites**: Part IV
**Learning goals**: Design error correction schemes, approach Shannon limit

---

## ⚙️ Part VI: Practical System Design

**End-to-end wireless systems**

- [[Signal Chain (End-to-End Processing)]] - Complete TX/RX pipeline
- Synchronization (Carrier, Symbol, Frame) *(coming soon)*
- Channel Equalization *(coming soon)*
- Real-World System Examples (WiFi 802.11, LTE, DVB-S2) *(coming soon)*
- Performance Analysis & BER Curves *(coming soon)*

**Prerequisites**: Parts IV-V
**Learning goals**: Design complete communication systems, debug real-world issues

---

## 🚀 Part VII: Advanced Topics

**Modern wireless techniques**

- OFDM & Multicarrier Modulation *(coming soon)*
- Spread Spectrum (DSSS, FHSS) *(coming soon)*
- MIMO & Spatial Multiplexing *(coming soon)*
- Adaptive Modulation & Coding *(coming soon)*
- mmWave & THz Communications *(coming soon)*

**Prerequisites**: Part VI
**Learning goals**: Understand state-of-the-art wireless systems (5G, WiFi 6, satellite)

---

## 🧠 Part VIII: Speculative & Emerging Topics

**Frontier research: Quantum biology meets wireless engineering**

⚠️ **Note**: This section explores speculative applications grounded in cutting-edge research. Content clearly distinguishes established science from theoretical extrapolation.

### A. Theoretical Framework
- [[Hyper-Rotational Physics (HRP) Framework]] - M-theory extension: consciousness-matter coupling via quantum coherence

### B. THz Technology & Biology
- [[Terahertz (THz) Technology]] - QCLs, applications, propagation, bioeffects
- THz Propagation in Biological Tissue *(coming soon)*
- THz Bioeffects (Thermal & Non-Thermal) *(coming soon)*

### C. Quantum Biology & Consciousness
- Microtubule Structure & Function *(coming soon)*
- [[Orchestrated Objective Reduction (Orch-OR)]] - Penrose-Hameroff quantum consciousness theory
- Quantum Coherence in Biological Systems *(coming soon)*
- THz Resonances in Microtubules *(coming soon)*

### D. Non-Linear Biological Demodulation
- Intermodulation Distortion (IMD) in Biology *(coming soon)*
- Acoustic Heterodyning *(coming soon)*
- Frey Microwave Auditory Effect *(coming soon)*

### E. Applied Case Study: HRP-Based THz Neuromodulation
- [[AID Protocol Case Study]] - Rigorous application of HRP framework to THz wireless neuromodulation

**Prerequisites**: Parts I-VII + open mind
**Learning goals**: Apply RF engineering to novel scenarios, practice interdisciplinary thinking, distinguish speculation from established science

---

## 🛠️ Chimera Implementation

**How Chimera applies these concepts**

Chimera is a browser-based DSP simulator implementing:
- **Modulation**: QPSK (see [[QPSK Modulation]])
- **Channel**: AWGN (see [[Additive White Gaussian Noise (AWGN)]])
- **FEC**: LDPC codes (see [[LDPC Codes]])
- **Visualization**: Real-time constellation diagrams, BER analysis
- **Goal**: Learn wireless communications interactively

### Chimera-Specific Pages
- [[Signal Chain (End-to-End Processing)]] - Chimera's TX/RX pipeline
- [Chimera Technical Overview](../docs/chimera_technical_overview.md) - Architecture details
- [UI Controls Specification](../docs/ui_controls_specification.md) - Interface guide

---

## 📊 Practical Guides (Coming Soon)

- Reading the Constellation - Interpreting TX/RX scatter plots
- Understanding BER Curves - Performance analysis
- Tuning Parameters - Optimizing SNR and link loss settings
- Building Your First Link - Step-by-step tutorial

---

## 📚 Recommended Textbooks

### Undergraduate Level
- **Proakis & Salehi**, *Digital Communications* (5th ed.)
- **Haykin**, *Communication Systems* (5th ed.)
- **Sklar**, *Digital Communications: Fundamentals and Applications*

### Graduate Level
- **Tse & Viswanath**, *Fundamentals of Wireless Communication*
- **Goldsmith**, *Wireless Communications*
- **Richardson & Urbanke**, *Modern Coding Theory*

### Quantum Biology (Part VIII)
- **Penrose**, *The Emperor's New Mind* (Orch-OR origins)
- **Al-Khalili & McFadden**, *Life on the Edge: The Coming Age of Quantum Biology*

---

## 🔗 External Resources

- [IEEE Communications Society](https://www.comsoc.org/) - Professional organization
- [RF Café](https://www.rfcafe.com/) - Calculators and references
- [GNURadio](https://www.gnuradio.org/) - Open-source SDR toolkit
- [Wireless Pi](https://wirelesspi.com/) - Educational resources

---

## 🗺️ Navigation Tips

**Linear learning**: Follow Parts I → VIII in order (builds knowledge systematically)

**Topic-based**: Use search or browse sidebar alphabetically

**Chimera users**: Start with [[Signal Chain (End-to-End Processing)]], then explore referenced topics

**Visual learners**: Look for pages with diagrams: [[Constellation Diagrams]], [[IQ Representation]], [[QPSK Modulation]]

**Theory enthusiasts**: Jump to Parts V (Coding Theory) or VIII (Quantum Biology)

---

**Wiki Status**: 🚧 Under active development | **28/~50 pages complete (56%)**
- **Part I**: 3/5 pages (Maxwell ✓, EM Spectrum ✓, Antenna Theory ✓)
- **Part II**: 4/6 pages (FSPL ✓, Propagation Modes ✓, Atmospheric Effects ✓, Weather Effects ✓)
- **Part IV**: 7/13 pages (OOK ✓, FSK ✓, BPSK ✓, QPSK ✓, Constellation ✓, IQ ✓, Symbols ✓)
- **Part V**: 4/9 pages (Shannon ✓, FEC ✓, LDPC ✓, BER ✓)
- **Part VI**: 2/5 pages (Signal Chain ✓, AWGN ✓)
- **Part VIII**: 4/13 pages (HRP ✓, THz ✓, Orch-OR ✓, AID ✓)
- **Rain attenuation**: Comprehensive coverage (C/Ku/Ka/V-band, ITU model, temperate/tropical, satellite/terrestrial)
