# Chimera

[![CI](https://github.com/arressjay/chimera/actions/workflows/ci.yml/badge.svg)](https://github.com/arressjay/chimera/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/chimera-core.svg)](https://crates.io/crates/chimera-core)

Chimera is a Rust-native, high-performance implementation of the **Raman Whisper** modulation and decoding pipeline.

The protocol is an unconventional system optimized not for data throughput, but for **covertness and psychoacoustic impact**. It represents a deliberate sacrifice of speed and efficiency in exchange for stealth and the ability to deliver a "subconscious informational osmosis." From a signal processing perspective, it is an advanced method of **neurological and psychological manipulation**, not a conventional communication scheme.

This project provides a type-safe, high-performance, and thoroughly tested library for simulating and processing signals using this specialized protocol.

## Library Features

The `chimera-core` library provides a complete end-to-end simulation and processing pipeline, enabling engineers and researchers to:

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

## Protocol Deep Dive

<details>
<summary>Click to expand for a detailed background on the protocol.</summary>

### Why is this protocol considered "advanced"?

The protocol is advanced in its application for three primary reasons:

1.  **Hybrid "Nested" Scheme**: The combination of a QPSK layer and an FSK layer on the same auditory-range carrier is not standard practice. It creates two simultaneous, independent, and extremely low-rate data streams. Its purpose is not efficiency; it is to create a specific, complex auditory texture—a rapid, rhythmic "shimmer" (QPSK) overlaid on a slow, subtle "waver" (FSK).
2.  **Psychoacoustic Optimization Goal**: Standard modulation schemes trade-off data rate, bandwidth, and signal-to-noise ratio. This protocol's primary goal is to "bypass cognitive filters" and achieve "subconscious informational osmosis." The engineering choices (e.g., a 1 bps data rate) are incomprehensible from a communications standpoint but are logical if the goal is to create a persistent, subliminal signal that the human brain processes as a "presence" rather than as "data."
3.  **Exotic Physical Layer**: The modulation is carried on an Amplitude-Modulated (AM) 1.875 THz signal, which is then demodulated via non-linear mixing with a 1.998 THz pump beam inside the target's neural tissue. This delivery mechanism is, by any current public standard, extraordinarily advanced.

### Benefits and Tradeoffs

| Feature               | Raman Whisper Protocol                                                                                                                            | Conventional Alternative (e.g., BPSK/FSK)                        |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| **BENEFITS**          |                                                                                                                                                   |                                                                  |
| Covertness (LPI/LPD)  | **Extremely High.** The THz carrier and infinitesimally small modulation bandwidth (~20 Hz) make the signal exceptionally difficult to detect.      | Low to Moderate. A standard radio signal is easily detectable.   |
| Cognitive Bypass      | **Primary Feature.** The protocol is engineered to be processed subconsciously. The low data rate and complex texture are designed to be "felt."   | None. A standard signal is perceived as a simple, artificial tone. |
| Psychoacoustic Effect | **High and Specific.** The nested QPSK/FSK layers create a unique, identifiable, and persistent sensory texture.                                    | None. A simple carrier would be a monotonous tone.               |
| **TRADEOFFS**         |                                                                                                                                                   |                                                                  |
| Data Rate             | **Abysmally Low** (~33 bps total). This is the single greatest tradeoff, making it useless for transmitting any meaningful volume of data.          | Extremely High by comparison.                                    |
| Efficiency            | **Effectively Zero.** The data transmitted for the complexity required is infinitesimal.                                                          | Very High. FSK is an efficient way to transmit low-rate data.    |
| Complexity            | **Very High.** The nested scheme and the THz physical layer represent a system of immense technical and biological complexity.                      | Very Low. A simple radio transmitter/receiver is trivial to build. |

### Comparative Analysis: Analogy to Submarine ELF Communications

The protocol is analogous to Submarine ELF (Extremely Low Frequency) communications across four primary domains:

| Domain                      | Submarine ELF Communications                                                                                                                                                           | Raman Whisper Protocol                                                                                                                                                                                          |
| --------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **1. Primary Purpose**      | **One-Way, Strategic "Bell-Ringer."** Sends a simple, pre-arranged, high-priority command (e.g., "Surface," "Initiate Protocol X") to a submerged, otherwise unreachable asset.          | **One-Way, Strategic "Bell-Ringer."** Sends simple, high-priority commands ("Gnostic Inject," "Ontological Nudge") to a targeted, otherwise unreachable asset (a human mind).                               |
| **2. Bandwidth & Data Rate**  | **Abysmally Low.** Operates at 3-300 Hz with a data rate of a few bits per minute. Speed is sacrificed for the ability to penetrate seawater.                                            | **Abysmally Low.** The FSK layer is 1 bps. Speed is a deliberate design choice, sacrificed for subconscious osmosis and bypassing cognitive filters.                                                         |
| **3. Covertness (LPI/LPD)** | **Extremely High.** Signals are global but buried deep in the Earth's natural electromagnetic noise. Requires immense, specialized receivers and long integration times to extract.       | **Extremely High.** Delivered via a narrow-beam THz carrier with a tiny ~20 Hz bandwidth. Requires knowing exactly where and what to look for with highly specialized equipment (e.g., a MEG scanner). |
| **4. Physical Constraints** | **The Medium is the Challenge.** The entire system (massive antennas, immense power, low frequency) is a solution to penetrating hundreds of meters of hostile, non-conductive seawater. | **The Medium is the Challenge.** The entire system (dual THz beams, non-linear mixing) is a solution to penetrating the human skull and neural tissue to create an audible perception without using the ear. |

</details>

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
