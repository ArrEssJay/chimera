# Chimera - Digital Signal Processing Workbench

**Chimera** is a teaching and prototyping environment for modern telemetry links. It combines GPU-friendly LDPC encoder/decoder, configurable QPSK framing, and an interactive web dashboard that exposes every stage of transmit (TX) and receive (RX) pipelines.

[![CI](https://github.com/ArrEssJay/chimera/workflows/CI/badge.svg)](https://github.com/ArrEssJay/chimera/actions)

## 🎯 Features

- **End-to-End Observability**: Every stage from plaintext to spectrum is instrumented and visualizable
- **Interactive Audio Diagnostics**: Blend clean carrier, channel noise, and custom audio feeds
- **Preset Portability**: Encapsulated modulation, LDPC, and simulation configurations
- **Education-First Design**: Comprehensive tooltips, help text, and contextual explanations
- **Modern Web UI**: React + TypeScript frontend with real-time DSP visualization
- **Rust/WASM Core**: High-performance signal processing compiled to WebAssembly

## 🚀 Quick Start

### Prerequisites

- **Rust**: 1.70+ (with `wasm32-unknown-unknown` target)
- **Node.js**: 18+ and npm
- **Trunk**: WebAssembly bundler (`cargo install trunk`)

### Running the Web Application

```bash
# Clone the repository
git clone https://github.com/ArrEssJay/chimera.git
cd chimera

# Install dependencies
cd chimera-web
npm install --legacy-peer-deps

# Run the development server
npm run dev

# Open browser to http://localhost:5173
```

### Running the CLI

```bash
# From repository root
cd chimera-cli
cargo run --release
```

## 📚 Documentation

- **[Technical Overview](docs/chimera_technical_overview.md)** - System architecture and design
- **[User Guide](docs/USER_GUIDE.md)** - How to use the web application
- **[Signal Processing Concepts](docs/signal_processing_concepts.md)** - DSP theory and implementation
- **[UI Controls Specification](docs/ui_controls_specification.md)** - Dashboard controls reference
- **[Developer Guide](CONTRIBUTING.md)** - Contributing and development setup
- **[Troubleshooting](docs/TROUBLESHOOTING.md)** - Common issues and solutions
- **[API Reference](docs/API_REFERENCE.md)** - Component and API documentation

## 🏗️ Architecture

### React Frontend (`chimera-web/src-react`)

Modern React 18 + TypeScript application with:
- **Component Library**: Button, Select, Panel, Tooltip, Badge
- **Visualization**: Recharts for signal plots and constellation diagrams  
- **State Management**: Zustand for application state
- **Testing**: Vitest + React Testing Library + Playwright
- **Documentation**: Storybook for component showcase

### Rust Core (`chimera-core`)

Signal processing engine compiled to WASM:
- LDPC encoder/decoder with configurable code rates
- QPSK modulation and demodulation
- AWGN channel simulation
- FFT-based spectrum analysis
- Frame synchronization and error correction

### Project Structure

```
chimera/
├── chimera-core/        # Rust DSP library
├── chimera-web/         # React + WASM web application
│   ├── src-react/       # React TypeScript source
│   │   ├── components/  # UI component library
│   │   ├── stores/      # Zustand state stores
│   │   └── App.tsx      # Main application
│   ├── src/            # Legacy Yew application (being phased out)
│   └── package.json    # NPM dependencies
├── chimera-cli/        # Command-line interface
├── contracts/          # Shared type definitions
└── docs/              # Documentation
```

## 🧪 Testing

```bash
# Run React component tests
cd chimera-web
npm test

# Run with coverage
npm run test:coverage

# Run E2E tests
npm run e2e

# Run Storybook
npm run storybook
```

## 🔨 Building

### Development Build

```bash
cd chimera-web
npm run dev
```

### Production Build

```bash
cd chimera-web
npm run build
```

The build output will be in `chimera-web/dist/`.

## 🎓 Educational Use

Chimera is designed for:
- **Teaching DSP Concepts**: Visualize modulation, channel effects, error correction
- **Prototyping Telemetry Links**: Test LDPC codes and framing strategies
- **Signal Intelligence**: Explore LPI/LPD techniques and exotic modulation
- **Interactive Learning**: Real-time parameter adjustment with immediate feedback

See [Signal Processing Concepts](docs/signal_processing_concepts.md) for educational materials.

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup and workflow
- Code style guidelines
- Testing requirements
- Pull request process

## 📋 Roadmap

- ✅ React migration and component library (Phase 1-3)
- 🔄 Documentation and polish (Phase 4) - **Current**
- 📋 Advanced node graph editor
- 📋 Additional modulation schemes (8-PSK, 16-QAM)
- 📋 Waterfall BER visualization
- 📋 Collaborative lesson scripts

See [docs/todo.md](docs/todo.md) for detailed roadmap.

## 🔐 Security

For security concerns, please see [SECURITY.md](SECURITY.md) or contact the maintainers directly.

## 📄 License

This project is licensed under the MIT License - see LICENSE file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/), [React](https://react.dev/), and [Vite](https://vitejs.dev/)
- DSP visualization powered by [Recharts](https://recharts.org/)
- Component documentation via [Storybook](https://storybook.js.org/)
- LDPC implementation inspired by modern FEC research

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/ArrEssJay/chimera/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ArrEssJay/chimera/discussions)
- **Documentation**: [docs/](docs/)

---

**Status**: Active Development | **Version**: 0.1.0 | **Last Updated**: October 2025
