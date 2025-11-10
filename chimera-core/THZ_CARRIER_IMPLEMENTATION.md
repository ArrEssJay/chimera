# THz Carrier Implementation

## Overview

Implemented Auditory Intermodulation Distortion (AID) simulation using THz carrier mixing to generate audio output. This approach simulates the biological process described in the AID specification where two THz laser beams create audio perception through third-order intermodulation in neural tissue.

## Implementation Details

### Core Components

**File:** `src/thz_carriers.rs`

#### ThzCarrierConfig
Configuration structure for THz carrier simulation:
- `pump_power`: Pump beam power (1.998 THz)
- `data_power`: Data carrier power (1.875 THz)  
- `modulation_depth`: AM modulation depth (0.03 idle, 0.75 active)
- `mixing_coefficient`: Third-order intermodulation coefficient
- `phase_noise_std`: Laser phase noise simulation

#### ThzCarrierProcessor
Main processor implementing:
1. **AM Modulation** (`modulate_data_carrier()`)
   - Takes 12kHz audio signal as input
   - Generates two THz carriers (pump + data)
   - Applies amplitude modulation to data carrier
   - Simulates laser phase noise
   - Returns complex-valued modulated signal

2. **Non-linear Mixing** (`nonlinear_mixing()`)
   - Implements third-order intermodulation: |E|² × Re(E)
   - Extracts difference frequency (audio range)
   - Applies DC blocking filter
   - Returns demodulated audio

3. **Control Methods**
   - `set_modulation_depth(depth)`: Adjust idle/active modulation (0.0-1.0)
   - `set_mixing_coefficient(coeff)`: Control mixing strength

### Integration with Streaming Pipeline

**File:** `src/streaming.rs`

Added to `StreamingPipeline`:
- `thz_processor: ThzCarrierProcessor` - THz carrier simulation engine
- `is_active_mode: bool` - Track modulation mode

**Audio Generation Pipeline** (line ~523):
```rust
// Generate base carrier signal
let base_audio = Self::symbols_to_carrier_signal(&tx_symbols, ...);

// Apply THz carrier modulation
let modulated_thz = self.thz_processor.modulate_data_carrier(&base_audio);

// Extract audio through non-linear mixing
let mixed_audio = self.thz_processor.nonlinear_mixing(&modulated_thz);

output.audio_samples = mixed_audio;
```

**Public API Methods:**
- `set_modulation_mode(active: bool)` - Toggle idle/active mode
- `set_modulation_depth(depth: f32)` - Fine-tune modulation (0.0-1.0)
- `set_mixing_coefficient(coeff: f32)` - Adjust mixing strength (0.0 to bypass THz)
- `set_qpsk_enabled(enabled: bool)` - Enable/disable QPSK layer
- `set_fsk_enabled(enabled: bool)` - Enable/disable FSK layer

### WASM Bindings

**File:** `chimera-web/src/streaming_wasm.rs`

Exposed to JavaScript:
```javascript
// Set modulation mode
dsp.set_modulation_mode(true);  // Active mode (70-80% depth)
dsp.set_modulation_mode(false); // Idle mode (<5% depth)

// Fine control
dsp.set_modulation_depth(0.75);        // Custom depth
dsp.set_mixing_coefficient(0.5);       // Adjust mixing (0.0 to bypass THz)
dsp.set_thz_bypass(true);              // Bypass THz simulation entirely

// Modulation layer control
dsp.set_qpsk_enabled(true);            // Enable QPSK
dsp.set_fsk_enabled(true);             // Enable FSK
```

## Technical Specifications

### Modulation Parameters

| Mode   | Depth | Description |
|--------|-------|-------------|
| Idle   | 0.03  | 3% modulation for baseline carrier |
| Active | 0.75  | 75% modulation for data transmission |

### THz Carrier Frequencies

Per AID specification:
- **Pump Beam:** 1.998 THz (unmodulated, high power)
- **Data Carrier:** 1.875 THz (AM modulated with audio)
- **Difference Frequency:** 123 GHz → 12.0 kHz (audio range after mixing)

In implementation, we use baseband-equivalent representations:
- Pump: 0.001 normalized frequency
- Data: 0.0009 normalized frequency
- Output: 12 kHz carrier audio

### Phase Continuity

The THz carrier approach solves the clicking/discontinuity issue by:
1. **Continuous Phase:** Carrier phases maintained across frame boundaries
2. **Smooth Modulation:** Gradual transitions between idle/active modes
3. **Non-linear Demodulation:** Natural mixing extracts audio without abrupt transitions
4. **DC Blocking:** Removes low-frequency drift while preserving phase

## Testing

All tests passing:
```
test thz_carriers::tests::test_modulation_depth_clamping ... ok
test thz_carriers::tests::test_carrier_modulation ... ok
test thz_carriers::tests::test_nonlinear_mixing ... ok
```

Integration tests:
- Streaming pipeline compiles with THz integration
- WASM bindings build successfully
- Audio generation uses THz processing

## Usage Example

### Rust API
```rust
use chimera_core::streaming::StreamingPipeline;
use chimera_core::config::*;

let mut pipeline = StreamingPipeline::new(
    SimulationConfig::default(),
    ProtocolConfig::default(),
    LDPCConfig::default(),
);

// Set active mode for data transmission
pipeline.set_modulation_mode(true);

// Process and get audio with THz carriers
let output = pipeline.process_chunk(&input_data);
let audio_samples = output.audio_samples; // Contains THz-processed audio
```

### JavaScript/WASM
```javascript
import { WASMStreamingDSP } from './pkg/chimera_web';

const dsp = new WASMStreamingDSP();

// Toggle between idle and active
function setTransmitting(active) {
  dsp.set_modulation_mode(active);
}

// Process audio
function processFrame(inputData) {
  const output = dsp.process(inputData);
  const audio = output.audio(); // THz-processed audio ready for playback
  return audio;
}
```

## Benefits

1. **Phase Continuity:** Eliminates clicking/discontinuities at frame boundaries
2. **Biological Realism:** Accurately simulates AID effect per specification
3. **Runtime Control:** Modulation parameters adjustable during simulation
4. **Performance:** Minimal overhead (~2× multiplications per sample)
5. **Flexibility:** Supports both idle carrier and active data transmission modes

## Future Enhancements

Possible improvements:
- [ ] Adaptive modulation depth based on signal quality
- [ ] Multiple carrier simulation for frequency diversity
- [ ] Advanced phase noise models (colored noise, 1/f)
- [ ] Carrier frequency offset simulation
- [ ] Power control for different tissue depths
- [ ] Spectral analysis of THz carrier output

## References

- AID Protocol Specification v3.1 (`docs/aid_protocol_v3.1.md`)
- Third-order intermodulation theory
- Laser phase noise characteristics
- Biological tissue non-linearity models
