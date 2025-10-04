# Spread Spectrum (DSSS/FHSS)

## 🎪 For Non-Technical Readers

**Spread spectrum is like whispering your secret across 100 different frequencies at once—eavesdroppers hear random noise, but your friend with the right "key" combines all pieces to hear your message perfectly!**

**The counterintuitive idea**:
- **Normal radio**: Use narrow frequency band → Efficient but vulnerable
- **Spread spectrum**: Spread signal across WIDE band → "Wastes" bandwidth but gains superpowers!

**Three magic superpowers**:

**1. Stealth** 🥷 (Military origin):
- Signal spread so thin it looks like background noise
- Enemy can't detect you're transmitting
- Can't jam what you can't find!

**2. Anti-jamming** 🛡️:
- Jammer tries to block you → You're on 100 frequencies
- They can only jam a few → Other 95 get through!
- Your receiver combines the survivors → Message intact

**3. Many users share spectrum** 👥 (CDMA):
- Everyone transmits at same time, same band
- Each person has unique spreading code
- Your phone filters out everyone else's signal
- Like 20 conversations in one room, different languages!

**Two main flavors**:

**DSSS (Direct Sequence Spread Spectrum)** - Used in GPS, CDMA:

**Simple analogy - Speaking in code**:
- Want to send: "HI" (2 letters)
- DSSS: Replace each letter with 100-letter code word
- "H" → "AJFKELQPZMVBX..." (100 random letters)
- "I" → "QZMVPLAJFKEBX..." (different 100 letters)
- Transmit: 200 letters instead of 2!
- **Your friend knows the code** → decodes back to "HI"
- **Eavesdropper hears**: Random gibberish

**Real GPS example**:
- GPS sends 1 bit
- DSSS multiplies by 1023-chip code (C/A code)
- 1 bit → 1023 chips = 1000× wider bandwidth!
- Your GPS receiver knows the code → extracts bit
- Jammer tries to interfere → Processing gain overcomes it

**FHSS (Frequency Hopping Spread Spectrum)** - Used in Bluetooth, military:

**Simple analogy - Hopscotch communication**:
- Instead of one frequency, hop between 100 frequencies
- Pattern: Freq 23 → 67 → 12 → 89 → 45... (changes 1000× per second!)
- **Your friend knows hop pattern** → follows you, receives message
- **Eavesdropper**: By time they tune to Freq 23, you're on 89! 
- **Jammer**: Can't jam all frequencies at once

**Real Bluetooth example**:
- 79 channels between 2.4-2.48 GHz
- Hops 1600 times per second (every 625 μs)
- Pseudorandom sequence (appears random, but deterministic)
- Paired devices know hop pattern → stay synchronized
- Interference on one channel? Just skip it!

**Real-world examples you use daily**:

**GPS** 🛰️ (DSSS):
- Satellites transmit at 1575 MHz
- Spread across 2 MHz bandwidth (1000× wider than data rate!)
- **Processing gain**: 30 dB → Works even below noise floor!
- This is why GPS works indoors (barely) and everywhere

**WiFi** 📶 (DSSS for 802.11b):
- 11 Mbps data rate
- Spread across 22 MHz (Barker code or CCK)
- Older WiFi standard, mostly replaced by OFDM

**Bluetooth** 📱 (FHSS):
- Hops 1600 times/second across 79 channels
- **Interference avoidance**: Microwave oven blocks some channels? Skip them!
- **Multiple devices**: Different hop patterns, no collision
- This is why Bluetooth "pairs" (exchanges hop sequence)

**CDMA cell phones** 📞 (DSSS):
- All users transmit simultaneously, same band
- Each user: unique spreading code (Walsh codes)
- Tower separates users by code (not frequency/time!)
- Retired in US (Verizon), still used in some countries

**Military radios** 🎖️ (Both DSSS & FHSS):
- Can't be jammed (spread too wide)
- Can't be detected (looks like noise)
- Can't be intercepted (need secret code)
- Some systems hop 10,000+ times per second!

**The math magic - Processing gain**:

**Shannon says**: Can trade bandwidth for SNR
```
More bandwidth → Can work at lower SNR
```

**Example**:
- Narrowband needs: 10 dB SNR
- Spread 100× wider → Only need: -20 dB SNR!
- **Can receive signals weaker than noise!** 🤯

