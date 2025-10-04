# Shannon's Channel Capacity Theorem

**Shannon's Channel Capacity Theorem** (1948) is one of the most important results in information theory, establishing the **fundamental limit** of reliable communication over a noisy channel.

---

## 🌟 For Non-Technical Readers

Imagine you're trying to have a conversation in a noisy room. The noisier it gets, the harder it is to understand what the other person is saying. You might speak louder, or talk more slowly and clearly, but there's a limit to how much information you can reliably communicate.

**Shannon's theorem tells us exactly what that limit is.**

### The Big Ideas (In Plain English)

**1. Every communication channel has a speed limit**

Just like highways have speed limits, communication channels (WiFi, radio, fiber optics, etc.) have a maximum rate at which you can send information reliably. This limit depends on two things:
- **How much "space" you have** (bandwidth - like having more lanes on a highway)
- **How noisy it is** (signal-to-noise ratio - like trying to talk in a quiet library vs. a rock concert)

**2. You can always send slower to be more reliable**

If you're below the speed limit, you can add "error correction" (like repeating yourself or spelling things out) to make sure the message gets through perfectly. Shannon proved that with good enough error correction, you can get the error rate as close to zero as you want.

**3. You can never go faster than the limit**

No matter how clever your technology, you cannot exceed this fundamental limit without making mistakes. It's a law of nature, like the speed of light.

### Real-World Example: Your WiFi

Your WiFi router constantly adjusts its speed based on Shannon's theorem:
- **Far from router (weak signal, noisy)**: Sends data slowly but reliably (maybe 10 Mbps)
- **Close to router (strong signal, clean)**: Sends data fast (maybe 500 Mbps)
- **Through thick walls (very noisy)**: Slows way down to maintain connection

The router is always trying to send as fast as possible **without exceeding Shannon's limit**, because going faster would just cause errors and make things worse.

### Why This Matters for Chimera

In Chimera's space communication scenarios, we're often working in **extremely noisy** conditions (think: trying to hear a whisper from across a football field during a thunderstorm). Shannon's theorem tells us:
- We **must** use very strong error correction (LDPC codes)
- We **cannot** send data very fast (maybe 32 bits per second instead of millions)
- But we **can** still communicate reliably if we respect the limits

**The theorem is like a GPS for engineers**: it tells us where the cliff edge is, so we know how close we can safely get.

---

## 🎯 The Main Result

**Channel Capacity (C)**: Maximum rate at which information can be transmitted over a channel with **arbitrarily small error probability**.

**For AWGN channel** (Additive White Gaussian Noise):

```
C = B · log₂(1 + SNR)  bits/second

where:
- B = bandwidth (Hz)
- SNR = signal-to-noise ratio (linear, not dB!)
```

**In terms of [[Energy-Ratios-(Es-N0-and-Eb-N0)]]**:

```
C/B = log₂(1 + (Eb/N₀) · (R/B))

where R = data rate (bps)
```

---

## 💡 Physical Interpretation

### What Shannon Proved

**Two-part theorem**:

1. **Achievability**: If data rate R < C, there exists a coding scheme that allows transmission with **arbitrarily low error probability** (BER → 0)

2. **Converse**: If R > C, **no** coding scheme can achieve reliable communication (BER bounded away from zero)

**Shannon limit**: The boundary R = C is the **hard limit** of communication.

---

### Information-Theoretic Perspective

```
Reliable Communication Regions:

BER
 ↑
10⁰|████████████████  ← R > C (impossible)
   |████████████████
10⁻³|████████████████
   |
10⁻⁶|  Possible with
   |  good codes
10⁻⁹|         ↓
   |         │ R < C (Shannon says codes exist!)
   └─────────┼────────→ R/C
             1.0 (Shannon limit)
```

---

## 📐 Derivations & Examples

### Example 1: WiFi Channel

```
Given:
- Bandwidth: B = 20 MHz
- SNR: 20 dB = 100 (linear)

Capacity:
C = 20×10⁶ · log₂(1 + 100)
  = 20×10⁶ · log₂(101)
  = 20×10⁶ · 6.66
  = 133 Mbps

Conclusion: No matter how clever your coding, you can't reliably transmit more than 133 Mbps on this channel.
```

---

### Example 2: Deep Space Link

```
Given:
- Bandwidth: B = 1 MHz
- SNR: -3 dB = 0.5 (linear!) (very noisy!)

Capacity:
C = 1×10⁶ · log₂(1 + 0.5)
  = 1×10⁶ · log₂(1.5)
  = 1×10⁶ · 0.585
  = 585 kbps

Conclusion: Even at negative SNR, communication is possible (but rate must be low).
```

