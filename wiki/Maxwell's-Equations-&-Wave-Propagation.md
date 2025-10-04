# Maxwell's Equations & Wave Propagation

**Maxwell's Equations** are the fundamental laws of electromagnetism, describing how electric and magnetic fields interact and propagate through space.

---

## 🌟 For Non-Technical Readers

**What are Maxwell's Equations?**

Imagine you're trying to understand how your phone communicates with a cell tower, how light travels from the sun to Earth, or how a radio picks up music from thin air. All of these phenomena are explained by four elegant mathematical rules discovered by James Clerk Maxwell in the 1860s.

**The Big Picture (in plain English):**

1. **Electric charges create invisible "force fields"** around them
   - Think of static electricity making your hair stand up
   - Positive and negative charges attract or repel each other through these fields

2. **Magnetic fields always come in pairs** (north and south poles together)
   - You can't have a magnet with just a north pole or just a south pole
   - If you break a magnet in half, you get two smaller magnets, each with both poles

3. **Changing magnetic fields create electric fields**
   - This is how generators work: spin a magnet near a wire, and electricity flows
   - It's also why transformers can change voltage levels

4. **Moving electric charges (currents) and changing electric fields create magnetic fields**
   - This is how electromagnets work
   - It's also how antennas transmit radio waves

**Why does this matter?**

Maxwell discovered something profound: when you combine these four rules, they predict that electromagnetic "waves" can travel through empty space at a specific speed. When he calculated that speed, it turned out to be exactly the speed of light!

This meant **light itself is an electromagnetic wave** - the same type of wave as radio, WiFi, X-rays, and microwaves, just at different frequencies.

**Real-world impact:**
- 📱 Every wireless device (phone, WiFi, Bluetooth, GPS)
- 💡 All lighting and solar panels
- 📡 Radio, TV, and satellite communication
- 🏥 Medical imaging (MRI, X-rays)
- 🌈 Why the sky is blue and sunsets are red
- 👁️ How your eyes see color

Without Maxwell's Equations, the modern wireless world wouldn't exist!

**What you'll find below:**

The rest of this page dives into the mathematical details. Don't worry if the equations look intimidating - the key concepts above are what matter for understanding how electromagnetic waves work in practice.

---

## 📐 The Four Maxwell's Equations

### In Differential Form (Local)

**1. Gauss's Law** (Electric charge creates electric field)
```
∇·E = ρ/ε₀

where:
- E = electric field vector (V/m)
- ρ = charge density (C/m³)
- ε₀ = permittivity of free space (8.854×10⁻¹² F/m)
```

**Physical meaning**: Electric field lines originate from positive charges and terminate on negative charges.

---

**2. Gauss's Law for Magnetism** (No magnetic monopoles)
```
∇·B = 0

where:
- B = magnetic field vector (Tesla)
```

**Physical meaning**: Magnetic field lines always form closed loops (no isolated north/south poles).

---

**3. Faraday's Law** (Changing magnetic field creates electric field)
```
∇×E = -∂B/∂t

where:
- ∇× = curl operator (measures rotation)
- ∂B/∂t = time rate of change of B
```

**Physical meaning**: A time-varying magnetic field induces a circulating electric field (basis of generators, transformers).

---

**4. Ampère-Maxwell Law** (Current + changing electric field creates magnetic field)
```
∇×B = μ₀J + μ₀ε₀ ∂E/∂t

where:
- μ₀ = permeability of free space (4π×10⁻⁷ H/m)
- J = current density (A/m²)
- ∂E/∂t = displacement current (Maxwell's addition!)
```

**Physical meaning**: Moving charges (current) AND time-varying electric fields create circulating magnetic fields.

**Maxwell's insight**: The ∂E/∂t term was missing from Ampère's original law. Adding it made electromagnetic waves possible!

---

## 🌊 The Wave Equation

### Derivation

Taking curl of Faraday's law:
```
∇×(∇×E) = -∇×(∂B/∂t) = -∂(∇×B)/∂t
```

Substitute Ampère-Maxwell law (in vacuum, J=0):
```
∇×(∇×E) = -μ₀ε₀ ∂²E/∂t²
```

