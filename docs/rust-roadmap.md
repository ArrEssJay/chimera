# Chimera Rust Port Roadmap

This document captures the next implementation milestones for the `chimera-core` crate and its CLI companion.

## Test Vector Strategy

1. **Generate golden data from Python reference**
   - `test_vectors/source_payload.bin`
   - `test_vectors/qpsk_bitstream.bin`
   - `test_vectors/modulated_signal.f64`
   - `test_vectors/received_symbols.c64`
   - `test_vectors/decoded_payload.bin`
2. **Unit tests** within each module (`config`, `utils`, `ldpc`, `encoder`, `decoder`) to validate the basic helpers.
3. **Integration tests** in `chimera-core/tests/` that replay the golden vectors and assert the Rust output matches (within tolerance for floating point operations).

## Module TODOs

- **`ldpc.rs`** – integrate a production-ready LDPC crate (`sparse-ldpc` or alternative), expose matrix builders, and wrap the decoder API.
- **`encoder.rs`** – port modulation pipeline, including frame construction and noise injection.
- **`decoder.rs`** – implement Gardner timing recovery, Costas loop, matched filtering, and LDPC decode orchestration.
- **`diagnostics.rs`** – enrich structures with display helpers (tables/CSV) to support CLI reporting.
- **`cli` crate** – load optional configuration files, stream progress, and export diagnostics in user-selected formats (stdout, JSON, or files).

## Tooling Notes

- Cargo is not currently available in this environment (`which cargo` → `cargo not found`).
  - All workspace files were created manually; install Rust/Cargo locally before building.
- Once Cargo is installed, run `cargo fmt`, `cargo clippy`, and `cargo test` inside the workspace root to validate.