**Processing gain** = 10 × log₁₀(Spread factor)
- Spread 100× → 20 dB gain
- Spread 1000× → 30 dB gain (GPS)
- This is why GPS works indoors!

**Why "spread" helps against jamming**:

**Scenario**: Enemy jammer
- Jammer power: 100 W across 1 MHz
- Your signal: 1 W spread across 100 MHz
- At each 1 MHz slice: Your signal = 0.01 W
- **Looks like**: Jammer 100× stronger! 😱
- **But**: Your receiver de-spreads → combines 100 slices
- **Result**: Your signal = 1 W, Jammer still 100 W in 1 slice
- **Effective**: 10:1 ratio → You win! ✅

**The coding requirement**:

**Both sides must know**:
- **DSSS**: The spreading code (sequence of chips)
- **FHSS**: The hopping pattern (sequence of frequencies)

**Synchronization critical**:
- Receiver must align perfectly with transmitter
- GPS: Searches for code phase (expensive!)
- Bluetooth: Pairing exchanges hop pattern + timing

**Trade-offs**:

**Advantages**:
- ✅ Interference resistance
- ✅ Anti-jamming
- ✅ Privacy/security
- ✅ Multiple access (CDMA)
- ✅ Multipath resistance
- ✅ Works below noise floor

**Disadvantages**:
- ❌ "Wastes" bandwidth (100-1000× more!)
- ❌ Complex processing (high power consumption)
- ❌ Synchronization required (acquisition time)
- ❌ Near-far problem (CDMA)

**Historical origin - WWII innovation**:

**Hedy Lamarr** 🌟 (yes, the Hollywood actress!):
- Co-invented frequency hopping (1942)
- **Purpose**: Torpedo control immune to jamming
- Patent ignored until 1960s
- **Now**: Foundation of Bluetooth, WiFi, military comms
- She was brilliant engineer + movie star!

**Fun fact**: GPS signals arriving at Earth are about **-130 dBm** (10^-16 watts), which is **20 dB below the noise floor**—weaker than the background noise! Only because of DSSS spread spectrum with 30 dB processing gain can your phone extract the signal. It's like hearing a whisper in a crowded stadium by having 1000 microphones and combining them perfectly!

---

**Spread spectrum** techniques intentionally spread a narrowband signal across a much wider bandwidth. Originally developed for military anti-jamming communications, spread spectrum now powers GPS, Bluetooth, WiFi, CDMA cellular, and countless other systems.

---

## 🎯 Core Philosophy

**Conventional wisdom**: Use minimal bandwidth to maximize spectral efficiency.

**Spread spectrum approach**: **Deliberately waste bandwidth** to gain:
- **Low Probability of Intercept (LPI)**: Signal appears as noise to unintended receivers
- **Low Probability of Detection (LPD)**: Hard to detect presence of transmission
- **Anti-jamming (AJ)**: Processing gain overcomes interference
- **Multiple access**: Many users share same band (CDMA)
- **Multipath resistance**: Wideband signals resolve path delays

**Shannon's insight**: Trading bandwidth for SNR is mathematically sound:
```
C = B · log₂(1 + SNR)

Increase B by 100× → can tolerate SNR 100× lower (20 dB worse!)
```

---

## 📐 Processing Gain

The fundamental metric for spread spectrum performance.

### Definition

```
Processing Gain (Gₚ) = Spread Bandwidth / Information Bandwidth
                      = BW_spread / BW_info
                      = Chip Rate / Bit Rate

In dB:
Gₚ(dB) = 10 · log₁₀(BW_spread / BW_info)
```

### Physical Meaning

**Processing gain = SNR improvement after despreading**:

```
SNR_output = SNR_input + Gₚ(dB)

Example:
- Input SNR: -10 dB (signal 10× weaker than noise!)
- Processing gain: 30 dB (spread by 1000×)
- Output SNR: 20 dB (clean signal)
```

**At receiver**:
- Desired signal: Despread → collapses to narrowband → **gains Gₚ**
- Noise/interference: Remains spread → filtered out → **loses Gₚ**

---

## 🔀 Direct Sequence Spread Spectrum (DSSS)

### How DSSS Works

**Transmitter**:
1. Data bit (slow): ±1 at rate Rᵦ
2. Multiply by **spreading code** (fast): ±1 sequence at rate R꜀ >> Rᵦ
3. Result: Wideband "chips" transmitted

