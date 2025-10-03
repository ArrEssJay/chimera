# Chimera Web UI Controls Specification

This document provides a complete specification of all user interface controls in the Chimera web dashboard. It ensures that all controls are documented and serves as a reference for future development.

## Version Information
- **Last Updated**: January 2025
- **Chimera Version**: 0.1.0
- **Web Framework**: Yew 0.21

## Overview

The Chimera web dashboard provides an interactive interface for configuring, running, and analyzing telemetry signal simulations. All controls are organized into logical sections to support the end-to-end pipeline workflow.

## Control Sections

### 1. Simulation Controls Panel

Located at the top of the dashboard, this panel controls simulation execution and parameter configuration.

#### 1.1 Execution Controls

**Run Now Button**
- **Type**: Primary action button
- **States**: 
  - Normal: "Run Now" (blue with highlight when changes pending)
  - Running: "Running…" (disabled)
- **Function**: Executes the simulation pipeline with current parameters
- **Behavior**: 
  - Stops any playing audio before execution
  - Updates to show "Running…" during execution
  - Re-enables after completion

**Status Badge**
- **Type**: Read-only indicator
- **States**:
  - "Up to date" (green, idle) - No pending changes
  - "Changes pending" (yellow) - Configuration changed since last run
  - "Running…" (blue, animated) - Simulation in progress
- **Function**: Shows simulation execution status
- **Behavior**: Updates automatically based on input changes and execution state

#### 1.2 Configuration Controls

**Preset Selector**
- **Type**: Dropdown select
- **Options**: All available frame presets (e.g., RamanWhisper, etc.)
- **Function**: Selects the frame preset configuration
- **Behavior**: 
  - Updates simulation parameters to preset defaults
  - Resets plaintext to preset default
  - Clears external audio attachment
  - Marks changes as pending
- **Associated Info**: Description text below showing preset details

**Plaintext Input**
- **Type**: Multi-line text area
- **Function**: Input text/data to be encoded and transmitted
- **Constraints**: None (character count displayed)
- **Behavior**: Marks changes as pending on edit
- **Associated Info**: Character count display

**Channel SNR Control**
- **Type**: Number input
- **Range**: -30 to 0 dB
- **Step**: 0.5 dB
- **Function**: Sets signal-to-noise ratio for AWGN channel
- **Behavior**: Marks changes as pending on change
- **Associated Info**: 
  - Help text explaining Es/N₀ concept
  - Processing gain information
  - Link to signal processing concepts documentation

