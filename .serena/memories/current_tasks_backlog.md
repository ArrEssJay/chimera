# Current Tasks & Backlog

**Last Updated:** 2025-10-04

## 🎯 Immediate Priority

### Epic #40: Visual Node Graph DSP Environment
Primary focus for all current development efforts.

**Active Issues:**
- **#45** [OPEN] Setup React + TypeScript Infrastructure - `critical`
- **#47** [OPEN] Node Graph Core Engine (WASM) - `high`
- **#48** [OPEN] Built-in Processing Nodes - `critical`

---

## 🛰️ Preset & Configuration UX
- [ ] Persist active preset and custom overrides in browser storage
- [ ] Allow cloning preset into editable "Custom" profile
- [ ] Surface protocol metadata (sync sequence, opcode) in dashboard with tooltips
- [ ] Add validation warnings when user overrides violate preset constraints

## 📡 Real-Time Diagnostics
- [ ] Transition from batch `run_pipeline` to incremental streaming (web-worker/async)
- [ ] Plot soft metrics over time (line chart) alongside constellation scatter
- [ ] Render per-symbol phase error and frequency offset trends with sparklines
- [ ] Export symbol decision logs as JSON/CSV for offline analysis

## ⚙️ Core Pipeline Enhancements
- [ ] Parameterize LDPC configuration per preset
- [ ] Validate presets against modulation protocol spec nightly (CI check)
- [ ] Add support for burst-mode simulations with jitter and Doppler modeling
- [ ] Extend encoder to accept external target ID lists and rotate per frame

## 🧪 Testing & Tooling
- [ ] Add wasm-bindgen UI integration tests (preset switching in headless browsers)
- [ ] Record golden symbol-decision fixtures for regression testing
- [ ] Benchmark `run_simulation` across presets and publish baseline timing
- [ ] Create lint to flag stale presets when protocol constants change

## 📚 Documentation & Onboarding
- [ ] Update `docs/modulation_protocol_v4.2.md` with preset rationale
- [ ] Produce quickstart guide for web dashboard
- [ ] Embed animated GIFs of dashboard in `chimera-web/README.md`

---

## ✅ Recently Completed
- [x] Configure continuous deployment to GitHub Pages via Trunk
- [x] Surface modulation audio playback with clean/noisy previews
- [x] Support vocoder-style mixing with uploaded audio and wet/dry control
- [x] Build UI Component Library (Issue #46)
- [x] Node Graph Core Engine - Phase 1 (Issue #64)
- [x] Built-in Processing Nodes - Phase 2 (Issue #65)
- [x] Graph Editor UI with React Flow - Phase 3 (Issue #66)

---

## 🚀 Launch Status

**Infrastructure:** ✅ READY
- GitHub Actions workflows configured
- Contracts locked and protected
- Auto-merge enabled
- GitHub Copilot agents ready

**Next Action:** Assign issues to GitHub Copilot agents for parallel work

---

**Note:** Revise this list as goals are completed or reprioritized.