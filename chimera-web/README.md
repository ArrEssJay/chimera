# Chimera Web Dashboard

WebAssembly-based visualization for the Chimera modulation/demodulation pipeline. Built with
[Yew](https://yew.rs/) and integrates the shared `chimera-core` crate.

## Features

- Configure plaintext, sample rate, and SNR for frame generation.
- Run the full encode → modulate → AWGN → demodulate pipeline in the browser.
- Inspect constellation plots alongside encoder/decoder logs and key BER statistics.

## Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target.
- [`trunk`](https://trunkrs.dev/) for local development.

Install requirements:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

> **Tip for macOS/Homebrew users**: if your default toolchain comes from Homebrew,
> install the Rust toolchain via `rustup` and run `cargo` from that toolchain when
> building for `wasm32-unknown-unknown`, otherwise the standard library for that
> target will be missing.

## Running locally

```bash
cd chimera-web
trunk serve --open
```

The dashboard will be served at `http://127.0.0.1:8080/`.

## Building for release

```bash
cd chimera-web
trunk build --release
```

The optimized WASM bundle is emitted in `dist/` ready for static hosting.

## Testing

```bash
cargo test --package chimera-core
cargo test --package chimera-web
```

Yew-specific wasm tests can be executed with `wasm-pack test --chrome --headless` if desired.
