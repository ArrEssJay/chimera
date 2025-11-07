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

### React Components Testing

The React-based UI components use multiple testing approaches:

**Unit Tests (Vitest):**
```bash
cd chimera-web
npm test                  # Run tests
npm run test:ui           # Run tests with UI
npm run test:coverage     # Run tests with coverage report
```

**End-to-End Tests (Playwright):**
```bash
cd chimera-web
npm run e2e              # Run E2E tests
npm run e2e:ui           # Run E2E tests with UI
```

**Visual Regression Tests (Chromatic):**
```bash
cd chimera-web
npm run chromatic        # Run visual regression tests locally
```

Visual regression testing is automated via Chromatic and runs on every PR to detect UI changes:
- **Baseline Captures:** Chromatic captures screenshots of all Storybook stories
- **Automated Comparison:** Visual changes are detected automatically on each PR
- **Review Workflow:** Team members review and approve/reject visual changes in the Chromatic dashboard
- **CI Integration:** The `.github/workflows/chromatic.yml` workflow runs on push and PR events

### Storybook

Interactive component documentation and development:

```bash
cd chimera-web
npm run storybook        # Start Storybook dev server
npm run build-storybook  # Build static Storybook
```
