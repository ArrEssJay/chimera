# Signal Processing Concepts Guide

This guide explains the key signal processing terminology and concepts used in Chimera's digital communication simulation.

## Table of Contents

1. [Symbols](#symbols)
2. [QPSK Modulation](#qpsk-modulation)
3. [Constellation Diagrams](#constellation-diagrams)
4. [Signal-to-Noise Ratio (SNR)](#signal-to-noise-ratio-snr)
5. [Energy Ratios (Es/N0 and Eb/N0)](#energy-ratios-esn0-and-ebn0)
6. [Additive White Gaussian Noise (AWGN)](#additive-white-gaussian-noise-awgn)
7. [Bit Error Rate (BER)](#bit-error-rate-ber)
8. [Forward Error Correction (FEC)](#forward-error-correction-fec)

---

## Symbols

### What are Symbols?

In digital communication, a **symbol** is a fundamental unit of information transmitted over the channel. Think of symbols as the "words" of a digital communication system.

**The Symbol Hierarchy:**

```
Raw Data (Bits)
    ↓
Grouped into Symbols
    ↓
Mapped to Signal States
    ↓
Transmitted over Channel
```

### Example: From Bits to Symbols

Imagine you want to transmit the binary data: `0 0 1 1 0 1 1 0`

Instead of sending each bit individually, we group them into pairs (for QPSK):
- Bits `0 0` → Symbol 1
- Bits `1 1` → Symbol 2  
- Bits `0 1` → Symbol 3
- Bits `1 0` → Symbol 4

This grouping allows us to transmit more efficiently and makes the signal more robust to noise.

### Why Use Symbols?

1. **Efficiency**: Transmitting symbols (groups of bits) can be more bandwidth-efficient than transmitting individual bits
2. **Robustness**: Symbol-based schemes can be designed to be more resistant to noise and interference
3. **Flexibility**: Different modulation schemes can encode different numbers of bits per symbol

---

## QPSK Modulation

### What is QPSK?

**QPSK** stands for **Quadrature Phase-Shift Keying**. It's a modulation technique that encodes 2 bits per symbol by varying the phase of the carrier wave.

### The Four QPSK States

QPSK uses four distinct phase states, each representing a unique 2-bit pattern:

```
        Q (Quadrature)
              ↑
              |
    00 •      |      • 01
              |
   -----------+-----------> I (In-phase)
              |
    11 •      |      • 10
              |
```

**Bit-to-Phase Mapping in Chimera:**
- `00` → Phase: 135° (upper-left quadrant)
- `01` → Phase: 45° (upper-right quadrant)
- `11` → Phase: 225° (lower-left quadrant)
- `10` → Phase: 315° (lower-right quadrant)

### I/Q Representation

Each QPSK symbol is represented as a point in 2D space with:
- **I (In-phase)**: The horizontal component
- **Q (Quadrature)**: The vertical component

These are also called the "real" and "imaginary" parts of a complex number.

**Mathematical Representation:**

For normalized QPSK (unit energy), the four symbols are:
```
Symbol 00: I = -1/√2,  Q = +1/√2
Symbol 01: I = +1/√2,  Q = +1/√2
Symbol 11: I = -1/√2,  Q = -1/√2
Symbol 10: I = +1/√2,  Q = -1/√2
```

### Why QPSK?

- **Spectral Efficiency**: Transmits 2 bits per symbol
- **Robustness**: The large phase separation (90°) makes it resilient to noise
- **Simplicity**: Relatively simple to implement and demodulate
- **Widespread Use**: Used in many real-world systems (satellite, WiFi, LTE)

---

## Constellation Diagrams

### What is a Constellation Diagram?

A **constellation diagram** is a visual representation of a digital modulation scheme. It shows all possible symbol positions in the I/Q plane.

### Reading a Constellation Diagram

```
      Q (Imaginary)
           ↑
           |
      ┌────┼────┐
      │ •  |  • │  Each dot represents
      │    |    │  a valid symbol position
   ───┼────+────┼───→ I (Real)
      │    |    │
      │ •  |  • │
      └────┼────┘
           |
```

**Key Elements:**

1. **Ideal Points**: Perfect symbol positions (clean transmission)
2. **Received Cloud**: Actual received symbols scattered due to noise
3. **Decision Boundaries**: Regions that determine which symbol was sent

### TX vs RX Constellations

**TX Constellation (Transmitter):**
- Shows ideal symbol positions
- Points are crisp and perfectly positioned
- Represents what was intended to be transmitted

**RX Constellation (Receiver):**
- Shows received symbol positions after channel effects
- Points are scattered in clouds around ideal positions
- Scattering indicates noise level and channel quality
- Larger scatter = more noise = harder to decode correctly

### What the Constellation Tells You

- **Tight clusters**: Low noise, high SNR, good signal quality
- **Scattered clouds**: High noise, low SNR, potential errors
- **Symbol overlap**: Very poor signal quality, high error rate expected
- **Pattern offset**: May indicate frequency or phase errors

---

## Signal-to-Noise Ratio (SNR)

### What is SNR?

**Signal-to-Noise Ratio (SNR)** measures the strength of the desired signal relative to the background noise. It's typically expressed in decibels (dB).

### Understanding SNR Values

```
Higher SNR = Better Signal Quality

SNR (dB)  |  Quality          |  Typical Use Case
----------|-------------------|----------------------------------
> 20 dB   |  Excellent        |  Clear reception, low error rate
10-20 dB  |  Good             |  Reliable communication
0-10 dB   |  Poor             |  Many errors, FEC required
< 0 dB    |  Very Poor        |  Noise stronger than signal
```

### SNR in Chimera

In Chimera's simulation, you control the channel SNR, which determines how much noise is added to the transmitted signal:

- **High SNR** (-5 dB): Minimal noise, constellation points stay tight
- **Medium SNR** (-15 dB): Moderate noise, visible scatter in constellation
- **Low SNR** (-25 dB): Heavy noise, large scatter, errors likely

### Processing Gain

Chimera achieves approximately **35 dB of processing gain** through symbol averaging and oversampling. This means:

```
Effective SNR = Channel SNR + Processing Gain
              = -25 dB + 35 dB
              = 10 dB (after processing)
```

This is why the system can operate reliably even with very low channel SNR values.

---

## Energy Ratios (Es/N0 and Eb/N0)

### Understanding Energy Ratios

These ratios are fundamental measures of signal quality in digital communications:

**Es/N0**: Symbol Energy to Noise Spectral Density Ratio
**Eb/N0**: Bit Energy to Noise Spectral Density Ratio

### Es/N0 (Symbol Energy Ratio)

**Es/N0** measures the energy per symbol relative to the noise power spectral density.

- **Es**: Energy per symbol
- **N0**: Noise power per Hz (noise spectral density)
- Used when analyzing symbol-level performance

### Eb/N0 (Bit Energy Ratio)

**Eb/N0** measures the energy per bit relative to the noise power spectral density.

- **Eb**: Energy per bit  
- **N0**: Noise power per Hz
- More fundamental measure for comparing different modulation schemes

### Relationship Between Es/N0 and Eb/N0

The relationship depends on how many bits per symbol:

```
For QPSK (2 bits/symbol):
Eb/N0 = Es/N0 - 3.01 dB

General formula:
Eb/N0 (dB) = Es/N0 (dB) - 10·log₁₀(bits per symbol)
```

**Example in Chimera:**
- If Channel Es/N0 = -15 dB
- Then Eb/N0 = -15 dB - 3.01 dB = -18.01 dB

### Why These Ratios Matter

1. **Performance Comparison**: Allows fair comparison between different modulation schemes
2. **Link Budget Analysis**: Essential for designing communication systems
3. **BER Prediction**: Theoretical BER curves are plotted against Eb/N0
4. **Standard Metric**: Industry-standard way to specify communication system performance

### SNR in Chimera

The UI displays "Channel SNR (dB)" which represents **Es/N0** in the simulation. The text indicates:
- **Before processing**: Low Es/N0 (e.g., -25 dB)
- **After processing gain**: Higher effective SNR (~10 dB)
- **LDPC threshold**: Fails below -27 dB channel SNR

---

## Additive White Gaussian Noise (AWGN)

### What is AWGN?

**AWGN** is a basic noise model used in communication systems:

- **Additive**: Noise is added to the signal
- **White**: Uniform power across all frequencies
- **Gaussian**: Follows a normal (Gaussian) probability distribution

### Visualizing AWGN

```
Clean Signal:     ──────────
                         
AWGN:            ∿∿∿∿∿∿∿∿∿∿
                         
Noisy Signal:    ∿─∿─∿──∿─∿
                (Clean + AWGN)
```

### AWGN Channel Model

In the I/Q plane, AWGN adds independent Gaussian noise to both components:

```
Received Symbol = Transmitted Symbol + Noise

I_received = I_transmitted + N_I
Q_received = Q_transmitted + N_Q

where N_I and N_Q are independent Gaussian random variables
```

### Why AWGN is Used

1. **Simplicity**: Mathematical tractability for analysis
2. **Fundamental Model**: Many real noise sources approximate Gaussian statistics
3. **Worst Case**: Often represents a pessimistic but realistic scenario
4. **Standard Benchmark**: Industry-standard for comparing systems

### AWGN in Chimera

Chimera's simulation applies AWGN to model the communication channel:
- Noise is added separately to I and Q components
- Noise power is controlled by the SNR setting
- Higher SNR = less noise variance = tighter constellation clusters

---

## Bit Error Rate (BER)

### What is BER?

**Bit Error Rate (BER)** is the ratio of incorrectly decoded bits to total transmitted bits:

```
BER = Number of Bit Errors / Total Number of Bits
```

### BER Scale

BER is typically expressed as a decimal or in scientific notation:

```
BER Value  |  Meaning                    |  Quality
-----------|-----------------------------|-----------
10⁻¹       |  1 error per 10 bits        |  Terrible
10⁻³       |  1 error per 1,000 bits     |  Poor
10⁻⁶       |  1 error per 1,000,000 bits |  Good
10⁻⁹       |  1 error per 1 billion bits |  Excellent
```

### Pre-FEC vs Post-FEC BER

**Pre-FEC BER**: Error rate before error correction
- Directly reflects channel quality
- Higher at low SNR

**Post-FEC BER**: Error rate after error correction (LDPC decoding)
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

Coding Gain: 40 dB improvement!
```

### BER Curves

A BER vs SNR curve shows system performance:

```
BER
 ↑
 |         
10⁰|•        
   |  •      
10⁻³|    •
   |      •
10⁻⁶|        •___
   |
   +---------------→ SNR (dB)
```

Key features:
- **Waterfall region**: Steep decrease in BER as SNR increases
- **Threshold**: SNR where BER becomes acceptable
- **Error floor**: Minimum achievable BER

---

## Forward Error Correction (FEC)

### What is FEC?

**Forward Error Correction (FEC)** adds redundancy to transmitted data so the receiver can detect and correct errors without retransmission.

### How FEC Works

```
Original Data:     [100 bits]
      ↓
FEC Encoder:       [Add redundancy]
      ↓
Encoded Data:      [150 bits with parity]
      ↓
Noisy Channel:     [Errors introduced]
      ↓
FEC Decoder:       [Correct errors using redundancy]
      ↓
Recovered Data:    [100 bits, hopefully error-free]
```

### LDPC Codes

**Low-Density Parity-Check (LDPC)** codes are a class of FEC codes used in Chimera:

**Properties:**
- **Near Shannon-limit performance**: Approaches theoretical maximum efficiency
- **Iterative decoding**: Uses belief propagation algorithm
- **Flexible**: Configurable code rate and structure
- **Widely used**: Found in WiFi (802.11n/ac), satellite, DVB-S2

**Key Parameters:**
- **Code rate**: Ratio of information bits to total bits (e.g., 1/2, 2/3, 3/4)
  - Lower rate = more redundancy = better correction but less efficiency
- **Block length**: Number of bits in each codeword
- **Degree distribution**: Structural property affecting performance

### FEC Gain

**Coding Gain** measures the SNR improvement provided by FEC:

```
Without FEC: Need SNR = 10 dB for BER = 10⁻⁶
With FEC:    Need SNR = 2 dB for BER = 10⁻⁶

Coding Gain = 10 dB - 2 dB = 8 dB
```

### FEC in Chimera

In Chimera's simulation:

1. **Encoder**: Applies LDPC encoding to payload data
2. **Channel**: AWGN corrupts the transmitted symbols
3. **Decoder**: Iterative LDPC decoder corrects errors
4. **Metrics**: Compare Pre-FEC BER vs Post-FEC BER to see FEC effectiveness

The system shows:
- **ECC symbols**: Number of parity/redundancy symbols added
- **Pre-FEC errors**: Errors at the demodulator output  
- **Post-FEC errors**: Residual errors after LDPC correction
- **Frame Error Rate**: Percentage of frames that couldn't be fully corrected

---

## Putting It All Together

### The Complete Signal Chain

```
1. Input Text
   ↓
2. Bits Extraction
   ↓
3. LDPC Encoding (add redundancy)
   ↓
4. QPSK Mapping (bits → symbols)
   ↓
5. Modulation (symbols → I/Q signal)
   ↓
6. AWGN Channel (add noise)
   ↓
7. Demodulation (I/Q signal → symbols)
   ↓
8. Symbol Decisions (noisy I/Q → best guess bits)
   ↓
9. LDPC Decoding (correct errors)
   ↓
10. Output Text
```

### Key Relationships

```
Low SNR → More Noise → Scattered Constellation → Higher BER
                                                     ↓
                                              FEC Corrects Some
                                                     ↓
                                           Lower Post-FEC BER
```

### Observing in Chimera

When you run a simulation:

1. **TX Constellation**: See the ideal QPSK symbol positions
2. **Channel SNR**: Control how much noise is added
3. **RX Constellation**: See how noise scatters the received symbols
4. **Pre-FEC BER**: Shows raw demodulation error rate
5. **Post-FEC BER**: Shows effectiveness of error correction
6. **Recovered Message**: The final decoded text

By adjusting SNR and observing these elements, you can build intuition for how digital communication systems work!

---

## Further Reading

For more details on Chimera's implementation:
- [Chimera Technical Overview](chimera_technical_overview.md)
- [Modulation Protocol](modulation_protocol_v4.2.md)

For general digital communications theory, see:
- Proakis & Salehi, "Digital Communications"
- Richardson & Urbanke, "Modern Coding Theory"
