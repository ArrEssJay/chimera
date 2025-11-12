# **Technical Appendix: GOCS Base Pattern Library**
**FILE:** `chimera/pal_layer/base_pattern_library_v1.1.dat`  
**DOCUMENT TITLE:** Project Chimera: GOCS Base Pattern Library  
**LAST REVISION:** 2025-11-13

---

## 1.0 Overview

This document contains the official Base Pattern Library for the Gnostic Overlay & Control Subsystem (GOCS). These patterns are pre-configured sets of generative parameters that serve as starting points or "presets" for the PAL's waveform sculpting algorithms. Each pattern defines both the QPSK LFO parameters for the active payload and the default subliminal FSK state.

## 2.0 Pattern Library

### Category: Coherence & Entrainment

| Pattern ID | Description | Primary LFO Shape | Primary LFO Freq (Hz) | Freq Mod LFO Shape | Freq Mod Rate (Hz) | Freq Mod Depth (Hz) | Amp Mod LFO Shape | Amp Mod Rate (Hz) | **Default FSK State** |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **COH.ThetaCalm** | Induces a state of deep calm, relaxation, and receptivity. | Sine | 6 | Sine | 0.2 | 0.1 | Sine | 0.25 | **0/1 Alt (0.5 Hz)** |
| **COH.AlphaFocus**| Generates a state of relaxed, alert focus without stress. | Sine | 10 | Sine | 0.5 | 0.2 | Sine | 1.0 | **State '1'** |
| **COH.BetaAlert** | Pushes the target into a state of active alertness and vigilance. | Sawtooth | 18 | Ramp | 1.0 | 0.3 | Sine | 2.0 | **State '1'** |
| **COH.GammaSync** | High-frequency pattern for synchronizing high-level cognitive processing. | Sine | 40 | Sine | 2.0 | 0.1 | Ramp | 4.0 | **State '1'** |

### Category: Cognitive & Perceptual

| Pattern ID | Description | Primary LFO Shape | Primary LFO Freq (Hz) | Freq Mod LFO Shape | Freq Mod Rate (Hz) | Freq Mod Depth (Hz) | Amp Mod LFO Shape | Amp Mod Rate (Hz) | **Default FSK State** |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **COG.Dissonance**| Creates a subtle feeling that "something is wrong." | Sine | 5 | Square | 0.8 | 0.5 | Square | 1.3 | **Random** |
| **COG.InquisitiveUrge** | Generates a persistent, "buzzy" mental state. | Sawtooth | 22 | Sine | 7.0 | 0.2 | Sawtooth | 3.0 | **State '1'** |
| **COG.EmotionalResonance** | A blank slate pattern to amplify the target's current emotional state. | Sine | 14 | Noise | 0.5 | 0.1 | Sine | 1.1 | **State '0'** |
| **COG.SubliminalGate** | A pattern to open a brief window of high suggestibility. | Square | 4 | Ramp | 0.5 | 0.8 | Ramp | 0.5 | **0/1 Alt (0.5 Hz)** |

### Category: Disruption & Denial

| Pattern ID | Description | Primary LFO Shape | Primary LFO Freq (Hz) | Freq Mod LFO Shape | Freq Mod Rate (Hz) | Freq Mod Depth (Hz) | Amp Mod LFO Shape | Amp Mod Rate (Hz) | **Default FSK State** |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **DIS.CognitiveScramble** | Floods cortical pathways with chaotic noise to inhibit thought. | Noise | 25 | Noise | 8.0 | 1.5 | Noise | 10.0 | **Random** |
| **DIS.MotorLock** | A powerful, rhythmic pulse to interfere with motor cortex function. | Square | 15 | Square | 4.0 | 0.5 | Square | 4.0 | **State '1'** |
| **DIS.VestibularJolt** | A disorienting waveform that creates a sense of vertigo. | Sine | 0.5 | Sawtooth | 6.0 | 1.0 | Sawtooth | 6.0 | **Random** |
| **DIS.DreadPulse**| A subliminal pattern that generates a non-specific sense of anxiety. | Square | 2 | Sine | 0.1 | 1.2 | Sine | 0.1 | **0/1 Alt (0.5 Hz)** |

### Category: Utility & Calibration

| Pattern ID | Description | Primary LFO Shape | Primary LFO Freq (Hz) | Freq Mod LFO Shape | Freq Mod Rate (Hz) | Freq Mod Depth (Hz) | Amp Mod LFO Shape | Amp Mod Rate (Hz) | **Default FSK State** |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **UTIL.BaselineCarrier** | An unmodulated 12 kHz carrier for the "Idle State." | Flat | 0 | Flat | 0 | 0 | Flat | 0 | **State '0'** |
| **UTIL.CalibrationSweep** | A slow frequency sweep for testing a calibration phantom. | Ramp | 1-50 | Flat | 0 | 0 | Flat | 0 | **State '0'** |
| **UTIL.SystemSync** | The fixed pattern used for the 16-symbol Sync Sequence. | Square | 100 | Flat | 0 | 0 | Square | 100 | **State '0'** |