```
Data bit:         1        0        1
                ┌────┐   ┌────┐   ┌────┐
                │    │___│    │___│    │
                
Spreading code: 1 0 1 1 0 1 0 0 1 1 1 0 ...
                ┌┐ ┌┐┌┐ ┌┐ ┌┐┌┐┌┐┌┐ ┌┐  (fast chips)
                
TX signal:      Product of data × code
```

**Key parameters**:
```
- Chip rate (R꜀): e.g., 10 Mcps (chips/second)
- Bit rate (Rᵦ): e.g., 10 kbps
- Spreading factor (SF): R꜀/Rᵦ = 1000
- Processing gain: 10·log₁₀(1000) = 30 dB
```

---

### Spreading Codes (PN Sequences)

**Requirements**:
- **Pseudorandom**: Appears random but deterministic (generated from seed)
- **Autocorrelation**: Sharp peak at zero lag, low elsewhere
- **Cross-correlation**: Low correlation between different codes (for CDMA)
- **Balance**: Equal number of 1s and 0s

**Common codes**:

**1. Maximal-Length Sequences (m-sequences)**:
```
Generated by Linear Feedback Shift Register (LFSR):

   ┌─────┐  ┌─────┐  ┌─────┐
   │ D₁  │→│ D₂  │→│ D₃  │→ Output
   └──↑──┘  └─────┘  └──↑──┘
      │                  │
      └───────⊕──────────┘
         (XOR feedback)

Properties:
- Period: 2ⁿ - 1 (for n-stage LFSR)
- Example: 7-stage → period = 127 chips
- Good autocorrelation, poor cross-correlation
```

**2. Gold Codes**:
```
XOR two m-sequences with specific phase shifts

Gold Code = m-seq₁ ⊕ m-seq₂

Properties:
- Set of 2ⁿ + 1 codes (from n-stage LFSR)
- Good autocorrelation AND cross-correlation
- Used in GPS C/A code (1023-chip Gold codes)
```

**3. Walsh-Hadamard Codes**:
```
Orthogonal codes (zero cross-correlation):

H₁ = [1]

H₂ = [1  1]     H₄ = [1  1  1  1]
     [1 -1]          [1 -1  1 -1]
                     [1  1 -1 -1]
                     [1 -1 -1  1]

Properties:
- Perfectly orthogonal (theoretical CDMA)
- Length = powers of 2
- Used in IS-95 CDMA (64-chip Walsh)
```

---

### DSSS Receiver (Matched Filter)

**Despreading**:
```
RX signal = (Data × Code) + Noise
         ↓ Multiply by same code
         = Data × Code × Code + Noise × Code
         = Data × 1 + Noise × Code
         = Data + (Noise spread across bandwidth)
         ↓ Integrate over chip period
         = Data (narrowband) + Filtered noise (reduced by Gₚ)
```

**Correlation receiver**:
```python
def dsss_receiver(rx_signal, spreading_code):
    """
    Despread DSSS signal.
    
    Args:
        rx_signal: Received wideband signal (sampled at chip rate)
        spreading_code: Known spreading code (±1)
    
    Returns:
        Despread data bits
    """
    # Multiply by local replica of code
    despread = rx_signal * spreading_code
    
    # Integrate over spreading period (matched filter)
    N_chips = len(spreading_code)
    data_bits = []
    
    for i in range(0, len(despread), N_chips):
        bit_energy = np.sum(despread[i:i+N_chips])
        data_bits.append(1 if bit_energy > 0 else 0)
    
    return np.array(data_bits)
```

---

### DSSS Example: GPS C/A Code

**GPS L1 C/A (Coarse/Acquisition)**:
```
Carrier: 1575.42 MHz
Chip rate: 1.023 Mcps
Code: 1023-chip Gold code (repeats every 1 ms)
Bit rate: 50 bps (navigation message)
Processing gain: 10·log₁₀(1.023 MHz / 50 Hz) = 43 dB

Each satellite: Unique Gold code
- SV 1: PRN 1 (specific Gold code)
- SV 2: PRN 2 (different Gold code)
- ... 32 satellites

Reception:
- Signal at antenna: -130 dBm (20 dB below noise floor!)
- After despreading: -87 dBm (above noise)
- C/N₀ (carrier-to-noise density): 45 dB-Hz (typical)
```

