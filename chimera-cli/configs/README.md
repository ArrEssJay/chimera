# Chimera Configuration Files

This directory contains example configuration files for the Chimera protocol demonstration.

## Configuration Files

### `reference.toml`
**Comprehensive reference configuration** showing all available parameters with defaults and extensive documentation. Use this as a starting point to understand all configuration options.

Key sections:
- Protocol parameters (command codes, target IDs)
- Simulation settings (SNR, link loss, test messages)
- Audio mixing configuration
- Realtime adjustable parameters (channel, THz modulation, signal processing)
- Terminal and logging configuration
- LDPC error correction parameters

### `simple.toml`
**Minimal configuration** for basic operation. Only specifies the essential parameters needed to run a simulation. Good starting point for new users.

### `audio_mixing_example.toml`
**Audio intermodulation demonstration**. Shows how to configure external audio mixing with the AID signal, including:
- Test tone generation (440Hz A4 note)
- Intermodulation coefficients
- Gain settings for audio/signal mixing
- WAV file output

### `high_fidelity.toml`
**Clean channel test configuration**. Uses high SNR (30 dB) and realtime parameter control for validation testing:
- Excellent signal quality
- Full realtime parameter exposure
- Debug-level logging
- Active THz modulation mode

### `noisy_channel.toml`
**Challenging conditions test**. Demonstrates error correction capability under adverse conditions:
- Low SNR (5 dB)
- Additional link loss (3 dB)
- Multipath propagation
- Doppler shift
- Increased gains to compensate

### `validation_bypass.toml`
**Direct protocol testing**. Bypasses THz carrier simulation for validating the core protocol without intermodulation effects:
- THz simulation disabled
- Clean signal path
- Debug logging for protocol analysis

## Using Configuration Files

### CLI Usage

```bash
# Use a specific config file
chimera-cli --config configs/simple.toml

# Use with output file
chimera-cli --config configs/audio_mixing_example.toml
```

### Configuration Structure

All configuration files follow the same structure:

```toml
# Protocol parameters (user-configurable)
[protocol_params]
command_opcode = 0x0001
target_id_hex = "DEADBEEF"

# Simulation settings
[simulation]
plaintext_source = "Your message here"
snr_db = 20.0

# Realtime adjustable parameters
[realtime.channel]
snr_db = 20.0
link_loss_db = 0.0

[realtime.thz_modulation]
enabled = true
modulation_depth = 0.5

[realtime.signal_processing]
enable_qpsk = true
enable_fsk = true

# Terminal output
[terminal]
telemetry_interval_secs = 1.0
```

## Internal Protocol Parameters

The following parameters are **hardcoded** in the Whisper protocol and cannot be changed:

- **Protocol**: `"whisper"` (fixed)
- **Carrier frequency**: 12 kHz
- **Symbol rate**: 16 symbols/second
- **FSK frequencies**: 11.999 kHz / 12.001 kHz
- **Frame layout**: 128 symbols (16 sync + 16 target + 16 command + 64 data + 16 ECC)
- **Sync sequence**: `0xA5A5A5A5`

These are part of the protocol specification and ensure compatibility between transmitters and receivers.

## Parameter Guidelines

### SNR (Signal-to-Noise Ratio)
- **30+ dB**: Excellent, validation testing
- **20 dB**: Good, typical usage
- **10 dB**: Moderate, error correction active
- **5 dB**: Challenging, heavy FEC required
- **< 0 dB**: Extreme, may fail to decode

### Modulation Depth
- **0.01-0.05**: Idle mode (subtle modulation)
- **0.50-0.60**: Moderate activity
- **0.70-0.80**: Active mode (strong modulation)

### Gain Settings
- **1.0**: Unity gain (default)
- **0.5-0.9**: Attenuate signal
- **1.1-1.5**: Boost signal (use cautiously)

## WASM/Web Interface

The WASM interface exposes the same configuration structure through JavaScript:

```javascript
const dsp = new WASMStreamingDSP();

// Configure via JSON
const config = {
  protocol_params: {
    command_opcode: 0x0001,
    target_id_hex: "DEADBEEF"
  },
  simulation: {
    plaintext_source: "Hello from web!",
    snr_db: 20.0
  }
};

dsp.configure(JSON.stringify(config));

// Realtime controls
dsp.update_channel(15.0, 2.0);  // Adjust SNR and link loss
dsp.set_modulation_depth(0.75);  // Adjust THz modulation
dsp.set_qpsk_enabled(true);      // Toggle modulation
```

## See Also

- `/docs/chimera_technical_overview.md` - System architecture
- `/docs/modulation_protocol_v4.2.md` - Protocol specification
- `/chimera-core/src/config.rs` - Configuration type definitions
