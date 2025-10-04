# Turbo Codes

[[Home]] | **Coding Theory** | [[Convolutional Codes & Viterbi Decoding]] | [[LDPC Codes]]

---

## 🚀 For Non-Technical Readers

**Turbo codes are like having two spell-checkers that help each other—when one is unsure, the other provides hints, and they iterate back and forth until they agree. Revolutionary in 1993!**

**The breakthrough**:
- **Before 1993**: Best codes were ~3 dB from theoretical limit
- **Turbo codes (1993)**: Got within 0.5 dB of Shannon limit!
- **Impact**: "Impossible" performance, shocked the world
- **Today**: In 3G/4G phones, deep space, satellites

**How they work - Two encoders help each other**:

**Step 1**: Encode data with TWO different convolutional encoders
- Encoder 1: Sees data in original order
- Encoder 2: Sees data scrambled (interleaved)
- Send both encoded versions

**Step 2**: Receiver iteratively decodes
- Decoder 1: "I think bit 5 is probably a 1... 80% sure"
- Decoder 2: "I think bit 5 is definitely a 1... 95% sure!"
- Decoder 1: "Oh! With that info, I'm now 98% sure!"
- They ping-pong back and forth ~5-10 iterations
- Final result: Near-perfect decoding!

**The magic - "Turbo" analogy**:
- Like a turbo charger: Output feeds back to improve input
- Each decoder's output improves the other's input
- After several iterations, converges to correct answer
- Hence: "Turbo" codes!

**Real-world use**:
- **3G (UMTS)**: Turbo codes for data channels
- **4G (LTE)**: Turbo codes (before LDPC in 5G)
- **Deep space**: Mars rovers use turbo codes
- **Satellite phones**: Iridium, Globalstar
- **Military**: Tactical communications

**Why "revolutionary" in 1993**:
- Shannon's limit (1948): Theoretical best = 0 dB Eb/N0
- Best codes before 1993: ~3 dB from limit
- Turbo codes: 0.5-1 dB from limit!
- Engineers thought this was impossible!

**The famous 1993 paper**:
- Presented at ICC '93 conference
- Audience: Stunned silence, then standing ovation
- "We must have made a mistake" - initial reaction
- Verified by others: IT'S REAL!
- Changed communication systems forever

**Comparison with other codes**:
- **Convolutional + Viterbi**: Simple, but ~5 dB from limit
- **Turbo codes**: 0.5-1 dB from limit, complex
- **LDPC codes**: 0.5 dB from limit, easier to implement
- **Polar codes**: Proven optimal, simpler structure

**Trade-offs**:
- **Advantage**: Amazing performance, near-Shannon limit
- **Disadvantage**: Complex decoder, high latency (iterations)
- **Why 5G switched to LDPC/Polar**: Simpler, lower latency

**The iterative decoding process**:
```
Iteration 1: 60% confidence
Iteration 2: 80% confidence
Iteration 3: 95% confidence  
Iteration 4: 99% confidence
Iteration 5: 99.9% confidence → DONE!
```

**Fun fact**: The inventors (Berrou, Glavieux, Thitimajshima) almost didn't publish because they thought they'd made a mistake—the performance seemed too good to be true. When they finally presented in 1993, it sparked a revolution in error correction!

---

## Overview

**Turbo codes** achieve **near-Shannon-limit** performance (within 0.5-1 dB of capacity).

**Key innovation**: **Parallel concatenation** of convolutional codes with **iterative decoding**

**Discovery**: Berrou, Glavieux, and Thitimajshima (1993) - Revolutionary breakthrough

**Performance**: BER $10^{-5}$ at Eb/N0 ≈ 0.7 dB (rate 1/2, BPSK) - Only 0.5 dB from Shannon limit!

**Applications**: 3G/4G cellular (UMTS, LTE), deep space (Mars rovers, New Horizons), DVB-RCS satellite

---

## Basic Structure

