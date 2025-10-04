# THz Propagation in Biological Tissue

[[Home]] | [[Terahertz (THz) Technology]] | [[THz Bioeffects Thermal and Non-Thermal]] | [[mmWave & THz Communications]]

---

## Overview

**Terahertz (THz) radiation** (0.1-10 THz, 30 μm-3 mm wavelength) occupies the spectral gap between microwaves and infrared. THz waves interact strongly with biological tissue due to resonances with molecular vibrations and rotations, particularly water. Understanding THz propagation is critical for:
- **Medical imaging**: Cancer detection, burn assessment
- **Security**: Concealed weapons/explosives detection through clothing
- **Neuromodulation**: Speculative applications in neural stimulation (see [[AID Protocol Case Study]])

---

## Quick Start for Non-Technical Readers

**What is THz radiation?** Think of it as invisible light that sits between microwaves (used in your microwave oven) and infrared (heat you feel from a fireplace). It can pass through some materials but not others.

**Why study it in tissue?** Scientists and doctors want to use THz waves to see inside skin without cutting it open—like an X-ray, but safer and better for soft tissue.

**The main challenge: Water blocks THz waves.** Your body is mostly water, and water absorbs THz radiation very strongly. This means:
- ✅ **Good news**: THz waves can create detailed images of skin and surface tissues
- ❌ **Bad news**: They can't penetrate deep (only a fraction of a millimeter)

**Real-world analogy**: Imagine shining a flashlight through fog. The light gets absorbed quickly, so you can only see a short distance. THz waves in wet tissue behave the same way.

**Key takeaways**:
1. **THz imaging works for skin**: Doctors can detect skin cancer, assess burn depth, or check dental cavities
2. **THz cannot image deep organs**: Unlike X-rays, THz stops at the surface (it can't see your heart or brain through skin)
3. **Safety**: THz is non-ionizing (unlike X-rays), so it doesn't damage DNA. At low power, it's considered safe
4. **The physics**: Water molecules spin in response to THz waves, absorbing energy and turning it into heat

**Who uses this?**
- Dermatologists (skin doctors) for cancer detection
- Security personnel for airport body scanners
- Researchers exploring futuristic applications (like non-invasive brain stimulation—still very experimental)

**Want more detail?** The sections below explain the physics, but you now have the gist!

---

## 1. Electromagnetic Properties of Biological Tissue at THz Frequencies

### 1.1 Complex Permittivity

Biological tissue's response to EM waves is characterized by **complex relative permittivity**:
$$\epsilon_r(\omega) = \epsilon'(\omega) - i\epsilon''(\omega)$$
where:
- $\epsilon'$: Real part (polarization, refractive index)
- $\epsilon''$: Imaginary part (absorption, dissipation)

**Refractive index**: $n = \sqrt{\epsilon' \mu_r} \approx \sqrt{\epsilon'}$ (assuming $\mu_r \approx 1$ for non-magnetic tissue)

**Absorption coefficient**:
$$\alpha(\omega) = \frac{\omega}{c} \sqrt{\frac{\epsilon'}{2} \left( \sqrt{1 + \left(\frac{\epsilon''}{\epsilon'}\right)^2} - 1 \right)} \approx \frac{\omega \epsilon''}{2cn}$$
for $\epsilon'' \ll \epsilon'$.

### 1.2 Frequency Dependence

**Water dominates** (tissue is ~70-90% water by mass):

**Debye relaxation model** (single relaxation time):
$$\epsilon_r(\omega) = \epsilon_\infty + \frac{\epsilon_s - \epsilon_\infty}{1 + i\omega \tau}$$
where:
- $\epsilon_s$: Static permittivity (~80 for water at DC)
- $\epsilon_\infty$: High-frequency limit (~5 for water)
- $\tau$: Relaxation time (~8 ps for bulk water at 20°C)

**Relaxation frequency**: $f_0 = 1/(2\pi \tau) \approx 20$ GHz

**At THz frequencies** (0.1-10 THz >> 20 GHz):
- $\epsilon'$ approaches $\epsilon_\infty \approx 5$
- $\epsilon''$ increases linearly with frequency (Lorentzian tail)
- **Absorption increases with frequency**: $\alpha \propto \omega \epsilon''(\omega)$

### 1.3 Hydration State

**Free vs. bound water**:
- **Free water**: Bulk-like rotational dynamics, strong THz absorption
- **Bound water**: Near protein surfaces, restricted rotation, reduced absorption

**Tissue hydration** varies:
- **Skin (stratum corneum)**: ~20% water → lower absorption
- **Muscle**: ~75% water → high absorption
- **Fat (adipose)**: ~10% water → low absorption (relatively transparent at THz)

**Temperature dependence**: Absorption increases with temperature (faster molecular relaxation)

---

## 2. Absorption Mechanisms

### 2.1 Water Rotational Modes

**Dominant mechanism**: Dipolar water molecules rotate in response to THz electric field.

**Debye absorption peak**: ~20 GHz (microwave range)
**THz tail**: Absorption continues into THz due to:
- Hindered rotations (librational modes)
- Collective hydrogen bond network dynamics

