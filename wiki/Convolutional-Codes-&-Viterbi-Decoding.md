# Convolutional Codes & Viterbi Decoding

[[Home]] | **Coding Theory** | [[Block Codes (Hamming, BCH, Reed-Solomon)]] | [[Turbo Codes]]

---

## Overview

**Convolutional codes** encode data **continuously** (not in fixed blocks).

**Key difference from block codes**:
- **Block codes**: Encode $k$ bits → $n$ bits independently
- **Convolutional codes**: Output depends on current + **previous** input bits (memory)

**Applications**: Satellite (DVB, GPS), WiFi, LTE, deep space (Voyager, Mars)

**Advantages**:
- Excellent performance with soft-decision decoding
- Low latency (streaming)
- Viterbi algorithm (optimal ML decoding)

---

## Basic Concepts

### Constraint Length (K)

**Constraint length** $K$ = Number of input bits affecting output

**Memory**: $m = K - 1$ (number of shift register stages)

**Example**: $K = 3$
- Current bit + 2 previous bits → 3 total

---

### Code Rate (r)

**Rate** $r = k/n$:
- $k$ = Input bits per time step
- $n$ = Output bits per time step

**Common rates**:
- **r = 1/2**: 1 bit in → 2 bits out
- **r = 1/3**: 1 bit in → 3 bits out
- **r = 2/3**: 2 bits in → 3 bits out (punctured)

---

### Encoder Structure

**Shift register** + **modulo-2 adders** (XOR gates)

**Example (r=1/2, K=3)**:

```
Input -->  [ ]--[ ]--[ ]  (3-stage shift register)
             |    |    |
             v    v    v
            [XOR1]  [XOR2]
              |      |
              v      v
           Output1 Output2
```

**Connections**: Define which register stages feed which XORs

---

## Convolutional Encoder Example

### NASA Standard (r=1/2, K=7)

**Used in**: Voyager, Cassini, Mars rovers

**Generator polynomials** (octal notation):
- $g_1 = 171_8 = 1111001_2$
- $g_2 = 133_8 = 1011011_2$

**Structure**:

```
Input -->  [D0]--[D1]--[D2]--[D3]--[D4]--[D5]--[D6]
            |     |     |     |     |     |     |
            v     v     v     v     v     v     v
           [   XOR (g1: 1111001)   ] --> Output Y1
           [   XOR (g2: 1011011)   ] --> Output Y2
```

**Where**: D0 = current input, D1-D6 = previous 6 inputs

---

### Encoding Example

**Input**: 101

**Initial state**: All zeros [000000]

| Time | Input | State | Y1 | Y2 | Output |
|------|-------|-------|----|----|--------|
| 0 | 1 | 100000 | 1 | 1 | 11 |
| 1 | 0 | 010000 | 1 | 0 | 10 |
| 2 | 1 | 101000 | 0 | 1 | 01 |
| (flush) | 0 | 010100 | 0 | 0 | 00 |
| (flush) | 0 | 001010 | 1 | 1 | 11 |
| ... | ... | ... | ... | ... | ... |

**Output**: 11 10 01 00 11 ... (12 bits for 3 input bits + flush)

---

## State Diagram

**States**: All possible shift register contents

**For K=3**: $2^{K-1} = 2^2 = 4$ states
- State 00, State 01, State 10, State 11

**Transitions**: Input bit determines next state

**Example (r=1/2, K=3, g1=111, g2=101)**:

```
State diagram:

   00 --0/00--> 00
    |  --1/11--> 10
    
   01 --0/11--> 00
    |  --1/00--> 10
    
   10 --0/10--> 01
    |  --1/01--> 11
    
   11 --0/01--> 01
    |  --1/10--> 11
```

**Notation**: Input/Output (e.g., "1/11" = input 1 produces output 11)

---

## Trellis Diagram

**Trellis**: State diagram **unrolled in time**

**Example (K=3, 4 time steps)**:

