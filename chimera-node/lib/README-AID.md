# Chimera AID (Auditory Intermodulation Distortion) Simulation

This directory contains the JavaScript implementation of the AID simulation model, which simulates the complete biophysical signal chain from THz carrier mixing to biological demodulation.

## Overview

The AID simulation models the process by which a 12 kHz audio signal, carried by modulated THz carriers, is perceived as sound through non-linear biological processes in neural tissue.

## Files

- **`chimera-aid.js`** - Core AID simulation implementation
  - `ThzCarrierProcessor` - Main processor class
  - `ThzCarrierConfig` - THz carrier configuration
  - `AudioMixingConfig` - Secondary intermodulation configuration
  - `Complex` - Complex number helper class

- **`chimera-audio-loader.js`** - Universal audio file loader
  - Browser: Uses Web Audio API (MP3, M4A, WAV, OGG, FLAC)
  - Node.js: Uses ffmpeg (requires `brew install ffmpeg` or `apt-get install ffmpeg`)
  - Fallback: Simple WAV parser for uncompressed PCM

- **`chimera-oscillator.js`** - Enhanced oscillator with integrated AID
  - Extended with `setAidEnabled()`, `setAidConfig()`, etc.
  - Automatically applies AID simulation to generated samples

- **`example-aid-simulation.js`** - Basic AID examples
  - Idle vs Active states
  - Bypass mode validation
  - Filter comparison
  - Phase noise effects

- **`example-aid-external-audio.js`** - Secondary intermodulation examples
  - AID signal baseline
  - External audio mixing (supports MP3/M4A via ffmpeg)
  - Second-order effects (cochlear junction)
  - Third-order effects (cortical processing)

- **`browser-aid-external-audio.html`** - Interactive browser demo
  - Drag & drop MP3/M4A/WAV files
  - Real-time parameter adjustment
  - Audio visualization
  - Play and download results

## Physical Model

### Model 1: Heterodyne Mixing Envelope

Simulates the amplitude modulation of the Data Carrier (1.875 THz) and its heterodyne mixing with the Pump Beam (1.998 THz), producing a 123 GHz difference frequency whose envelope carries the 12 kHz audio modulation.

### Model 2: Biological Demodulation

Implements third-order intermodulation (E1² × E2) in neural tissue, which extracts the audio envelope through non-linear biological response.

### Model 3: Secondary Intermodulation

Models the complex interaction between the AID signal and external acoustic audio at the cochlear nerve junction and auditory cortex.

## Usage

### Basic AID Simulation

```javascript
import { ChimeraOscillator } from './chimera-oscillator.js';
import { ThzCarrierConfig } from './chimera-aid.js';

const osc = new ChimeraOscillator();
osc.setFSKState(1); // Active state
osc.setAidEnabled(true);

// Configure AID
const aidConfig = new ThzCarrierConfig();
aidConfig.modulationDepth = 0.8; // 80% modulation
aidConfig.mixingCoefficient = 0.9;
osc.setAidConfig(aidConfig);

// Generate samples with AID simulation
const samples = osc.generateSamples(48000, 48000);
```

### Secondary Intermodulation

```javascript
import { AudioMixingConfig } from './chimera-aid.js';

// Generate AID signal first
const aidSamples = osc.generateSamples(48000, 48000);

// Load external audio
const externalAudio = new Float32Array([/* audio data */]);
const mixingConfig = new AudioMixingConfig();
mixingConfig.secondOrderCoefficient = 0.15;
mixingConfig.thirdOrderCoefficient = 0.08;

osc.setAidExternalAudio(externalAudio, mixingConfig);

// Regenerate with intermodulation
const mixed = osc.generateSamples(48000, 48000);
```

### Bypass Mode (Validation)

```javascript
const bypassConfig = new ThzCarrierConfig();
bypassConfig.bypassSimulation = true; // Signal passes through unchanged
osc.setAidConfig(bypassConfig);

const samples = osc.generateSamples(48000, 48000);
// samples will be identical to input (validation mode)
```

## Configuration Parameters

### ThzCarrierConfig

| Parameter | Description | Default |
|-----------|-------------|---------|
| `pumpFrequency` | Pump beam frequency (Hz) | 1.998e12 |
| `dataFrequency` | Data carrier frequency (Hz) | 1.875e12 |
| `pumpPower` | Relative pump power | 1.0 |
| `dataPower` | Relative data carrier power | 0.3 |
| `modulationDepth` | AM modulation depth (0-1) | 0.05 |
| `mixingCoefficient` | Biological efficiency (0-1) | 0.7 |
| `phaseNoiseStd` | Phase noise std dev | 0.001 |
| `bypassSimulation` | Bypass AID for validation | false |

