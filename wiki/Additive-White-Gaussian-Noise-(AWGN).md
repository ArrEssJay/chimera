# Additive White Gaussian Noise (AWGN)

**AWGN** is a basic noise model used in communication systems.

## What is AWGN?

- **Additive**: Noise is added to the signal
- **White**: Uniform power across all frequencies
- **Gaussian**: Follows a normal (Gaussian) probability distribution

## Visualizing AWGN

```
Clean Signal:     ──────────
                         
AWGN:            ∿∿∿∿∿∿∿∿∿∿
                         
Noisy Signal:    ∿─∿─∿──∿─∿
                (Clean + AWGN)
```

## AWGN Channel Model

In the I/Q plane, AWGN adds independent Gaussian noise to both components:

```
Received Symbol = Transmitted Symbol + Noise

I_received = I_transmitted + N_I
Q_received = Q_transmitted + N_Q

where N_I and N_Q are independent Gaussian random variables
```

## Mathematical Properties

The noise samples have:
- **Mean**: 0 (centered around zero)
- **Variance**: σ² (determined by noise power)
- **Probability Distribution**: Gaussian (bell curve)

```
Probability Density:
  ↑
  |    📊
  |   ╱  ╲
  |  ╱    ╲
  | ╱      ╲___
  |╱            ╲___
  +─────────────────→ Amplitude
       0
```

## Why AWGN is Used

1. **Simplicity**: Mathematical tractability for analysis
2. **Fundamental Model**: Many real noise sources approximate Gaussian statistics
3. **Worst Case**: Often represents a pessimistic but realistic scenario
4. **Standard Benchmark**: Industry-standard for comparing systems

## Sources of Noise in Real Systems

- **Thermal Noise**: Random motion of electrons (kTB)
- **Amplifier Noise**: Electronic component noise
- **Cosmic Noise**: Background radiation
- **Interference**: Other signals (approximates Gaussian when many sources)

## AWGN in Chimera

Chimera's simulation applies AWGN to model the communication channel:
- Noise is added separately to I and Q components
- Noise power is controlled by the [[Signal to Noise Ratio (SNR)]] setting
- Higher SNR = less noise variance = tighter constellation clusters

### Noise Power Calculation

```
Noise Variance (σ²) = Signal Power / SNR_linear
                    = Signal Power / 10^(SNR_dB/10)
```

For unit signal power and SNR = 10 dB:
```
σ² = 1 / 10^(10/10) = 1/10 = 0.1
σ = √0.1 ≈ 0.316
```

## Impact on Constellation

```
High Noise (σ = 0.5):          Low Noise (σ = 0.1):
  
  Q                               Q
  ↑                               ↑
  |  ◦◦◦                          |  ◦
  | ◦◦•◦◦                         | ◦•◦
  |  ◦◦◦                          |  ◦
──┼────→ I                    ────┼──→ I

Large scatter = high errors    Tight cluster = low errors
```

## See Also

- [[Signal to Noise Ratio (SNR)]] - Controls noise power
- [[Link Loss vs Noise]] - Distinction between attenuation and noise
- [[Constellation Diagrams]] - Visualizing noise effects
- [[IQ Representation]] - How noise affects I/Q components
