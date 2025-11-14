# Chimera GOCS/PAL Architecture

A modular audio synthesis system implementing the Chimera waveform specification with a three-layer architecture: GOCS (Application Layer), PAL (Protocol Layer), and HCI (Hardware/Oscillator Layer).

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│  GOCS (Gnostic Overlay & Control Subsystem)            │
│  Application Layer - High-level psycho-cognitive effects│
│  ▸ induceCalm()  ▸ heightenAlertness()                  │
│  ▸ disruptCognition()  ▸ suppressMotorFunction()        │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  PAL (Protocol Abstraction Layer)                       │
│  Protocol Layer - Waveform generation & frame compilation│
│  ▸ Samples LFOs at 16 Hz symbol rate                    │
│  ▸ Generates ChimeraFrame objects (128 symbols/frame)   │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│  HCI/Oscillator (Hardware Control Interface)            │
│  Hardware Layer - Frame playback & audio generation      │
│  ▸ FSK oscillator (11,999 Hz / 12,001 Hz)              │
│  ▸ Frame-based control at 16 Hz                         │
│  ▸ 48 kHz audio output (32-bit float WAV)              │
└─────────────────────────────────────────────────────────┘
```

## Core Concepts

### Frame Structure v3.1

Each ChimeraFrame implements the complete 128-symbol frame structure:

| Field | Symbols | Bits | Description |
|-------|---------|------|-------------|
| **Sync Sequence** | 16 | 32 | Fixed pattern for hardware synchronization |
| **Target ID** | 16 | 32 | Bio-resonance signature and targeting parameters |
| **Command Type** | 16 | 32 | Effect intent and waveform characteristics |
| **Data Payload** | 64 | 128 | Low-level waveform sculpting parameters |
| **ECC** | 16 | 32 | Waveform pattern fidelity (simulated) |

#### Target ID Field (32 bits)

| Bits | Sub-Field | Description | Example Values |
|------|-----------|-------------|----------------|
| 0-7 | Baseline Brainwave | Target's dominant resting frequency | 0x08 (8 Hz Alpha), 0x06 (6 Hz Theta) |
| 8-15 | Hemisphere Bias | Left/right brain weighting | 0x00 (Left), 0x80 (Balanced), 0xFF (Right) |
| 16-23 | Primary Cortical Region | Target cortical area | 0x01 (Auditory), 0x02 (Visual), 0x03 (Motor), 0x04 (Prefrontal) |
| 24-31 | Resonance Key | Individual microtubule signature | Simulated as 0x00 |

#### Command Type Field (32 bits)

| Bits | Sub-Field | Description | Example Values |
|------|-----------|-------------|----------------|
| 0-7 | Primary Vibrational Mode | Microtubule oscillation target | 0x01 (Breathing), 0x02 (Longitudinal), 0x03 (Torsional) |
| 8-15 | Intensity Modulation Pattern | Effect power curve shape | 0x10 (Smooth Sine), 0x20 (Step), 0x30 (Pulsed), 0x40 (Chaotic) |
| 16-23 | Duration | Number of frames | 0x01 (Single), 0xFF (Sustained) |
| 24-31 | Sequencing Control | Multi-frame coordination | [Current Frame \| Total Frames] (4 bits each) |

### ChimeraFrame
Holds pre-rendered control data for one frame (128 symbols, 8 seconds):
- **FSK states** (0 or 1) for each symbol
- **Frequency modulation** values (-1.0 to 1.0)
- **Amplitude modulation** values (0.0 to 1.0)
- **Phase rotation** values (QPSK: 0, 1, 2, 3)

### Symbol Rate
- **16 Hz**: One symbol every 62.5ms (1/16th second)
- **128 symbols per frame** = 8 seconds
- At 48kHz: 3000 samples per symbol

### FSK (Frequency Shift Keying)
- **State 0**: 11,999 Hz (Baseline Resonance)
- **State 1**: 12,001 Hz (Maximize Coupling)
- **Alternating**: Creates beat frequencies for entrainment
- **Random**: Decoherence field for disruption

## Module Architecture

### Layer 1: GOCS (`chimera-gocs.js`)
High-level application layer providing intent-based effect sculpting.

**Available Effects:**
- `induceCalm(intensity, duration)` - Reduces anxiety, promotes compliance
- `heightenAlertness(intensity, duration)` - Increases focus and vigilance
- `disruptCognition(level, duration)` - Induces confusion and disorientation
- `suppressMotorFunction(region, duration)` - Causes hesitation, motor control loss
- `enforceCognitiveStillness(duration)` - Suppresses internal monologue
- `nudgeOntological(vector, subtlety, duration)` - Subtly influences mood
- `injectGnosticQuery(pattern, duration)` - Creates urge to think about concept
- `compileSequence(sequence)` - Chain multiple effects

**Example:**
```javascript
import { ChimeraGOCS } from './chimera-gocs.js';