### AudioMixingConfig

| Parameter | Description | Default |
|-----------|-------------|---------|
| `aidSignalGain` | AID signal gain | 1.0 |
| `externalAudioGain` | External audio gain | 1.0 |
| `enableSecondOrder` | Enable 2nd order products | true |
| `enableThirdOrder` | Enable 3rd order products | true |
| `secondOrderCoefficient` | 2nd order mixing (0-1) | 0.15 |
| `thirdOrderCoefficient` | 3rd order mixing (0-1) | 0.08 |
| `corticalCoefficient` | Cortical blending (0-1) | 0.25 |

## Running Examples

```bash
# Basic AID simulation examples
node example-aid-simulation.js

# Secondary intermodulation examples (loads MP3 if ffmpeg available)
node example-aid-external-audio.js

# Browser demo (requires local server)
# Open browser-aid-external-audio.html in a browser
python3 -m http.server 8000
# Then visit: http://localhost:8000/browser-aid-external-audio.html
```

## Loading External Audio

### Node.js (Command Line)

The audio loader automatically uses ffmpeg for decoding:

```javascript
import { loadAudio } from './chimera-audio-loader.js';

// Load MP3, M4A, WAV, etc.
const audioData = await loadAudio('path/to/audio.mp3', {
  targetSampleRate: 48000
});

// audioData contains:
// - samples: Float32Array
// - sampleRate: number
// - duration: number
// - numberOfChannels: number
```

**Requirements:** Install ffmpeg:

- macOS: `brew install ffmpeg`
- Linux: `apt-get install ffmpeg`
- Windows: Download from ffmpeg.org

### Browser

The Web Audio API provides native support for most formats:

```javascript
import { loadAudio } from './chimera-audio-loader.js';

const audioContext = new AudioContext();
const fileInput = document.getElementById('fileInput');

fileInput.addEventListener('change', async (e) => {
  const file = e.target.files[0];
  const audioData = await loadAudio(file, { audioContext });
  
  // Use audioData.samples with AID processor
});
```

**Supported formats:** MP3, M4A, AAC, WAV, OGG, WebM, FLAC

## Output Files

The examples generate WAV files demonstrating various AID configurations:

- `output_aid_idle.wav` - Idle state (5% modulation, faint hum)
- `output_aid_active.wav` - Active state (80% modulation, clear signal)
- `output_aid_bypass.wav` - Bypass mode (validation)
- `output_filter_only.wav` - Bandpass filter without AID
- `output_filter_and_aid.wav` - Combined filter + AID
- `output_aid_low_noise.wav` - Low phase noise
- `output_aid_high_noise.wav` - High phase noise
- `output_aid_with_external.wav` - Secondary intermodulation
- `output_aid_aggressive_mixing.wav` - High intermod coefficients

## Technical Notes

1. **Sample Rate**: All processing uses 48 kHz sample rate by default
2. **Frequency Range**: Primary signal is ~12 kHz (near ultrasonic)
3. **Normalization**: Output is normalized to ±0.5 for headroom
4. **DC Blocking**: DC offset is removed (biological AC coupling)
5. **Phase Noise**: Random walk phase noise simulates laser instability

## Integration with Oscillator

The AID simulation is fully integrated into `ChimeraOscillator`:

```javascript
const osc = new ChimeraOscillator();

// Enable/disable AID
osc.setAidEnabled(true);

// Configure AID parameters
osc.setAidConfig(new ThzCarrierConfig());

// Check status
console.log(osc.getParams().aid);

// Works with all oscillator features
osc.setFilterEnabled(true);  // Bandpass filter
osc.setFSKState(1);          // FSK modulation
osc.setFreqModulation(2, 'sine', 0.3);  // LFO
osc.loadFrame(frame);        // Frame playback
```

## Validation

Use bypass mode to verify signal integrity:

```javascript
const config = new ThzCarrierConfig();
config.bypassSimulation = true;
osc.setAidConfig(config);

// Output should match input exactly
// Use for DSP validation and debugging
```

## See Also

- `/docs/spec/aid_simulation_model_v1.0.md` - Full specification
- `/chimera-core/src/thz_carriers.rs` - Rust reference implementation