Use vector identity: ∇×(∇×E) = ∇(∇·E) - ∇²E

In vacuum (ρ=0), Gauss's law gives ∇·E = 0, so:
```
∇²E = μ₀ε₀ ∂²E/∂t²
```

**This is the wave equation!**

Similar derivation for B gives:
```
∇²B = μ₀ε₀ ∂²E/∂t²
```

---

### Wave Speed

The standard wave equation is:
```
∇²f = (1/v²) ∂²f/∂t²
```

Comparing to electromagnetic wave equation:
```
v² = 1/(μ₀ε₀)

v = 1/√(μ₀ε₀) 
  = 1/√[(4π×10⁻⁷)(8.854×10⁻¹²)]
  = 2.998×10⁸ m/s
  = c (speed of light!)
```

**Maxwell's triumph**: Light is an electromagnetic wave!

---

## 📻 Plane Wave Solutions

### General Solution

For propagation in +z direction:
```
E(z,t) = E₀ cos(kz - ωt + φ) x̂
B(z,t) = B₀ cos(kz - ωt + φ) ŷ

where:
- k = 2π/λ = wave number (rad/m)
- ω = 2πf = angular frequency (rad/s)
- λ = wavelength (m)
- f = frequency (Hz)
- φ = phase constant
```

**Relationship between E and B**:
```
B₀ = E₀/c

B = (1/c) k̂ × E

where k̂ is propagation direction
```

**Key insight**: E and B are perpendicular to each other AND to propagation direction (transverse wave).

---

### Dispersion Relation

From wave equation:
```
ω = ck  (in vacuum)

or:  v = fλ  (wave speed = frequency × wavelength)
```

**In vacuum**: All frequencies travel at same speed c (non-dispersive)

**In matter**: v = c/n (where n = refractive index, depends on frequency → dispersion)

---

## ⚡ Energy and Power

### Energy Density

**Electric field energy density**:
```
u_E = (1/2)ε₀E²  (J/m³)
```

**Magnetic field energy density**:
```
u_B = (1/2μ₀)B²  (J/m³)
```

**Total electromagnetic energy density**:
```
u = u_E + u_B = ε₀E²  (since B = E/c and c² = 1/μ₀ε₀)
```

---

### Poynting Vector (Power Flow)

**Poynting vector** S points in direction of energy flow:
```
S = (1/μ₀) E × B  (W/m²)

Magnitude: |S| = (1/μ₀c) E₀²  (for plane wave)
```

**Physical meaning**: Energy flux (power per unit area) carried by EM wave.

**Power through area A**:
```
P = ∫∫ S·dA  (Watts)
```

---

### Intensity

For time-harmonic wave, **intensity** (time-averaged power density):
```
I = <|S|> = (1/2μ₀c) E₀² = (cε₀/2) E₀²

or in terms of B:
I = (c/2μ₀) B₀²
```

**Units**: W/m² (same as irradiance, power density)

---

## 📡 Radiation from Sources

### Dipole Radiation

**Oscillating electric dipole** (simplest antenna):
```
Radiated power:
P = (μ₀/12πc) ω⁴ p₀²

where:
- ω = oscillation frequency
- p₀ = dipole moment amplitude
```

**Key insight**: Radiated power ∝ ω⁴ (higher frequencies radiate much more efficiently!)

**Radiation pattern**: Doughnut shape (maximum perpendicular to dipole, zero along dipole axis)

---

### Accelerating Charges

**Larmor formula** (non-relativistic):
```
P = (μ₀q²a²)/(6πc)

where:
- q = charge
- a = acceleration
```

**Physical meaning**: Any accelerating charge radiates EM waves. This is basis of:
- Antennas (oscillating current = accelerating charges)
- Synchrotron radiation (electrons in magnetic fields)
- Bremsstrahlung (decelerating electrons)

---

## 🌍 Propagation in Media

### Material Properties

**Permittivity** ε: How much material opposes electric field
- Vacuum: ε₀
- Material: ε = ε_r ε₀ (where ε_r = relative permittivity)

**Permeability** μ: How much material opposes magnetic field
- Vacuum: μ₀
- Material: μ = μ_r μ₀ (where μ_r = relative permeability)

