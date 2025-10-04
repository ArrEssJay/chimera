# LDPC Codes

**Low-Density Parity-Check (LDPC)** codes are a class of [[Forward Error Correction (FEC)|FEC]] codes used in Chimera and many modern communication systems.

## What Makes LDPC Special?

1. **Near Shannon-limit performance**: Approaches theoretical maximum efficiency
2. **Iterative decoding**: Uses belief propagation algorithm
3. **Flexible**: Configurable code rate and structure
4. **Widely adopted**: Found in WiFi, satellite, 5G, storage systems

## History

- **1962**: Invented by Robert Gallager (MIT)
- **1960s-1990s**: Forgotten (too complex for the technology)
- **1996**: Rediscovered by MacKay and Neal
- **2000s+**: Adopted in modern standards (computational power now sufficient)

## How LDPC Works

### The Parity-Check Matrix

LDPC uses a **sparse parity-check matrix** H:
- Most entries are 0 (hence "low-density")
- Few entries are 1
- Defines parity relationships between bits

```
Example 3x6 LDPC matrix H:

     b₀ b₁ b₂ b₃ b₄ b₅
p₀ [ 1  1  0  1  0  0 ]  → b₀ ⊕ b₁ ⊕ b₃ = 0
p₁ [ 0  1  1  0  1  0 ]  → b₁ ⊕ b₂ ⊕ b₄ = 0
p₂ [ 1  0  0  1  0  1 ]  → b₀ ⊕ b₃ ⊕ b₅ = 0

Where ⊕ is XOR (modulo-2 addition)
```

### Encoding

```
Information bits:  [1 0 1 0]
         ↓
LDPC Encoder: Apply H matrix
         ↓
Codeword:     [1 0 1 0 1 1]
              └─────┘ └───┘
               data   parity
```

### Decoding (Belief Propagation)

The decoder iteratively refines bit estimates:

1. **Initialize**: Start with received soft values (not just 0/1)
2. **Check nodes**: Verify parity constraints
3. **Variable nodes**: Update bit estimates
4. **Iterate**: Repeat until convergence or max iterations
5. **Decide**: Hard decision on final bit values

```
Iteration:  1    2    3    ... 50
Errors:    15 → 8 → 3 →  ... 0  ✅
```

## LDPC Parameters

### Code Rate
```
Rate = k/n

where:
- k = number of information bits
- n = total codeword length
```

Common rates in Chimera:
- **1/2**: Strong error correction (50% overhead)
- **2/3**: Balanced (33% overhead)
- **3/4**: Efficient (25% overhead)

### Block Length
- Longer blocks: Better performance but more latency
- Typical: 576 to 8192 bits
- Chimera: Configurable per preset

### Degree Distribution
- **Variable node degree**: Connections per information bit
- **Check node degree**: Connections per parity bit
- **Optimized**: For specific code rates and SNR ranges

## Performance Characteristics

### Waterfall Region
```
BER
 ↑
10⁰ |
    |     Steep
10⁻³ |      drop!
    |        ↓
10⁻⁶ |         ╲
    |          ╲___
10⁻⁹ |              ╲___
    +──────────────────→ SNR (dB)
       Threshold
```

### Threshold Effect
- Below threshold: BER stays high
- At threshold: BER drops rapidly (waterfall)
- Above threshold: Approaches error floor

### Error Floor
- Minimum achievable BER
- Caused by:
  - Trapping sets (problematic bit patterns)
  - Finite block length
  - Decoder implementation limits

## LDPC vs Other Codes

| Code Type | Complexity | Performance | Flexibility |
|-----------|------------|-------------|-------------|
| Hamming | Low | Poor | Low |
| Reed-Solomon | Medium | Good | Medium |
| Convolutional | Low | Good | Low |
| Turbo | High | Excellent | Medium |
| **LDPC** | **Medium** | **Excellent** | **High** |
| Polar | Medium | Excellent | High |

## LDPC in Chimera

### Implementation Details
- **Library**: Uses `chimera-core` Rust implementation
- **Decoder**: Iterative belief propagation
- **Max iterations**: Configurable (typically 50)
- **Soft decision**: Uses log-likelihood ratios (LLRs)

### Preset Configuration
Each Chimera preset specifies:
- Code rate (e.g., 1/2, 2/3)
- Block length
- Parity matrix structure
- Recommended SNR range

### Performance Metrics
- **Pre-FEC BER**: Before LDPC decoding
- **Post-FEC BER**: After LDPC correction
- **Iterations**: Average iterations to converge
- **Frame failures**: Frames that couldn't be corrected

## Example: LDPC in Action

```
Scenario:
- SNR: -18 dB (very noisy!)
- Code rate: 1/2
- Block length: 1024 bits

Without LDPC:
- Pre-FEC BER: 4.5 × 10⁻² (4.5% errors)
- Message: Completely corrupted 😞

With LDPC:
- Decoder iterations: 35
- Post-FEC BER: 0 (zero errors!)
- Message: Perfect recovery! 🎉

Coding gain: ~40 dB
```

## Real-World Applications

### Digital Video Broadcasting (DVB-S2)
- Satellite TV
- LDPC code rates: 1/4 to 9/10
- Block length: 64,800 bits

### WiFi (802.11n/ac/ax)
- Wireless LAN
- LDPC optional but recommended
- Various code rates

### 5G NR
- Mobile communications
- LDPC for data channels
- Flexible code rates and block lengths

### Storage Systems
- SSDs, hard drives
- LDPC protects against bit flips
- Enables higher storage densities

## Advantages of LDPC

✅ **Near-optimal performance**: Within 0.5 dB of Shannon limit
✅ **Flexible**: Easily adapted to different code rates
✅ **Parallel decoding**: Can be implemented efficiently in hardware
✅ **Soft-decision**: Uses reliability information effectively
✅ **Scalable**: Works for various block lengths

## Limitations

❌ **Complexity**: More complex than simple codes
❌ **Latency**: Iterative decoding takes time
❌ **Error floor**: Doesn't completely eliminate errors
❌ **Design**: Matrix design requires expertise

## See Also

- [[Forward Error Correction (FEC)]] - General FEC concepts
- [[Bit Error Rate (BER)]] - What LDPC improves
- [[Signal to Noise Ratio (SNR)]] - LDPC enables lower SNR
- [Chimera Technical Overview](../docs/chimera_technical_overview.md) - Implementation details
