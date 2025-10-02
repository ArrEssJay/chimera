# Chimera Feature Planning Todo

Track upcoming work items for the Raman Whisper visualization stack. Use the checkbox status to manage progress during implementation cycles.

## ✅ Recently Completed
- [x] Configure continuous deployment to GitHub Pages via Trunk build artifact.
- [x] Surface modulation audio playback with clean/noisy previews inside the web dashboard.
- [x] Support vocoder-style mixing with uploaded audio and wet/dry control.

## 🛰️ Preset & Configuration UX
- [ ] Persist the active preset and custom overrides in browser storage to survive reloads.
- [ ] Allow cloning a preset into an editable "Custom" profile with per-field overrides.
- [ ] Surface protocol metadata (sync sequence, opcode) in the dashboard with tooltips and copy-to-clipboard buttons.
- [ ] Add validation warnings when user overrides violate preset frame-layout constraints.

## 📡 Real-Time Diagnostics
- [ ] Transition from batch `run_pipeline` results to incremental streaming (web-worker or async channel) so soft decisions render live.
- [ ] Plot soft metrics over time (line chart) alongside constellation scatter for richer demod visibility.
- [ ] Render per-symbol phase error and frequency offset trends with sparklines.
- [ ] Introduce export of symbol decision logs as JSON/CSV for offline analysis.

## ⚙️ Core Pipeline Enhancements
- [ ] Parameterize LDPC configuration per preset instead of relying on defaults inside the library call chain.
- [ ] Validate presets against the modulation protocol spec nightly (CI check comparing docs and code).
- [ ] Add support for burst-mode simulations with jitter and Doppler modeling hooks.
- [ ] Extend encoder to accept external target ID lists and rotate them per frame.

## 🧪 Testing & Tooling
- [ ] Add wasm-bindgen-driven UI integration tests that exercise preset switching in headless browsers.
- [ ] Record golden symbol-decision fixtures for regression testing of decoder metrics.
- [ ] Benchmark `run_simulation` across presets and publish baseline timing in docs.
- [ ] Create lint to flag stale presets when protocol constants change.

## 📚 Documentation & Onboarding
- [ ] Update `docs/modulation_protocol_v4.2.md` with preset rationale and usage guidance.
- [ ] Produce a quickstart guide for the web dashboard covering presets and diagnostics.
- [ ] Embed animated GIFs of the dashboard in `chimera-web/README.md` showcasing symbol decision views.

Revise this list as goals are completed or reprioritized.