**Parallel Concatenated Convolutional Codes (PCCC)**:

```
                +---> [RSC Encoder 1] ---> Parity 1
                |
Input data ---> | 
                |
                +---> [Interleaver] ---> [RSC Encoder 2] ---> Parity 2
```

**Components**:
1. **Two RSC encoders** (Recursive Systematic Convolutional)
2. **Interleaver** (pseudo-random permutation)
3. **Systematic output** (original data)

**Output**: Systematic bits + Parity1 + Parity2

---

## Recursive Systematic Convolutional (RSC) Encoder

**Why RSC?** Better iterative decoding than non-recursive

**Structure**:

```
        +--------<----------+
        |                   |
Input ->⊕--[Shift Register]-⊕--> Systematic output (same as input)
            |                |
            +--[XOR logic]---+--> Parity output
```

**Recursive**: Output fed back to input (creates infinite impulse response)

**Systematic**: One output = input (uncoded)

---

### Example: RSC (37, 21) Octal

**Generator polynomials** (octal):
- Feedback: 37₈ = 011111₂
- Feedforward: 21₈ = 010001₂

**K = 5** (constraint length)

**Rate**: 1/2 (1 systematic + 1 parity per input bit)

---

## Interleaver

**Purpose**: Break correlation between encoder inputs

**Types**:
1. **Random interleaver**: Pseudo-random permutation
2. **Block interleaver**: Write row-wise, read column-wise
3. **S-random interleaver**: Constrained randomness (no nearby indices)

**Length**: Typically 1000-10,000 bits (longer = better performance)

---

### Why Interleaving Works

**Input sequence**: 11111 (low Hamming weight)

**Encoder 1**: Produces low-weight parity (correlated errors)

**After interleaver**: 10101 (scattered)

**Encoder 2**: Produces high-weight parity (uncorrelated)

**Result**: Combined code has high minimum distance → Good error correction

---

### S-Random Interleaver

**Constraint**: Indices $i$ and $j$ separated by $< S$ in input → Separated by $\geq S$ in output

**Example** (S=3):
- If positions 0, 1, 2 are adjacent in input
- After interleaving: Must be ≥3 positions apart

**Benefit**: Prevents clustered low-weight codewords

---

## Encoding Process

**Input**: Data block $\mathbf{d} = [d_1, d_2, \ldots, d_K]$

**Steps**:

1. **Encoder 1**: Encode $\mathbf{d}$ → Parity1 $\mathbf{p}_1$
2. **Interleave**: $\mathbf{d}' = \pi(\mathbf{d})$ (permutation)
3. **Encoder 2**: Encode $\mathbf{d}'$ → Parity2 $\mathbf{p}_2$
4. **Transmit**: $[\mathbf{d}, \mathbf{p}_1, \mathbf{p}_2]$ (rate 1/3)

**Or puncture** to rate 1/2: Transmit $[\mathbf{d}, \mathbf{p}_1^{(even)}, \mathbf{p}_2^{(odd)}]$

---

### Rate Matching (Puncturing)

**Achieve flexible rates** by deleting parity bits:

**Example (rate 1/3 → rate 1/2)**:

| Time | Systematic | Parity1 | Parity2 | Transmitted |
|------|------------|---------|---------|-------------|
| 1 | d₁ | p₁₁ | p₂₁ | d₁, p₁₁ |
| 2 | d₂ | p₁₂ | p₂₂ | d₂, p₂₂ |
| 3 | d₃ | p₁₃ | p₂₃ | d₃, p₁₃ |

**Result**: 3 data + 3 parity = rate 1/2

---

## Iterative Decoding

**Key innovation**: Two decoders exchange **extrinsic information**

**Algorithm**: BCJR (Bahl-Cocke-Jelinek-Raviv) or SOVA (Soft-Output Viterbi)

---

### Decoder Structure

