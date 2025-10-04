# Polar Codes

[[Home]] | **Coding Theory** | [[Turbo-Codes]] | [[LDPC-Codes]]

---

## 🏆 For Non-Technical Readers

**Polar codes are the newest champion of error correction—the first codes with mathematical PROOF they reach the theoretical limit. That's why 5G uses them!**

**What makes them special**:
- **First provably optimal codes**: Math proof they're perfect!
- **Channel polarization**: Clever trick that splits channel into good/bad parts
- **Simpler than LDPC**: Easier to implement in hardware
- **5G standard**: Chosen for 5G control channels!

**The discovery - Recent breakthrough**:
- **2008**: Erdal Arıkan (Turkish professor) invents polar codes
- **2016**: Adopted by 5G standard (Huawei championed them)
- **Today**: In every 5G phone for control signaling

**The magic trick - Channel polarization**:

Imagine you have a noisy channel:
- Some bits get through clean (lucky!)
- Some bits get corrupted (unlucky!)
- But you don't know which is which!

**Polar code solution**:
1. Use clever math to "sort" the channel
2. Some sub-channels become PERFECT (polarized to good)
3. Others become USELESS (polarized to bad)
4. Send data on perfect channels, known patterns on bad ones
5. Receiver uses known patterns to decode data!

**Simple analogy - Sorting students**:
- 100 students with mixed abilities
- Polar coding: Group them by strength
- Put hard problems to strong students (they'll succeed)
- Put easy problems to weak students (they'll succeed too!)
- Result: Maximum overall success!

**Comparison with other codes**:
- **Turbo codes**: Amazing, but complex, no optimality proof
- **LDPC codes**: Near-optimal, but no explicit proof
- **Polar codes**: PROVEN optimal, simpler structure! ✅

**Where they're used**:
- **5G control channels**: Polar codes for critical signaling
  - LDPC for data (better at high rates)
  - Polar for control (better at low rates)
- **Research**: Future standards, deep space, quantum

**Why 5G chose them**:
- **Low latency**: Fast decoding for control messages
- **Flexible**: Work at any code rate
- **Simple**: Easier to implement in 5G chips
- **Proven optimal**: Mathematical guarantee!

**Performance**:
- **Shannon limit**: Theoretical best
- **Polar codes**: Proven to reach limit as block size → ∞
- **Practical**: Within 0.8-1.5 dB of limit at reasonable block sizes
- Comparable to LDPC, but with optimality proof!

**The debate**:
- **Huawei pushed Polar**: They hold many patents
- **Qualcomm pushed LDPC**: They have LDPC expertise
- **5G compromise**: Polar for control, LDPC for data
- Both sides win!

**Fun fact**: Polar codes are the only error-correcting codes with a mathematical proof that they achieve Shannon's theoretical limit. Every other code (even LDPC) is "just" really good in practice without the theoretical guarantee!

---

## Overview

**Polar codes** are the **first provably capacity-achieving codes** with explicit construction.

**Discovery**: Erdal Arıkan (2008) - Major theoretical breakthrough

**Key property**: **Channel polarization** - Split channel into perfect + useless subchannels

**Performance**: 0.8-1.5 dB from Shannon limit (rate 1/2, block length 1024+)

**Applications**: **5G NR control channels** (eMBB, URLLC), future satellite, IoT

---

## Channel Polarization

**Fundamental idea**: Recursive channel combining + splitting

**Input**: N uses of channel W with capacity I(W)

**Output**: N synthesized channels $W_i$, each with capacity I($W_i$)

**Polarization**: As N → ∞:
- Some channels → I($W_i$) → 1 (perfect, **noiseless**)
- Others → I($W_i$) → 0 (useless, **pure noise**)

**Strategy**: Transmit data on good channels, freeze bad channels (set to 0)

---

### Simple Example (N=2)

**Base transformation**:

```
u₁ --⊕--> y₁
      |
u₂ ---+--> y₂
```

**Channel combining**: $y_1 = u_1 \oplus u_2$, $y_2 = u_2$

**After decoding**:
- **Channel for $u_1$**: Worse than W (joint decoding, less reliable)
- **Channel for $u_2$**: Better than W (uses $u_1$ as side info)

**Result**: Two channels split—one better, one worse (polarization starts!)

---

## Polar Transform

**N = 2ⁿ** (power of 2)

**Encoding**: $\mathbf{x} = \mathbf{u} G_N$

Where:
- $\mathbf{u}$ = $(u_1, u_2, \ldots, u_N)$ (information + frozen bits)
- $G_N$ = Polar generator matrix
- $\mathbf{x}$ = Transmitted codeword

---

### Generator Matrix

**Base matrix** (N=2):

$$
G_2 = \begin{bmatrix} 1 & 0 \\ 1 & 1 \end{bmatrix}
$$

**Recursive construction**:

$$
G_N = \begin{bmatrix} G_{N/2} & G_{N/2} \\ 0 & G_{N/2} \end{bmatrix}
$$

**Example** (N=4):

$$
G_4 = \begin{bmatrix}
1 & 0 & 0 & 0 \\
1 & 1 & 0 & 0 \\
1 & 0 & 1 & 0 \\
1 & 1 & 1 & 1
\end{bmatrix}
$$

**Example** (N=8):

$$
G_8 = G_2 \otimes G_2 \otimes G_2 = \begin{bmatrix}
1 & 0 & 0 & 0 & 0 & 0 & 0 & 0 \\
1 & 1 & 0 & 0 & 0 & 0 & 0 & 0 \\
1 & 0 & 1 & 0 & 0 & 0 & 0 & 0 \\
\vdots & & & \ddots & & & & \\
1 & 1 & 1 & 1 & 1 & 1 & 1 & 1
\end{bmatrix}
$$

**Kronecker product**: $G_N = G_2^{\otimes n}$ for $N = 2^n$

---

## Code Construction

**Steps**:

1. **Choose N** (block length, power of 2)
2. **Compute channel reliabilities**: $Z(W_i)$ or $I(W_i)$ for $i = 1, \ldots, N$
3. **Select K best channels** (highest reliability)
4. **Information set** $\mathcal{A}$: Indices of K best channels
5. **Frozen set** $\mathcal{A}^c$: Remaining N-K indices (set to 0)

**Code rate**: $R = K/N$

---

### Channel Reliability Metrics

**Bhattacharyya parameter** $Z(W)$:

$$
Z(W) = \sum_{y} \sqrt{W(y|0) \cdot W(y|1)}
$$

**Mutual information** $I(W)$:

$$
I(W) = \sum_{y} \sum_{x \in \{0,1\}} W(y|x) \log_2\frac{W(y|x)}{\sum_{x'} W(y|x')}
$$

**Properties**:
- $Z(W) \in [0, 1]$: Lower is better
- $I(W) \in [0, 1]$: Higher is better
- Perfect channel: $Z = 0$, $I = 1$
- Useless channel: $Z = 1$, $I = 0$

---

### Density Evolution

**Compute reliabilities recursively**:

**Channel combining** (worse):

$$
Z(W^-) \approx 2Z(W) - Z(W)^2
$$

**Channel splitting** (better):

$$
Z(W^+) \approx Z(W)^2
$$

**Starting point**: Binary symmetric channel (BSC) with crossover probability $\epsilon$

$$
Z_0 = 2\sqrt{\epsilon(1-\epsilon)}
$$

**Recursion**: Apply transformations $n$ times for $N = 2^n$ channels

---

## Encoding

**Input**:
- Data bits: $\mathbf{d} = [d_1, d_2, \ldots, d_K]$
- Information set: $\mathcal{A} = \{i_1, i_2, \ldots, i_K\}$

**Set vector** $\mathbf{u}$:

$$
u_i = \begin{cases}
d_j & \text{if } i = i_j \in \mathcal{A} \\
0 & \text{if } i \in \mathcal{A}^c
\end{cases}
$$

**Encode**:

$$
\mathbf{x} = \mathbf{u} G_N
$$

**Complexity**: $O(N \log N)$ using FFT-like butterfly structure

---

### Example (N=8, K=4)

**Information set**: $\mathcal{A} = \{4, 6, 7, 8\}$ (best 4 channels)

**Frozen set**: $\mathcal{A}^c = \{1, 2, 3, 5\}$ (worst 4 channels)

**Data**: $\mathbf{d} = [1, 0, 1, 1]$

**Vector** $\mathbf{u}$:

$$
\mathbf{u} = [0, 0, 0, 1, 0, 0, 1, 1]
$$

(Frozen bits at positions 1,2,3,5 set to 0)

**Codeword**: $\mathbf{x} = \mathbf{u} G_8 = [0, 0, 0, 1, 0, 0, 1, 0]$

---

## Successive Cancellation (SC) Decoding

**Optimal** for polarized channels (as N → ∞)

**Idea**: Decode bits sequentially, use previous decisions as side info

---

### Algorithm

**Receive**: $\mathbf{y} = [y_1, y_2, \ldots, y_N]$ (soft values or LLRs)

**For** $i = 1$ to $N$:

1. **If** $i \in \mathcal{A}^c$ (frozen): Set $\hat{u}_i = 0$

2. **If** $i \in \mathcal{A}$ (information):
   - Compute LLR: $L_i = \log\frac{P(u_i=0|\mathbf{y}, \hat{u}_1^{i-1})}{P(u_i=1|\mathbf{y}, \hat{u}_1^{i-1})}$
   - Decide: $\hat{u}_i = 0$ if $L_i > 0$, else $\hat{u}_i = 1$

**Recursive computation** (tree structure):

```
            [y₁, y₂, y₃, y₄]
                   |
         +---------+---------+
         |                   |
    [y₁⊕y₂, y₃⊕y₄]        [y₂, y₄]
         |                   |
      (decode u₁)        (decode u₂)
```

**Complexity**: $O(N \log N)$

---

### LLR Recursion

**Left child** (channel combining, worse):

$$
L_i^{(s)} = 2 \tanh^{-1}\left(\tanh\left(\frac{L_{2i-1}^{(s+1)}}{2}\right) \cdot \tanh\left(\frac{L_{2i}^{(s+1)}}{2}\right)\right)
$$

**Right child** (channel splitting, better):

$$
L_i^{(s)} = L_{2i}^{(s+1)} + (1 - 2\hat{u}_{2i-1}^{(s)}) L_{2i-1}^{(s+1)}
$$

**Where**: $s$ = Stage index (0 to $\log_2 N$)

---

## SC List (SCL) Decoding

**Problem**: SC is suboptimal for finite N

**Solution**: Keep **L candidate paths** (like Viterbi)

**SCL Algorithm**:

1. Start with single path (all frozen bits = 0)
2. At each information bit:
   - Branch each path (try 0 and 1)
   - Compute path metrics
   - **Keep L best paths** (prune others)
3. Select best final path

**List size** L = 2, 4, 8, 16, 32

**Performance**: SCL-32 ≈ ML performance (near-optimal)

---

### Path Metric

**Log-likelihood** for path:

$$
\text{PM} = \sum_{i=1}^{N} \log P(y_i | x_i)
$$

**Update**: Add branch metric for each decision

**Complexity**: $O(L \cdot N \log N)$

**Example**: L=8, N=1024 → ~8× SC complexity

---

## CRC-Aided Polar (CA-Polar)

**Problem**: SCL doesn't know which path is correct

**Solution**: Append **CRC** to data before encoding

**Decoding**:
1. SCL decoding produces L candidate paths
2. Check CRC for each path
3. **Select path with valid CRC**

**CRC length**: 8-24 bits (11-bit CRC typical for 5G)

**Performance**: CA-SCL-8 ≈ Turbo/LDPC (practical systems)

---

### 5G NR Implementation

**Control channels**: Use CA-Polar

**Parameters**:
- Block length: N = 512, 1024 (adaptable)
- Code rate: 1/12 to 1/2 (puncturing/shortening)
- CRC: 11-bit or 16-bit
- List size: L = 8

**Advantage**: Low latency (no iterations), good short-block performance

---

## Rate Matching

**5G supports flexible rates**: Puncturing, shortening, repetition

---

### Puncturing

**Transmit fewer bits** than N → Higher rate

**Method**: Don't transmit first $P$ bits (known frozen bits)

**Example**: N=512, K=256, puncture 128
- Transmit 384 bits
- Effective rate: 256/384 = 2/3

---

### Shortening

**Transmit fewer bits**, freeze last bits

**Method**: Set last $S$ input bits to 0 (frozen), don't transmit corresponding outputs

**Example**: N=512, K=200, shorten 112
- Effective N = 400
- Transmit 400 bits
- Rate: 200/400 = 1/2

---

### Repetition

**Transmit more bits** → Lower rate, more reliability

**Method**: Repeat some output bits

**Example**: N=256, K=64, repeat 256
- Transmit 512 bits
- Effective rate: 64/512 = 1/8

---

## Performance Analysis

### BER vs Eb/N0

**Typical performance** (rate 1/2, N=1024, CA-SCL-8):

| Eb/N0 (dB) | Uncoded | SC | SCL-8 | CA-SCL-8 | Shannon Limit |
|------------|---------|----|----|---------|---------------|
| 0 | 0.08 | 0.02 | 0.005 | 0.003 | 0 (capacity) |
| 1.0 | 0.02 | 0.005 | 8×10⁻⁴ | 5×10⁻⁴ | - |
| 1.5 | 0.01 | 2×10⁻³ | 10⁻⁵ | 10⁻⁶ | Gap ≈ 0.9 dB |
| 2.0 | 5×10⁻³ | 5×10⁻⁴ | 10⁻⁷ | 10⁻⁸ | - |

**Gap to Shannon**: 0.8-1.5 dB (CA-SCL-32, N ≥ 1024)

---

### Finite-Length Performance

**Short blocks** (N < 512): Polar competitive with Turbo/LDPC

**Long blocks** (N > 2048): Polar slightly behind LDPC

| Block Length | Code | Eb/N0 @ 10⁻⁵ | Gap to Shannon |
|--------------|------|--------------|----------------|
| **128** | Polar (SCL-8) | 2.5 dB | +2.0 dB |
| **512** | Polar (CA-SCL-8) | 1.5 dB | +1.2 dB |
| **1024** | Polar (CA-SCL-8) | 1.2 dB | +0.9 dB |
| **2048** | LDPC | 0.8 dB | +0.5 dB |

**Polar advantage**: Better short-block performance, lower latency

---

## Complexity Comparison

| Aspect | Polar (SC) | Polar (SCL-8) | Turbo | LDPC |
|--------|------------|---------------|-------|------|
| **Encoding** | $O(N \log N)$ | $O(N \log N)$ | $O(N)$ | $O(N)$ |
| **Decoding** | $O(N \log N)$ | $O(8N \log N)$ | $O(N \cdot I)$ | $O(N \cdot I)$ |
| **Latency** | Low | Low | High (iterations) | Moderate |
| **Memory** | $O(N \log N)$ | $O(8N \log N)$ | $O(N)$ | $O(N)$ |
| **Parallelism** | Sequential | Sequential | Parallel decoders | Highly parallel |

**Polar trade-off**: Low latency but harder to parallelize (sequential decoding)

---

## Advantages of Polar Codes

1. **Provably capacity-achieving**: Theoretical guarantee
2. **Low latency**: No iterations (SC/SCL)
3. **Short-block performance**: Good for N = 128-1024
4. **Systematic construction**: Explicit, no search (unlike LDPC)
5. **Flexible rate matching**: Puncture/shorten easily
6. **5G standardized**: Future-proof

---

## Disadvantages of Polar Codes

1. **Sequential decoding**: Hard to parallelize (vs LDPC)
2. **List decoder complexity**: SCL-8/32 needed for good performance
3. **Power-of-2 block lengths**: N = 2ⁿ (though can shorten)
4. **Slightly behind LDPC**: Long blocks (N > 2048)
5. **CRC overhead**: CA-Polar needs 11-24 bit CRC

---

## Practical Applications

### 1. 5G NR Control Channels

**eMBB** (Enhanced Mobile Broadband):
- DCI (Downlink Control Information)
- UCI (Uplink Control Information)
- Block lengths: 12-1706 bits (shortened from N=512, 1024)

**URLLC** (Ultra-Reliable Low-Latency):
- Short blocks (40-200 bits)
- Low latency (<1 ms)
- CA-Polar with CRC-11

**mMTC** (Massive Machine-Type): Future use

---

### 2. Future Satellite

**Low Earth Orbit (LEO)**: Short latency, bursty traffic
- Polar codes fit well (low-latency decoding)
- Adaptive rate matching (varying link quality)

---

### 3. IoT (Internet of Things)

**NB-IoT**: Narrowband, low power
- Short blocks (100-500 bits)
- Polar candidate for uplink control

---

## Code Construction Algorithms

### 1. Density Evolution (DE)

**Compute** $Z(W_i)$ or $I(W_i)$ for each subchannel

**Complexity**: $O(N \log N)$ preprocessing

**Accuracy**: Exact as N → ∞

---

### 2. Gaussian Approximation (GA)

**Approximate** subchannel distributions as Gaussian

**Mean**: $\mu_i$, **Variance**: $\sigma_i^2$

**Update rules** (simplified):

$$
\mu^- = \mu^2 / 2, \quad \mu^+ = 2\mu - \mu^2 / 2
$$

**Complexity**: $O(N)$ (faster than DE)

**Accuracy**: Good for practical SNR

---

### 3. Monte Carlo

**Simulate** SC decoding, count errors for each bit position

**Select K positions** with lowest error rate

**Complexity**: High (simulation-based)

**Accuracy**: Best for specific channel/SNR

---

## Python Example: Polar Encoder

```python
import numpy as np

def polar_transform(u):
    """Apply polar transform (Kronecker product construction)."""
    N = len(u)
    n = int(np.log2(N))
    x = u.copy()
    
    for stage in range(n):
        stride = 2 ** (n - stage - 1)
        for i in range(0, N, 2 * stride):
            for j in range(stride):
                x[i + j] = x[i + j] ^ x[i + j + stride]  # XOR
    
    return x

def polar_encode(data, frozen_set, N):
    """Encode using polar code.
    
    Args:
        data: Information bits (K bits)
        frozen_set: Indices of frozen bit positions (N-K positions)
        N: Code length (power of 2)
    
    Returns:
        Codeword (N bits)
    """
    K = len(data)
    u = np.zeros(N, dtype=int)
    
    # Information set = complement of frozen set
    info_set = [i for i in range(N) if i not in frozen_set]
    
    # Place data bits in information positions
    for idx, i in enumerate(info_set):
        u[i] = data[idx]
    
    # Frozen bits already 0
    
    # Apply polar transform
    x = polar_transform(u)
    
    return x

# Example: N=8, K=4
N = 8
K = 4

# Frozen set (worst 4 channels): positions 0,1,2,4 (0-indexed)
frozen_set = [0, 1, 2, 4]

# Information set: positions 3,5,6,7
# Data
data = np.array([1, 0, 1, 1])

# Encode
codeword = polar_encode(data, frozen_set, N)

print(f"Data (K={K}):      {data}")
print(f"Frozen set:        {frozen_set}")
print(f"Codeword (N={N}):  {codeword}")
print(f"Code rate:         {K}/{N} = {K/N}")

# Example output:
# Data (K=4):      [1 0 1 1]
# Frozen set:        [0, 1, 2, 4]
# Codeword (N=8):  [0 0 0 1 0 0 1 0]
# Code rate:         4/8 = 0.5
```

**Note**: SC/SCL decoding is complex (~200+ lines). Use libraries like `sionna` (TensorFlow) or custom MATLAB for research.

---

## Design Guidelines

### Choose Polar Codes When:

1. **5G NR** control channels (standardized)
2. **Short blocks** (100-1000 bits) with low latency
3. **Flexible rate matching** needed (puncture/shorten)
4. **Low-latency** critical (< 1 ms)
5. **Systematic construction** preferred (no random search)

### Avoid Polar Codes If:

1. **Long blocks** (> 2048 bits) → LDPC better
2. **Highest throughput** needed → LDPC more parallelizable
3. **No CRC available** → CA-Polar needs CRC for good performance
4. **Legacy systems** → Turbo/LDPC already deployed

---

## Comparison Summary

| Code | Year | Gap to Shannon | Latency | Parallelism | Standard |
|------|------|----------------|---------|-------------|----------|
| **Convolutional** | 1955 | 6 dB | Low | Sequential | GPS, WiFi |
| **Turbo** | 1993 | 0.5 dB | High | Moderate | 3G, 4G |
| **LDPC** | 1960/1996 | 0.3 dB | Moderate | High | 5G data, WiFi 6 |
| **Polar** | 2008 | 0.8 dB | Low | Sequential | **5G control** |

---

## Related Topics

- **[[Turbo-Codes]]**: Iterative near-capacity codes
- **[[LDPC-Codes]]**: Modern capacity-approaching codes
- **[[Convolutional-Codes-&-Viterbi-Decoding]]**: Classical FEC
- **[[Forward-Error-Correction-(FEC)]]**: General FEC overview
- **[[Shannon's-Channel-Capacity-Theorem]]**: Theoretical limit

---

**Key takeaway**: **Polar codes are the first provably capacity-achieving codes with explicit construction.** Channel polarization splits N channel uses into perfect + useless subchannels. Transmit data on good channels (information set $\mathcal{A}$), freeze bad channels. SC decoding: sequential, $O(N \log N)$ complexity. SCL decoding with CRC (CA-Polar) achieves near-optimal performance. 5G NR uses CA-Polar for control channels (low latency, good short-block performance). Gap to Shannon: 0.8-1.5 dB (CA-SCL-8, N=1024). Advantages: Low latency, short-block performance, systematic construction. Disadvantages: Sequential (hard to parallelize), slightly behind LDPC for long blocks. Generator matrix $G_N = G_2^{\otimes n}$ (Kronecker product). 2008 discovery by Arıkan—major theoretical milestone!

---

*This wiki is part of the [[Home|Chimera Project]] documentation.*
