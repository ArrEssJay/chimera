# Chimera GOCS Real-time Interface

A real-time React + WebAudio interface for generating and monitoring Chimera waveforms with GOCS (Guided Oscillator Control System) and AID (Auditory Intermodulation Distortion) simulation.

## Features

### Real-time Audio Generation
- **Fixed 16 Hz symbol rate** (per HCI spec)
- **8-second frames** with 128 symbols
- Continuous real-time generation using WebAudio API
- Sample-accurate frame boundaries

### GOCS Control
- Function selection (induceCalm, heightenAlertness, disruptCognition, etc.)
- Real-time intensity adjustment (0-100%)
- Duration control (number of frames)
- Frame timing display with 8-second countdown

### Pattern Library
- Base Pattern Library integration
- Coherence & Entrainment patterns (COH.*)
- Cognitive & Perceptual patterns (COG.*)
- Disruption & Denial patterns (DIS.*)
- Utility & Calibration patterns (UTIL.*)

### AID Simulation
- Real-time enable/disable toggle
- Modulation depth control (0-100%)
- Mixing coefficient adjustment
- Phase noise parameter tuning
- Bypass mode for validation
- Live parameter updates without audio interruption

### Visualizations
- **Oscilloscope**: Time-domain waveform display
- **Spectrum Analyzer**: Frequency-domain FFT visualization
- **Frame Inspector**: Real-time frame parameters and timing

### Transport Controls
- Play/Pause audio generation
- Frame advance (skip to next frame)
- Status indicators
- Frame counter and timing display

## Project Structure

```
chimera-node/
├── lib/                    # Core Chimera library (moved from root)
│   ├── chimera-gocs.js
│   ├── chimera-oscillator.js
│   ├── chimera-pal.js
│   ├── chimera-aid.js
│   └── ...
├── src/                    # React application
│   ├── audio/
│   │   ├── AudioEngine.ts      # WebAudio management
│   │   └── AudioProvider.tsx   # React context
│   ├── components/
│   │   ├── GOCSControls.tsx
│   │   ├── AIDControls.tsx
│   │   ├── PatternSelector.tsx
│   │   ├── Oscilloscope.tsx
│   │   ├── SpectrumAnalyzer.tsx
│   │   ├── FrameInspector.tsx
│   │   ├── TransportBar.tsx
│   │   ├── ControlPanel.tsx
│   │   └── VisualizationPanel.tsx
│   ├── types/
│   │   └── audio.ts            # TypeScript interfaces
│   ├── App.tsx
│   └── main.tsx
├── examples/               # Example scripts (moved from root)
├── tests/                  # Test files
└── index.html             # Vite entry point
```

## Installation

```bash
npm install
```

## Development

Start the development server:

```bash
npm run dev
```

The application will open at `http://localhost:3000`

## Build for Production

```bash
npm run build
```

The built files will be in the `dist/` directory.

## Usage

1. **Start Audio**: Click the "Play" button in the transport bar
2. **Select GOCS Function**: Choose from the dropdown (Induce Calm, Heighten Alertness, etc.)
3. **Adjust Intensity**: Use the slider to control effect strength (0-100%)
4. **Enable AID**: Toggle AID simulation and adjust parameters in real-time
5. **Monitor Visualizations**: Watch the oscilloscope, spectrum analyzer, and frame inspector
6. **Advance Frames**: Use "Next Frame" to skip to the next 8-second frame

## Technical Details

### Frame Timing (per HCI Spec)
- Symbol rate: **16 Hz** (fixed, non-adjustable)
- Symbols per frame: **128**
- Frame duration: **8.0 seconds** (128 symbols ÷ 16 Hz)
- Symbol interval: **62.5 ms** (1 ÷ 16 Hz)

### Audio Processing
- Sample rate: 48 kHz
- Processing: ScriptProcessorNode (AudioWorklet support planned)
- Buffer size: 4096 samples
- Latency: < 10ms target

### GOCS Functions
- `induceCalm` - Theta entrainment
- `heightenAlertness` - Alpha enhancement
- `disruptCognition` - Cognitive scrambling
- `suppressMotorFunction` - Motor disruption
- `enforceCognitiveStillness` - Attention narrowing
- `nudgeOntological` - Perceptual shift
- `injectGnosticQuery` - Query injection

### AID Parameters
- **Modulation Depth**: Controls THz carrier modulation (0-100%)
- **Mixing Coefficient**: Heterodyne mixing strength (0-100%)
- **Phase Noise**: Simulates carrier phase noise (0-0.01)
- **Bypass Mode**: Disables simulation for validation

## Examples

Run example scripts from the command line:

```bash
# Basic GOCS generation
npm run gocs

# Pattern library examples
npm run patterns

# Frame structure inspection
npm run inspect
```

## Notes

- Frame timing is **fixed at 16 Hz** as per the PAL specification
- Frames are **always 8 seconds long** (128 symbols)
- AID parameters can be adjusted **in real-time** without interrupting audio
- GOCS function changes take effect on the **next frame boundary**

## License

ISC
