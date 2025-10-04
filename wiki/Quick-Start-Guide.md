# Quick Start Guide

[[Home]] | **Getting Started**

---

## Welcome to the Chimera DSP Wiki!

This guide helps you navigate the **52 wiki pages** based on your background and learning goals.

---

## 🎯 Choose Your Path

### Path 1: **Complete Beginner** (No RF background)

**Goal**: Build solid foundation from physics → practical systems

**Recommended sequence** (15-20 hours):

1. **Week 1: Physical Foundations**
   - [[Maxwell's-Equations-&-Wave-Propagation]] (30 min)
   - [[Electromagnetic-Spectrum]] (45 min)
   - [[Antenna-Theory-Basics]] (1 hour)
   - [[Free-Space-Path-Loss-(FSPL)]] (45 min)
   
2. **Week 2: Signals & Modulation**
   - [[Baseband-vs-Passband-Signals]] (45 min)
   - [[What-Are-Symbols]] (15 min) ⭐ Start here for quick intro
   - [[IQ-Representation]] (30 min)
   - [[Constellation-Diagrams]] (30 min)
   - [[QPSK-Modulation]] (30 min)

3. **Week 3: System Performance**
   - [[Signal-to-Noise-Ratio-(SNR)]] (30 min)
   - [[Bit-Error-Rate-(BER)]] (1 hour)
   - [[Additive-White-Gaussian-Noise-(AWGN)]] (30 min)
   - [[Forward-Error-Correction-(FEC)]] (45 min)

4. **Week 4: Putting It Together**
   - [[Signal-Chain-(End-to-End-Processing)]] (1 hour) ⭐ See Chimera implementation
   - [[Complete-Link-Budget-Analysis]] (1.5 hours)
   - **Try Chimera simulator!**

---

### Path 2: **Software Engineer** (Familiar with DSP concepts)

**Goal**: Understand RF implementation details

**Fast track** (5-8 hours):

1. **Core Concepts** (2 hours)
   - [[What-Are-Symbols]] (15 min)
   - [[IQ-Representation]] (30 min)
   - [[QPSK-Modulation]] (30 min)
   - [[Constellation-Diagrams]] (30 min)
   - [[LDPC-Codes]] (45 min) ⭐ Used in Chimera

2. **Implementation Details** (2 hours)
   - [[Signal-Chain-(End-to-End-Processing)]] (1 hour) ⭐ **Start here!**
   - [[Baseband-vs-Passband-Signals]] (45 min) - Upconversion/downconversion
   - [[Synchronization-(Carrier,-Timing,-Frame)]] (30 min)

3. **Performance Analysis** (2 hours)
   - [[Bit-Error-Rate-(BER)]] (1 hour)
   - [[Energy-Ratios-(Es-N0-and-Eb-N0)]] (30 min)
   - [[Complete-Link-Budget-Analysis]] (1 hour)

4. **Advanced Topics** (pick based on interest)
   - [[OFDM-&-Multicarrier-Modulation]] - WiFi/LTE internals
   - [[Spread-Spectrum-(DSSS-FHSS)]] - GPS/Bluetooth
   - [[Channel-Equalization]] - ISI mitigation

---

### Path 3: **EE Student** (Taking comm systems course)

**Goal**: Supplement textbook with practical examples

**Study companion** (use as needed):

**For homework/exams**:
- [[Formula-Reference-Card]] ⭐ Quick lookup
- [[Glossary]] ⭐ Acronym decoder
- Part-specific pages matching your syllabus

**For lab work**:
- [[Signal-Chain-(End-to-End-Processing)]]
- [[Real-World-System-Examples]] - WiFi, LTE, GPS analyzed
- [[Synchronization-(Carrier,-Timing,-Frame)]]
- [[Channel-Equalization]]

**For projects**:
- [[Complete-Link-Budget-Analysis]] - End-to-end calculations
- [[OFDM-&-Multicarrier-Modulation]] - Modern systems
- [[MIMO-&-Spatial-Multiplexing]] - 5G techniques

---

### Path 4: **RF Engineer** (Expanding into DSP/modulation)

**Goal**: Bridge analog RF → digital modulation

**Recommended sequence** (6-10 hours):

1. **Digital Modulation Basics** (2 hours)
   - [[Baseband-vs-Passband-Signals]] (45 min) ⭐ IQ modulation explained
   - [[What-Are-Symbols]] (15 min)
   - [[Constellation-Diagrams]] (30 min)
   - [[QPSK-Modulation]] (30 min)

2. **Channel & Propagation** (2 hours)
   - [[Multipath-Propagation-&-Fading-(Rayleigh,-Rician)]] (1 hour)
   - [[Atmospheric-Effects-(Ionospheric,-Tropospheric)]] (45 min)
   - [[Weather-Effects-(Rain-Fade,-Fog-Attenuation)]] (45 min)

3. **System-Level Analysis** (2 hours)
   - [[Complete-Link-Budget-Analysis]] (1.5 hours) ⭐ Your bread and butter
   - [[Noise-Sources-&-Noise-Figure]] (1 hour)
   - [[Power-Density-&-Field-Strength]] (45 min)

4. **Advanced RF Topics** (pick based on role)
   - [[Wave-Polarization]] - Antenna design
   - [[mmWave-&-THz-Communications]] - 5G/6G
   - [[Adaptive-Modulation-&-Coding-(AMC)]] - Link adaptation
   - [[Military-&-Covert-Communications]] - LPI/LPD systems

---

## 🗺️ Navigation by Topic

### By Application

**Cellular (LTE/5G)**:
- [[OFDM-&-Multicarrier-Modulation]]
- [[MIMO-&-Spatial-Multiplexing]]
- [[Adaptive-Modulation-&-Coding-(AMC)]]
- [[Quadrature-Amplitude-Modulation-(QAM)]]

**WiFi (802.11)**:
- [[OFDM-&-Multicarrier-Modulation]]
- [[MIMO-&-Spatial-Multiplexing]]
- [[Channel-Equalization]]
- [[Real-World-System-Examples]]

**Satellite Communications**:
- [[Free-Space-Path-Loss-(FSPL)]]
- [[Complete-Link-Budget-Analysis]]
- [[Wave-Polarization]] (Circular polarization)
- [[Weather-Effects-(Rain-Fade,-Fog-Attenuation)]]

**GPS/GNSS**:
- [[Spread-Spectrum-(DSSS-FHSS)]]
- [[Wave-Polarization]] (RHCP)
- [[Binary-Phase-Shift-Keying-(BPSK)]]
- [[Real-World-System-Examples]]

---

### By Difficulty Level

**Beginner** (📗 Start here):
- [[What-Are-Symbols]] ⭐
- [[On-Off-Keying-(OOK)]]
- [[Constellation-Diagrams]]
- [[Signal-to-Noise-Ratio-(SNR)]]

**Intermediate** (📘 Some math required):
- [[QPSK-Modulation]]
- [[IQ-Representation]]
- [[Binary-Phase-Shift-Keying-(BPSK)]]
- [[Bit-Error-Rate-(BER)]]
- [[Forward-Error-Correction-(FEC)]]

**Advanced** (📙 Significant math):
- [[Shannon's-Channel-Capacity-Theorem]]
- [[LDPC-Codes]]
- [[Convolutional-Codes-&-Viterbi-Decoding]]
- [[OFDM-&-Multicarrier-Modulation]]

**Expert** (📕 Research-level):
- [[Turbo-Codes]]
- [[Polar-Codes]]
- [[Military-&-Covert-Communications]]

---

## 🛠️ Using Chimera Simulator

**New to Chimera?**

1. **Read foundation pages first** (90 min):
   - [[What-Are-Symbols]] (15 min)
   - [[QPSK-Modulation]] (30 min)
   - [[Constellation-Diagrams]] (30 min)
   - [[Signal-Chain-(End-to-End-Processing)]] (1 hour) ⭐ **Essential**

2. **Experiment**:
   - Adjust SNR → Watch constellation spreading
   - Change link loss → See BER curves
   - Toggle LDPC → Compare coded vs uncoded

---

## ❓ Common Questions

### "Where do I start if I have zero RF knowledge?"
→ [[What-Are-Symbols]] (15 min gentle intro) then follow **Path 1: Complete Beginner**

### "I'm a software engineer building a comm system. What do I need?"
→ [[Signal-Chain-(End-to-End-Processing)]] first, then **Path 2: Software Engineer**

### "I need to calculate a satellite link budget for a project"
→ Jump directly to [[Complete-Link-Budget-Analysis]]

### "What's the minimum to understand Chimera?"
→ 3 pages: [[What-Are-Symbols]], [[QPSK-Modulation]], [[Signal-Chain-(End-to-End-Processing)]]

### "I want to understand 5G"
→ [[OFDM-&-Multicarrier-Modulation]] + [[MIMO-&-Spatial-Multiplexing]] + [[Adaptive-Modulation-&-Coding-(AMC)]]

---

## 🔗 Essential Reference Pages

Keep these bookmarked:

- [[Formula-Reference-Card]] - Quick math lookup
- [[Glossary]] - Acronyms and definitions
- [[Home]] - Full table of contents

---

## 💡 Study Tips

1. **Math-heavy pages**: Read once for concepts, twice for derivations
2. **Use examples**: Every page has numerical examples - work through them
3. **Cross-reference**: Follow `[[wiki links]]` to build connections
4. **Experiment**: Use Chimera to validate theoretical concepts

---

*Updated: October 4, 2025*
