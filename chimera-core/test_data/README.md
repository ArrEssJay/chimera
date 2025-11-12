# Chimera Test Data

This directory contains test fixtures and configurations for comprehensive end-to-end testing of the Chimera pipeline.

## Directory Structure

```
test_data/
├── configs/          # Test TOML configuration files
├── audio/            # Test audio files for mixing (if needed)
└── expected/         # Expected outputs for regression testing
```

## Test Configurations

### `test_simple.toml`
Basic configuration for simple message roundtrip testing.
- Message: "Hello from Chimera!"
- SNR: 20dB (clean channel)
- Use case: Basic functionality verification

### `test_clean_channel.toml`
Perfect channel conditions for verifying encoding/decoding correctness.
- Message: "Clean channel test"
- SNR: 100dB (essentially no noise)
- Use case: Ensure pipeline works without channel effects

### `test_long_message.toml`
Multi-frame message spanning several frames.
- Message: ~170 bytes (spans multiple 16-byte frames)
- SNR: 20dB
- Use case: Frame assembly and multi-frame synchronization

### `test_high_noise.toml`
Challenging noise conditions to test error correction.
- Message: "Testing high noise"
- SNR: 8dB (challenging)
- Link loss: 2dB
- Use case: FEC effectiveness under stress

### `test_audio_mixing.toml`
Tests intermodulation with external audio.
- Message: "Audio mixing test"
- Audio source: Pink noise generator
- Use case: Verify audio mixing and intermodulation

### `test_tone_mixing.toml`
Tests intermodulation with single frequency.
- Message: "1kHz tone mixing"
- Audio source: 1kHz sine wave
- Use case: Verify tone mixing behavior

## Running Tests

### Run all integration tests:
```bash
cargo test --package chimera-core --test integration_tests
```

### Run specific test category:
```bash
# Config loading tests
cargo test --package chimera-core --test integration_tests test_load

# Message roundtrip tests
cargo test --package chimera-core --test integration_tests test_message_roundtrip

# Audio generation tests
cargo test --package chimera-core --test integration_tests test_audio

# Pipeline integration tests
cargo test --package chimera-core --test integration_tests test_qpsk
cargo test --package chimera-core --test integration_tests test_ldpc

# Error handling tests
cargo test --package chimera-core --test integration_tests test_various_snr
```

### Run test harness binary:
```bash
# Run default test suite
cargo run --example pipeline_test

# Run all configs
cargo run --example pipeline_test -- --all

# Run specific config
cargo run --example pipeline_test -- --config test_simple.toml
```

### Run performance tests:
```bash
cargo test --package chimera-core --test integration_tests --ignored -- --nocapture
```

## Test Coverage

The integration tests verify:

1. **Basic Message Roundtrip**
   - Various message lengths (short, medium, long)
   - Exact match verification (when SNR permits)
   - BER metrics validation
   - Multi-frame handling

2. **Configuration Loading**
   - TOML parsing and validation
   - Parameter propagation through pipeline
   - Different config scenarios
   - Runtime parameter application

3. **Audio Output Generation**
   - WAV file generation
   - Valid audio characteristics
   - Correct sample rate (48kHz)
   - Signal levels within range (-1.0 to 1.0)
   - Different mixing configurations

4. **Pipeline Component Integration**
   - QPSK modulation flow
   - FSK modulation flow
   - Combined QPSK+FSK operation
   - LDPC encoding/decoding
   - Frame assembly and parsing

5. **Error Handling**
   - Various SNR levels
   - High noise conditions
   - Gradual degradation verification
   - FEC correction effectiveness

6. **Performance & Diagnostics**
   - Throughput measurement
   - Memory usage tracking
   - Diagnostic data collection
   - Report completeness

## Test Criteria

Each test validates:
- **Correctness**: Decoded message matches input (within BER constraints)
- **Configuration**: All parameters properly applied
- **Metrics**: BER values are reasonable and improve with FEC
- **Audio**: Generated files are valid and contain expected signals
- **Performance**: Pipeline completes in reasonable time

## Known Issues

Currently (as documented in tests), the pipeline has challenges with:
- Message recovery accuracy (high BER)
- Synchronization in some scenarios
- Phase ambiguity resolution

The tests are designed to:
1. Document current behavior
2. Catch regressions when fixes are applied
3. Verify improvements incrementally

## Adding New Tests

To add new test scenarios:

1. Create a new TOML config in `configs/`
2. Add corresponding test cases in `integration_tests.rs`
3. Update this README with the new test description
4. Consider adding to the test harness if it's a common scenario

## Test Philosophy

These tests follow an **integration testing** approach:
- Test the full pipeline end-to-end
- Use realistic configurations
- Verify behavior at system boundaries
- Document current state honestly
- Enable incremental improvement

Rather than expecting perfection immediately, tests are calibrated to:
- Pass when basic functionality works
- Provide detailed diagnostics for debugging
- Serve as regression tests when fixes are applied
