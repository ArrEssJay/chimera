# Chimera

**Build, test, and visualize communication systems in your browser**

[![Try Live Demo](https://img.shields.io/badge/🚀_Try_Live_Demo-impermanent.io-blue?style=for-the-badge)](https://impermanent.io)

> **What is Chimera?** A browser-based visual tool for creating and experimenting with digital communication systems. Build signal processing pipelines by connecting blocks together—inspired by GNU Radio Companion, but designed for the web with zero installation required. 


---

## What Can You Do With It?

### 🎓 Learn Signal Processing
- See how data travels from bits to radio waves and back
- Watch what noise does to your signals in real-time
- Understand error correction by breaking and fixing things
- Visualize constellation diagrams, FFT, and other DSP concepts

### 🔬 Prototype Communication Systems
- Test QPSK, QAM, and other modulation schemes
- Experiment with LDPC forward error correction
- Simulate realistic channel conditions (AWGN, fading, interference)
- Measure BER, SNR, and other performance metrics

### 🎨 Build Visually (Coming Soon)
- Drag and drop signal processing blocks
- Connect them with type-safe connections
- Run your pipeline and see results instantly
- Save and share your configurations

---

## How It Works

```
┌─────────────┐   ┌───────────┐   ┌──────────┐   ┌─────────┐
│   Your      │   │  Encode   │   │ Modulate │   │  Add    │
│   Data      │──▶│  (LDPC)   │──▶│  (QPSK)  │──▶│  Noise  │
└─────────────┘   └───────────┘   └──────────┘   └─────────┘
                                                        │
┌─────────────┐   ┌───────────┐   ┌──────────┐       │
│  Decoded    │   │  Decode   │   │Demodulate│◀──────┘
│   Data      │◀──│  (LDPC)   │◀──│  (QPSK)  │
└─────────────┘   └───────────┘   └──────────┘

         Watch every step • Tweak parameters • Learn by doing
```

### Current Features
- ✅ **QPSK Modulation**: See your bits become radio signals
- ✅ **LDPC Error Correction**: Test how FEC saves your data from noise
- ✅ **Interactive Controls**: Adjust SNR, view constellations, hear audio
- ✅ **Real-time Visualization**: Constellation diagrams, FFT, BER stats
- ✅ **Preset Configurations**: Start with working examples

### Coming Soon (WIP - Epic #40)
- 🚧 **Visual Node Graph**: Drag-and-drop pipeline builder
- 🚧 **15+ Processing Blocks**: Mix and match sources, processors, sinks
- 🚧 **Save/Load Graphs**: Share your configurations
- 🚧 **Custom Blocks**: Write your own processing nodes

---

## Try It Now

### 🌐 Web Version (Easiest)

Visit **[impermanent.io](https://impermanent.io)** - no installation needed.

**What you'll see:**
1. A preset QPSK telemetry link configuration
2. Controls to adjust SNR, input data, and more
3. Real-time constellation diagram showing signal quality
4. Before/after comparison of error correction

**Try this:**
- Lower the SNR slider and watch the constellation spread out
- Click "Run" to see how LDPC error correction performs
- Upload an audio file to hear your voice transmitted through a simulated radio link

---

## For Developers

Want to build or extend Chimera?

### Quick Start
```bash
# Clone the repository
git clone https://github.com/ArrEssJay/chimera.git
cd chimera

# Build and run (Rust required)
cargo build --release
cargo run --bin chimera-cli --help

# Or run the web version locally
cd chimera-web
trunk serve
# Open http://localhost:8080
```

### Architecture
- **Core DSP Engine**: Rust (fast, safe, runs anywhere via WASM)
- **Web Interface**: React + TypeScript (new), Yew (legacy)
- **Signal Processing**: LDPC, QPSK, AWGN, soft-decision decoding

### Contributing
We welcome contributions! Check out:
- [Open Issues](https://github.com/ArrEssJay/chimera/issues) - pick something that interests you
- [Developer Guide](./.github/copilot-instructions.md) - how we work
- [Architecture Docs](./docs/) - understand the system

**Important**: We're building this FOR USERS. Every feature should make learning or prototyping easier. If a feature doesn't help users understand signals better or build systems faster, we don't need it.

---

## Use Cases

### 📚 Education
- **Signal Processing Course**: Visual demonstrations of modulation, FEC, channel effects
- **Self-Study**: Experiment without expensive SDR hardware
- **Tutorials**: Build along with step-by-step guides

### 🔬 Research & Prototyping
- **Algorithm Testing**: Try new coding schemes before hardware implementation
- **Performance Analysis**: Compare different modulation formats
- **Link Budget Analysis**: Model real-world communication scenarios

### 🛰️ Communication System Design
- **Proof of Concept**: Validate design choices early
- **Parameter Optimization**: Find the best SNR, code rate, modulation
- **Documentation**: Share working examples with your team

---

## What Makes Chimera Different?

### ✨ Browser-Based
- Zero installation required
- Works on any device with a modern web browser
- Share links to your configurations

### 🎓 Learning-Focused
- Every parameter explained with tooltips
- Visual feedback for every change
- Start with working examples, then customize

### ⚡ Fast
- Powered by Rust/WebAssembly
- Real-time processing even with complex FEC
- Smooth, responsive interface

### 🔓 Open Source
- See how it works
- Extend it for your needs
- Learn from the code

---

## FAQ

**Q: Do I need signal processing knowledge to use this?**  
A: No! Start with presets and learn as you experiment. Tooltips explain every parameter.

**Q: Can I use this for real radio systems?**  
A: Chimera is designed for simulation, prototyping, and education. Use it to validate your designs and parameters, then implement them in production hardware (SDRs, embedded systems, etc.).

**Q: Is it free?**  
A: Yes! Open source and free to use. Forever.

**Q: What browsers are supported?**  
A: Chrome, Firefox, Safari, Edge - anything modern with WebAssembly support.

**Q: Can I run it offline?**  
A: The web version needs internet. Clone and build locally for offline use.

---

## Links

- 🌐 **Try it**: [impermanent.io](https://impermanent.io)
- 📖 **Wiki**: [Comprehensive DSP Wiki](https://github.com/ArrEssJay/chimera/wiki) | [📄 PDF Download](https://github.com/ArrEssJay/chimera/raw/main/chimera-wiki.pdf)
- 📚 **Docs**: [Signal Processing Concepts](./docs/signal_processing_concepts.md)
- 🐛 **Report Issues**: [GitHub Issues](https://github.com/ArrEssJay/chimera/issues)
- 💬 **Discuss**: [GitHub Discussions](https://github.com/ArrEssJay/chimera/discussions)
- 📰 **Epic #40**: [Visual Node Graph](https://github.com/ArrEssJay/chimera/issues/40) (WIP)

---

<div align="center">

**Built to help YOU understand and build communication systems**

*Made with 🦀 Rust • ⚛️ React • ❤️ for learners and builders*

</div>
