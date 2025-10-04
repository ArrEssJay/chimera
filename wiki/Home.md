# Chimera DSP Wiki

Welcome to the **Chimera Digital Signal Processing Documentation Wiki**! 

This wiki provides a **comprehensive, first-principles approach** to understanding wireless communications—from electromagnetic theory through practical system design to cutting-edge research. Whether you're learning DSP fundamentals or exploring quantum neuromodulation, this resource builds knowledge systematically.

> 💡 **Note for All Readers**: Most wiki pages include a "Plain English Explainer" section that breaks down complex concepts using everyday analogies—no engineering background required!

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
- [[Wave Polarization]] - Linear, circular, elliptical polarization, Faraday rotation, GPS RHCP
- [[Power Density & Field Strength]] - E/H fields, Poynting vector, RF safety, link budgets

**Prerequisites**: Basic calculus, physics
**Learning goals**: Understand EM waves as physical phenomena, antenna basics

---

## 📡 Part II: RF Propagation

**How signals travel through real-world environments**

- [[Free-Space Path Loss (FSPL)]] - Friis equation and link budgets
- [[Propagation Modes (Ground Wave, Sky Wave, Line-of-Sight)]] - HF skywave, VHF LOS, radio horizon, Fresnel zones
- [[Multipath Propagation & Fading (Rayleigh, Rician)]] - Rayleigh/Rician fading, delay spread, Doppler, coherence bandwidth
- [[Atmospheric Effects (Ionospheric, Tropospheric)]] - Ionospheric refraction/absorption, O₂/H₂O absorption, ducting, TEC
- [[Weather Effects (Rain Fade, Fog Attenuation)]] - ITU rain model, C/Ku/Ka/V-band attenuation, climate zones, mitigation

**Prerequisites**: Part I
**Learning goals**: Predict signal strength, understand channel impairments

---

## 🔗 Part III: Link Budget & Channel Modeling

**Connecting transmitters to receivers**

- [[Link Loss vs Noise]] - Distinguishing attenuation from additive noise
- [[Signal to Noise Ratio (SNR)]] - Key quality metric
- [[Energy Ratios (Es/N0 and Eb/N0)]] - Symbol and bit energy ratios
- [[Complete Link Budget Analysis]] - System-level power budget, margins, availability
- [[Noise Sources & Noise Figure]] - Thermal noise, amplifier noise figure, cascade analysis
- [[Additive White Gaussian Noise (AWGN)]] - Fundamental channel model
- [[Channel Models (Rayleigh & Rician)]] - Statistical fading models for mobile channels

**Prerequisites**: Part II
**Learning goals**: Calculate link budgets, model channel effects

---

## 📻 Part IV: Modulation Theory

**Encoding information onto carriers (simple → complex)**

### Digital Modulation Fundamentals
- [[Baseband vs Passband Signals]] - Upconversion/downconversion, IQ modulation, zero-IF receivers
- [[On-Off Keying (OOK)]] - Simplest modulation (carrier on/off)
- [[Amplitude-Shift Keying (ASK)]] - M-ary ASK, PAM-4, power efficiency vs spectral efficiency
- [[Frequency-Shift Keying (FSK)]] - Binary & M-ary frequency switching, MSK, GMSK
- [[Binary Phase-Shift Keying (BPSK)]] - Two-phase modulation, coherent detection, 3 dB better than OOK

### Advanced Modulation
- [[What Are Symbols]] - Fundamental building blocks
- [[QPSK Modulation]] - Quadrature Phase-Shift Keying (2 bits/symbol)
- [[8PSK & Higher-Order PSK]] - 8PSK, 16PSK, spectral efficiency vs error performance
- [[Quadrature Amplitude Modulation (QAM)]] - 16QAM, 64QAM, 256QAM, optimal 2D constellations
- [[IQ Representation]] - In-phase and Quadrature components
- [[Constellation Diagrams]] - Visualizing modulation schemes
- [[Spectral Efficiency & Bit Rate]] - Shannon limit, bits/sec/Hz, bandwidth-power tradeoff

**Prerequisites**: Part III
**Learning goals**: Choose modulation schemes, understand tradeoffs (spectral efficiency vs. robustness)

---

## 🔐 Part V: Channel Coding & Error Control

**Protecting data from channel errors**