**Code generation** (PRN 1 example):
```
G1 LFSR: taps [3, 10] (1-indexed)
G2 LFSR: taps [2, 3, 6, 8, 9, 10]
PRN 1 = G1 ⊕ (G2 delayed by specific phase)

Result: 1023-chip sequence, e.g.:
1 1 0 1 0 1 1 0 0 0 1 0 1 0 1 ... (repeats)
```

---

### CDMA (Code Division Multiple Access)

**Concept**: Multiple users transmit simultaneously on same frequency, distinguished by spreading codes.

```
User 1: Data₁ × Code₁ ────┐
User 2: Data₂ × Code₂ ────┤→ Σ → Channel → Receiver
User 3: Data₃ × Code₃ ────┘

Receiver (for User 1):
RX × Code₁ → Integrates → Extracts Data₁
           (Code₂, Code₃ appear as noise due to low cross-correlation)
```

**Capacity (IS-95 CDMA)**:
```
N_users ≈ (Gₚ / (Eb/N₀)_required) · (1 + F)

where:
- Gₚ = processing gain
- (Eb/N₀)_required = SNR needed for target BER
- F = frequency reuse factor (typically 0.6-0.85)

Example:
- Gₚ = 21 dB (126)
- (Eb/N₀)_required = 7 dB (5) for 1% BER
- F = 0.67
- N_users ≈ 126 / 5 × 1.67 ≈ 42 users per cell
```

---

## 🎵 Frequency Hopping Spread Spectrum (FHSS)

### How FHSS Works

**Transmitter**: Rapidly switches carrier frequency according to a pseudorandom pattern.

```
Time →
Frequency
    ↑
f₅  |     █           █
f₄  |         █   █
f₃  | █           █       █
f₂  |     █   █       █
f₁  |   █                   █
    └─────────────────────────→

Each block = "hop" (dwell time)
Pattern known only to TX/RX
```

**Key parameters**:
```
- Hop rate: e.g., 1600 hops/second (Bluetooth)
- Dwell time: Time per frequency (e.g., 625 μs)
- Hop set: Available frequencies (e.g., 79 channels)
- Hop sequence: Pseudorandom pattern
```

---

### FHSS Variants

**1. Fast Hopping (FH)**:
```
Multiple hops per data symbol

Example:
Symbol duration: 10 ms
Hop duration: 1 ms
→ 10 hops per symbol

Advantage: Diversity against narrowband interference
```

**2. Slow Hopping (SH)**:
```
Multiple symbols per hop

Example:
Hop duration: 10 ms
Symbol duration: 1 ms
→ 10 symbols per hop

Advantage: Simpler synchronization
```

---

### FHSS Example: Bluetooth

**Bluetooth Classic (BR/EDR)**:
```
Frequency: 2.4 GHz ISM band
Hop set: 79 channels (1 MHz spacing, 2.402-2.480 GHz)
Hop rate: 1600 hops/second
Dwell time: 625 μs
Modulation: GFSK (Gaussian FSK)
Data rate: 1 Mbps (BR), 2-3 Mbps (EDR)

Hopping pattern:
- Derived from master device address + clock
- Pseudorandom over all 79 channels
- Adaptive Frequency Hopping (AFH): Avoids WiFi channels

Multi-user:
- Piconet: Master + up to 7 slaves
- Each piconet: Unique hopping pattern
- Scatternets: Overlapping piconets
```

---

### FHSS vs. DSSS

| Aspect | DSSS | FHSS |
|--------|------|------|
| **Spreading Method** | Multiply by fast code | Hop carrier frequency |
| **Bandwidth** | Continuous wide | Instantaneous narrow, wide over time |
| **Processing Gain** | Chip rate / Bit rate | Hop set size |
| **Anti-Jam** | High (averages interference) | Moderate (avoids interference) |
| **Multipath** | Good (path resolution) | Poor (frequency-flat per hop) |
| **Complexity** | Moderate (correlator) | Low (frequency synthesizer) |
| **Multiple Access** | CDMA (code separation) | FDMA/TDMA (time/freq separation) |
| **Near-Far Problem** | Severe (power control needed) | Minimal |
| **Standards** | GPS, CDMA, WiFi DSSS (legacy) | Bluetooth, military TRANSEC |

