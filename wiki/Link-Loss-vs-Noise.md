# Link Loss vs Noise

## 📉 For Non-Technical Readers

**Link loss vs noise is like the difference between someone whispering (weak signal) vs shouting in a loud room (noise interference)—two different problems!**

**Link Loss** - Signal gets weaker:
- **What it is**: Your WiFi router is far away, so signal is weak by the time it reaches you
- **Analogy**: Shouting across a football field—your voice spreads out and gets quieter
- **Predictable**: Same distance = same loss every time
- **Solution**: Move closer, use bigger antenna, increase transmit power

**Examples of link loss**:
- **WiFi**: 50 meters away = 10,000× weaker signal
- **Cell phone**: Far from tower = fewer bars
- **Satellite**: Space is far! Signal arrives incredibly weak

**Noise** - Random interference added:
- **What it is**: Random electrical static from electronics, thermal energy, cosmic rays
- **Analogy**: Trying to hear someone whisper in a noisy restaurant—extra sound covers the signal
- **Random**: Unpredictable, changes moment-to-moment
- **Solution**: Can't remove it! Must send stronger signal or use error correction

**Examples of noise**:
- **Bluetooth stuttering near microwave**: Microwave adds noise
- **AM radio crackle**: Thunderstorms add noise
- **TV static**: No signal? You're seeing pure noise!

**Key difference**:
- **Link loss**: Makes signal weaker (deterministic, predictable)
- **Noise**: Adds random garbage on top (random, unpredictable)
- **Both hurt you**: Weak signal (loss) covered by noise = errors!

**The engineering ratio: SNR (Signal-to-Noise Ratio)**
- Strong signal + low noise = high SNR = perfect communication
- Weak signal (loss) + high noise = low SNR = errors everywhere

**When you see it**: Your phone shows "5 bars" (link loss is low) but internet is slow (noise is high from interference).

---

In a real communication system, the received signal is degraded by **two distinct mechanisms**: link loss and additive noise.

## Link Loss (Path Loss)

**Link Loss** represents the reduction in signal power as it travels from transmitter to receiver.

### Characteristics
- **Deterministic**: Same loss every time (for a given scenario)
- **Multiplicative**: Scales the entire signal uniformly
- **Predictable**: Can be calculated from path distance, frequency, antenna gains

### Sources
- Free-space path loss
- Antenna gains (TX and RX)
- Cable losses
- Atmospheric absorption
- Rain attenuation

### Mathematical Model

```
P_received = P_transmitted / Loss_Factor

In dB: P_received (dBm) = P_transmitted (dBm) - Link_Loss (dB)
```

### Example Link Budget

```
Transmit Power:        +30 dBm
Antenna Gain (TX):     +10 dB
Free-Space Loss:       -120 dB
Antenna Gain (RX):     +5 dB
Cable/Implementation:  -5 dB
─────────────────────────────
Received Signal Power: -80 dBm
Total Link Loss:       100 dB
```

## Additive Noise

**Noise** adds random fluctuations on top of the received signal.

### Characteristics
- **Random**: Different every time, unpredictable
- **Additive**: Added to the signal (not multiplicative)
- **Stochastic**: Described by statistical properties (power, distribution)

### Sources
- Thermal noise (kTB)
- Amplifier noise
- Cosmic background
- Interference from other signals

### Mathematical Model

```
Received Signal = (Transmitted Signal / √Link_Loss) + Noise

where Noise has power determined by SNR or N₀
```

## Combined Channel Model

In Chimera's simulation, both effects are applied:

```
1. Transmit Signal (Power = P_tx)
        ↓
2. Apply Link Loss (Power reduced to P_tx / 10^(Loss_dB/10))
        ↓
3. Add AWGN (Noise power = Attenuated_Signal_Power / SNR)
        ↓
4. Received Signal (Attenuated + Noisy)
```

## Why Both Matter

### Link Loss Affects Signal Power
- High link loss (100+ dB) is typical in many systems
- Reduces signal level but doesn't add randomness
- Can be compensated with amplification (but amplifies noise too!)

### Noise Affects Signal Quality (SNR)
- Adds random errors that can't be predicted
- Sets the fundamental limit on achievable BER
- Can be improved with processing gain, error correction

## Link Budget and SNR

The combination determines receiver performance:

```
Received Signal Power = P_tx - Link_Loss_dB
Noise Power = N₀ × Bandwidth

SNR (dB) = Received_Signal_Power (dBm) - Noise_Power (dBm)
```

### Example 1: Good Link
- Transmit power: +30 dBm
- Link loss: 100 dB
- Received signal: **-70 dBm**
- Noise floor: -90 dBm
- **Resulting SNR: 20 dB** ✅ Good!

### Example 2: Challenging Link
- Transmit power: +30 dBm
- Link loss: 120 dB
- Received signal: **-90 dBm**
- Noise floor: -90 dBm
- **Resulting SNR: 0 dB** ⚠️ Very challenging!

## Link Loss in Chimera

Chimera allows you to model link loss separately from SNR:
- **Link Loss**: Simulates the signal power reduction (path loss, antenna gains, etc.)
- **SNR**: Controls the additive noise level
- Both combine to determine the received signal quality
- This separation helps understand link budget analysis

With **0 dB link loss**, the SNR setting directly determines signal quality.

With **100 dB link loss**, the signal is attenuated by 10¹⁰, but the SNR still controls the noise-to-signal ratio at the receiver input.

## Key Insight

```
More TX Power → Overcomes Link Loss → Higher RX Power → Better SNR
                                                              ↓
                                                      Lower BER ✅
```

But there are practical limits:
- Transmitter power constraints (battery, regulations)
- Receiver sensitivity (noise floor)
- Cost and complexity

## See Also

- [[Signal-to-Noise-Ratio-(SNR)]] - The quality metric
- [[Additive-White-Gaussian-Noise-(AWGN)]] - The noise model
- [[Complete-Link-Budget-Analysis|Link Budget Analysis]] - Calculating system performance
- [[Energy-Ratios-(Es-N0-and-Eb-N0)]] - Energy-based metrics