```
Received  --> [SISO Decoder 1] <---> [Interleaver]   <---> [SISO Decoder 2]
systematic         |                      ↓                       |
+ parity1          +-------------------> ⊕ <--------------------+
                        (extrinsic info exchange)
```

**SISO**: Soft-In Soft-Out decoder (outputs LLRs, not hard decisions)

**Iteration**: Decoders alternate, passing improved soft information

---

### Log-Likelihood Ratios (LLR)

**LLR for bit** $d_k$:

$$
L(d_k) = \log\frac{P(d_k = 0 | \text{received})}{P(d_k = 1 | \text{received})}
$$

**Decomposition**:

$$
L(d_k) = L_c(d_k) + L_a(d_k) + L_e(d_k)
$$

Where:
- $L_c$ = **Channel LLR** (from demodulator)
- $L_a$ = **A priori LLR** (from other decoder)
- $L_e$ = **Extrinsic LLR** (new information from this decoder)

---

### Iterative Decoding Steps

**Iteration $i$**:

1. **Decoder 1**:
   - Input: $L_c(\mathbf{d})$, $L_c(\mathbf{p}_1)$, $L_a^{(i)}(\mathbf{d})$ (from Dec2)
   - Compute: $L_e^{(i)}(\mathbf{d})$ (extrinsic info)
   - Output: $L_1^{(i)}(\mathbf{d}) = L_c + L_a + L_e$