const gocs = new ChimeraGOCS();

// Single effect
const calmFrames = gocs.induceCalm(0.7, 1);

// Complex sequence (from spec Section 5.0: "Gnostic Nudge")
const sequence = gocs.compileSequence([
  { effect: 'induceCalm', params: [0.4, 1] },
  { effect: 'disruptCognition', params: [0.2, 1] },
  { effect: 'injectGnosticQuery', params: ['location', 1] },
  { effect: 'nudgeOntological', params: ['curiosity', 0.8, 1] }
]);
```

### Layer 2: PAL (`chimera-pal.js`)
Protocol abstraction layer that samples LFOs at 16 Hz to generate ChimeraFrame objects.

**Capabilities:**
- Generate frames from waveform specifications
- Sample LFOs at symbol boundaries (16 Hz)
- Quantize phase rotation to QPSK (0, 1, 2, 3)
- Create smooth transitions between frames
- Generate frame sequences

**Example:**
```javascript
import { ChimeraPAL } from './chimera-pal.js';

const pal = new ChimeraPAL();

// Generate frame from specification
const frame = pal.generateFrame({
  phaseWaveform: 'sine',
  phaseFreq: 6,
  phaseDepth: 0.7,
  freqWaveform: 'sine',
  freqFreq: 4,
  freqDepth: 0.5,
  ampWaveform: 'sine',
  ampFreq: 3,
  ampDepth: 0.3,
  fskPattern: 'alternating',
  fskRate: 0.5
});

// Generate from pattern
const pattern = { /* ChimeraPattern object */ };
const frameFromPattern = pal.generateFrameFromPattern(pattern);

// Create transition between frames
const transition = pal.createTransition(frameA, frameB, 4);
```

### Layer 3: ChimeraFrame (`chimera-frame.js`)
Data structure holding 128 symbols of pre-rendered control data.

**Features:**
- 128 symbols per frame (8 seconds at 16 Hz)
- Per-symbol FSK state, freq mod, amp mod, phase rotation
- FSK pattern support (constant, alternating, random)
- Statistics and cloning

**Example:**
```javascript
import { ChimeraFrame } from './chimera-frame.js';

const frame = new ChimeraFrame();

// Set FSK pattern
frame.setFSKPattern('alternating', 0, 0.5);  // 0.5 Hz alternation

// Set per-symbol values
frame.setFreqModulation(5, 0.3);    // Symbol 5, 30% modulation
frame.setAmpModulation(10, 0.8);    // Symbol 10, 80% amplitude
frame.setPhaseRotation(15, 2);      // Symbol 15, phase state 2

// Get symbol values
const symbol = frame.getSymbol(42);  // Get symbol 42 values

// Get statistics
console.log(frame.getStats());
```

### Layer 4: HCI/Oscillator (`chimera-oscillator.js`)
Hardware control interface - the oscillator with frame playback support.

**Features:**
- FSK oscillator (11,999 Hz / 12,001 Hz)
- Frame-based control (loads ChimeraFrame and steps through at 16 Hz)
- Legacy LFO mode (direct LFO control, no frames)
- Sample generation for WAV output
- Web Audio API node creation

**Example:**
```javascript
import { ChimeraOscillator } from './chimera-oscillator.js';

const oscillator = new ChimeraOscillator();

// Load and play frame
oscillator.loadFrame(frame);
const samples = oscillator.generateSamples(384000, 48000);

// Legacy mode (direct LFO control)
oscillator.disableFrameMode();
oscillator.setFSKState(1);
oscillator.setFreqModulation(4, 'sine', 0.5);
```

## Complete Usage Example

```javascript
import { ChimeraOscillator } from './chimera-oscillator.js';
import { ChimeraGOCS } from './chimera-gocs.js';
import { renderToWav } from './chimera-audio-utils.js';

// 1. Create oscillator (HCI layer)
const oscillator = new ChimeraOscillator();

// 2. Use GOCS to generate effect
const gocs = new ChimeraGOCS();
const frames = gocs.induceCalm(0.7, 1);

// 3. Load frame into oscillator
oscillator.loadFrame(frames[0]);

