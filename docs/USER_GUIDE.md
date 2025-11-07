# Chimera User Guide

Welcome to Chimera! This guide will help you understand and use the web application for signal processing experiments and learning.

## 🎯 What is Chimera?

Chimera is an interactive digital signal processing (DSP) workbench designed for:
- **Education**: Learning about modulation, error correction, and channel effects
- **Experimentation**: Testing telemetry link configurations and LDPC codes
- **Visualization**: Seeing how signals transform through each processing stage

## 🚀 Getting Started

### Accessing the Application

1. **Local Development**: If running locally, navigate to http://localhost:5173
2. **Production**: Access the deployed application URL

### Interface Overview

The Chimera interface consists of several key panels:

```
┌─────────────────────────────────────────────────┐
│  Header: Chimera - Signal Processing Workbench  │
├─────────────────────────────────────────────────┤
│  Simulation Controls Panel                      │
│  ├─ Preset Selection                            │
│  ├─ Plaintext Input                             │
│  ├─ SNR Configuration                           │
│  └─ Run Simulation Button                       │
├─────────────────────────────────────────────────┤
│  Pipeline Visualization                         │
│  ├─ Transmitter (TX)                            │
│  ├─ Channel                                     │
│  └─ Receiver (RX)                               │
├─────────────────────────────────────────────────┤
│  Results Panels                                 │
│  ├─ Frame Telemetry                             │
│  ├─ Constellation Diagrams                      │
│  ├─ Audio Playback Controls                     │
│  └─ Diagnostics                                 │
└─────────────────────────────────────────────────┘
```

## 📋 Using the Application

### 1. Simulation Controls

#### Preset Selection

Choose a pre-configured signal processing preset:

- **QPSK Basic**: Standard QPSK modulation with rate 1/2 LDPC
- **QPSK High-Rate**: Higher data rate configuration
- **Custom**: Define your own parameters

**To change preset:**
1. Click the preset dropdown
2. Select desired preset
3. Configuration automatically updates

#### Plaintext Input

Enter the message to transmit:

1. Type text in the plaintext input field (max 100 characters)
2. Text is encoded into bits, protected with error correction, and modulated
3. Default: "The quick brown fox jumps over the lazy dog"

#### SNR Configuration

Control the signal-to-noise ratio:

- **Range**: -10 dB to +20 dB
- **Effect**: Lower SNR = more noise = more errors
- **Typical values**:
  - 15 dB: Excellent conditions
  - 5-10 dB: Normal operation
  - 0 dB: Challenging
  - Below 0 dB: Severe degradation

**To adjust SNR:**
1. Use the slider or input field
2. Watch live updates to baseline SNR
3. Observe effect on error rates after simulation

#### Run Simulation

Execute the signal processing pipeline:

1. Click **"Run Now"** button
2. Status changes to "Running..."
3. Processing completes in 1-3 seconds
4. Results populate all visualization panels

### 2. Pipeline Visualization

#### Transmitter (TX)

Shows the transmitted signal:

- **Frame Layout**: Symbol breakdown (sync, payload, ECC)
- **TX Constellation**: Ideal QPSK symbol positions
- **Modulation Info**: Symbol rate, encoding details

**Tooltips**: Hover over metrics for explanations

#### Channel

Displays channel effects:

- **Channel SNR**: Actual vs. requested noise levels
- **Audio Integration**: Optional audio impairment blending
- **Magnitude Spectrum**: Frequency domain view

#### Receiver (RX)

Shows received and decoded signal:

- **RX Constellation**: Noisy symbol positions with decision regions
- **LDPC Decoder**: Iterative error correction status
- **Recovered Payload**: Decoded plaintext with error highlighting

### 3. Audio Playback Controls

Listen to the signals:

- **Play Clean**: Original carrier without noise
- **Play Noisy**: Signal with channel noise applied
- **Stop**: Stop playback
- **Volume**: Adjust playback volume (0-100%)

**Note**: Audio is generated at 8 kHz sample rate for audibility

### 4. Frame Telemetry

Detailed frame statistics:

- **Total Symbols**: Complete frame length
- **Sync Symbols**: Frame synchronization overhead
- **Payload Symbols**: User data symbols
- **ECC Symbols**: Error correction parity

### 5. Constellation Diagrams

Visual representation of symbol positions:

#### TX Constellation
- Shows ideal QPSK points (±1±j)
- Four clear clusters at cardinal points
- No noise or distortion

#### RX Constellation
- Shows received symbols after channel
- Noise spreading visible
- Decision boundaries shown
- Color-coded by detection confidence

**Interpretation:**
- Tight clusters = low noise, good SNR
- Spread clusters = high noise, low SNR
- Overlapping clusters = high error rate

