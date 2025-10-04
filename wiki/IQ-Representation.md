# IQ Representation

## 📊 For Non-Technical Readers

**IQ representation is like describing a location on a map using X and Y coordinates—it lets you pinpoint any radio signal position in 2D space!**

**What is IQ?**
- **I (In-phase)**: Horizontal axis, like "East-West" on a map
- **Q (Quadrature)**: Vertical axis, like "North-South" on a map
- Together: Any point in 2D = any signal you can send!

**Why two dimensions?**
- Radio waves have **amplitude** (strength) AND **phase** (timing)
- Phase is like "what part of the wave cycle are you at?"
- Two dimensions let you control BOTH simultaneously

**Real-world analogy - Clock hands**:
- **12 o'clock position**: I = max, Q = 0
- **3 o'clock position**: I = 0, Q = max  
- **6 o'clock position**: I = -max, Q = 0
- **9 o'clock position**: I = 0, Q = -max
- Any angle = unique IQ coordinate!

**Why it's everywhere**:
- **Software Defined Radio (SDR)**: All processing uses IQ data
- **Digital audio**: Left/Right channels → I/Q channels
- **Your phone**: Baseband chip outputs IQ samples, radio transmits them
- **WiFi chips**: Process IQ data to decode constellations

**The magic trick**:
- One wire carries I signal, another carries Q signal
- At transmitter: Combine using 90° phase-shifted carriers
- At receiver: Split using 90° phase-shifted carriers
- Result: Two independent data channels on same frequency!

**Fun fact**: IQ representation is why "quadrature" modulation (QPSK, QAM) is so efficient—you're using two perpendicular dimensions, doubling capacity compared to just varying amplitude!

---

Each QPSK symbol is represented as a point in 2D space with two components:

- **I (In-phase)**: The horizontal component (real part)
- **Q (Quadrature)**: The vertical component (imaginary part)

## What is I/Q?

**I** and **Q** are the two orthogonal (perpendicular) components of a modulated signal. They're called:
- **In-phase** (I): Aligned with the carrier wave
- **Quadrature** (Q): 90° out of phase with the carrier wave

## Mathematical Representation

Any modulated signal can be expressed as:

```
Signal(t) = I(t)·cos(2πft) - Q(t)·sin(2πft)
```

Where:
- `f` is the carrier frequency
- `I(t)` is the in-phase amplitude
- `Q(t)` is the quadrature amplitude

## Complex Number Notation

In DSP, we often use complex number notation:

```
Symbol = I + jQ
```

Where `j` is the imaginary unit (√-1)

### QPSK Example

For normalized QPSK symbols:

| Bits | I | Q | Complex | Phase |
|------|---|---|---------|-------|
| 00 | -0.707 | +0.707 | -0.707+j0.707 | 135° |
| 01 | +0.707 | +0.707 | +0.707+j0.707 | 45° |
| 11 | -0.707 | -0.707 | -0.707-j0.707 | 225° |
| 10 | +0.707 | -0.707 | +0.707-j0.707 | 315° |

## Why Use I/Q?

1. **Efficient Processing**: Easy to implement in digital hardware/software
2. **Phase and Amplitude**: Naturally represents both characteristics
3. **Orthogonality**: I and Q don't interfere with each other
4. **Standard Format**: Universal in modern communications

## I/Q in Chimera

Chimera's constellation diagrams plot:
- **X-axis (horizontal)**: I component
- **Y-axis (vertical)**: Q component

When you see a dot at position (I=0.707, Q=0.707), that represents the QPSK symbol for bits `01`.

## Adding Noise

When [[Additive White Gaussian Noise (AWGN)]] is added:

```
I_received = I_transmitted + N_I
Q_received = Q_transmitted + N_Q
```

Where `N_I` and `N_Q` are independent Gaussian random variables. This is why you see **clouds** instead of **points** in the RX constellation!

## See Also

- [[QPSK Modulation]] - How bits map to I/Q values
- [[Constellation Diagrams]] - Visualizing I/Q space
- [[Additive White Gaussian Noise (AWGN)]] - How noise affects I/Q
