# Energy Ratios: Es/N0 and Eb/N0

These ratios are fundamental measures of signal quality in digital communications.

## Es/N0: Symbol Energy Ratio

**Es/N0** measures the energy per symbol relative to the noise power spectral density.

- **Es**: Energy per symbol
- **N0**: Noise power per Hz (noise spectral density)
- Used when analyzing symbol-level performance

## Eb/N0: Bit Energy Ratio

**Eb/N0** measures the energy per bit relative to the noise power spectral density.

- **Eb**: Energy per bit  
- **N0**: Noise power per Hz
- More fundamental measure for comparing different modulation schemes

## Relationship Between Es/N0 and Eb/N0

The relationship depends on how many bits per symbol:

```
For QPSK (2 bits/symbol):
Eb/N0 = Es/N0 - 3.01 dB

General formula:
Eb/N0 (dB) = Es/N0 (dB) - 10·log₁₀(bits per symbol)
```

## Example in Chimera

- If Channel Es/N0 = -15 dB
- For QPSK (2 bits/symbol):
- Then Eb/N0 = -15 dB - 3.01 dB = **-18.01 dB**

## Why These Ratios Matter

1. **Performance Comparison**: Allows fair comparison between different modulation schemes
2. **Link Budget Analysis**: Essential for designing communication systems
3. **BER Prediction**: Theoretical BER curves are plotted against Eb/N0
4. **Standard Metric**: Industry-standard way to specify communication system performance

## Comparison Table

| Modulation | Bits/Symbol | Es/N0 to Eb/N0 Conversion |
|------------|-------------|---------------------------|
| BPSK       | 1           | Eb/N0 = Es/N0 (0 dB) |
| **QPSK**   | **2**       | **Eb/N0 = Es/N0 - 3.01 dB** |
| 8PSK       | 3           | Eb/N0 = Es/N0 - 4.77 dB |
| 16QAM      | 4           | Eb/N0 = Es/N0 - 6.02 dB |

## SNR vs Es/N0 vs Eb/N0

These terms are related but distinct:

- **SNR**: Ratio of signal power to noise power (may include bandwidth effects)
- **Es/N0**: Symbol energy to noise spectral density (symbol-level metric)
- **Eb/N0**: Bit energy to noise spectral density (bit-level metric, most fundamental)

In many contexts (including Chimera's simple channel model), **SNR ≈ Es/N0**.

## Theoretical BER for QPSK

```
BER_QPSK ≈ (1/2) · erfc(√(Eb/N0))

where erfc is the complementary error function
```

## See Also

- [[Signal to Noise Ratio (SNR)]] - Related power ratio
- [[Bit Error Rate (BER)]] - Performance metric
- [[QPSK Modulation]] - 2 bits per symbol
- [[Link Budget Analysis]] - Using energy ratios in system design
