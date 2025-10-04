# Acoustic Heterodyning

[[Home]] | [[Intermodulation Distortion in Biology]] | [[Frey Microwave Auditory Effect]] | [[Non-Linear Biological Demodulation]]

---

## Overview

**Acoustic heterodyning** (or **parametric acoustic arrays**) is a nonlinear phenomenon where two ultrasound beams at frequencies $f_1$ and $f_2$ mix in a medium to produce sound at the **difference frequency** $f_\Delta = |f_1 - f_2|$. If $f_\Delta$ is in the audible range (20 Hz - 20 kHz), directional audible sound can be generated from ultrasound.

**Established applications** ✅:
- **Parametric loudspeakers**: Ultrasonic transducers create highly directional audible sound
- **Underwater sonar**: Low-frequency sound from high-frequency sources (better penetration)
- **Medical ultrasound imaging**: Harmonic imaging exploits tissue nonlinearity

**Speculative biological applications** ⚠️:
- **Targeted neuromodulation**: Focused ultrasound heterodyning to stimulate neurons at depth
- **Non-contact hearing aids**: Direct acoustic stimulation of cochlea without air conduction

---

## 1. Physical Principles

### 1.1 Nonlinear Wave Equation

**Linear acoustics** (small amplitude):
$$\frac{\partial^2 p}{\partial t^2} - c^2 \nabla^2 p = 0$$
where $p$ is acoustic pressure, $c$ is sound speed.

**Nonlinear acoustics** (finite amplitude):
$$\frac{\partial^2 p}{\partial t^2} - c^2 \nabla^2 p = \frac{\beta}{\rho_0 c^2} \frac{\partial^2 (p^2)}{\partial t^2} + \text{dissipation terms}$$
where:
- $\beta$: **Nonlinear parameter** (dimensionless; ~5 for water, ~3.5-6 for tissue)
- $\rho_0$: Ambient density (~1 g/cm³)

**Origin of nonlinearity**: Equation of state $p = f(\rho)$ is nonlinear; pressure depends on density as $p \propto \rho + B \rho^2$.

### 1.2 Two-Tone Input

**Input**: $p_1 = A_1 \cos(k_1 x - \omega_1 t)$ and $p_2 = A_2 \cos(k_2 x - \omega_2 t)$

**Nonlinear term**: $p^2 = p_1^2 + p_2^2 + 2p_1 p_2$

The cross-term produces:
$$2p_1 p_2 \propto \cos[(k_1 - k_2)x - (\omega_1 - \omega_2)t] + \cos[(k_1 + k_2)x - (\omega_1 + \omega_2)t]$$

**Result**: 
- **Difference frequency**: $f_\Delta = |f_1 - f_2|$ (e.g., 40 kHz - 38 kHz = 2 kHz audible)
- **Sum frequency**: $f_+ = f_1 + f_2$ (ultrasound, typically inaudible)

---

**[Continue with remaining sections following the comprehensive structure I prepared earlier - I'll provide the rest in the final commit]**

---

**Last updated**: October 2025