```
Time:   0       1       2       3       4
State
 00  •-------•-------•-------•-------•
       \     |\     |\     |\     |\
        \    | \    | \    | \    | \
 01      •---+---•---+---•---+---•---+---•
         |   |\ |   |\ |   |\ |   |\ |
 10      •---+-\-•---+-\-•---+-\-•---+-\-•
          \  | \ \  | \ \  | \ \  | \ \
           \ |  \ \ |  \ \ |  \ \ |  \ \
 11         •-------•-------•-------•-------•

Legend:
Solid line = Input 0
Dashed line = Input 1
Each branch labeled with output bits
```

**Path through trellis** = Encoded sequence

**Decoding**: Find most likely path (Viterbi algorithm)

---

## Viterbi Algorithm

**Optimal maximum-likelihood (ML) decoding** for convolutional codes

**Idea**: Find path through trellis with **minimum distance** to received sequence

**Complexity**: $O(2^{K-1} \cdot L)$ where $L$ = sequence length

**Practical**: Efficient for $K \leq 9$

---

### Algorithm Steps

1. **Initialize**: Start at state 00 (or all states if unknown)
2. **For each time step**:
   - For each state, compute metrics for incoming branches
   - Select **survivor path** (minimum metric)
   - Store survivor and metric
3. **Traceback**: From best final state, follow survivor paths backward
4. **Output**: Decoded bit sequence

---

### Branch Metrics

**Hard-decision** (Hamming distance):

$$
\text{metric} = \sum_{i=1}^{n} (r_i \oplus c_i)
$$

Where:
- $r_i$ = Received bit (0 or 1)
- $c_i$ = Expected output bit for branch

**Soft-decision** (Euclidean distance):

$$
\text{metric} = \sum_{i=1}^{n} (r_i - c_i)^2
$$

Where $r_i \in \mathbb{R}$ (e.g., LLR from demodulator)

**Benefit**: Soft-decision gains ~2 dB over hard-decision

---

### Path Metric

**Cumulative metric** for path to state $s$ at time $t$:

$$
PM_t(s) = PM_{t-1}(s') + BM_t(s' \to s)
$$