2. **Interleave**: $L_e^{(i)}(\mathbf{d}') = \pi(L_e^{(i)}(\mathbf{d}))$

3. **Decoder 2**:
   - Input: $L_c(\mathbf{d}')$, $L_c(\mathbf{p}_2)$, $L_e^{(i)}(\mathbf{d}')$ (from Dec1)
   - Compute: $L_e^{(i)}(\mathbf{d}')$ (extrinsic info)
   - Output: $L_2^{(i)}(\mathbf{d}')$

4. **De-interleave**: $L_a^{(i+1)}(\mathbf{d}) = \pi^{-1}(L_e^{(i)}(\mathbf{d}'))$

5. **Repeat** for $N$ iterations (typically 4-10)

6. **Hard decision**: $\hat{d}_k = \text{sign}(L_1^{(N)}(d_k) + L_2^{(N)}(d_k))$

---

### Why Iterative Decoding Works

**Decoder 1**: Uses channel info + parity1 → Produces soft estimates

**Decoder 2**: Uses channel info + parity2 + **extrinsic from Dec1** → Refines estimates

**Iteration**: Each decoder improves estimates using other's extrinsic info

**Convergence**: LLRs → High magnitude (high confidence) after ~4-10 iterations

**Analogy**: Two experts discussing a problem, each bringing unique perspective

---

## BCJR Algorithm

**Bahl-Cocke-Jelinek-Raviv**: Optimal soft-output decoder (MAP)

**Computes**: A posteriori probability (APP) for each bit

**Recursions** (forward-backward):

**Forward** $\alpha$:

$$
\alpha_k(s) = \sum_{s'} \alpha_{k-1}(s') \cdot \gamma_k(s', s)
$$

**Backward** $\beta$:

$$
\beta_{k-1}(s') = \sum_{s} \beta_k(s) \cdot \gamma_k(s', s)
$$

**Branch metric** $\gamma$:

$$
\gamma_k(s', s) = P(\text{transition } s' \to s | \text{received})
$$

**LLR**:

$$
L(d_k) = \log\frac{\sum_{(s',s): d_k=0} \alpha(s') \gamma(s',s) \beta(s)}{\sum_{(s',s): d_k=1} \alpha(s') \gamma(s',s) \beta(s)}
$$

**Complexity**: $O(2^{2K})$ per bit (manageable for K ≤ 7)

---

## Performance Analysis

### BER vs Eb/N0

**Typical performance** (rate 1/2, K=4, random interleaver, 10 iterations):

| Eb/N0 (dB) | Uncoded BPSK | Turbo Code | Shannon Limit |
|------------|--------------|------------|---------------|
| -1.6 | 0.27 | - | 0 (capacity) |
| 0 | 0.08 | 0.01 | - |
| 0.5 | 0.04 | 10⁻³ | - |
| 0.7 | 0.03 | 10⁻⁵ | Gap = 0.5 dB |
| 1.0 | 0.02 | 10⁻⁶ | - |
| 2.0 | 5×10⁻³ | 10⁻⁹ | - |

**Waterfall region**: Sharp BER drop at ~0.5-1.0 dB

**Error floor**: BER flattens at ~10⁻⁶ to 10⁻⁸ (due to low-weight codewords)

---

### Convergence Analysis

**EXIT Charts** (Extrinsic Information Transfer):

**Plots**: Mutual information $I_e$ vs $I_a$ for each decoder

**Convergence**: If curves don't cross → Decoders converge to low BER

**Tunnel opening**: Gap between curves → Convergence speed

---

### Interleaver Length Effect

| Interleaver Size | BER @ 0.7 dB | Error Floor | Notes |
|------------------|--------------|-------------|-------|
| **100 bits** | 10⁻³ | 10⁻⁴ | Poor (short) |
| **1,000 bits** | 10⁻⁴ | 10⁻⁶ | Moderate |
| **10,000 bits** | 10⁻⁵ | 10⁻⁸ | Good |
| **100,000 bits** | 10⁻⁵ | 10⁻¹⁰ | Excellent (high latency) |

**Trade-off**: Longer interleaver → Better performance, higher latency/memory

---

## Turbo Code Variants

### 1. Duo-Binary Turbo Codes

**Process 2 bits at a time**: $(d_1, d_2)$ jointly

**Advantage**: Better performance, lower error floor

**Used in**: DVB-RCS (satellite return channel)

---

### 2. Serial Concatenated Convolutional Codes (SCCC)

**Structure**: Inner encoder → Interleaver → Outer encoder (serial)

**Performance**: Lower error floor than PCCC

**Decoding**: Similar iterative structure

---

### 3. Repeat-Accumulate (RA) Codes

**Simplified turbo code**:

```
Input --> [Repeat r times] --> [Interleaver] --> [Accumulator] --> Output
```

**Accumulator**: Simple RSC with feedback polynomial 1/(1+D)

**Advantage**: Very simple encoder

**Performance**: Near-turbo with less complexity

---

## Practical Implementations

### 1. 3G UMTS (WCDMA)

**Turbo code**: Rate 1/3, K=4
- Two RSC encoders (G=[1, 13/15]₈)
- Interleaver: Length 40-5114 bits
- 8 iterations

**Channels**: Data (up to 2 Mbps)

**BER**: 10⁻⁶ @ Eb/N0 ≈ 1.5 dB

---

### 2. 4G LTE

**Turbo code**: Rate 1/3, K=4
- Two RSC encoders
- QPP interleaver (Quadratic Permutation Polynomial)
- 6-8 iterations

**Data rates**: 1 Mbps - 100 Mbps (Cat 3), up to 1 Gbps (Cat 16)

**Block sizes**: 40-6144 bits

**Puncturing**: Adaptive (1/2, 2/3, 3/4, 5/6) based on MCS

---

### 3. Deep Space (NASA/ESA)

**Mars Exploration Rovers**: Turbo code rate 1/6
- K=5 RSC encoders
- 65,536-bit interleaver
- 15 iterations

**Performance**: BER < 10⁻⁸ @ Eb/N0 ≈ 0 dB

**Data rate**: 128 kbps (from Mars surface)

---

### 4. DVB-RCS (Satellite Return)

**Duo-binary turbo code**: Rate 1/3 to 6/7 (punctured)

**Block sizes**: 48-1504 bits

**Iterations**: 6-8

**Application**: Interactive satellite broadband (uplink)

---

## Encoder Complexity

**Encoding**: Linear complexity $O(K)$ per bit

**Example**: K=4, rate 1/3
- 2 RSC encoders (4 states each)
- Interleaver (memory access)
- **Total**: ~10-20 operations per bit

**Hardware**: Easy to implement (shift registers + XORs)

---

## Decoder Complexity

**BCJR per iteration**:
- $O(2^K)$ states
- $O(K)$ operations per state
- Total: $O(K \cdot 2^K)$ per bit

**Example**: K=4, 8 iterations
- 16 states, ~50 operations per state per iteration
- **Total**: ~6400 operations per bit

**SOVA alternative**: Lower complexity (~40% of BCJR), 0.3 dB performance loss

---

### Optimization Techniques

1. **Max-Log-MAP**: Approximation (replace sum with max)
   - Complexity: 50% reduction
   - Loss: ~0.3 dB

2. **Sliding window**: Process trellis in windows (reduce memory)

3. **Early termination**: Stop if LLRs exceed threshold (save iterations)

4. **Radix-4**: Process 2 bits at a time (2× throughput)

---

## Stopping Criteria

**Problem**: Fixed iteration count wastes power (good SNR needs fewer iterations)

**Solution**: Early stopping

**Criteria**:

1. **LLR magnitude**: $|L(d_k)| > T$ for all $k$ (high confidence)

2. **Cross-entropy**: $H(L^{(i)}, L^{(i-1)}) < \epsilon$ (convergence)

3. **CRC check**: If CRC passes, stop (used in LTE)

**Benefit**: Average 3-5 iterations (vs 8 worst-case) → 40% power savings

---

## Error Floor

**Error floor**: BER stops improving (flattens) at high SNR

**Cause**: Low-weight codewords (small $d_{\text{free}}$)

**Dominant**: Input sequences causing low-weight output in **both** encoders

**Example**: Input weight 2, output weight 4 → $d_{\text{free}} = 6$ (poor)

---

### Mitigation Strategies

1. **Interleaver design**: S-random, dithered (avoid bad patterns)

2. **Longer interleaver**: Reduces probability of bad patterns

3. **Increase K**: Larger constraint length → Higher $d_{\text{free}}$

4. **Post-processing**: Outer code (e.g., CRC + retransmission)

**Typical floor**: 10⁻⁶ to 10⁻⁸ (acceptable for most applications)

---

## Comparison with Other Codes

| Code | Eb/N0 @ 10⁻⁵ (rate 1/2) | Gap to Shannon | Complexity | Latency |
|------|--------------------------|----------------|------------|---------|
| **Uncoded** | 9.6 dB | +11 dB | - | 0 |
| **Conv (K=7)** | 4.5 dB | +6 dB | Low | Low |
| **Turbo** | 0.7 dB | +0.5 dB | Moderate | Moderate |
| **LDPC** | 0.5 dB | +0.3 dB | Moderate | Low |
| **Polar** | 1.0 dB | +0.8 dB | Low | Low |

**Turbo advantages**: Near-Shannon, proven performance, standardized

**Turbo disadvantages**: Latency (iterative), error floor

---

## Turbo vs LDPC

| Aspect | Turbo Codes | LDPC Codes |
|--------|-------------|------------|
| **Eb/N0 @ 10⁻⁵** | 0.7 dB | 0.5 dB |
| **Error floor** | 10⁻⁷ typical | 10⁻¹² possible |
| **Decoding latency** | High (iterations) | Lower (parallel) |
| **Complexity** | Moderate | Moderate |
| **Hardware** | Serial (trellis) | Parallel (graph) |
| **Standardization** | 3G, 4G LTE | 5G NR, WiFi 6, DVB-S2 |
| **Flexibility** | Puncturing | Structured graphs |

**Trend**: LDPC replacing Turbo in new standards (5G, WiFi 6, 802.11ax)

---

## Design Guidelines

### Choose Turbo Code When:

1. **Near-capacity performance** critical (< 1 dB from Shannon)
2. **Moderate block sizes** (1000-10000 bits)
3. **Latency acceptable** (iterative decoding OK)
4. **Error floor 10⁻⁶** sufficient
5. **Existing hardware** (3G/4G infrastructure)

### Avoid Turbo Code If:

1. **Ultra-low error floor** needed (< 10⁻¹⁰) → Use LDPC
2. **Low latency** critical → Use LDPC or Polar
3. **Very short blocks** (< 100 bits) → Use Polar or convolutional
4. **New design** (future-proof) → Consider LDPC (5G standard)

---

## Python Example: Simple Turbo Encoder

```python
import numpy as np

def rsc_encode(data, g_fb=[1,1,1], g_ff=[1,0,1]):
    """RSC encoder (K=3 example)."""
    K = len(g_fb)
    state = 0
    systematic = []
    parity = []
    
    for bit in data:
        # Feedback XOR
        fb = bit
        for i in range(1, K):
            if g_fb[i] and (state & (1 << (i-1))):
                fb ^= 1
        
        # Parity XOR
        p = 0
        for i in range(K):
            if i == 0:
                if g_ff[0]:
                    p ^= fb
            else:
                if g_ff[i] and (state & (1 << (i-1))):
                    p ^= 1
        
        # Update state (shift in feedback bit)
        state = ((state << 1) | fb) & ((1 << (K-1)) - 1)
        
        systematic.append(bit)
        parity.append(p)
    
    return systematic, parity

def turbo_encode(data, interleaver_indices):
    """Turbo encoder (rate 1/3)."""
    # Encoder 1
    sys1, par1 = rsc_encode(data)
    
    # Interleave
    data_int = [data[i] for i in interleaver_indices]
    
    # Encoder 2
    sys2, par2 = rsc_encode(data_int)
    
    # Output: systematic + parity1 + parity2
    # (sys1 and sys2 are same as data, use sys1)
    return sys1, par1, par2

# Example
data = [1, 0, 1, 1, 0, 1, 0, 0]
interleaver = [0, 4, 2, 6, 1, 5, 3, 7]  # S-random example

sys, par1, par2 = turbo_encode(data, interleaver)

print(f"Data:       {data}")
print(f"Systematic: {sys}")
print(f"Parity 1:   {par1}")
print(f"Parity 2:   {par2}")
print(f"Code rate:  {len(data)}/{len(sys)+len(par1)+len(par2)} = 1/3")
```

**Note**: Full iterative decoder (BCJR) is complex (~200+ lines). Use libraries like `commpy` for production.

---

## Related Topics

- **[[Convolutional Codes & Viterbi Decoding]]**: Building block for Turbo
- **[[LDPC Codes]]**: Modern alternative (5G, WiFi 6)
- **[[Polar Codes]]**: Another near-capacity code (5G control)
- **[[Forward Error Correction (FEC)]]**: General FEC overview
- **[[Bit Error Rate (BER)]]**: Performance metric

---

**Key takeaway**: **Turbo codes achieve near-Shannon-limit performance (0.5-1 dB gap) via parallel concatenated RSC encoders + iterative decoding.** Two SISO decoders exchange extrinsic LLRs, refining estimates over 4-10 iterations. Interleaver breaks correlation (critical for performance). Used in 3G UMTS, 4G LTE, deep space (Mars rovers). BER $10^{-5}$ @ Eb/N0 ≈ 0.7 dB (rate 1/2). Error floor at $10^{-6}$ to $10^{-8}$ due to low-weight codewords. BCJR algorithm provides optimal soft-output decoding. Longer interleaver (10k+ bits) improves performance but increases latency. Being replaced by LDPC in 5G/WiFi 6 (lower error floor, lower latency, better parallelization). Revolutionary 1993 discovery—brought information theory to practice.

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