**Absorption coefficient** (water at 1 THz, 20°C): $\alpha \approx 250$ cm⁻¹
**Penetration depth**: $\delta = 1/\alpha \approx 40$ μm (very shallow!)

### 2.2 Protein Vibrational Modes

**Proteins contribute** secondary absorption:
- **Low-frequency modes** (0.1-3 THz): Collective vibrations, domain motions
- **Amide bands**: ~6 THz (C=O stretch overtones)

**Effect**: Protein-rich tissues (e.g., collagen in dermis) have enhanced absorption at specific frequencies.

### 2.3 Lipid and Membrane Absorption

**Lipids**: Lower absorption than water/protein
- **Fatty acids**: CH₂ rocking modes at ~2-4 THz
- **Phospholipid headgroups**: Hydrated, contribute dielectric relaxation

**Cell membranes**:
- Thin (~7 nm lipid bilayer) → minimal direct absorption
- But membrane-associated water has altered dynamics

---

## 3. Scattering Mechanisms

### 3.1 Rayleigh Scattering (Small Particles)

**Condition**: Particle size $d \ll \lambda$ (THz wavelength ~100 μm)

**Scattering cross-section**:
$$\sigma_s \propto \left(\frac{d}{\lambda}\right)^4 \propto \omega^4$$

**In tissue**:
- **Organelles** (mitochondria ~1 μm, lysosomes ~0.5 μm): Weak Rayleigh scattering
- **Cellular nuclei** (~10 μm): Transition to Mie regime

**Result**: Scattering is weak compared to absorption at THz frequencies (unlike visible light, where scattering dominates in tissue)

### 3.2 Mie Scattering (Comparable Size)

**Condition**: Particle size $d \approx \lambda$

**Applicable to**:
- Cells (~10-20 μm diameter) at low THz (~0.3 THz, $\lambda \approx 1000$ μm → Rayleigh regime)
- Cells at high THz (~3 THz, $\lambda \approx 100$ μm → Mie regime)

**Mie theory**: Complex calculation; depends on refractive index contrast and particle geometry.

### 3.3 Interface Reflections

**Fresnel reflection** at interfaces with refractive index mismatch:
$$R = \left| \frac{n_1 - n_2}{n_1 + n_2} \right|^2$$

**Tissue interfaces**:
- **Air-skin**: $n_1 = 1$, $n_2 \approx 2.2$ → $R \approx 0.15$ (15% reflected)
- **Dermis-fat**: Small contrast → $R < 0.01$
- **Tissue-bone**: Large contrast → $R \approx 0.2$

**Implication**: Surface reflections significant; impedance matching needed for efficient coupling

---

## 4. Penetration Depth

### 4.1 Beer-Lambert Law

Intensity decays exponentially:
$$I(z) = I_0 e^{-\alpha z}$$

**Penetration depth** (1/e attenuation):
$$\delta = \frac{1}{\alpha}$$

### 4.2 Frequency Dependence

**Typical values** (in vivo human tissue):

| Frequency | Absorption (cm⁻¹) | Penetration depth |
|-----------|-------------------|-------------------|
| 0.1 THz   | ~10               | 1 mm              |
| 0.5 THz   | ~50               | 200 μm            |
| 1.0 THz   | ~200              | 50 μm             |
| 3.0 THz   | ~700              | 15 μm             |

**Key trend**: Penetration decreases rapidly with frequency. At 1 THz, THz waves barely penetrate beyond epidermis (~100 μm thick).

### 4.3 Tissue-Specific Penetration

**Stratum corneum (dry outer skin)**: Lower water content → deeper penetration (~500 μm at 0.5 THz)
**Muscle**: High water → shallow (~50 μm at 1 THz)
**Fat**: Low water → relatively transparent (~2 mm at 1 THz)
**Bone**: High mineral content, low water → moderate absorption

**Clinical implication**: THz imaging ideal for skin pathology (basal cell carcinoma, burns); poor for deep tissue imaging.

---

## 5. Wave Propagation Models

### 5.1 Plane Wave Approximation

**Assumption**: Infinite homogeneous medium

**Electric field**:
$$E(z,t) = E_0 e^{-\alpha z/2} e^{i(kz - \omega t)}$$
where $k = \omega n/c$ is the wavenumber.

**Limitations**: Ignores interfaces, scattering, finite beam effects

### 5.2 Stratified Media Model

**Skin structure**: Multi-layer (stratum corneum, epidermis, dermis, fat)

**Transfer matrix method**:
1. Divide tissue into $N$ layers
2. Apply boundary conditions at each interface (Fresnel reflection/transmission)
3. Multiply transfer matrices: $\mathbf{M}_{\text{total}} = \mathbf{M}_N \cdots \mathbf{M}_2 \mathbf{M}_1$
4. Calculate total reflection/transmission

**Result**: Oscillatory reflection spectrum due to interference (etalon effect in thin layers)

### 5.3 Diffusion Approximation

