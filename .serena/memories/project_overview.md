# Chimera Project Overview

## Purpose
Chimera is a teaching and prototyping environment for modern telemetry links, combining:
- GPU-friendly LDPC encoder/decoder
- Configurable QPSK framing engine
- Interactive web dashboard exposing every stage of TX/RX pipeline
- **Epic #40**: Visual Node Graph DSP Environment (GNU Radio Companion-like for web)

## Vision
Building a visual, drag-and-drop node graph environment for DSP pipeline construction, allowing users to:
- Create signal processing pipelines visually
- Connect nodes (sources, processors, analyzers, sinks)
- Execute graphs and see real-time results
- Export/import graph configurations

## Tech Stack

### Backend/Core
- **Language**: Rust (nightly)
- **Architecture**: Workspace with 3 crates
  - `chimera-core`: Core DSP logic, LDPC, modulation/demodulation
  - `chimera-cli`: CLI tools
  - `chimera-web`: WASM frontend
- **Key Libraries**: ndarray, num-complex, ldpc, rayon, serde

### Frontend
- **Currently**: Yew (Rust WASM framework)
- **Migration Target**: React + TypeScript (Epic #40)
- **Build Tool**: Trunk (WASM), Vite (React)
- **Testing**: Playwright for E2E

### Infrastructure
- **Deployment**: GitHub Pages (custom domain: impermanent.io)
- **CI/CD**: GitHub Actions
- **AI Development**: GitOps with AI agents

## Key Features
1. End-to-end observability of DSP pipeline
2. Interactive audio diagnostics with Raman feed blending
3. Preset-driven synthesis for different protocols
4. Real-time constellation diagrams and FFT
5. LDPC forward error correction with soft-decision decoding

## Current State (October 2025)
- Legacy Yew UI operational
- Epic #40 in progress: Node graph environment
- Issues #45-66 tracking parallel development phases
- GitOps strategy with AI agents for accelerated development