---

### Example 3: Chimera Simulation

```
Given:
- Bandwidth: B ≈ 20 Hz (QPSK @ 16 sym/s)
- SNR: Variable (-25 to +10 dB in typical presets)

At SNR = -15 dB (0.0316 linear):
C = 20 · log₂(1 + 0.0316)
  = 20 · log₂(1.0316)
  = 20 · 0.045
  = 0.9 bps

Chimera uses: R = 32 bps (16 sym/s × 2 bits/sym)

R/C = 32/0.9 = 35.6 >> 1  ← Operating FAR above capacity!

This is why [[LDPC-Codes]] are essential. Without FEC, BER would be ~50% (random guessing).
With LDPC (rate 1/2), effective R = 16 bps, still R/C = 17.8 (high, but FEC provides ~35 dB coding gain).
```

---

## 📊 Spectral Efficiency

**Spectral efficiency**: η = R/B (bits/s/Hz)

**Shannon limit** on spectral efficiency:

```
η_max = log₂(1 + SNR)

Examples:
- SNR = 1 (0 dB): η_max = 1 bit/s/Hz
- SNR = 3 (4.8 dB): η_max = 2 bits/s/Hz
- SNR = 15 (11.8 dB): η_max = 4 bits/s/Hz
- SNR = 255 (24 dB): η_max = 8 bits/s/Hz
```

**Practical systems** (including overhead):

| System | SNR (dB) | η (bits/s/Hz) | η/η_max |
|--------|----------|---------------|---------|
| GSM | ~10 | 0.5 | ~30% |
| WiFi 802.11n | ~20 | 3-4 | ~60% |
| LTE Advanced | ~25 | 5-6 | ~75% |
| [[LDPC Codes\|LDPC]] (DVB-S2) | Variable | Adaptive | ~90% |

**Modern codes** ([[LDPC-Codes]], Turbo, Polar) achieve **> 95% of Shannon limit**!

---

## ⚡ Power-Limited vs Bandwidth-Limited

### Power-Limited Regime

**Low SNR** (deep space, satellite):
```
C ≈ B · (SNR / ln 2)    (for SNR << 1)

Power efficiency dominates:
- Use low spectral efficiency
- Heavy error correction (rate 1/4, 1/2)
- Example: Voyager (rate 1/6 conv code)
```

---

### Bandwidth-Limited Regime

**High SNR** (fiber optics, mmWave backhaul):
```
C ≈ B · log₂(SNR)       (for SNR >> 1)

Spectral efficiency dominates:
- Use high-order modulation (256-QAM)
- Light error correction (rate 9/10)
- Example: Fiber (SNR > 30 dB, use LDPC rate 0.9)
```

---

## 🔬 Shannon-Hartley Theorem (Historical)

**Original 1948 form**:

```
C = B · log₂(1 + S/N)

where S and N are signal and noise POWER (not ratios)
```

**Assumptions**:
1. AWGN channel (additive white Gaussian noise)
2. Average power constraint on transmitter
3. Unlimited complexity allowed for encoder/decoder
4. Infinite delay acceptable (block codes of arbitrary length)

**What Shannon did NOT provide**: 
- How to construct codes that achieve capacity (left as exercise for humanity!)
- Complexity or delay bounds
- Performance at finite blocklength

---

## 🏗️ Building Towards Capacity

### Historical Progress

| Year | Code Type | Performance (dB from Shannon limit) |
|------|-----------|-------------------------------------|
| 1948 | **Shannon proves limit exists** | - |
| 1950 | Hamming codes | ~7 dB |
| 1960 | Convolutional codes | ~3 dB |
| 1993 | **Turbo codes** | ~0.7 dB (breakthrough!) |
| 1996 | **[[LDPC-Codes]] rediscovered** | ~0.5 dB |
| 2008 | **Polar codes** | ~0.5 dB |
| 2020 | Modern LDPC (DVB-S2X) | ~0.2 dB |

**We're essentially there!** Modern codes are within 0.2 dB of the theoretical limit.

---

## 📉 The BER "Waterfall"

For codes approaching Shannon limit:

```
BER
 ↑
10⁰|
   |
10⁻³|          Steep
   |           cliff!
10⁻⁶|             ╲
   |              ╲
10⁻⁹|               ╲____
   └────────────────────────→ Eb/N0 (dB)
                    Shannon limit
```