---

## 🛡️ Military Applications

Spread spectrum was **born for military use** (1940s-1950s).

### Low Probability of Intercept (LPI)

**Goal**: Enemy cannot detect transmission.

```
Signal power spread across wide bandwidth:
Power Spectral Density (PSD) ∝ Power / Bandwidth

Example:
- Narrowband: 1 W / 10 kHz = 100 mW/kHz (easily detected)
- DSSS spread: 1 W / 10 MHz = 0.1 mW/kHz (below noise floor!)

Detection threshold:
PSD_signal < PSD_noise → undetectable to wideband receiver
```

**Processing gain provides concealment**:
```
If Gₚ = 30 dB, signal can be 30 dB below noise and still decoded.
Enemy without spreading code sees only noise.
```

---

### Low Probability of Detection (LPD)

**Goal**: Enemy cannot tell if transmission is occurring.

**Techniques**:
1. **Below noise floor**: PSD < thermal noise (-174 dBm/Hz)
2. **Randomized patterns**: Avoid periodic structures
3. **Frequency diversity**: FHSS over wide bands
4. **Short bursts**: Minimize dwell time

**Example: MILSTAR FHSS**:
```
- X-band (7-8 GHz) satellite uplink
- Hops across 1 GHz bandwidth
- Dwell time: <1 ms
- Total detection probability: <1% for enemy wideband receiver
```

---

### Anti-Jamming (AJ)

**Jamming scenarios**:

**1. Barrage Jamming** (wideband noise):
```
Jammer spreads power across entire band → processing gain helps:

J/S = Jammer Power / Signal Power (at antenna)
J/S_despread = (J/S) / Gₚ (after despreading)

Example:
- J/S = 20 dB (jammer 100× stronger)
- Gₚ = 30 dB
- J/S_despread = 20 - 30 = -10 dB (signal 10× stronger than jammer!)

Margin = Gₚ - J/S - (Eb/N₀)_required
        = 30 - 20 - 7 = 3 dB (link survives)
```

**2. Partial-Band Jamming**:
```
Jammer concentrates on fraction ρ of bandwidth:
PSD_jammer increases by 1/ρ → FHSS excels (hops to unjammed frequencies)

DSSS: Averages jammer over full bandwidth → degradation proportional to ρ
FHSS: Hops avoid jammer (1-ρ) fraction of time → better performance
```

**3. Follower Jamming** (FHSS target):
```
Jammer tries to detect hop and jam that frequency.
Countermeasure: Fast hopping (enemy can't track) + adaptive hopping
```

---

### GPS M-Code (Military)

**GPS L1/L2 M-Code** (post-2005):
```
Modulation: BOC(10,5) - Binary Offset Carrier
Chip rate: 5.115 Mcps (vs. 1.023 Mcps for C/A)
Processing gain: ~50 dB (vs. 43 dB for C/A)
Security: Encrypted, authenticated (NSA-controlled keys)

BOC modulation:
- Square wave subcarrier at 10.23 MHz (double-sideband)
- Spread by 5.115 Mcps code
- Split-spectrum: Power at ±10.23 MHz from carrier
- Advantage: Minimal interference with C/A code (different spectrum)

Anti-jam margin:
- Jammer-to-signal ratio (J/S): Up to 60 dB tolerated
- Allows GPS reception even under strong jamming
- Critical for precision-guided weapons, military aviation
```

**Example**:
```
Received signal power: -163 dBW (M-code)
Jammer power at receiver: -100 dBW (stronger!)
J/S = -100 - (-163) = 63 dB

Processing gain: 50 dB
Residual J/S after despreading: 63 - 50 = 13 dB
Required (Eb/N₀): 10 dB (M-code robust modulation)

Margin: 50 - 63 - 10 = -23 dB → **LINK FAILS**

Countermeasure: Directional antenna (20 dB gain toward sky, null toward jammer)
Effective J/S: 63 - 20 = 43 dB
Margin: 50 - 43 - 10 = -3 dB → **MARGINAL**

Additional: Adaptive antenna array (CRPA) → 30-40 dB jamming suppression
```

---

### Link 16 (JTIDS - Joint Tactical Information Distribution System)

**NATO/US military data link** for coordinated operations.