### 6. Diagnostics Panel

Performance metrics:

- **Pre-FEC BER**: Bit error rate before error correction
- **Post-FEC BER**: Bit error rate after LDPC decoding
- **Residual Errors**: Uncorrected bit errors
- **LDPC Iterations**: Decoder convergence information

## 🔧 Advanced Features

### Audio Impairment Injection

Blend custom audio into the channel:

1. Click "Upload Audio" (feature in development)
2. Audio is resampled to match simulation rate
3. Non-linear mixing simulates Raman feed effects
4. Observe impact on constellation and BER

### Preset Customization

Create custom presets:

1. Modify existing preset parameters
2. Adjust:
   - Symbol rate
   - LDPC code rate
   - Frame structure
   - Channel model
3. Save as new preset

### Batch Analysis

Run multiple simulations (CLI):

```bash
cd chimera-cli
cargo run --release -- --snr-sweep 0:20:1 --output results.csv
```

See CLI documentation for batch processing options.

## 📊 Understanding the Metrics

### Bit Error Rate (BER)

**Definition**: Ratio of incorrect bits to total bits

**Typical values:**
- `BER < 1e-6`: Excellent, error-free operation
- `BER ~ 1e-3`: Moderate errors, FEC effective
- `BER > 1e-2`: Severe errors, link degraded

### Signal-to-Noise Ratio (SNR)

**Definition**: Signal power relative to noise power (dB)

**Rule of thumb:**
- SNR > 15 dB: Near error-free with FEC
- SNR 5-15 dB: Normal operation
- SNR < 5 dB: Significant errors

### LDPC Decoder Performance

**Metrics to watch:**
- **Iterations**: Fewer = cleaner signal
- **Convergence**: "Converged" = successful decode
- **Pre vs. Post FEC**: Large reduction = effective coding

## 💡 Tips and Best Practices

### For Learning

1. **Start with high SNR** (15+ dB) to see ideal operation
2. **Gradually decrease SNR** to observe degradation
3. **Compare TX and RX constellations** to visualize noise
4. **Listen to audio** to hear channel effects
5. **Watch BER metrics** to quantify performance

### For Experimentation

1. **Test different presets** to compare configurations
2. **Vary SNR systematically** for BER curves
3. **Try different message lengths** to see frame efficiency
4. **Note LDPC iteration counts** for decoder complexity

### Performance Tips

1. **Use shorter messages** for faster simulation
2. **Disable audio** if not needed
3. **Run CLI** for batch processing
4. **Monitor browser console** for debug info

## 🐛 Troubleshooting

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common issues and solutions.

### Quick Fixes

**Simulation won't run**:
- Check browser console for errors
- Refresh the page
- Verify WASM loaded successfully

**Audio not playing**:
- Check browser audio permissions
- Verify volume is not muted
- Try different browser (Chrome/Firefox work best)

**Slow performance**:
- Close other browser tabs
- Reduce frame size (use shorter text)
- Clear browser cache

## 📚 Learning Resources

### Signal Processing Concepts

See [signal_processing_concepts.md](signal_processing_concepts.md) for:
- QPSK modulation theory
- LDPC error correction
- Constellation diagrams
- Channel effects

### Technical Details

See [chimera_technical_overview.md](chimera_technical_overview.md) for:
- Architecture design
- Implementation details
- Advanced features

## 🆘 Getting Help

- **Documentation**: Check docs/ directory
- **Issues**: Report bugs on GitHub Issues
- **Questions**: Ask in GitHub Discussions
- **In-app Help**: Hover over ⓘ icons for tooltips

## 📝 Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl/Cmd + Enter` | Run simulation |
| `Space` | Play/pause audio |
| `Esc` | Stop audio |
| `Tab` | Navigate between controls |
| `?` | Show help (when implemented) |

## 🎓 Example Workflows

### Basic Link Analysis

1. Select "QPSK Basic" preset
2. Enter short test message
3. Set SNR to 15 dB
4. Run simulation
5. Observe near-perfect reception
6. Decrease SNR to 5 dB
7. Run again and compare results

### Error Correction Study

1. Note Pre-FEC BER at certain SNR
2. Compare to Post-FEC BER
3. Calculate coding gain (dB)
4. Observe LDPC iterations
5. Try different code rates

### Audio Exploration

1. Run simulation with moderate SNR
2. Play clean audio - hear carrier tone
3. Play noisy audio - hear noise
4. Compare magnitude spectrum
5. Adjust volume to comfort level

---

**Need more help?** See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) or open a GitHub Discussion.

**Ready to contribute?** See [CONTRIBUTING.md](../CONTRIBUTING.md) for developer setup.