**Threshold effect**: Below Shannon-limit Eb/N0, BER drops rapidly (waterfall region).

**Practical threshold**: Where BER = 10⁻⁶ (typical target).

---

## 🎓 Implications for System Design

### 1. Trade-offs

**Shannon says**: You can trade bandwidth for power (and vice versa):

```
C = B · log₂(1 + P/(N₀B))

Increase B → Need less P for same C
Decrease B → Need more P for same C
```

**Spread spectrum** exploits this: Use wide B, tolerate low SNR.

---

### 2. [[Forward-Error-Correction-(FEC)]]

**FEC adds redundancy** (lowers rate):
```
R_code = R_data · (k/n)    (rate k/n code)

Without FEC: Need high Eb/N0 for low BER
With FEC: Lower Eb/N0 sufficient (coding gain!)
```

**Goal**: Design codes with R → C as blocklength → ∞

---

### 3. Adaptive Modulation & Coding

**Modern systems** (LTE, WiFi, DVB-S2):
- Measure SNR dynamically
- Select modulation and code rate to maximize throughput while R < C
- **Always operate near Shannon limit!**

```
High SNR: 256-QAM, rate 9/10 → η ≈ 7 bits/s/Hz
Low SNR: QPSK, rate 1/4 → η ≈ 0.5 bits/s/Hz
```

---

## 🔗 Capacity for Other Channels

### Fading Channels

**Rayleigh fading**:
```
C_fading < C_AWGN  (capacity reduced by fading)

Mitigation:
- Diversity (space, time, frequency)
- Channel coding with interleaving
- Adaptive modulation
```

---

### MIMO Channels

**Multiple antennas** (N_T transmit, N_R receive):
```
C_MIMO ≈ min(N_T, N_R) · B · log₂(1 + SNR)

Capacity grows LINEARLY with min(N_T, N_R)!
```

**This is why 5G uses massive MIMO** (64-256 antennas).

---

### Non-Gaussian Noise

**Shannon's theorem assumes Gaussian noise** (worst case for given variance).

**Other noise types** (impulsive, colored):
- Capacity can be higher or lower
- Requires different coding strategies

---

## 🧮 Mathematical Details

### Information Theory Foundation

**Mutual information** I(X;Y):
```
I(X;Y) = H(Y) - H(Y|X)

where:
- H(Y) = entropy of received signal
- H(Y|X) = entropy of noise
```

**Capacity**: 
```
C = max_p(x) I(X;Y)

Maximization over input distribution p(x).
```

**For AWGN**: Optimal input is Gaussian → Shannon capacity formula.

---

## 🔑 Key Takeaways

1. **Fundamental limit**: C = B · log₂(1 + SNR) is hard barrier
2. **Always achievable**: Codes exist that reach arbitrarily close to C
3. **Trade-offs**: Can exchange bandwidth for power (spread spectrum)
4. **Modern codes**: [[LDPC-Codes]], Turbo, Polar are within 0.2-0.5 dB of limit
5. **System design**: Target R < C, use adaptive coding/modulation
6. **Chimera context**: Extreme low SNR (-25 dB) requires very low rate or powerful FEC

---

## 🔗 See Also

- [[Forward-Error-Correction-(FEC)]] - How to approach Shannon limit
- [[LDPC-Codes]] - Modern capacity-approaching codes
- [[Bit-Error-Rate-(BER)]] - Performance metric
- [[Energy-Ratios-(Es-N0-and-Eb-N0)]] - Related to SNR and capacity
- [[Signal-to-Noise-Ratio-(SNR)]] - Key parameter in capacity formula

---

## 📚 References

### Primary Sources

1. **Shannon, C.E.** (1948) "A Mathematical Theory of Communication" *Bell Syst. Tech. J.* 27, 379-423, 623-656
   - **The foundational paper** - still highly readable!

2. **Shannon, C.E.** (1949) "Communication in the Presence of Noise" *Proc. IRE* 37, 10-21
   - Shannon-Hartley theorem for AWGN

### Textbooks

3. **Cover, T.M. & Thomas, J.A.** (2006) *Elements of Information Theory* 2nd ed. (Wiley)
4. **MacKay, D.J.C.** (2003) *Information Theory, Inference, and Learning Algorithms* (Cambridge UP) - Free online!
5. **Gallager, R.G.** (1968) *Information Theory and Reliable Communication* (Wiley)

### Historical Context

6. **Verdú, S.** (1998) "Fifty Years of Shannon Theory" *IEEE Trans. Info. Theory* 44, 2057-2078
