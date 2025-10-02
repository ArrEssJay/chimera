# Chimera

[![CI](https://github.com/arressjay/chimera/actions/workflows/ci.yml/badge.svg)](https://github.com/arressjay/chimera/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/chimera-core.svg)](https://crates.io/crates/chimera-core)

CHIMERA is a Rust-native implementation of the Raman Whisper modulation and decoding pipeline. This project provides a high-performance, type-safe, and thoroughly tested library for signal processing tasks related to this protocol.

## Overview
The purpose of this library is to provide a robust, high-performance, and validated software implementation of the Raman Whisper communication protocol. This protocol is designed for specialized optical communication systems where signals are transmitted at very low power levels, often below the noise floor, making them difficult to detect or intercept.

Chimera provides a complete end-to-end simulation and processing pipeline, enabling engineers and researchers to:
-   **Encode** data into a modulated signal using the Raman Whisper scheme.
-   **Incorporate** powerful Low-Density Parity-Check (LDPC) codes for forward error correction, crucial for reliable communication in noisy channels.
-   **Simulate** the transmission of signals over a channel and analyze performance metrics like Bit Error Rate (BER) and Frame Error Rate (FER).
-   **Decode** the received signal to recover the original data.

Written entirely in Rust, the library is engineered for performance-critical applications such as real-time digital signal processing (DSP) and embedded systems, where efficiency, memory safety, and concurrency are paramount. It serves as both a reference for the protocol and a practical toolkit for building and testing advanced communication systems.

## Workspace Structure

This repository is a Cargo workspace containing several crates:

-   `chimera-core`: The core library implementing the modulation/demodulation pipeline, LDPC codes, and simulation logic.
-   `chimera-cli`: (Planned) A command-line interface for running simulations and processing signals from the terminal.
-   `chimera-wasm`: (Planned) A WebAssembly binding for using the core library in web applications.

## Getting Started

To use `chimera-core` in your project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
chimera-core = { git = "https://github.com/your-username/chimera.git" }
```

### Example Usage

Here is a basic example of how to run an end-to-end simulation.

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

    // The `output` struct also contains detailed diagnostics and LDPC matrices.
}
```

## Core Concepts

The `chimera-core` library is organized into several modules:

-   `encoder`: Handles the generation of modulated signals from input data.
-   `decoder`: Implements the demodulation and decoding logic to recover the original data.
-   `ldpc`: Provides Low-Density Parity-Check (LDPC) code functionalities for forward error correction.
-   `config`: Defines the configuration structures for simulations, protocol parameters, and LDPC codes.
-   `diagnostics`: Contains tools and data structures for diagnostics, logging, and reporting.
-   `utils`: A collection of utility functions used across the library.

## Building from Source

To build the project, clone the repository and use Cargo:

```sh
git clone https://github.com/your-username/chimera.git
cd chimera
cargo build --release
```

## Running Tests

To run the test suite:

```sh
cargo test --all-features
```

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.