**Conductivity** σ: How well material conducts current
- Insulator: σ ≈ 0
- Conductor: σ → ∞ (ideally)

---

### Wave Speed in Media

```
v = 1/√(εμ) = c/√(ε_r μ_r) = c/n

where n = √(ε_r μ_r) is refractive index
```

**Examples**:
- Air: n ≈ 1.0003 (v ≈ c)
- Water: n ≈ 1.33 (v ≈ 0.75c)
- Glass: n ≈ 1.5 (v ≈ 0.67c)

---

### Attenuation in Lossy Media

In conductive medium, wave amplitude decays:
```
E(z) = E₀ e^(-αz) cos(kz - ωt)

where α = skin depth parameter:
α = √(πfμσ)  (for good conductors)

Skin depth: δ = 1/α (depth where amplitude drops to 1/e)
```

**Examples** (at 1 GHz):
- Copper: δ ≈ 2 μm (EM waves don't penetrate conductors!)
- Seawater: δ ≈ 0.2 m (poor penetration)
- Air: δ → ∞ (negligible loss)

---

## 📊 Frequency Spectrum

Maxwell's equations apply to **all frequencies**:

| Band | Frequency | Wavelength | Applications |
|------|-----------|------------|--------------|
| **ELF** | 3-30 Hz | 10,000-100,000 km | Submarine communication |
| **VLF** | 3-30 kHz | 10-100 km | Navigation |
| **LF** | 30-300 kHz | 1-10 km | AM radio |
| **MF** | 300 kHz-3 MHz | 100-1000 m | AM broadcast |
| **HF** | 3-30 MHz | 10-100 m | Shortwave |
| **VHF** | 30-300 MHz | 1-10 m | FM radio, TV |
| **UHF** | 300 MHz-3 GHz | 10 cm-1 m | Cell phones, WiFi |
| **SHF** | 3-30 GHz | 1-10 cm | Radar, satellite |
| **EHF** | 30-300 GHz | 1-10 mm | mmWave, 5G |
| **THz** | 0.3-3 THz | 0.1-1 mm | Imaging, spectroscopy |
| **IR** | 300 THz-430 THz | 700 nm-1 mm | Thermal imaging |
| **Visible** | 430-750 THz | 400-700 nm | Human vision |
| **UV** | 750 THz-30 PHz | 10-400 nm | Sterilization |
| **X-ray** | 30 PHz-30 EHz | 0.01-10 nm | Medical imaging |
| **Gamma** | > 30 EHz | < 0.01 nm | Nuclear medicine |

**All obey Maxwell's equations!** (though quantum effects important at high frequencies)

---

## 🔑 Key Insights

1. **Unification**: Electricity, magnetism, and light are different manifestations of the same phenomenon
2. **Self-propagation**: EM waves don't need a medium (unlike sound)
3. **Speed limit**: c is the maximum speed in universe (relativity!)
4. **Transverse**: E, B, and propagation direction are mutually perpendicular
5. **Duality**: E and B are inseparable (changing one creates the other)
6. **Scale invariance**: Same equations for radio → gamma rays (though quantum effects matter at high f)

---

## 🔗 See Also

- [[Electromagnetic-Spectrum]] - Detailed frequency breakdown
- [[Antenna-Theory-Basics]] - How to radiate/receive EM waves
- [[Wave-Polarization]] - E field orientation
- [[Free-Space Path Loss]] - How waves weaken with distance
- [[Terahertz-(THz)-Technology]] - Specific THz band applications

---

## 📚 References

1. **Maxwell, J.C.** (1865) "A Dynamical Theory of the Electromagnetic Field" *Phil. Trans. R. Soc.* 155, 459-512
2. **Jackson, J.D.** (1999) *Classical Electrodynamics* 3rd ed. (Wiley)
3. **Griffiths, D.J.** (2017) *Introduction to Electrodynamics* 4th ed. (Cambridge UP)
4. **Feynman, R.P., Leighton, R.B., Sands, M.** (1964) *The Feynman Lectures on Physics* Vol. 2 (Addison-Wesley)
