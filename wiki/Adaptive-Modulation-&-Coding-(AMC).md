# Adaptive Modulation & Coding (AMC)

**Adaptive Modulation and Coding (AMC)** dynamically adjusts transmission parameters (modulation order, code rate, bandwidth) based on real-time channel conditions to maximize throughput while maintaining target error rates. AMC is fundamental to modern wireless standards (LTE, 5G NR, WiFi 6/7) and enables systems to track [[Shannon's-Channel-Capacity-Theorem|Shannon capacity]] in time-varying channels.

**Core principle**: Match data rate to instantaneous channel quality—use aggressive modulation when channel is good, fall back to robust modulation when channel degrades.

---

## 🎯 The AMC Concept

### Without AMC (Fixed Modulation)

```
Fixed 64-QAM, Rate-1/2 FEC:
→ Data rate: Constant (e.g., 50 Mbps)
→ High SNR: Wasted capacity (could use 256-QAM)
→ Low SNR: High BER, retransmissions, failures

Result: Suboptimal throughput, especially in fading channels
```

---

### With AMC

```
Adapt modulation + coding to channel:

Good channel (SNR = 30 dB):
→ 256-QAM, Rate-5/6 → 100 Mbps, BER = 10⁻⁶ ✅

Moderate channel (SNR = 20 dB):
→ 64-QAM, Rate-3/4 → 60 Mbps, BER = 10⁻⁶ ✅

Poor channel (SNR = 10 dB):
→ QPSK, Rate-1/2 → 10 Mbps, BER = 10⁻⁶ ✅

Faded channel (SNR = 5 dB):
→ BPSK, Rate-1/3 → 3 Mbps, BER = 10⁻⁶ ✅

Result: Maximize throughput while maintaining quality
```

---

## 📐 Link Adaptation Framework

### Channel State Information (CSI)

**CSI acquisition**:
```
Downlink (BS → UE):
1. BS transmits pilot/reference signals
2. UE measures channel (amplitude, phase per subcarrier)
3. UE reports CSI feedback to BS
4. BS selects MCS (Modulation and Coding Scheme)

Uplink (UE → BS):
1. UE transmits sounding reference signal (SRS)
2. BS measures channel directly
3. BS selects MCS (no feedback needed if TDD reciprocity)
```

**CSI feedback types**:
```
Full CSI:
- H matrix (nT × nR complex gains per subcarrier)
- High overhead (bits ∝ nT × nR × N_subcarriers)
- Used: Massive MIMO (TDD reciprocity → no feedback)

Quantized CSI:
- Codebook-based: Index to predefined precoding matrices
- CQI (Channel Quality Indicator): Scalar metric
- Low overhead
- Used: LTE, 5G NR FDD
```

---

### Channel Quality Indicator (CQI)

**CQI definition**:
```
CQI = f(SINR, interference, fading statistics)

Mapping:
CQI → (Modulation, Code Rate) → Spectral Efficiency

Example (LTE):
CQI 1:  QPSK, Rate-1/8  → 0.15 bits/s/Hz (SINR < 0 dB)
CQI 5:  QPSK, Rate-1/2  → 1.0 bits/s/Hz (SINR ≈ 5 dB)
CQI 10: 64-QAM, Rate-3/4 → 4.5 bits/s/Hz (SINR ≈ 20 dB)
CQI 15: 256-QAM, Rate-7/8 → 7.0 bits/s/Hz (SINR ≈ 30 dB)

Target: <10% BLER (Block Error Rate) after first transmission
```

**CQI calculation**:
```
Instantaneous SINR per subcarrier:
SINRₖ = |Hₖ|² · P / (N₀ + I)

where:
- Hₖ = channel gain on subcarrier k
- P = transmit power
- N₀ = noise power
- I = interference power

Effective SINR (over all subcarriers):
SINR_eff = f(SINR₁, SINR₂, ..., SINR_N)

Methods:
1. Mean SINR: SINR_eff = mean(SINRₖ)
2. EESM (Exponential Effective SINR Mapping):
   SINR_eff = -β·ln(mean(exp(-SINRₖ/β)))
3. MIESM (Mutual Information ESM):
   SINR_eff based on mutual information

CQI = Quantize(SINR_eff)
```

---

### Modulation and Coding Schemes (MCS)

**MCS Table (LTE example)**:

| MCS Index | Modulation | Code Rate | Spectral Eff. (bits/s/Hz) | Required SINR (dB) |
|-----------|------------|-----------|---------------------------|--------------------|
| 0 | QPSK | 0.076 | 0.15 | -6 |
| 5 | QPSK | 0.439 | 0.88 | 2 |
| 10 | 16-QAM | 0.478 | 1.91 | 10 |
| 15 | 64-QAM | 0.553 | 3.32 | 18 |
| 20 | 64-QAM | 0.750 | 4.50 | 24 |
| 28 | 256-QAM | 0.926 | 7.41 | 32 |

**Selection algorithm**:
```
Given CQI (estimated SINR):
1. Find highest MCS where SINR ≥ Required_SINR
2. Verify: Predicted_BLER < 10%
3. Transmit with selected MCS

If BLER > 10% (ACK/NACK feedback):
→ Fall back to lower MCS (more robust)

If BLER < 1% (excellent channel):
→ Attempt higher MCS (increase throughput)
```

---

## 🔄 Hybrid ARQ (HARQ)

**Automatic Repeat Request** with **Forward Error Correction**—retransmissions carry additional redundancy.

### HARQ Types

**Type I - Chase Combining**:
```
First transmission: Original codeword
Retransmission(s): Same codeword (identical)

Receiver: Combine multiple copies (soft combining)
→ Effective SNR increases with each retransmission

Example:
- TX 1: SNR = 5 dB → NACK (failed)
- TX 2: SNR = 5 dB → Combined SNR = 8 dB → ACK ✅

Advantage: Simple
Disadvantage: No incremental redundancy
```

**Type II/III - Incremental Redundancy (IR)**:
```
First transmission: High code rate (less redundancy)
Retransmission 1: Additional parity bits (lower effective rate)
Retransmission 2: Even more parity (lowest rate)

Example (Rate-compatible punctured code):
- TX 1: Rate-3/4 (fast, fragile) → NACK
- TX 2: Rate-2/3 (add parity) → Combined rate-1/2 → NACK
- TX 3: Rate-1/2 (add more parity) → Combined rate-1/3 → ACK ✅

Advantage: Adaptive coding without re-encoding
Disadvantage: More complex receiver (soft buffer management)
```

---

### HARQ in LTE/5G

**LTE HARQ process**:
```
8 parallel HARQ processes (downlink), 8 (uplink)
RTT (Round-Trip Time): 8 ms
Max retransmissions: 4 (configurable)

Timeline:
t = 0 ms:    TX initial transmission (Process 0)
t = 4 ms:    RX decodes, sends ACK/NACK
t = 8 ms:    If NACK, retransmit (Process 0)
             Meanwhile, Process 1-7 active (pipelined)

Result: 8 simultaneous processes → continuous transmission
```

**5G NR HARQ**:
```
16+ parallel HARQ processes (flexible)
RTT: 2-8 ms (depends on numerology)
Adaptive retransmission:
- Same MCS (Chase combining)
- Different MCS (adapt to channel change)
- Different RV (Redundancy Version) for IR
```

---

## 📊 AMC Performance Analysis

### Shannon-Capacity Tracking

**Ideal AMC** approaches Shannon capacity:
```
C(SNR) = B · log₂(1 + SNR)

Without AMC (fixed QPSK, rate-1/2):
R_fixed = B · 1 bits/s/Hz (for all SNR)
Efficiency: R_fixed / C(SNR) = low at high SNR

With AMC:
R_AMC(SNR) ≈ C(SNR) - Δ

where Δ = implementation gap (typically 2-5 dB from Shannon)

Throughput gain: 3-5× in typical fading scenarios
```

---

### Throughput in Fading Channels

**Rayleigh fading channel** (urban/indoor):
```
Instantaneous SNR: γ (exponentially distributed)
Average SNR: γ̄

Outage probability:
P_out(R) = P(C(γ) < R) = 1 - exp(-R / (γ̄ · B))

Without AMC (fixed rate R):
- Outage when γ < γ_threshold → complete failure
- Average throughput: R · (1 - P_out)

With AMC:
- Adapt R = C(γ) continuously
- No outage (always some rate achievable)
- Average throughput: E[C(γ)] = ∫₀^∞ C(γ) · p(γ) dγ

Ergodic capacity:
C_ergodic = B · E[log₂(1 + γ)]

For Rayleigh: C_ergodic ≈ B · log₂(e · γ̄) (high SNR)
```

**Numerical example**:
```
Channel: Rayleigh fading, γ̄ = 20 dB, B = 20 MHz

Fixed 64-QAM (rate-3/4):
- Required SNR: 18 dB
- Outage: P(γ < 18 dB) = 37%
- Average throughput: 4.5 × 20 MHz × 0.63 = 57 Mbps

AMC (QPSK to 256-QAM):
- Always adapts to channel
- Average throughput: ≈ 100 Mbps

Gain: 1.75× throughput improvement
```

---

## 🌐 AMC in Standards

### LTE Adaptive Modulation

**Downlink (eNodeB → UE)**:
```
CQI reporting:
- Periodicity: 5-10 ms (semi-static)
- Wideband or subband (per RB - Resource Block)
- UE measures RSRP, RSRQ, SINR → computes CQI

MCS selection:
- eNodeB scheduler receives CQI
- Selects MCS per UE per RB
- Goals: Maximize cell throughput, maintain fairness

Resource allocation:
- Time-frequency (OFDMA)
- 1 RB = 12 subcarriers × 1 slot (0.5 ms)
- Assign high MCS to users with good CQI
```

**Uplink (UE → eNodeB)**:
```
UE transmits SRS (Sounding Reference Signal)
eNodeB measures channel directly (TDD reciprocity helps)
eNodeB commands MCS via PDCCH (Physical Downlink Control Channel)

Uplink challenges:
- Limited UE power → coverage-limited
- Lower MCS typical (vs. downlink)
```

---

### 5G NR Ultra-Lean Design

**Dynamic adaptation**:
```
Ultra-flexible frame structure:
- Slot duration: 0.125-1 ms (depends on numerology)
- Mini-slots: <1 ms (ultra-low latency)
- HARQ feedback: 2-4 slots (faster than LTE)

Beam management:
- Massive MIMO: Beamformed transmissions
- CSI-RS: Beam-specific channel measurement
- Adapt MCS per beam (spatial dimension)

Grant-free transmission (URLLC):
- Pre-configured MCS (no dynamic CQI)
- Used for ultra-reliable, low-latency (factory automation)
```

**Massive MIMO adaptation**:
```
Per-user MCS:
- User 1 (cell center, high SINR): 256-QAM, rate-5/6
- User 2 (cell edge, low SINR): QPSK, rate-1/3
- Simultaneous (MU-MIMO) on same resource blocks

Spectral efficiency:
Sum rate = Σᵢ R_i (bits/s/Hz per user)
         = 7 + 1 = 8 bits/s/Hz (vs. 4 for single-user)
```

---

### WiFi 6/7 (802.11ax/be)

**Rate adaptation**:
```
WiFi metrics:
- RSSI (Received Signal Strength Indicator)
- PER (Packet Error Rate)
- Retry count

MCS selection:
- Minstrel / SampleRate algorithms (open-source)
- Proprietary vendor algorithms (Cisco, Qualcomm)
- Test higher MCS occasionally (probing)

Spatial stream adaptation:
- 1 stream: Long range, reliable
- 4 streams: Short range, high throughput
- Adapt based on distance, interference

Example (WiFi 6, 80 MHz):
- Close (1 m): 4 streams, 1024-QAM, rate-5/6 → 1.2 Gbps
- Medium (10 m): 2 streams, 256-QAM, rate-3/4 → 600 Mbps
- Far (50 m): 1 stream, QPSK, rate-1/2 → 30 Mbps
```

---

## 🔬 Advanced AMC Techniques

### Outer-Loop Link Adaptation (OLLA)

**Motivation**: CQI can be inaccurate (channel estimation errors, feedback delay).

**OLLA principle**:
```
Adjust MCS based on ACK/NACK history, not just CQI.

Algorithm:
1. Start with MCS based on CQI
2. If NACK: Decrease MCS (Δ_down = 1 dB)
3. If ACK: Increase MCS (Δ_up = 0.01 dB)

Asymmetric adjustment:
- Fast decrease (avoid errors)
- Slow increase (test cautiously)

Result: Converges to optimal MCS despite CQI errors
```

**Implementation**:
```python
def olla_adaptation(cqi, ack_history, target_bler=0.1):
    """
    Outer-loop link adaptation.
    
    Args:
        cqi: Reported channel quality indicator
        ack_history: Recent ACK/NACK outcomes
        target_bler: Target block error rate
    
    Returns:
        Adjusted MCS index
    """
    mcs_base = cqi_to_mcs(cqi)
    
    # Compute recent BLER
    recent_bler = 1 - np.mean(ack_history)
    
    # Offset adjustment
    if recent_bler > target_bler:
        offset_db = -1.0  # More conservative
    elif recent_bler < target_bler / 2:
        offset_db = +0.5  # More aggressive
    else:
        offset_db = 0.0  # Stay
    
    # Adjust MCS
    mcs_adjusted = mcs_base + int(offset_db / 2)  # ~2 dB per MCS
    mcs_adjusted = np.clip(mcs_adjusted, 0, 28)
    
    return mcs_adjusted
```

---

### Cross-Layer Optimization

**Joint optimization** of PHY (MCS) and MAC (scheduling):

**Proportional Fair Scheduler**:
```
Maximize: Σᵢ log(R_i)  (sum log throughput)

User priority:
Priority_i = R_instantaneous_i / R_average_i

where:
- R_instantaneous: Rate achievable now (AMC-selected MCS)
- R_average: Long-term average throughput

Result:
- User with good channel gets high MCS → high R_instantaneous
- If already has high R_average, priority decreases
- Balances throughput and fairness
```

**Buffer-aware AMC**:
```
If buffer almost empty:
→ Use lower MCS (reliable, avoid stalls)

If buffer full:
→ Use higher MCS (aggressive, drain buffer)

Delay-sensitive (VoIP):
→ Conservative MCS (avoid retransmissions)

Throughput-oriented (file download):
→ Aggressive MCS (maximize rate, tolerate retries)
```

---

### Predictive AMC

**Anticipate channel changes** before they occur.

**Method 1 - Doppler-based prediction**:
```
High mobility (vehicular):
- Channel changes rapidly (coherence time ~10 ms)
- CQI feedback outdated by RTT (8 ms)

Prediction:
1. Estimate Doppler shift (f_D = v/λ)
2. Predict channel evolution: H(t + Δt) = f(H(t), f_D)
3. Select MCS for predicted channel

Autoregressive model:
H[n+1] = a₁·H[n] + a₂·H[n-1] + ... + noise

Wiener filter / Kalman filter for prediction
```

**Method 2 - Machine learning**:
```
Train neural network:
- Input: [H[n], H[n-1], ..., H[n-k], velocity, location]
- Output: H[n+1] (predicted channel)

Online learning:
- Update weights based on prediction error
- Adapt to user-specific channel patterns

Benefit: 2-3 dB gain in high-mobility scenarios
```

---

## 🧮 Python Implementation Example

### AMC Simulator

```python
import numpy as np

def generate_fading_channel(n_samples, coherence_time=100):
    """
    Generate Rayleigh fading channel (Clarke's model).
    
    Args:
        n_samples: Number of time samples
        coherence_time: Channel coherence (samples)
    
    Returns:
        Channel gains (linear)
    """
    # Generate complex Gaussian samples
    h_i = np.random.randn(n_samples)
    h_q = np.random.randn(n_samples)
    h = (h_i + 1j*h_q) / np.sqrt(2)
    
    # Low-pass filter (coherence time)
    from scipy.signal import butter, lfilter
    b, a = butter(3, 1/coherence_time)
    h = lfilter(b, a, h)
    
    # Normalize to unit average power
    h /= np.sqrt(np.mean(np.abs(h)**2))
    
    return h

def snr_to_cqi(snr_db):
    """
    Map SNR to CQI index (0-15).
    """
    cqi_table = [
        (-6, 0), (0, 5), (5, 8), (10, 11), 
        (15, 13), (20, 15), (25, 15)
    ]
    for snr_thresh, cqi in cqi_table:
        if snr_db < snr_thresh:
            return max(0, cqi - 1)
    return 15

def cqi_to_mcs(cqi):
    """
    Map CQI to MCS parameters.
    
    Returns:
        (modulation_order, code_rate, spectral_efficiency)
    """
    mcs_table = [
        (2, 0.08, 0.15),   # CQI 0-1: QPSK
        (2, 0.44, 0.88),   # CQI 5: QPSK
        (4, 0.48, 1.91),   # CQI 10: 16-QAM
        (6, 0.55, 3.32),   # CQI 15: 64-QAM
        (8, 0.93, 7.41),   # CQI 15+: 256-QAM
    ]
    idx = min(cqi // 4, len(mcs_table) - 1)
    return mcs_table[idx]

def compute_bler(snr_db, mcs_params):
    """
    Compute Block Error Rate for given SNR and MCS.
    
    Uses Shannon bound approximation.
    """
    mod_order, code_rate, spec_eff = mcs_params
    
    # Required SNR for target BER = 10^-6
    required_snr = {
        2: 9.6,   # QPSK
        4: 16.5,  # 16-QAM
        6: 22.0,  # 64-QAM
        8: 28.0,  # 256-QAM
    }[mod_order]
    
    # Adjust for code rate
    required_snr -= 10 * np.log10(code_rate)
    
    # BLER approximation (exponential model)
    if snr_db > required_snr:
        bler = np.exp(-0.5 * (snr_db - required_snr))
    else:
        bler = 0.5  # High error rate
    
    return min(bler, 0.5)

# Simulation
n_samples = 10000
avg_snr_db = 20
coherence_time = 100

# Generate fading channel
h = generate_fading_channel(n_samples, coherence_time)
snr_inst = avg_snr_db + 20*np.log10(np.abs(h))  # Instantaneous SNR

# AMC simulation
throughput_amc = []
throughput_fixed = []
bler_amc = []

# Fixed MCS (64-QAM, rate-3/4)
fixed_mcs = (6, 0.75, 4.5)

for snr_db in snr_inst:
    # AMC: Select MCS based on CQI
    cqi = snr_to_cqi(snr_db)
    mcs = cqi_to_mcs(cqi)
    
    # Compute BLER
    bler = compute_bler(snr_db, mcs)
    
    # Throughput (accounting for retransmissions)
    tput_amc = mcs[2] * (1 - bler)  # bits/s/Hz
    tput_fixed = fixed_mcs[2] * (1 - compute_bler(snr_db, fixed_mcs))
    
    throughput_amc.append(tput_amc)
    throughput_fixed.append(tput_fixed)
    bler_amc.append(bler)

# Results
print(f"Average SNR: {avg_snr_db} dB")
print(f"AMC average throughput: {np.mean(throughput_amc):.2f} bits/s/Hz")
print(f"Fixed MCS throughput: {np.mean(throughput_fixed):.2f} bits/s/Hz")
print(f"AMC gain: {np.mean(throughput_amc) / np.mean(throughput_fixed):.2f}×")
print(f"AMC average BLER: {np.mean(bler_amc):.2%}")
```

---

## 🎯 When to Use AMC

### AMC Excels:

✅ **Time-varying channels** (mobility, fading)  
✅ **Wide SNR range** (cell-edge to cell-center users)  
✅ **Throughput-oriented** applications (web, video streaming)  
✅ **Multi-user systems** (fairness via per-user adaptation)  
✅ **OFDM systems** (per-subcarrier or per-RB adaptation)  

### AMC Challenges:

❌ **Fast fading** (feedback delay > coherence time)  
❌ **Feedback overhead** (high for wideband, MIMO)  
❌ **Latency-sensitive** (delay from MCS switching)  
❌ **Low SNR regime** (limited MCS choices)  

---

## 📚 Further Reading

### Textbooks
- **Goldsmith**, *Wireless Communications* (Chapter 9: Adaptive Modulation) - Comprehensive treatment
- **Tse & Viswanath**, *Fundamentals of Wireless Communication* (Chapter 5: Capacity of fading channels)
- **Hanzo et al.**, *Adaptive Wireless Transceivers* - Deep dive into AMC algorithms

### Key Papers
- **Goldsmith & Varaiya** (1997): "Capacity of fading channels with CSI" - Foundational theory
- **Caire et al.** (1999): "Optimum power control over fading channels" - Water-filling for fading
- **Ekström et al.** (2006): "Technical Solutions for 3G LTE" - Practical LTE AMC

### Standards Documents
- **3GPP TS 36.213**: LTE Physical Layer Procedures (CQI, MCS tables)
- **3GPP TS 38.214**: 5G NR Physical Layer (AMC, HARQ)
- **IEEE 802.11ax**: WiFi 6 (rate adaptation algorithms)

### Related Topics
- [[Shannon's-Channel-Capacity-Theorem]] - Theoretical foundation for AMC
- [[OFDM-&-Multicarrier-Modulation]] - Per-subcarrier adaptation
- [[MIMO-&-Spatial-Multiplexing]] - Per-stream MCS adaptation
- [[Forward-Error-Correction-(FEC)]] - Code rate adaptation
- [[Real-World-System-Examples]] - LTE, 5G, WiFi implementations

---

**Summary**: Adaptive Modulation and Coding (AMC) is the bridge between [[Shannon's-Channel-Capacity-Theorem|Shannon theory]] and practical wireless systems. By dynamically selecting modulation order (BPSK → 256-QAM) and code rate (1/3 → 7/8) based on Channel Quality Indicator (CQI) feedback, AMC systems track instantaneous channel capacity and maximize throughput while maintaining target error rates (typically <10% BLER). LTE/5G use CQI reporting (1-15) mapped to MCS tables, combined with HARQ for robustness. AMC provides 2-5× throughput gain in fading channels compared to fixed modulation. Outer-loop link adaptation (OLLA) corrects for CQI errors. Cross-layer optimization integrates AMC with scheduling (proportional fair) and buffer management. Predictive AMC uses Doppler estimation or machine learning to anticipate channel changes. AMC is essential for spectral efficiency in modern cellular and WiFi networks, enabling gigabit-per-second data rates while serving users across wide SNR ranges (cell-edge to cell-center).