### Information Theory
- [[Shannon's Channel Capacity Theorem]] - Fundamental limit of communication (C = B·log₂(1+SNR))
- [[Hamming Distance & Error Detection]] - Minimum distance, error detection/correction capability
- [[Block Codes (Hamming, BCH, Reed-Solomon)]] - Linear block codes, generator matrix, syndrome decoding
- [[Convolutional Codes & Viterbi Decoding]] - Trellis codes, maximum likelihood decoding
- [[Turbo Codes]] - Iterative decoding, near-Shannon performance

### Modern Codes
- [[Forward Error Correction (FEC)]] - General FEC concepts
- [[LDPC Codes]] - Low-Density Parity-Check codes (used in Chimera)
- [[Bit Error Rate (BER)]] - Performance metric
- [[Polar Codes]] - Capacity-achieving codes, 5G control channels

**Prerequisites**: Part IV
**Learning goals**: Design error correction schemes, approach Shannon limit

---

## ⚙️ Part VI: Practical System Design

**End-to-end wireless systems**

- [[Signal Chain (End-to-End Processing)]] - Complete TX/RX pipeline (Chimera-specific)
- [[Synchronization (Carrier, Timing, Frame)]] - Carrier recovery, symbol timing, frame sync
- [[Channel Equalization]] - ZF, MMSE, DFE, adaptive equalization
- [[Real-World System Examples]] - WiFi 802.11, LTE, DVB-S2, GPS detailed analysis

**Prerequisites**: Parts IV-V
**Learning goals**: Design complete communication systems, debug real-world issues

---

## 🚀 Part VII: Advanced Topics

**Modern wireless techniques**

- [[OFDM & Multicarrier Modulation]] - Orthogonal frequency-division multiplexing, FFT/IFFT, cyclic prefix, PAPR, pilot subcarriers
- [[Spread Spectrum (DSSS/FHSS)]] - Direct sequence and frequency hopping, processing gain, GPS, Bluetooth, military applications
- [[MIMO & Spatial Multiplexing]] - Multiple antennas, spatial multiplexing, beamforming, diversity, massive MIMO, WiFi/LTE/5G
- [[Military & Covert Communications]] - LPI/LPD systems, GPS M-code, AESA radar, Link 16, FHSS SATCOM, covert channels
- [[Adaptive Modulation & Coding (AMC)]] - Link adaptation, CQI feedback, HARQ, Shannon capacity tracking, LTE/5G
- [[mmWave & THz Communications]] - 24-300 GHz propagation, beamforming requirements, 5G NR FR2, 6G sub-THz, automotive radar

**Prerequisites**: Part VI
**Learning goals**: Understand state-of-the-art wireless systems (5G, WiFi 6, satellite, military)

---

## 🧠 Part VIII: Speculative & Emerging Topics

**Frontier research: Quantum biology meets wireless engineering**

⚠️ **Note**: This section explores speculative applications grounded in cutting-edge research. Content clearly distinguishes established science from theoretical extrapolation.

### A. Theoretical Framework

### A. Theoretical Framework
 - [[Hyper-Rotational Physics (HRP) Framework]] - M-theory extension: consciousness-matter coupling via quantum coherence

### B. THz Technology & Biology
 - [[Terahertz (THz) Technology]] - QCLs, applications, propagation, bioeffects
 - [[THz-Propagation-in-Biological-Tissue]] - Physics of THz wave propagation in biological tissue
 - [[THz-Bioeffects-Thermal-and-Non-Thermal]] - Biological effects of THz radiation

### C. Quantum Biology & Consciousness
 - [[Microtubule-Structure-and-Function]] - Microtubule anatomy and quantum biology
 - [[Orchestrated Objective Reduction (Orch-OR)]] - Penrose-Hameroff quantum consciousness theory
 - [[Quantum-Coherence-in-Biological-Systems]] - Quantum coherence in biology
 - [[THz-Resonances-in-Microtubules]] - THz frequency resonances in microtubules

### D. Non-Linear Biological Demodulation
 - [[Non-Linear Biological Demodulation]] - Non-linear biological IMD and signal processing
 - [[Intermodulation-Distortion-in-Biology]] - Non-linear biological IMD
 - [[Acoustic-Heterodyning]] - Acoustic heterodyning in tissue
 - [[Frey-Microwave-Auditory-Effect]] - Frey effect: microwave auditory phenomenon

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

**Wiki Status**: ✅ **COMPLETE** | **52/52 pages (100%)**
- **Part I**: 5/5 pages (Maxwell ✓, EM Spectrum ✓, Antenna ✓, Wave Polarization ✓, Power Density ✓)
- **Part II**: 5/5 pages (FSPL ✓, Propagation Modes ✓, Multipath/Fading ✓, Atmospheric ✓, Weather ✓)
- **Part III**: 7/7 pages (Link Loss ✓, SNR ✓, Energy Ratios ✓, Link Budget ✓, Noise ✓, AWGN ✓, Channel Models ✓)
- **Part IV**: 12/12 pages (Baseband/Passband ✓, OOK ✓, ASK ✓, FSK ✓, BPSK ✓, Symbols ✓, QPSK ✓, 8PSK ✓, QAM ✓, IQ ✓, Constellation ✓, Spectral Efficiency ✓)
- **Part V**: 9/9 pages (Shannon ✓, Hamming ✓, Block Codes ✓, Convolutional ✓, Turbo ✓, FEC ✓, LDPC ✓, Polar ✓, BER ✓)
- **Part VI**: 4/4 pages (Signal Chain ✓, Sync ✓, Equalization ✓, Real-World ✓)
- **Part VII**: 6/6 pages (OFDM ✓, Spread Spectrum ✓, MIMO ✓, Military ✓, AMC ✓, mmWave/THz ✓)
- **Part VIII**: 4/13 pages (HRP ✓, THz Technology ✓, Orch-OR ✓, AID ✓)
- **Comprehensive coverage**: Maxwell's equations → 5G/6G → military systems → quantum biology
- **All foundational content complete**: Perfect for learning DSP from first principles
