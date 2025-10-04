# Bit Error Rate (BER)

**Bit Error Rate (BER)** is the ratio of incorrectly decoded bits to total transmitted bits.

## Definition

```
BER = Number of Bit Errors / Total Number of Bits
```

## BER Scale

BER is typically expressed as a decimal or in scientific notation:

| BER Value | Meaning | Quality |
|-----------|---------|---------|
| 10⁻¹ | 1 error per 10 bits | Terrible |
| 10⁻² | 1 error per 100 bits | Very Poor |
| 10⁻³ | 1 error per 1,000 bits | Poor |
| 10⁻⁴ | 1 error per 10,000 bits | Marginal |
| 10⁻⁶ | 1 error per 1,000,000 bits | Good |
| 10⁻⁹ | 1 error per 1 billion bits | Excellent |
| 10⁻¹² | 1 error per 1 trillion bits | Exceptional |

## Pre-FEC vs Post-FEC BER

### Pre-FEC BER
Error rate **before** error correction
- Directly reflects channel quality
- Higher at low SNR
- Called "raw BER" or "channel BER"

### Post-FEC BER
Error rate **after** error correction (LDPC decoding)
- Shows effectiveness of error correction
- Should be much lower than Pre-FEC
- The "residual errors" that couldn't be corrected

```
Example:
Pre-FEC BER:  10⁻² (1 error per 100 bits)
              ↓
        [LDPC Decoder]
              ↓
Post-FEC BER: 10⁻⁶ (1 error per million bits)

Coding Gain: 40 dB improvement! 🎉
```

## BER vs SNR Curves

A BER vs SNR curve shows system performance:

```
BER
 ↑
 |         
10⁰|•        Unusable
   |  •      
10⁻³|    •    Poor
   |      •
10⁻⁶|        •___ Good (threshold)
   |            ╲___
10⁻⁹|                ╲___ Excellent
   |
   +─────────────────────→ SNR (dB)
    -5  0   5  10  15  20
```

### Key Features
- **Waterfall region**: Steep decrease in BER as SNR increases
- **Threshold**: SNR where BER becomes acceptable (often 10⁻⁶)
- **Error floor**: Minimum achievable BER (implementation limits)

## Theoretical vs Measured BER

### Theoretical BER for QPSK
```
BER_QPSK ≈ (1/2) · erfc(√(Eb/N0))
```

### In Chimera
- **Theoretical**: Based on the formula above
- **Measured**: Actual errors observed in simulation
- **Difference**: Processing gain, implementation effects, finite sample size

## Factors Affecting BER

1. **[[Signal to Noise Ratio (SNR)]]**: Primary factor
   - Higher SNR → Lower BER
   
2. **Modulation Scheme**: 
   - QPSK more robust than 16QAM
   - Lower order = better BER at same SNR

3. **[[Forward Error Correction (FEC)]]**:
   - Can reduce BER by orders of magnitude
   - LDPC codes provide near-optimal performance

4. **Channel Impairments**:
   - Phase noise, frequency offset
   - Timing errors, multipath

5. **Implementation**:
   - Quantization effects
   - Synchronization accuracy

## BER in Chimera

Chimera displays multiple BER metrics:

### Pre-FEC Metrics
- **Symbol Errors**: Count of incorrect symbol decisions
- **Bit Errors (Pre-FEC)**: Bit errors before LDPC decoding
- **Pre-FEC BER**: Bit error rate at demodulator output

### Post-FEC Metrics
- **Bit Errors (Post-FEC)**: Residual errors after LDPC
- **Post-FEC BER**: Final bit error rate
- **Frame Error Rate (FER)**: Percentage of frames with uncorrectable errors

### Example Output
```
Pre-FEC BER:  2.3 × 10⁻² (2.3% bit errors)
Post-FEC BER: 0 (all errors corrected!) ✅
FER:          0% (no frame errors)
```

## Acceptable BER Thresholds

Different applications have different requirements:

| Application | Required BER | Rationale |
|-------------|--------------|-----------|
| Voice (analog) | 10⁻³ | Some crackling acceptable |
| Data (with retransmission) | 10⁻⁴ - 10⁻⁶ | Retries handle errors |
| Streaming video | 10⁻⁶ | Occasional glitch OK |
| File transfer | 10⁻⁹ | Data integrity critical |
| Financial transactions | 10⁻¹² | Zero tolerance |

## See Also

- [[Signal to Noise Ratio (SNR)]] - Primary BER determinant
- [[Forward Error Correction (FEC)]] - BER improvement technique
- [[Energy Ratios (Es N0 and Eb N0)]] - Used in BER formulas
- [[Understanding BER Curves]] - Interpreting performance plots
