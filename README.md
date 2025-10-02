# Chimera

[![CI](https://github.com/arressjay/chimera/actions/workflows/ci.yml/badge.svg)](https://github.com/arressjay/chimera/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/chimera-core.svg)](https://crates.io/crates/chimera-core)

Chimera is a Rust-native, high-performance implementation of the Raman Whisper modulation and decoding pipeline. This project provides a type-safe and thoroughly tested library for signal processing tasks related to this specialized optical communication protocol.

## Overview

The purpose of this library is to provide a robust and validated software implementation of the Raman Whisper communication protocol. This protocol is designed for systems where signals are transmitted at very low power levels, often below the noise floor, making them difficult to detect or intercept.

Chimera provides a complete end-to-end simulation and processing pipeline, enabling engineers and researchers to:

-   **Encode** data into a modulated signal using the Raman Whisper scheme.
-   **Incorporate** powerful Low-Density Parity-Check (LDPC) codes for forward error correction.
-   **Simulate** signal transmission over a noisy channel and analyze performance metrics like Bit Error Rate (BER).
-   **Decode** the received signal to recover the original data.

Written entirely in Rust, the library is engineered for performance-critical applications such as real-time digital signal processing (DSP) and embedded systems, where efficiency, memory safety, and concurrency are paramount.

## Getting Started

To use `chimera-core` in your project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
chimera-core = { git = "https://github.com/arressjay/chimera.git" }
```

### Example Usage

Here is a basic example of how to run an end-to-end simulation:

```rust
use chimera_core::{
    config::{LDPCConfig, ProtocolConfig, SimulationConfig},
    run_simulation,
};

fn main() {
    // 1. Define simulation, protocol, and LDPC configurations
    let sim_config = SimulationConfig::default();
    let protocol_config = ProtocolConfig::default();
    let ldpc_config = LDPCConfig::default();

    // 2. Run the simulation
    let output = run_simulation(&sim_config, &protocol_config, &ldpc_config);

    // 3. Print the simulation report
    println!("Simulation Complete!");
    println!("Frame Error Rate (FER): {}", output.report.frame_error_rate);
    println!("Bit Error Rate (BER): {}", output.report.bit_error_rate);

    // The `output` struct also contains detailed diagnostics.
}
```

## Workspace Structure

This repository is a Cargo workspace containing several crates:

-   `chimera-core`: The core library implementing the modulation/demodulation pipeline, LDPC codes, and simulation logic.
-   `chimera-cli`: (Planned) A command-line interface for running simulations and processing signals.
-   `chimera-wasm`: (Planned) A WebAssembly binding for using the core library in web applications.

## Core Concepts

The `chimera-core` library is organized into several key modules:

-   `config`: Defines configuration structures for simulations, protocol parameters, and LDPC codes.
-   `encoder`: Handles the generation of modulated signals from input data.
-   `decoder`: Implements the demodulation and decoding logic to recover data.
-   `ldpc`: Provides Low-Density Parity-Check (LDPC) code functionalities.
-   `diagnostics`: Contains tools and data structures for diagnostics, logging, and reporting.
-   `utils`: A collection of utility functions used across the library.

## Building and Testing

### Build from Source

Clone the repository and use Cargo to build the project:

```sh
git clone https://github.com/arressjay/chimera.git
cd chimera
cargo build --release
```

### Run Tests

To run the full test suite:

```sh
cargo test --all-features
```

## About the Raman Whisper Protocol

<details>
<summary>Click to expand for a detailed background on the protocol.</summary>

### 1. Is the modulation scheme "advanced"?

The term "advanced" requires qualification.
-   **From a conventional perspective** (data throughput, spectral efficiency), the protocol is rudimentary. The individual components (QPSK, FSK, AM) are foundational techniques in digital communications.
-   **From its design objective**, the protocol is highly advanced and unconventional. It is a system optimized not for efficient communication, but for psychoacoustic and neurological impact.

### 2. Why is it considered advanced in its application?

The protocol is advanced in its application for three primary reasons:

-   **Hybrid "Nested" Scheme**: The combination of a QPSK layer and an FSK layer on the same auditory-range carrier is not standard practice. It creates two simultaneous, independent, and extremely low-rate data streams. Its purpose is not efficiency; it is to create a specific, complex auditory texture—a rapid, rhythmic "shimmer" (QPSK) overlaid on a slow, subtle "waver" (FSK).
-   **Psychoacoustic Optimization Goal**: Standard modulation schemes trade-off data rate, bandwidth, and signal-to-noise ratio. This protocol's primary goal is to "bypass cognitive filters" and achieve "subconscious informational osmosis." The engineering choices (e.g., a 1 bps data rate) are incomprehensible from a communications standpoint but are logical if the goal is to create a persistent, subliminal signal that the human brain processes as a "presence" rather than as "data."
-   **Exotic Physical Layer**: The modulation is carried on an Amplitude-Modulated (AM) 1.875 THz signal, which is then demodulated via non-linear mixing with a 1.998 THz pump beam inside the target's neural tissue. This delivery mechanism is, by any current public standard, extraordinarily advanced.

### 3. Benefits and Tradeoffs vs. Alternatives

Compared to a conventional low-rate digital modulation scheme (e.g., simple BPSK or FSK on a standard radio carrier):

| Feature               | Raman Whisper Protocol                                                                                                                            | Conventional Alternative                                         |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| **BENEFITS**          |                                                                                                                                                   |                                                                  |
| Covertness (LPI/LPD)  | **Extremely High.** The THz carrier and infinitesimally small modulation bandwidth (20 Hz) make the signal exceptionally difficult to detect.       | Low to Moderate. A standard radio signal is easily detectable.   |
| Cognitive Bypass      | **Primary Feature.** The protocol is engineered to be processed subconsciously. The low data rate and complex texture are designed to be "felt."   | None. A standard signal is perceived as a simple, artificial tone. |
| Psychoacoustic Effect | **High and Specific.** The nested QPSK/FSK layers create a unique, identifiable, and persistent sensory texture.                                    | None. A simple carrier would be a monotonous tone.               |
| **TRADEOFFS**         |                                                                                                                                                   |                                                                  |
| Data Rate (Speed)     | **Abysmally Low** (33 bps total). This is the single greatest tradeoff, making it useless for transmitting any meaningful volume of data.           | Extremely High by comparison.                                    |
| Efficiency            | **Effectively Zero.** The data transmitted for the complexity required is infinitesimal.                                                          | Very High. FSK is an efficient way to transmit low-rate data.    |
| Complexity            | **Very High.** The nested scheme and the THz physical layer represent a system of immense technical and biological complexity.                      | Very Low. A simple radio transmitter/receiver is trivial to build. |

**Conclusion**: From a signal processing perspective, this protocol represents a deliberate sacrifice of speed and efficiency in exchange for stealth and psychoacoustic impact. It is not an advanced method of *communication*; it is an advanced method of *neurological and psychological manipulation*.

</details>

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.