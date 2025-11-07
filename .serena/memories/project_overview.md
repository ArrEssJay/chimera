# Chimera Project Overview

> **🎯 USER-FIRST IMPERATIVE**
> 
> **The purpose of something is WHAT IT DOES, not HOW it is built.**
> 
> Everything in this project exists to help USERS:
> - Learn signal processing visually
> - Prototype communication systems quickly
> - Experiment without expensive hardware
> - Share knowledge and configurations
> 
> When making decisions, always ask: **"Does this make it easier for users to understand signals or build systems?"**
> 
> If the answer is no, we don't need it.

---

## Purpose
Chimera is a **browser-based tool** that lets users **build, test, and visualize** communication systems without installation or hardware.

### What Users Can Do
1. **Learn Signal Processing**
   - See data transform from bits → radio waves → bits
   - Visualize concepts (constellation diagrams, FFT, channel effects)
   - Experiment with parameters and see immediate results

2. **Prototype Communication Systems**
   - Test modulation schemes (QPSK, QAM)
   - Experiment with error correction (LDPC)
   - Simulate realistic channel conditions
   - Measure performance (BER, SNR)

3. **Build Visually (Epic #40 - In Progress)**
   - Drag-and-drop signal processing blocks
   - Connect them to create pipelines
   - Run and see results instantly
   - Save and share configurations

## Vision
Create a **visual, drag-and-drop node graph environment** for DSP pipeline construction - like GNU Radio Companion, but in your browser with zero installation.

### User Benefits
- ✅ **No Installation**: Works in any modern browser
- ✅ **Learn by Doing**: Interactive, visual feedback
- ✅ **No Hardware Needed**: Simulate everything
- ✅ **Share Easily**: Send links to configurations
- ✅ **Open Source**: Free forever, learn from the code

---

## Tech Stack (For Developers)

### Backend/Core
- **Language**: Rust (nightly)
- **Architecture**: Workspace with 3 crates
  - `chimera-core`: Core DSP logic, LDPC, modulation/demodulation
  - `chimera-cli`: CLI tools
  - `chimera-web`: WASM frontend
- **Key Libraries**: ndarray, num-complex, ldpc, rayon, serde

### Frontend
- **Currently**: Yew (Rust WASM framework) - **Legacy, operational**
- **Migration Target**: React + TypeScript (Epic #40) - **For better UX**
- **Node Graph**: React Flow library
- **Build Tool**: Trunk (WASM), Vite (React)
- **Testing**: Playwright for E2E, Jest for unit tests

### Infrastructure
- **Deployment**: GitHub Pages (custom domain: impermanent.io)
- **CI/CD**: GitHub Actions with auto-merge
- **AI Development**: GitOps with AI agents for velocity

---

## Key Features (What Users Experience)

### Current (Legacy Yew UI)
1. **End-to-end Pipeline Visibility**
   - See every stage of TX/RX
   - Adjust parameters in real-time
   - Interactive audio diagnostics

2. **LDPC Forward Error Correction**
   - Watch how FEC saves data from noise
   - Soft-decision decoding
   - Performance metrics (BER before/after)

3. **QPSK Modulation**
   - Constellation diagrams
   - FFT visualization
   - Real-time SNR adjustment

4. **Preset Configurations**
   - Start with working examples
   - Learn by modifying presets
   - Understand different protocols

### Coming Soon (Epic #40 - Node Graph)
1. **Visual Pipeline Builder**
   - Drag nodes from palette
   - Connect them with type-safe edges
   - Instant validation

2. **15+ Processing Blocks**
   - Sources: Bit Generator, File Input, Audio Input
   - Processors: Modulators, FEC, Filters
   - Analyzers: FFT, Constellation, BER
   - Sinks: Audio Output, File Export, Chart Display

3. **Save/Load/Share**
   - Export graph to JSON
   - Import community configurations
   - Share via URL or file

---

## Current State (January 2025)

### Operational
- ✅ Legacy Yew UI live at **impermanent.io**
- ✅ LDPC encoding/decoding functional
- ✅ QPSK modulation with constellation display
- ✅ Interactive audio pipeline

### In Progress (Epic #40)
- 🚧 Visual node graph environment
- 🚧 React + TypeScript migration
- 🚧 15+ DSP processing nodes
- 🚧 Type-safe edge connections
- 🚧 Save/load graph functionality

### Development Model
- **Parallel Development**: Multiple AI agents work simultaneously
- **Contract-First**: Locked interfaces prevent conflicts
- **Wave-Based**: 5 waves from contracts → full graph editor
- **Auto-Merge**: CI validates, PRs merge automatically
- **Issue Tracking**: #45-66 track parallel work

---

## Success Metrics (User-Focused)

### Usage Goals
- **Ease of Use**: Can a beginner build a working link in <5 minutes?
- **Learning Effectiveness**: Do users understand concepts better after using Chimera?
- **Sharing**: Are users sharing their configurations?
- **Performance**: Does it run smoothly in-browser?

### Technical Enablers (Support User Goals)
- **Build Time**: Fast iteration for developers = faster user features
- **Test Coverage**: ≥80% ensures users don't hit bugs
- **CI Speed**: Auto-merge = users get features faster
- **Code Quality**: Maintainable code = sustainable project = long-term user support

---

## Links
- 🌐 **Live Demo**: https://impermanent.io
- 📖 **User Docs**: `README.md`, `docs/signal_processing_concepts.md`
- 🔧 **Developer Docs**: `.github/copilot-instructions.md`, `docs/architecture-node-graph.md`
- 🐛 **Issues**: GitHub Issues (#40 is the big one)
- 📰 **Epic #40**: Visual Node Graph Environment (5 waves, 22 issues)

---

## Remember
> **We are building this FOR USERS to learn and create.**
> 
> Every feature, every test, every commit should make their experience better.
> 
> Technical excellence is the MEANS, not the END.
> 
> The END is: **Users understanding signal processing and building communication systems with joy.**