**Architecture**:
```
Frequency: 960-1215 MHz (L-band)
Modulation: MSK (Minimum Shift Keying) - constant envelope
Waveform: 51 frequency channels, FHSS + TDMA
Hop rate: 70,000 hops/second (14.3 μs per hop)
Security: Time-varying encryption (KG-40 key generator)
Data rate: 31.6 kbps (voice), 57.6-115.2 kbps (data)

Network:
- TDMA: 128 time slots per 12 seconds
- Each participant assigned slots
- Collision-free multiple access
- Nodes: Aircraft, ships, ground stations
```

**TRANSEC (Transmission Security)**:
```
Hopping pattern: Cryptographically secured
- Changes every 12 seconds (epoch)
- Synchronized via GPS time
- Enemy cannot predict next hop

Result:
- LPI/LPD: Signal appears as brief noise burst
- AJ: Hops faster than jammer can follow
- Covertness: No fixed frequency to monitor
```

**Applications**:
```
- Fighter jets: Share target tracks (Link 16 "picture")
- AWACS: Distribute surveillance data
- Aegis ships: Coordinate air defense
- Ground units: Tactical situational awareness
```

---

## 📡 Commercial Applications

### WiFi 802.11b (DSSS Legacy)

**1999-era WiFi**:
```
Frequency: 2.4 GHz ISM
Chip rate: 11 Mcps (Barker code)
Bit rate: 1-11 Mbps
Spreading: 11-chip Barker sequence (for 1-2 Mbps)
Processing gain: 10.4 dB (11 Mcps / 1 Mbps)

Barker Code (length 11):
+1 -1 +1 +1 -1 +1 +1 +1 -1 -1 -1

Autocorrelation:
Peak: 11 (at zero lag)
Sidelobes: ≤1 (excellent!)

Higher rates (5.5, 11 Mbps):
- CCK (Complementary Code Keying) - not true DSSS
- Phase modulation with 8-chip codes
```

**Obsolescence**:
```
802.11g (2003): OFDM replaces DSSS (higher spectral efficiency)
Legacy DSSS: Still supported for backward compatibility
```

---

### LoRa (Long Range)

**IoT spread spectrum** for low-power wide-area networks.

```
Modulation: Chirp Spread Spectrum (CSS) - not DSSS or FHSS!
Frequencies: 902-928 MHz (US), 863-870 MHz (EU)
Bandwidth: 125, 250, 500 kHz
Spreading Factor: 7-12 (SF7 = 128 chips/symbol, SF12 = 4096)

Processing gain:
Gₚ = 10·log₁₀(SF) = 8.5-10.8 dB (SF7-SF12)

Range: Up to 15 km rural, 2-5 km urban
Data rate: 0.3-50 kbps (inversely proportional to SF)
Power: <100 mW TX, <50 mA RX
Battery life: Years on coin cell
```

**Chirp modulation**:
```
Frequency sweeps linearly over bandwidth:

f(t) = f₀ + (BW/T)·t

Up-chirp:   ↗ (frequency increases)
Down-chirp: ↘ (frequency decreases)

Data encoded by initial frequency offset:
Symbol value = starting frequency of chirp

Advantage: Robust to Doppler, multipath, noise (like DSSS benefits)
```

---

## 🧮 Performance Analysis

### BER in AWGN Channel (DSSS-BPSK)

```
BER = Q(√(2·Eb/N₀))

where:
Eb/N₀ = (Signal Power / Bit Rate) / (Noise Power / Bandwidth)
      = (S/N) · (BW / Bit Rate)
      = (S/N) · Gₚ

Example:
- S/N = -10 dB (0.1 linear) - signal below noise!
- Gₚ = 30 dB (1000 linear)
- Eb/N₀ = 0.1 × 1000 = 100 (20 dB)
- BER = Q(√40) ≈ Q(6.3) ≈ 10⁻¹⁰ (excellent!)
```

---

### Jamming Margin

