# Signal-to-Noise Ratio (SNR)

**Signal-to-Noise Ratio (SNR)** measures the strength of the desired signal relative to the background noise. It's typically expressed in decibels (dB).

## Understanding SNR Values

```
Higher SNR = Better Signal Quality

SNR (dB)  |  Quality          |  Typical Use Case
----------|-------------------|----------------------------------
> 20 dB   |  Excellent        |  Clear reception, low error rate
10-20 dB  |  Good             |  Reliable communication
0-10 dB   |  Poor             |  Many errors, FEC required
< 0 dB    |  Very Poor        |  Noise stronger than signal
```

## SNR Formula

```
SNR (linear) = Signal Power / Noise Power

SNR (dB) = 10 · log₁₀(Signal Power / Noise Power)
```

## SNR in Chimera

In Chimera's simulation, you control the **channel SNR**, which determines how much noise is added to the transmitted signal:

| Setting | Description | Constellation |
|---------|-------------|---------------|
| **High SNR** (-5 dB) | Minimal noise | Tight clusters |
| **Medium SNR** (-15 dB) | Moderate noise | Visible scatter |
| **Low SNR** (-25 dB) | Heavy noise | Large scatter, errors likely |

### Processing Gain

Chimera achieves approximately **35 dB of processing gain** through symbol averaging and oversampling. This means:

```
Effective SNR = Channel SNR + Processing Gain
              = -25 dB + 35 dB
              = 10 dB (after processing)
```

This is why the system can operate reliably even with very low channel SNR values.

## SNR vs Es/N0

In Chimera's UI, "Channel SNR (dB)" represents **Es/N0** (symbol energy to noise ratio):
- **Before processing**: Low Es/N0 (e.g., -25 dB)
- **After processing gain**: Higher effective SNR (~10 dB)
- **LDPC threshold**: Fails below -27 dB channel SNR

## Impact on Performance

### High SNR (>15 dB)
- ✅ Perfect constellation separation
- ✅ Zero or near-zero bit errors
- ✅ FEC not strictly needed
- 📊 BER: <10⁻⁶

### Medium SNR (5-15 dB)
- ⚠️ Visible constellation scatter
- ⚠️ Some bit errors occur
- ⚠️ FEC recommended
- 📊 BER: 10⁻³ to 10⁻⁶

### Low SNR (<5 dB)
- ❌ Heavy constellation scatter
- ❌ Many bit errors
- ❌ FEC required
- 📊 BER: >10⁻³

## See Also

- [[Energy Ratios (Es N0 and Eb N0)]] - Related energy metrics
- [[Additive White Gaussian Noise (AWGN)]] - What creates the noise
- [[Bit Error Rate (BER)]] - How SNR affects errors
- [[Constellation Diagrams]] - Visualizing SNR impact
