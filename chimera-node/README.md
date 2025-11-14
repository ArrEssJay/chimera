# Chimera Waveform Generator

A standalone implementation of the Chimera waveform generator for audio synthesis with PAL (Protocol Abstraction Layer) pattern support. Works in both Node.js and browser environments.

## Overview

This implementation provides an FSK (Frequency-Shift Keying) sine oscillator with LFO-driven modulation following the Chimera modulation protocol v4.2 and PAL waveform generation models v1.1:

- **FSK Control**: Switch between 11,999 Hz (State 0) and 12,001 Hz (State 1)
- **LFO Modulation**: Frequency and amplitude modulation at 16 Hz symbol rate
- **Pattern Library**: Pre-configured GOCS patterns for various cognitive effects

## Features

✅ **Dual Environment Support**

- Node.js: Generate and save WAV files
- Browser: Real-time Web Audio API playback

✅ **PAL Waveform Generation**

- Symbol rate: 16 Hz (62.5ms per symbol)
- LFO-driven frequency modulation (vibrational texture)
- LFO-driven amplitude modulation (breathing/pulsing)
- Waveform shapes: sine, sawtooth, square, noise

✅ **GOCS Pattern Library**

- 13 pre-configured patterns across 4 categories
- Coherence & Entrainment (theta, alpha, beta, gamma)
- Cognitive & Perceptual (dissonance, inquisitive, emotional, subliminal)
- Disruption & Denial (scramble, motor lock, vertigo, dread)
- Utility & Calibration (baseline, sync)

✅ **Simple API**

- FSK state control (0 or 1)
- Direct LFO parameter control
- Pattern-based control via ChimeraController
- Intensity scaling for patterns

## Installation

```bash
cd strudel
npm install
```

## Usage

### Node.js with Pattern Library

```javascript
import { createChimeraOscillator } from './chimera-oscillator.js';
import { ChimeraController } from './chimera-patterns.js';

const oscillator = createChimeraOscillator();
const controller = new ChimeraController(oscillator);

// Apply a pattern with intensity scaling
controller.applyPattern('COH.ThetaCalm', 0.7);

// Generate samples
const samples = oscillator.generateSamples(48000, 48000); // 1 second at 48kHz
```

**Run pattern examples:**

```bash
npm run patterns
```

This generates 9 WAV files demonstrating different patterns from the library.

### Node.js with Direct LFO Control

```javascript
import { createChimeraOscillator } from './chimera-oscillator.js';

const oscillator = createChimeraOscillator();

// Set FSK state
oscillator.setFSKState(0);  // 11,999 Hz

// Configure frequency modulation (vibrational texture)
oscillator.setFreqModulation(6, 'sine', 0.5);  // 6 Hz sine, 50% depth

// Configure amplitude modulation (breathing/pulsing)
oscillator.setAmpModulation(2, 'sine', 0.3);   // 2 Hz sine, 30% depth

// Generate audio samples
const samples = oscillator.generateSamples(48000, 48000);
```

**Run LFO examples:**

```bash
npm start
```

### Test Coverage

Comprehensive test patterns covering all waveform configurations:

```bash
npm test
```

This generates 20 test WAV files covering:

- **Pure Frequency Modulation** (4 patterns): sine, sawtooth, square, noise
- **Pure Amplitude Modulation** (4 patterns): sine, sawtooth, square, noise  
- **Combined Modulation** (4 patterns): all shape combinations
- **Edge Cases** (5 patterns): slow/fast rates, asymmetric, min/max depth
- **FSK States** (3 patterns): state 0, state 1, alternate, random

### Browser (Web Audio)

```javascript
const oscillator = createChimeraOscillator();
const audioContext = new AudioContext();

oscillator.setFSKState(1);
oscillator.setAmplitude(0.5);

const { oscillator: node, gainNode } = oscillator.createWebAudioNode(audioContext);
gainNode.connect(audioContext.destination);
node.start();
```

**Open the browser example:**

Simply open `browser-example.html` in a web browser for an interactive demo with real-time FSK control.

## API

### ChimeraOscillator

Core oscillator with FSK and LFO control.

#### Methods