**External Audio Payload**
- **Type**: File input
- **Accept**: audio/* (WAV, MP3, etc.)
- **Constraints**: Maximum 512 KB file size
- **Function**: Uploads audio file to embed in framed payload
- **Behavior**: 
  - Encodes file as base64
  - Embeds in plaintext with "AUDIO:{name}:{data}" format
  - Shows attachment status and filename
  - Provides "Remove audio" button when file is attached
- **Associated Info**: File size limit notice

### 2. Audio Playback Controls

Located in the Frame Telemetry panel, these controls enable playback of modulated audio signals.

**Play Clean Button**
- **Type**: Primary action button
- **Icon**: ▶
- **Label**: "Play Clean"
- **Function**: Plays clean (pre-noise) modulated audio signal
- **States**:
  - Active (highlighted) - Currently playing clean audio
  - Disabled - When already playing clean audio
  - Enabled - When stopped or playing noisy audio
- **Behavior**: 
  - Stops any currently playing audio
  - Starts playback of clean signal
  - Updates playback state indicator
  - Automatically stops when playback completes

**Play Noisy Button**
- **Type**: Primary action button
- **Icon**: ▶
- **Label**: "Play Noisy"
- **Function**: Plays noisy (post-AWGN) modulated audio signal
- **States**:
  - Active (highlighted) - Currently playing noisy audio
  - Disabled - When already playing noisy audio
  - Enabled - When stopped or playing clean audio
- **Behavior**: 
  - Stops any currently playing audio
  - Starts playback of noisy signal
  - Updates playback state indicator
  - Automatically stops when playback completes

**Stop Button**
- **Type**: Ghost button (secondary style)
- **Icon**: ⏹
- **Label**: "Stop"
- **Function**: Stops audio playback
- **States**:
  - Disabled - When no audio is playing
  - Enabled - When audio is playing
- **Behavior**: 
  - Immediately stops current audio playback
  - Releases audio resources
  - Updates playback state to stopped

**Volume Control**
- **Type**: Range slider
- **Range**: 0.0 to 1.0 (0% to 100%)
- **Step**: 0.01 (1%)
- **Default**: 0.5 (50%)
- **Function**: Controls playback volume/gain
- **Behavior**: 
  - Real-time volume adjustment during playback
  - Persists across playback sessions
  - Shows percentage value next to slider
- **Associated Info**: Volume percentage display

**Audio Information Display**
- **Type**: Read-only metric display
- **Shows**:
  - Sample rate (Hz)
  - Carrier frequency (Hz)
- **Function**: Displays technical specifications of generated audio
- **Visibility**: Only shown when simulation has been run and audio is available

### 3. Frame Telemetry Panel

Displays key metrics from simulation execution.

**BER Metrics Display**
- **Type**: Read-only metric cards
- **Metrics**:
  - Pre-FEC BER with error count
  - Post-FEC BER with residual error count
  - Recovered message text
- **Function**: Shows bit error rate statistics
- **Associated Info**: Links to BER and FEC documentation

### 4. Pipeline Visualization Section

Interactive node graph showing the signal processing pipeline.

**Pipeline Nodes** (Read-only displays)
- Input node: Payload size and SNR
- Encoder node: Symbol counts and frame layout
- Transmitter node: TX constellation diagram
- Channel node: Carrier frequency and symbol rate
- Receiver node: RX constellation diagram
- Decoder node: Error statistics
- Output node: Recovered plaintext

### 5. Frame Inspector Panel

**Frame Table**
- **Type**: Read-only data table
- **Columns**:
  - Index (frame number / total)
  - Label (frame type tag)
  - Opcode (command opcode)
  - Command Word (decoded command)
  - Payload Preview (first bytes of payload)
- **Function**: Shows decoded frame descriptors
- **Behavior**: Populated after simulation run

### 6. Diagnostics Panel

**Diagnostic Charts** (Read-only visualizations)
- Timing Error line chart
- NCO Frequency Offset line chart
- Clean Signal PSD line chart
- Noisy Signal PSD line chart
- Running BER line chart

**Log Viewers** (Read-only text displays)
- Encoder Log panel
- Decoder Log panel

## Control Flow and State Management

### Input State Changes
Any modification to configuration controls (preset, plaintext, SNR, audio) triggers:
1. Simulation input state update
2. Pending changes indicator activation
3. Run button highlight

### Simulation Execution
When "Run Now" is clicked:
1. Audio playback stops (if active)
2. Status changes to "Running…"
3. Button disables
4. Pipeline executes asynchronously
5. Results populate all output displays
6. Status changes to "Up to date"
7. Button re-enables
8. Audio playback controls become available

### Audio Playback State
The audio playback system maintains three states:
1. **Stopped**: No audio playing, all play buttons enabled
2. **PlayingClean**: Clean audio playing, clean button disabled/highlighted
3. **PlayingNoisy**: Noisy audio playing, noisy button disabled/highlighted

State transitions:
- Any play action → stops current audio → starts requested audio
- Stop action → immediate stop
- Simulation run → stops audio
- Audio completion (onended) → automatic return to stopped state

## Web Audio API Integration

The audio playback system uses the Web Audio API with the following architecture:

**Components:**
- `AudioContext`: Main audio context (created on first playback)
- `AudioBufferSourceNode`: Plays audio buffer (one per playback)
- `GainNode`: Controls volume (0.0 to 1.0 gain factor)
- Audio buffers: Created from f32 sample arrays

**Audio Chain:**
```
AudioBuffer → AudioBufferSourceNode → GainNode → AudioDestinationNode (speakers)
```

**Lifecycle:**
1. Create or reuse AudioContext
2. Create AudioBuffer from sample data
3. Create AudioBufferSourceNode
4. Create and configure GainNode
5. Connect nodes: source → gain → destination
6. Attach "ended" event listener for state cleanup
7. Start playback
8. On stop: disconnect and dispose nodes

## Browser Compatibility

All controls use standard HTML5 and Web Audio API features supported in:
- Chrome/Edge 90+
- Firefox 88+
- Safari 14+

## Accessibility Considerations

Current implementation includes:
- Semantic HTML elements
- Button state indication through visual styling
- Progress indication for long-running operations

Future improvements should include:
- ARIA labels for all interactive elements
- Keyboard navigation support
- Screen reader announcements for state changes
- Focus management during asynchronous operations

## Future Control Enhancements

Potential additions identified in project planning:

1. **Audio Controls**
   - Looping toggle
   - Playback speed control
   - Waveform visualization
   - Save audio file button

2. **Simulation Controls**
   - Preset cloning/customization
   - Parameter validation warnings
   - Configuration save/load (browser storage)
   - Batch simulation runs

3. **Visualization Controls**
   - Chart zoom/pan controls
   - Data export buttons (CSV, JSON)
   - Chart type toggles
   - Color scheme selection

4. **Advanced Controls**
   - Real-time streaming mode toggle
   - Worker thread offloading option
   - Performance profiling toggle

## Maintenance Notes

- All control callbacks are defined in `chimera-web/src/ui.rs`
- Styling is defined in `chimera-web/style.css`
- Web Audio API integration uses features from `web-sys` crate
- State management uses Yew hooks (`use_state`, `use_mut_ref`)

## Related Documentation

- [Signal Processing Concepts](./signal_processing_concepts.md)
- [Technical Overview](./chimera_technical_overview.md)
- [TODO Planning](./todo.md)
- [Playwright Testing Guide](../chimera-web/PLAYWRIGHT_TESTING.md)

## Change Log

### January 2025 - Initial Specification
- Documented all existing controls
- Added audio playback controls (Play Clean, Play Noisy, Stop, Volume)
- Established specification format and structure
