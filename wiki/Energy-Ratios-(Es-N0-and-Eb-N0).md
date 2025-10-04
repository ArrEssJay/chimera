# Energy Ratios: Es/N0 and Eb/N0

## 📊 For Non-Technical Readers

**Es/N0 and Eb/N0 are like measuring "signal per bit of information" vs "background noise"—higher ratio = cleaner signal = fewer errors!**

**The idea - Signal vs Noise ratios**:
- **Signal energy**: How much "juice" each bit/symbol has
- **Noise**: Random interference (always present)
- **Ratio**: Signal energy divided by noise = quality measure!

**Two related measurements**:

**1. Eb/N0 (Energy per BIT)**:
- How much energy in each individual bit?
- **Higher Eb/N0** = more energy per bit = easier to detect correctly
- Used to compare different systems fairly

**2. Es/N0 (Energy per SYMBOL)**:
- How much energy in each symbol (which may carry multiple bits)?
- QPSK symbol carries 2 bits, so Es/N0 = 2 × Eb/N0
- Used for actual system performance calculations

**Why it matters - Quality threshold**:
- **Low Eb/N0** (weak signal): Many bit errors, unusable
- **Medium Eb/N0**: Some errors, FEC can fix them
- **High Eb/N0** (strong signal): Nearly error-free

**Real-world example**:
- **WiFi close to router**: Eb/N0 = 25 dB → use 256-QAM (fast!)
- **WiFi far from router**: Eb/N0 = 10 dB → use QPSK (reliable!)
- **WiFi too far**: Eb/N0 = 5 dB → connection drops

**The relationship**:
```
Es/N0 = Eb/N0 × (bits per symbol)
```
- BPSK: 1 bit/symbol → Es/N0 = Eb/N0
- QPSK: 2 bits/symbol → Es/N0 = 2 × Eb/N0
- 16-QAM: 4 bits/symbol → Es/N0 = 4 × Eb/N0

**Why engineers love Eb/N0**:
- **Fair comparison**: Compares systems with different modulations
- **Theory matches practice**: Theoretical limits use Eb/N0
- **Standards use it**: 3GPP, WiFi specs quote Eb/N0 requirements

**When you see it**:
- **Satellite link budget**: "Requires 10 dB Eb/N0 for BER < 10⁻⁶"
- **Modem spec sheet**: "Sensitivity: -95 dBm at 8 dB Eb/N0"
- **Paper comparing modulations**: Always uses Eb/N0 for fair comparison!

**Fun fact**: The theoretical minimum Eb/N0 for error-free communication is -1.59 dB (Shannon limit)—but real systems need 5-15 dB due to practical limitations!

---

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

- [[Signal-to-Noise-Ratio-(SNR)]] - Related power ratio
- [[Bit-Error-Rate-(BER)]] - Performance metric
- [[QPSK-Modulation]] - 2 bits per symbol
- [[Complete-Link-Budget-Analysis|Link Budget Analysis]] - Using energy ratios in system design