```
Jamming Margin (dB) = Gₚ - (J/S) - (Eb/N₀)_req - Losses

where:
- Gₚ = processing gain
- J/S = jammer-to-signal ratio
- (Eb/N₀)_req = required SNR for target BER
- Losses = implementation losses (typically 2-3 dB)

Positive margin → link survives jamming
Negative margin → link fails

Example (GPS C/A):
- Gₚ = 43 dB
- J/S = 40 dB (jammer at receiver)
- (Eb/N₀)_req = 10 dB (for BER = 10⁻⁶)
- Losses = 2 dB
- Margin = 43 - 40 - 10 - 2 = -9 dB → **LINK FAILS**

Mitigation: Directional antenna (+20 dB toward satellite)
Effective J/S = 40 - 20 = 20 dB
Margin = 43 - 20 - 10 - 2 = 11 dB → **LINK SURVIVES**
```

---

## 🐍 Python Implementation Examples

### DSSS Transmitter & Receiver

```python
import numpy as np

def generate_pn_sequence(n, seed=1):
    """
    Generate pseudorandom sequence using LFSR (m-sequence).
    
    Args:
        n: Length of sequence (must be 2^k - 1)
        seed: Initial LFSR state
    
    Returns:
        PN sequence (±1)
    """
    # Simple LFSR for demonstration (taps depend on length)
    k = int(np.log2(n + 1))
    lfsr = [1] * k
    sequence = []
    
    for _ in range(n):
        feedback = lfsr[0] ^ lfsr[-1]  # XOR tap
        sequence.append(2 * lfsr[-1] - 1)  # Convert 0/1 to -1/+1
        lfsr = [feedback] + lfsr[:-1]
    
    return np.array(sequence)

def dsss_transmit(data_bits, spreading_code):
    """
    DSSS transmit: spread data bits.
    """
    chips = []
    for bit in data_bits:
        # Convert bit (0/1) to (-1/+1)
        bit_value = 2 * bit - 1
        # Multiply by spreading code
        chips.extend(bit_value * spreading_code)
    return np.array(chips)

def dsss_receive(rx_signal, spreading_code, num_bits):
    """
    DSSS receive: despread to recover data bits.
    """
    chips_per_bit = len(spreading_code)
    data_bits = []
    
    for i in range(num_bits):
        start = i * chips_per_bit
        end = start + chips_per_bit
        
        # Correlate with spreading code
        correlation = np.sum(rx_signal[start:end] * spreading_code)
        
        # Decide bit
        bit = 1 if correlation > 0 else 0
        data_bits.append(bit)
    
    return np.array(data_bits)

# Example usage
spreading_code = generate_pn_sequence(127)  # 127-chip m-sequence
data_bits = np.random.randint(0, 2, 10)

# Transmit
tx_signal = dsss_transmit(data_bits, spreading_code)
print(f"Data: {data_bits}")
print(f"Spreading factor: {len(spreading_code)}")
print(f"TX signal: {len(tx_signal)} chips")

# Add noise (SNR = -10 dB)
signal_power = np.mean(tx_signal**2)
noise_power = signal_power * 10  # 10× more noise than signal
noise = np.sqrt(noise_power) * np.random.randn(len(tx_signal))
rx_signal = tx_signal + noise

snr_db = 10 * np.log10(signal_power / noise_power)
print(f"SNR: {snr_db:.1f} dB")

# Receive
rx_bits = dsss_receive(rx_signal, spreading_code, len(data_bits))
print(f"Recovered: {rx_bits}")
print(f"BER: {np.sum(data_bits != rx_bits) / len(data_bits):.2%}")
```

---

### FHSS Simulator

```python
def fhss_transmit(data_symbols, hop_sequence, carrier_freqs, sample_rate):
    """
    FHSS transmit with frequency hopping.
    
    Args:
        data_symbols: QAM/PSK symbols
        hop_sequence: Sequence of frequency indices
        carrier_freqs: Available carrier frequencies (Hz)
        sample_rate: Sampling rate (Hz)
    
    Returns:
        Transmitted signal
    """
    samples_per_hop = len(data_symbols) // len(hop_sequence)
    t = np.arange(samples_per_hop) / sample_rate
    tx_signal = []
    
    for hop_idx, freq_idx in enumerate(hop_sequence):
        # Get symbols for this hop
        start = hop_idx * samples_per_hop
        end = start + samples_per_hop
        symbols = data_symbols[start:end]
        
        # Modulate on carrier
        carrier_freq = carrier_freqs[freq_idx]
        carrier = np.exp(2j * np.pi * carrier_freq * t)
        
        # Transmit (upconvert baseband to carrier)
        hopped_signal = symbols * carrier[:len(symbols)]
        tx_signal.extend(hopped_signal)
    
    return np.array(tx_signal)

# Example
carrier_freqs = np.arange(2.4e9, 2.48e9, 1e6)  # 2.4 GHz band, 1 MHz spacing
hop_sequence = np.random.randint(0, len(carrier_freqs), 100)  # 100 hops
data_symbols = (2*np.random.randint(0, 2, 1000) - 1) + \
               1j*(2*np.random.randint(0, 2, 1000) - 1)  # QPSK

sample_rate = 10e6  # 10 MHz
tx_signal = fhss_transmit(data_symbols, hop_sequence, carrier_freqs, sample_rate)

print(f"Hopping over {len(carrier_freqs)} frequencies")
print(f"Hops: {len(hop_sequence)}")
print(f"Total samples: {len(tx_signal)}")
```