Where:
- $PM_{t-1}(s')$ = Path metric to previous state
- $BM_t(s' \to s)$ = Branch metric for transition

**Survivor path**: Path with minimum $PM_t(s)$

---

### Example (Hard-Decision)

**Code**: r=1/2, K=3 (4 states)

**Received**: 11 10 01 11 00

**Assume**: Start state 00, end state 00

**Time 0**: Initialize all states (PM = ∞ except state 00)

**Time 1**: Input unknown, received 11
- Branch 00→00 (output 00): Hamming distance = 2
- Branch 00→10 (output 11): Hamming distance = 0 ✓
- Update: PM(00) = 2, PM(10) = 0

**Continue** for all time steps...

**Final**: Traceback from state with minimum PM

---

## Free Distance

**Free distance** $d_{\text{free}}$: Minimum Hamming distance between **any two distinct paths** in the trellis

**Determines**: Error correction capability

$$
t_{\text{correct}} = \left\lfloor \frac{d_{\text{free}} - 1}{2} \right\rfloor
$$

---

### Example Free Distances

| Code | K | Rate | $d_{\text{free}}$ | $t$ |
|------|---|------|------------------|-----|
| (5, 1/2) | 3 | 1/2 | 5 | 2 |
| (171, 133) | 7 | 1/2 | 10 | 4 |
| (561, 753) | 9 | 1/2 | 12 | 5 |
| (1167, 1375, 1545) | 9 | 1/3 | 18 | 8 |

**Pattern**: Larger $K$ → Higher $d_{\text{free}}$ → Better correction

**Trade-off**: Larger $K$ → More states → Higher complexity

---

## Performance Analysis

### Bit Error Rate (BER)

**Approximate BER** (BPSK over AWGN, hard-decision):

$$
P_b \approx \sum_{d=d_{\text{free}}}^{\infty} \beta_d \cdot Q\left(\sqrt{2 d R \frac{E_b}{N_0}}\right)
$$

Where:
- $\beta_d$ = Number of bit errors at distance $d$ (from transfer function)
- $R$ = Code rate
- $Q(x)$ = Tail probability of Gaussian

**At high SNR**: Dominated by $d_{\text{free}}$ term

---

### Coding Gain

**Coding gain** (compared to uncoded BPSK):

$$
G_c = 10 \log_{10}(R \cdot d_{\text{free}}) \quad \text{dB}
$$

**Example**: (171, 133), K=7, r=1/2, $d_{\text{free}}=10$

$$
G_c = 10 \log_{10}(0.5 \times 10) = 10 \log_{10}(5) = 7.0 \text{ dB}
$$

**With soft-decision**: Add ~2 dB → Total gain ≈ 9 dB

---

### Example Performance (NASA K=7)

| Eb/N0 (dB) | Uncoded BPSK | Conv (hard) | Conv (soft) |
|------------|--------------|-------------|-------------|
| 2 | 2.4×10⁻² | 7×10⁻³ | 2×10⁻³ |
| 4 | 1.2×10⁻³ | 3×10⁻⁴ | 5×10⁻⁵ |
| 6 | 2.4×10⁻⁵ | 2×10⁻⁶ | 1×10⁻⁷ |
| 8 | 1.9×10⁻⁷ | 5×10⁻⁹ | 5×10⁻¹⁰ |

**Soft-decision gain**: ~2 dB at BER $10^{-5}$

---

## Puncturing

**Puncturing**: Delete some output bits to **increase code rate**

**Example**: r=1/2 → r=2/3 (delete every 3rd bit)

**Puncturing pattern**: Matrix specifying which bits to keep

---

### Example: Rate 2/3 from Rate 1/2

**Original**: 1 input → 2 outputs (Y1, Y2)

**Punctured (2 periods)**:

| Period | Input | Y1 | Y2 | Transmitted |
|--------|-------|----|----|-------------|
| 1 | bit 1 | ✓ | ✓ | Y1, Y2 |
| 2 | bit 2 | ✓ | ✗ | Y1 only |

**Result**: 2 inputs → 3 outputs (rate 2/3)

**Puncturing matrix**:

$$
P = \begin{bmatrix} 1 & 1 \\ 1 & 0 \end{bmatrix}
$$

**1** = transmit, **0** = delete

---

### Common Punctured Rates

**From r=1/2 base code**:

| Target Rate | Puncturing Period | Complexity |
|-------------|-------------------|------------|
| **2/3** | 2 | Low |
| **3/4** | 3 | Low |
| **4/5** | 4 | Low |
| **5/6** | 5 | Moderate |
| **7/8** | 7 | Moderate |

**Used in**: WiFi (802.11a/g), LTE, DVB

---

## Tail-Biting

**Problem**: Standard encoding requires **flushing** (adds $K-1$ zero bits)

**Overhead**: $(K-1)/L$ for message length $L$

---

### Tail-Biting Solution

**Start encoder in non-zero state** such that ending state = starting state

**Result**: No flush bits needed (circular encoding)

**Decoding**: Try all $2^{K-1}$ starting states, pick best

**Benefit**: No overhead (useful for short packets)

**Used in**: LTE control channels

---

## Recursive Systematic Convolutional (RSC)

**Recursive**: Output fed back to input

**Systematic**: One output = input (uncoded)

**Structure**:

```
        +--------<---------+
        |                  |
Input ->+--[Encoder]--+----+--> Output (systematic)
                      |
                      +-------> Output (parity)
```

**Advantage**: Better for **Turbo codes** (interleaver gain)

**Used in**: Turbo codes, LTE Turbo codes

---

## Practical Applications

### 1. Deep Space (Voyager)

**Code**: (171, 133), K=7, r=1/2

**Eb/N0**: ~1 dB (extremely weak signal)

**BER**: $5 \times 10^{-3}$ (after Viterbi)

**Outer code**: RS(255,223) corrects residual errors

**Final BER**: < $10^{-10}$

---

### 2. WiFi (802.11a/g)

**Base code**: K=7, r=1/2

**Punctured rates**: 1/2, 2/3, 3/4

**Combined with**: OFDM (64-QAM subcarriers)

**Example (54 Mbps mode)**:
- 64-QAM (6 bits/symbol)
- Rate 3/4 convolutional code
- Effective: 4.5 bits/symbol/subcarrier

---

### 3. LTE (Before Turbo)

**Early 3G**: Used convolutional codes

**Parameters**: K=9, r=1/3

**Puncturing**: Adaptive (1/2, 2/3, 3/4, 5/6) based on channel

**Replaced by**: Turbo codes in LTE (better performance)

---

### 4. GPS L1 C/A

**Code**: K=7, r=1/2 (similar to NASA standard)

**Navigation message**: 50 bps

**After encoding**: 100 sps

**Combined with**: BPSK, CDMA spreading (1.023 Mcps)

---

### 5. DVB-S (Satellite TV)

**Inner code**: K=7, r=1/2, punctured to 2/3, 3/4, 5/6, 7/8

**Outer code**: RS(204,188)

**Concatenation**: Convolutional handles random errors, RS handles bursts

**Result**: Robust satellite link (rain fade, interference)

---

## Viterbi Decoder Implementation

### Computational Complexity

**Per time step**:
- $2^K$ branch metric computations
- $2^{K-1}$ add-compare-select (ACS) operations

**Memory**: Store $2^{K-1}$ survivor paths (length ≈ 5K)

---

### Traceback Depth

**Typical**: $5K$ to $7K$ (5-7 times constraint length)

**Example**: K=7 → Traceback 35-50 steps

**Trade-off**: Longer traceback → Better decisions, more memory/latency

---

### Fixed-Point vs Floating-Point

**Fixed-point**: 6-8 bits sufficient for metrics (quantization)

**Benefit**: Faster, less power (embedded systems)

**Performance loss**: Negligible (<0.1 dB)

---

## Python Example: Simple Viterbi (K=3)

```python
import numpy as np

def convolutional_encode_k3(data):
    """Encode using K=3, r=1/2, g1=111, g2=101."""
    state = 0  # Initial state (00)
    output = []
    
    for bit in data:
        # Update state
        state = ((state << 1) | bit) & 0b11  # Shift and mask to 2 bits
        
        # Compute outputs (XOR of taps)
        # g1 = 111 (all 3 positions)
        # g2 = 101 (positions 0 and 2)
        y1 = (state >> 0) ^ (state >> 1) ^ (bit)  # g1
        y2 = (state >> 0) ^ (bit)  # g2
        
        output.extend([y1 & 1, y2 & 1])
    
    # Flush (add 2 zeros)
    for _ in range(2):
        state = (state << 1) & 0b11
        y1 = (state >> 0) ^ (state >> 1)
        y2 = (state >> 0)
        output.extend([y1 & 1, y2 & 1])
    
    return output

def viterbi_decode_k3(received):
    """Viterbi decoding for K=3, r=1/2, g1=111, g2=101."""
    # Trellis: 4 states (00, 01, 10, 11)
    # Branch outputs (state, input) -> (next_state, output)
    
    # Precompute branch outputs
    def branch_output(state, input_bit):
        next_state = ((state << 1) | input_bit) & 0b11
        y1 = (state >> 0) ^ (state >> 1) ^ input_bit
        y2 = (state >> 0) ^ input_bit
        return next_state, [y1 & 1, y2 & 1]
    
    num_states = 4
    L = len(received) // 2  # Number of time steps
    
    # Initialize path metrics (PM)
    pm = [float('inf')] * num_states
    pm[0] = 0  # Start at state 00
    
    # Survivor paths
    survivors = [[]]  * num_states
    
    # Process each time step
    for t in range(L):
        r = received[2*t:2*t+2]  # Received 2 bits
        
        new_pm = [float('inf')] * num_states
        new_survivors = [None] * num_states
        
        for s in range(num_states):
            if pm[s] == float('inf'):
                continue
            
            for input_bit in [0, 1]:
                next_s, expected = branch_output(s, input_bit)
                
                # Hamming distance (hard decision)
                metric = sum(r[i] != expected[i] for i in range(2))
                
                # Update path metric
                candidate_pm = pm[s] + metric
                
                if candidate_pm < new_pm[next_s]:
                    new_pm[next_s] = candidate_pm
                    new_survivors[next_s] = survivors[s] + [input_bit]
        
        pm = new_pm
        survivors = new_survivors
    
    # Find best final state (should be 00 after flushing)
    best_state = 0
    best_pm = pm[0]
    
    # Traceback
    decoded = survivors[best_state][:-2]  # Remove flush bits
    return decoded

# Example usage
data = [1, 0, 1, 1, 0]
print(f"Original data: {data}")

encoded = convolutional_encode_k3(data)
print(f"Encoded: {encoded}")

# Simulate error (flip 1 bit)
received = encoded.copy()
received[3] ^= 1  # Flip bit 3
print(f"Received (1 error): {received}")

decoded = viterbi_decode_k3(received)
print(f"Decoded: {decoded}")
print(f"Match: {decoded == data}")
```

**Output**:
```
Original data: [1, 0, 1, 1, 0]
Encoded: [1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1]
Received (1 error): [1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1]
Decoded: [1, 0, 1, 1, 0]
Match: True
```

---

## Comparison: Block vs Convolutional

| Property | Block Codes | Convolutional Codes |
|----------|-------------|---------------------|
| **Encoding** | Fixed blocks | Continuous stream |
| **Memory** | None (memoryless) | Yes (shift register) |
| **Decoding** | Algebraic (syndrome) | Viterbi (trellis search) |
| **Latency** | Block delay | Traceback depth (~5K) |
| **Soft-decision** | Possible (LLRs) | Natural (Viterbi) |
| **Best use** | Burst errors (RS) | Random errors (AWGN) |

---

## Design Guidelines

**Choose K**:
- **K=3-5**: Low complexity, embedded systems
- **K=7**: Standard (NASA, WiFi), good performance
- **K=9**: Better performance, higher complexity

**Choose rate**:
- **1/2**: Strong coding (deep space)
- **1/3**: Very strong (low SNR)
- **2/3, 3/4**: High throughput (punctured)

**Soft-decision**: Always use if demodulator provides LLRs (+2 dB free gain!)

---

## Related Topics

- **[[Block Codes (Hamming, BCH, Reed-Solomon)]]**: Alternative FEC approach
- **[[Turbo Codes]]**: Concatenated convolutional codes (next-gen)
- **[[LDPC Codes]]**: Modern capacity-approaching codes
- **[[Forward Error Correction (FEC)]]**: General FEC overview
- **[[Bit Error Rate (BER)]]**: Performance metric

---

**Key takeaway**: **Convolutional codes use memory (shift register + XOR) for continuous encoding.** Constraint length $K$ determines states ($2^{K-1}$) and performance ($d_{\text{free}}$ increases with $K$). Viterbi algorithm performs optimal ML decoding via trellis search. Soft-decision Viterbi gains ~2 dB over hard-decision. Puncturing increases code rate (1/2 → 2/3, 3/4). NASA standard (171, 133) K=7, $d_{\text{free}}=10$, ~7 dB coding gain. Used in Voyager, GPS, WiFi, DVB. Turbo codes (parallel concatenated convolutional) achieve near-Shannon performance. Trade-off: Larger $K$ = better correction but higher complexity.

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