**When scattering dominates** (rare in THz):
$$\nabla^2 U - \frac{U}{L^2} = -S$$
where $U$ is fluence rate, $L = 1/\sqrt{3\mu_a \mu_s'}$ is diffusion length ($\mu_a$ = absorption, $\mu_s'$ = reduced scattering)

**Not applicable** to most THz tissue scenarios (absorption >> scattering)

---

## 6. Applications

### 6.1 Medical Imaging ✅ (Established)

**THz time-domain spectroscopy (THz-TDS)**:
- Ultrafast THz pulse ($\sim$ps duration) transmitted/reflected from tissue
- Time-of-flight → layer thickness
- Spectral features → molecular composition

**Clinical applications**:
- **Skin cancer detection**: Basal cell carcinoma has altered water content → contrast
- **Burn depth assessment**: Damaged tissue has different THz signature
- **Dental imaging**: Caries detection (demineralized enamel has higher water)

**Limitations**: Shallow penetration, slow acquisition (raster scanning), requires contact or near-contact

### 6.2 Security Screening ✅ (Established)

**Through-clothing imaging**:
- THz transparent to fabrics (low water)
- Opaque to skin (high water)
- Detect concealed weapons, explosives

**Systems**: Passive THz cameras (detect natural thermal emission) or active illumination

**Privacy concerns**: Can image body contours → "virtual strip search"

### 6.3 Neuromodulation ⚠️ (Speculative)

**Hypothesis**: THz pulses could stimulate neurons non-invasively.

**Mechanisms** (proposed):
- **Thermal**: Localized heating triggers temperature-sensitive ion channels (TRPV1)
- **Non-thermal**: THz resonates with protein vibrational modes → conformational changes
- **Quantum**: Vibronic coupling in microtubules (see [[THz Resonances in Microtubules]])

**Challenges**:
- Penetration: THz cannot reach deep brain structures transcranially (skull absorption + scalp)
- Power: High intensity needed → thermal damage risk
- Specificity: How to target specific neuron populations?

**Current status**: In vitro studies show weak effects; in vivo neuromodulation not demonstrated

---

## 7. Measurement Techniques

### 7.1 THz Time-Domain Spectroscopy (THz-TDS)

**Setup**:
1. Femtosecond laser generates THz pulse via photoconductive antenna or nonlinear crystal
2. THz pulse transmitted through sample
3. Detected via electro-optic sampling (time-resolved electric field)

**Advantages**: Phase-sensitive (extract $\epsilon'$ and $\epsilon''$), broadband (~0.1-5 THz)
**Disadvantages**: Slow (mechanical delay line), expensive

### 7.2 Continuous-Wave (CW) THz Systems

**Quantum cascade lasers (QCLs)**: High-power, narrow-band THz sources

**Advantage**: Fast imaging, compact
**Disadvantage**: Single frequency (must scan laser to get spectrum)

### 7.3 In Vivo Reflection Geometry

**Challenge**: Most tissue measurements done in transmission; in vivo requires reflection mode.

**Reflection coefficient**:
$$r(\omega) = \frac{E_{\text{ref}}(\omega)}{E_{\text{inc}}(\omega)}$$
Extract optical properties via model fitting (stratified media)

**Confounding factors**: Surface roughness, sweat, air gaps

---

## 8. Safety Considerations

**Thermal effects**: Dominant safety concern

**ICNIRP guidelines** (International Commission on Non-Ionizing Radiation Protection):
- **Power density limit**: 10 mW/cm² (averaged over 6 minutes) for 0.3-3 THz
- **Temperature rise**: Must stay below 1°C for prolonged exposure

**Non-thermal effects**: Controversial (see [[THz Bioeffects Thermal and Non-Thermal]])

**Current assessment**: THz radiation is non-ionizing (photon energy ~4 meV << 13.6 eV ionization potential). At low intensities (<1 W/cm²), considered safe for brief exposure.

---

## 9. Connections to Other Wiki Pages

- [[Terahertz (THz) Technology]] — THz sources and detectors
- [[THz Bioeffects Thermal and Non-Thermal]] — Biological effects
- [[THz Resonances in Microtubules]] — Speculative quantum effects
- [[mmWave & THz Communications]] — Wireless applications
- [[Free-Space Path Loss (FSPL)]] — Propagation fundamentals

---

## 10. References

### Tissue Optical Properties
1. **Pickwell & Wallace, *J. Phys. D* 39, R301 (2006)** — THz tissue review
2. **Smye et al., *Phys. Med. Biol.* 46, R101 (2001)** — Tissue dielectric properties

### Medical Imaging
3. **Woodward et al., *Phys. Med. Biol.* 47, 3853 (2002)** — THz skin imaging
4. **Wallace et al., *Br. J. Dermatol.* 151, 424 (2004)** — Basal cell carcinoma detection

### Propagation Models
5. **Pickwell et al., *Appl. Phys. Lett.* 84, 2190 (2004)** — Stratified media modeling

### Safety
6. **ICNIRP, *Health Phys.* 99, 818 (2010)** — THz exposure guidelines

---

**Last updated**: October 2025