- `setFSKState(state)` - Set FSK state (0 or 1)
- `getFSKState()` - Get current FSK state
- `getFrequency()` - Get current frequency in Hz
- `setAmplitude(amp)` - Set amplitude (0.0 to 1.0)
- `getAmplitude()` - Get current amplitude
- `setPhase(phase)` - Set phase offset (0.0 to 1.0, where 1.0 = 2π)
- `getPhase()` - Get current phase
- `setFreqModulation(frequency, shape, depth)` - Configure frequency LFO
- `setAmpModulation(frequency, shape, depth)` - Configure amplitude LFO
- `getFreqLFO()` - Get frequency modulation LFO instance
- `getAmpLFO()` - Get amplitude modulation LFO instance
- `getSymbolRate()` - Get symbol rate (16 Hz)
- `generateSamples(numSamples, sampleRate, startTime)` - Generate Float32Array of samples
- `createWebAudioNode(audioContext)` - Create Web Audio oscillator and gain nodes (browser only)
- `getParams()` - Get all current parameters as object

### ChimeraController

Pattern-based control interface.

#### Controller Methods

- `applyPattern(patternId, intensity)` - Apply a pattern from the library
- `getCurrentPattern()` - Get currently applied pattern
- `listPatterns(category)` - List available patterns (optional category filter)
- `getPattern(patternId)` - Get pattern by ID

### ChimeraPattern

Pattern definition from GOCS Base Pattern Library v1.1.

#### Available Patterns

**Coherence & Entrainment:**

- `COH.ThetaCalm` - Deep calm and receptivity (6 Hz)
- `COH.AlphaFocus` - Relaxed alert focus (10 Hz)
- `COH.BetaAlert` - Active alertness (18 Hz)
- `COH.GammaSync` - High-level cognitive processing (40 Hz)

**Cognitive & Perceptual:**

- `COG.Dissonance` - Subtle unease (5 Hz)
- `COG.InquisitiveUrge` - Buzzy mental state (22 Hz)
- `COG.EmotionalResonance` - Amplify current emotion (14 Hz)
- `COG.SubliminalGate` - High suggestibility window (4 Hz)

**Disruption & Denial:**

- `DIS.CognitiveScramble` - Chaotic noise (25 Hz)
- `DIS.MotorLock` - Motor cortex interference (15 Hz)
- `DIS.VestibularJolt` - Vertigo (0.5 Hz)
- `DIS.DreadPulse` - Anxiety (2 Hz)

**Utility & Calibration:**

- `UTIL.BaselineCarrier` - Unmodulated carrier

**Test Coverage:**

- `TEST.PureFreqSine/Sawtooth/Square/Noise` - Isolated frequency modulation
- `TEST.PureAmpSine/Sawtooth/Square/Noise` - Isolated amplitude modulation
- `TEST.SineSine/SawtoothSquare/SquareSawtooth/NoiseNoise` - Combined modulation
- `TEST.SlowModulation/FastModulation` - Rate extremes
- `TEST.AsymmetricRates` - Different freq/amp rates
- `TEST.MaxDepth/MinDepth` - Depth extremes
- `TEST.FSKState1/FSKAlternate/FSKRandom` - FSK state variations

## Architecture

```text
ChimeraOscillator
├── FSK State Control (0 or 1)
├── Frequency: 11,999 Hz or 12,001 Hz
├── Amplitude Control (0.0 to 1.0)
├── Phase Control (0.0 to 1.0)
├── generateSamples() → Float32Array (Node.js/Browser)
└── createWebAudioNode() → {oscillator, gainNode} (Browser only)
```

## Files

- **chimera-oscillator.js** - Core oscillator with LFO support
- **chimera-patterns.js** - Pattern library and controller (33 total patterns)
- **example.js** - Direct LFO control examples
- **example-patterns.js** - Pattern library demonstration
- **example-test-coverage.js** - Comprehensive test coverage suite
- **browser-example.html** - Interactive browser demo
- **package.json** - Node.js dependencies (wavefile for file I/O)

## Next Steps

Future enhancements could include:

- QPSK modulation layer (16 symbols/second)
- Amplitude modulation patterns
- Phase rotation sequences
- Alternating FSK mode (0.5 Hz switching)
- Random FSK mode (decoherence field)
- Real-time streaming with continuous phase

## References

See the protocol specifications in `/docs/spec/`:

- `modulation_protocol_v4.2.md` - Core modulation protocol
- `pal_waveform_models_v1.1.md` - Waveform generation models
- `frame_structure_v3.1.md` - Frame structure and encoding