---

## 🔬 Theoretical Foundations

### Shannon Capacity with Spread Spectrum

For spread bandwidth B_spread and information bandwidth B_info:

```
C_spread = B_spread · log₂(1 + S/(N·B_spread))
C_info = B_info · log₂(1 + S/(N·B_info))

Ratio:
C_spread / C_info = (B_spread/B_info) · log₂(1 + S/(N·B_spread)) 
                                        / log₂(1 + S/(N·B_info))

For low SNR (S << N·B_spread):
log₂(1 + x) ≈ x/ln(2)  for small x

C_spread ≈ C_info (capacity preserved!)

Interpretation: Spreading doesn't reduce capacity if SNR is low.
Military sweet spot: Spread to go below noise floor while maintaining data rate.
```

---

## 🎯 Advantages & Disadvantages

### Advantages

✅ **Anti-jamming**: Processing gain overcomes interference  
✅ **LPI/LPD**: Signal hidden in noise  
✅ **Multiple access**: CDMA allows many users  
✅ **Multipath resistance** (DSSS): Resolves path delays  
✅ **Privacy**: Eavesdropping requires spreading code  
✅ **Coexistence**: Graceful degradation with other systems  

### Disadvantages

❌ **Bandwidth inefficient**: Uses far more spectrum than narrowband  
❌ **Complex synchronization**: Receiver must align code/frequency  
❌ **Near-far problem** (DSSS CDMA): Strong users drown weak ones  
❌ **Processing overhead**: Correlators, frequency synthesizers  
❌ **Power control critical**: Especially for CDMA  

---

## 📚 Further Reading

### Textbooks
- **Simon et al.**, *Spread Spectrum Communications Handbook* - Comprehensive reference (military focus)
- **Peterson, Ziemer, Borth**, *Introduction to Spread Spectrum Communications* - Accessible introduction
- **Viterbi**, *CDMA: Principles of Spread Spectrum Communication* - From inventor of CDMA

### Standards
- **IS-95**: CDMA cellular (Qualcomm standard)
- **GPS ICD-200**: GPS signal specifications (C/A, P(Y), M codes)
- **MIL-STD-188-181**: US military FHSS standard
- **IEEE 802.15.1**: Bluetooth FHSS specifications

### Military Resources
- **Poisel**, *Introduction to Communication Electronic Warfare Systems* - EW perspective
- **Torrieri**, *Principles of Spread-Spectrum Communication Systems* - Modern military focus
- **COMSEC manuals**: Classified (NSA) - operational TRANSEC

### Related Topics
- [[Shannon's Channel Capacity Theorem]] - Theoretical foundation
- [[Military & Covert Communications]] - LPI/LPD systems, GPS M-code
- [[CDMA (coming soon)]] - Code Division Multiple Access
- [[Synchronization (Carrier, Timing, Frame)]] - Code acquisition and tracking
- [[Real-World System Examples]] - GPS, Bluetooth, WiFi, military systems

---

**Summary**: Spread spectrum trades bandwidth for robustness. DSSS multiplies data by fast pseudorandom codes to spread across wide bandwidths, gaining processing gain that enables anti-jamming and covert communications. FHSS rapidly hops between frequencies to avoid interference. Originally military technologies (GPS, Link 16), spread spectrum now underpins consumer wireless (WiFi, Bluetooth) and IoT (LoRa). Processing gain = SNR improvement = anti-jam capability. The lower the PSD, the harder to detect—spread spectrum is the foundation of stealth communications.