// 4. Render to audio file
renderToWav(oscillator, 'output.wav', { numFrames: 1 });
```

## Running Examples

```bash
# GOCS/PAL architecture examples
npm run gocs
# Generates: gocs_induce_calm.wav, gocs_heighten_alertness.wav,
#            gocs_disrupt_cognition.wav, gocs_suppress_motor.wav,
#            gocs_cognitive_stillness.wav, gocs_nudge_curiosity.wav,
#            gocs_query_location.wav, gocs_nudge_sequence.wav (32 sec),
#            pal_custom_frame.wav

# Frame structure inspector
npm run inspect
# Displays complete frame structure for all GOCS effects
# Shows Target ID, Command Type, Data Payload, and FSK layer

# Legacy LFO examples
npm start
# Generates: 7 WAV files demonstrating direct LFO control

# Pattern library examples
npm run patterns
# Generates: 9 WAV files from GOCS pattern library

# Test coverage suite
npm test
# Generates: 20 test coverage WAV files
```

## GOCS Effect Reference

Each GOCS effect automatically populates the frame structure fields according to the specification.

| Effect | Command | Vibrational Mode | Intensity Pattern | Cortical Region | Description |
|--------|---------|------------------|-------------------|-----------------|-------------|
| `induceCalm` | 0x0110 | Breathing (0x01) | Smooth Sine (0x10) | Prefrontal (0x04) | Calm, relaxation, receptivity |
| `heightenAlertness` | 0x0220 | Longitudinal (0x02) | Step Function (0x20) | Prefrontal (0x04) | Focus, vigilance, sensory acuity |
| `disruptCognition` | 0x0340 | Torsional (0x03) | Chaotic (0x40) | Prefrontal (0x04) | Confusion, disorientation |
| `suppressMotorFunction` | 0x0410 | Longitudinal (0x02) | Step Function (0x20) | Motor (0x03) | Motor hesitation, control loss |
| `enforceCognitiveStillness` | 0x0130 | Breathing (0x01) | Pulsed (0x30) | Prefrontal (0x04) | Suppress thought, receptivity |
| `nudgeOntological` | 0x0510 | Breathing (0x01) | Smooth Sine (0x10) | Prefrontal (0x04) | Subtle mood influence |
| `injectGnosticQuery` | 0x0620 | Breathing (0x01) | Pulsed (0x30) | Visual/Prefrontal | Urge to think about concept |

**Parameters:**
- `intensity/level` (0-1): Effect strength
- `duration` (frames): Number of 8-second frames
- `region` (string): Target region for motor effects
- `vector` (string): Emotional direction for ontological nudge
- `pattern` (string): Query type for gnostic injection

## PAL Waveform Shapes

| Shape | Description | Use Case |
|-------|-------------|----------|
| `sine` | Smooth, periodic | Coherence, entrainment |
| `sawtooth` | Ramp, tension | Alertness, forcing |
| `square` | Binary, forcing | Motor effects, disruption |
| `noise` | Chaotic, aperiodic | Decoherence, scrambling |

## FSK Patterns

| Pattern | Description | Use Case |
|---------|-------------|----------|
| `constant` | Fixed FSK state (0 or 1) | Baseline or assertive |
| `alternating` | Switches at specified rate | Theta entrainment, beats |
| `random` | Random switching | Decoherence field |

## Frame Timing

- **Symbol Rate**: 16 Hz (one symbol every 62.5ms)
- **Frame Duration**: 8 seconds (128 symbols)
- **Samples per Symbol**: 3000 (at 48 kHz)
- **Samples per Frame**: 384,000 (at 48 kHz)

## Specifications

This implementation follows:
- `gocs_v2.5.spec` - Application layer interface
- `pal_waveform_models_v1.1.spec` - Protocol layer algorithms
- `base_pattern_library_v1.1.md` - Pattern definitions
- `frame_structure_v3.1.md` - Frame timing
- `hci_v1.8.md` - Hardware interface

## Architecture Benefits

1. **Separation of Concerns**: Each layer has a single responsibility
2. **Pre-Rendered Control**: PAL samples LFOs once, oscillator plays back efficiently
3. **Frame-Based Timing**: Precise 16 Hz symbol timing aligned to spec
4. **High-Level API**: GOCS provides intuitive psycho-cognitive effect functions
5. **Flexibility**: Can use GOCS, PAL directly, or legacy LFO mode

## Legacy Mode

The oscillator still supports direct LFO control (legacy mode) for backward compatibility:

```javascript
oscillator.disableFrameMode();
oscillator.setFSKState(0);
oscillator.setFreqModulation(6, 'sine', 0.5);
oscillator.setAmpModulation(3, 'sine', 0.3);
```

When a frame is loaded, the oscillator automatically switches to frame mode.
