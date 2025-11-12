# **Technical Specification: GOCS Frame Structure & Payload Encoding**
**FILE:** `chimera/protocol_layer/frame_structure_v3.1.spec`  
**DOCUMENT TITLE:** Project Chimera: GOCS Frame Structure & Payload Encoding  
**LAST REVISION:** 2025-11-05

---

## 1.0 Overview

This document specifies the definitive bit-level encoding for the 128-symbol frame defined in `chimera/modulation_protocol_v4.2.dat`. This structure serves as the compilation target for the Gnostic Overlay & Control Subsystem (GOCS) via the Protocol Abstraction Layer (PAL).

It is critical to understand that the frame is not a data packet in the traditional sense. It is a **Temporal Sculpting Template**, where each bit contributes to the physical shape, intensity, and resonant characteristics of the THz waveform intended to interact directly with the target's neural microtubule lattice.

## 2.0 Frame Structure (128 QPSK Symbols / 256 Bits)

The 128-symbol frame is divided into five distinct fields, each with a specific role in shaping the biophysical effect.

| Field Name | Size (Symbols) | Size (Bits) | Description |
| :--- | :--- | :--- | :--- |
| Sync Sequence | 16 | 32 | A fixed pattern for hardware synchronization and frame delineation. |
| **Target ID** | **16** | **32** | **Quantum Resonance Key for target-specific coupling.** |
| **Command Type** | **16** | **32** | **Defines the high-level intent and character of the waveform effect.** |
| **Data Payload** | **64** | **128** | **Low-level parameters for sculpting the waveform's fine structure.** |
| **ECC** | **16** | **32** | **Ensures waveform pattern fidelity, not informational accuracy.** |

---

## 3.0 Field Bit-Level Specification

### 3.1 Target ID Field (32 Bits)
Encodes the Quantum Bio-Resonance Signature required to achieve selective coupling with the intended target.

| Bit Range | Sub-Field | Description | Example Values |
| :--- | :--- | :--- | :--- |
| `0-7` | **Baseline Brainwave** | The target's dominant resting-state brainwave frequency, used to calibrate the initial coupling. | `0x08` (8 Hz Alpha), `0x0D` (13 Hz Beta), `0x04` (4 Hz Theta) |
| `8-15` | **Hemisphere Bias** | Weighting for asymmetric effects, allowing for targeted influence on left or right brain functions. | `0x00` (Left), `0x80` (Balanced), `0xFF` (Right) |
| `16-23` | **Primary Cortical Region** | Tunes the THz holographic shape to focus on a specific cortical area's microtubule geometry. | `0x01` (Auditory), `0x02` (Visual), `0x03` (Motor), `0x04` (Prefrontal) |
| `24-31` | **Microtubule Resonance Key** | A unique hash derived from external analysis of the target's specific microtubule lattice vibrational modes. | `0x91`, `0x56`, `0xEF` (Individual-specific and non-transferable) |

### 3.2 Command Type Field (32 Bits)
Defines the high-level intent of the waveform, translated from a GOCS API call.

| Bit Range | Sub-Field | Description | Example Values |
| :--- | :--- | :--- | :--- |
| `0-7` | **Primary Vibrational Mode** | Selects the fundamental microtubule oscillation to be targeted. | `0x01` (Breathing), `0x02` (Longitudinal), `0x03` (Torsional) |
| `8-15` | **Intensity Modulation Pattern** | Defines the overall shape and onset of the effect's power curve. | `0x10` (Smooth Sine), `0x20` (Step Function), `0x30` (Pulsed), `0x40` (Chaotic) |
| `16-23` | **Duration** | The number of frames this specific command state will last for. | `0x01` (Single Frame), `0x50` (80 Frames), `0xFF` (Sustained) |
| `24-31` | **Sequencing Control** | Manages multi-frame effects. Encoded as `[Current Frame | Total Frames]` (4 bits each). | `0x01` (Frame 0 of 1), `0x1A` (Frame 1 of 10), `0x48` (Frame 4 of 8) |

### 3.3 Data Payload Field (128 Bits)
The core of the template, providing the low-level parameters for the PAL to sculpt the waveform's detailed structure and dynamics.

| Bit Range | Sub-Field | Description |
| :--- | :--- | :--- |
| `0-31` | **Phase Rotation Sequence** | A sequence of 16x 2-bit values defining the QPSK phase state for each of the first 16 symbols of the payload. This sculpts the primary quantum interference pattern. |
| `32-63` | **Frequency Modulation Envelope** | Defines the "vibrational texture" by controlling the micro-level dither of the 12 kHz carrier *within* the frame's duration. |
| `64-95` | **Amplitude Modulation Pattern** | Controls the fine-grained intensity variations (modulation depth) of the signal, synchronized with the phase rotations. |
| `96-127` | **Coherence Maintenance Parameters** | Instructions passed to the HCI to adjust the `Pump Beam` (Carrier 1) intensity/phase to proactively counter environmental decoherence. |

### 3.4 ECC Field (32 Bits)
Ensures the physical fidelity of the transmitted waveform pattern.

| Bit Range | Sub-Field | Description |
| :--- | :--- | :--- |
| `0-15` | **Redundant Phase Encoding** | Repeats the most critical phase transitions from the `Phase Rotation Sequence` to ensure the "attack" and primary harmonic of the waveform are established correctly. |
| `16-31` | **Temporal Sequence Checksum** | A hash of the entire 256-bit frame's temporal characteristics. Allows the receiver hardware to detect and discard corrupted frames that would produce a malformed physical effect. |

---

## 4.0 Nested FSK Layer Specification (1 bit/second)

The FSK layer operates independently of the QPSK frame rate and serves as a slow-acting, subliminal control channel. The GOCS sets the FSK state based on the overarching intent of an operational sequence.

| FSK State | Carrier Frequency | GOCS Intent / Application |
| :--- | :--- | :--- |
| **`0`** | 11,999 Hz | **Baseline / Standby:** Maintains a minimal connection without active entrainment. |
| **`1`** | 12,001 Hz | **Maximize Coupling:** Actively increases the efficiency of the quantum coherence link. |
| **`0/1 Alt`** | 0.5 Hz | **Theta Entrainment:** Creates a 0.5 Hz beat frequency to gently guide brainwaves toward a calm, receptive theta state. |
| **`Random`** | Random | **Disruptive Noise:** Introduces chaotic low-frequency noise to inhibit coherent thought. |