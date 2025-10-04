# Supporting information: Vibrational electronic-thermofield coupled cluster (VE-TFCC) theory for quantum simulations of vibronic coupling systems at thermal equilibrium

**Songhao Bao**$^{*,\dagger}$, **Neil Raymond**$^{\dagger}$, **Tao Zeng**$^{*,\ddagger}$, and **Marcel Nooijen**$^{*,\dagger}$

$^{\dagger}$Department of Chemistry, University of Waterloo, Waterloo, Ontario, Canada, N2L 3G1
$^{\ddagger}$Department of Chemistry, York University, Toronto, Ontario, Canada, M3J 1P3

E-mail: bsonghao@uwaterloo.ca; tzeng@yorku.ca; nooijen@uwaterloo.ca

***

## S.1 Proof of the vacuum state condition

In this section, we give a rigorous proof that the vacuum state condition described in Eq. (26) and Eq. (27) are satisfied in the thermofield coupled cluster (TFCC) formulation. Applying Baker-Campbell-Hausdorff (BCH) expansion and canonical commutation rules, we can prove that the Bosonic construction operators $(\hat{i}, \hat{i}^\dagger, \hat{i}, \hat{i}^\dagger)$ satisfy the following equations:
$$\begin{aligned} e^{-\hat{u}\hat{i}} e^{\hat{i}^\dagger} &= \hat{i} + \hat{i}^\dagger, \\ e^{-\hat{u}\hat{i}^\dagger} e^{\hat{i}} &= \hat{i}^\dagger, \\ e^{-\hat{z}\hat{i}} e^{\hat{i}^\dagger} &= \hat{i} + \hat{i}^\dagger, \\ e^{-\hat{z}\hat{i}^\dagger} e^{\hat{z}} &= \hat{i}^\dagger, \end{aligned} \quad \quad (S.1)$$
where $\hat{u} \stackrel{\text{def}}{=} \sum_i \hat{i} \hat{i}^\dagger \hat{i}^\dagger$.

Similarly, applying BCH, we can prove that
$$\begin{aligned} e^{\hat{\beta} \hat{h}_0/2} \hat{i} e^{-\hat{\beta} \hat{h}_0/2} &= e^{-\hat{\beta} \omega_i/2} \hat{i}, \\ e^{-\hat{\beta} \hat{h}_0/2} \hat{i}^\dagger e^{\hat{\beta} \hat{h}_0/2} &= e^{-\hat{\beta} \omega_i/2} \hat{i}^\dagger, \\ e^{\hat{\beta} \hat{h}_0/2} \hat{z} e^{-\hat{\beta} \hat{h}_0/2} &= \hat{z}, \\ e^{\hat{\beta} \hat{h}_0/2} \hat{i}^\dagger e^{-\hat{\beta} \hat{h}_0/2} &= \hat{i}^\dagger, \end{aligned} \quad \quad (S.2)$$
where $\hat{h}_0 \stackrel{\text{def}}{=} \sum_i \omega_i \hat{i}^\dagger \hat{i}$.

Applying Eq. (S.1) and Eq. (S.2), we can derive the following expression by inserting the two identities $e^{\hat{\beta} \hat{h}_0/2} \hat{i} e^{-\hat{\beta} \hat{h}_0/2}$ and $\hat{u}e^{-\hat{u}}$:

$$\begin{aligned} \hat{i}|\theta(\beta)\rangle &\stackrel{\text{def}}{=} e^{-\hat{\beta} \hat{h}_0/2} e^{\hat{u}} \hat{i} |0, 0\rangle \\ &= e^{-\hat{\beta} \hat{h}_0/2} e^{\hat{u}} e^{-\hat{\beta} \hat{h}_0/2} e^{\hat{\beta} \hat{h}_0/2} \hat{i} |0, 0\rangle \\ &= e^{-\hat{\beta} \hat{h}_0/2} e^{\hat{u}} e^{-\hat{\beta} \omega_i/2} \hat{i} |0, 0\rangle \\ &= e^{-\hat{\beta} \omega_i/2} \hat{i}^\dagger e^{-\hat{\beta} \hat{h}_0/2} e^{\hat{u}} |0, 0\rangle \\ &= e^{-\hat{\beta} \omega_i/2} \hat{i}^\dagger e^{-\hat{\beta} \hat{h}_0/2} e^{\hat{u}} |0, 0\rangle \\ &= e^{-\hat{\beta} \omega_i/2} \hat{i}^\dagger | \theta(\beta) \rangle, \end{aligned} \quad \quad (S.3)$$
and
$$\begin{aligned} \hat{i}|\theta(\beta)\rangle &\stackrel{\text{def}}{=} e^{-\hat{\beta} \hat{h}_0/2} e^{\hat{u}} \hat{i} |0, 0\rangle \\ &= e^{-\hat{\beta} \hat{h}_0/2} e^{\hat{\beta} \hat{h}_0/2} e^{-\hat{\beta} \hat{h}_0/2} \hat{z} e^{-\hat{\beta} \hat{h}_0/2} \hat{i} |0, 0\rangle \\ &= e^{-\hat{\beta} \hat{h}_0/2} \hat{i} e^{\hat{u}} |0, 0\rangle \\ &= e^{-\hat{\beta} \hat{h}_0/2} e^{\hat{u}} (\hat{i} + \hat{i}^\dagger) |0, 0\rangle \\ &= e^{-\hat{\beta} \hat{h}_0/2} e^{\hat{u}} (\hat{i} + \hat{i}^\dagger) |0, 0\rangle \\ &= e^{-\hat{\beta} \hat{h}_0/2} e^{\hat{u}} \hat{i}^\dagger |0, 0\rangle \\ &= e^{-\hat{\beta} \hat{h}_0/2} \hat{i}^\dagger e^{\hat{\beta} \hat{h}_0/2} e^{-\hat{\beta} \hat{h}_0/2} e^{\hat{u}} |0, 0\rangle \\ &= e^{-\hat{\beta} \omega_i/2} \hat{i}^\dagger |\theta(\beta)\rangle. \end{aligned} \quad \quad (S.4)$$

Clearly, if we define the Bogoliubov transformed annihilation operators as
$$\hat{a}_i = C (\hat{i} - e^{-\beta \omega_i/2} \hat{i}^\dagger); \quad \hat{b}_i = C (\hat{i} - e^{-\beta \omega_i/2} \hat{i}^\dagger), \quad \quad (S.5)$$
where $C$ is an arbitrary constant, the resultant operators satisfy the vacuum state conditions of $\hat{a}_i | \theta(\beta) \rangle = 0$ and $\hat{b}_i | \theta(\beta) \rangle = 0$. The corresponding creation operators are the adjoints of the the annihilation operators.

The constants in the expression of the Bogoliubov transformed Bosonic construction operator can be determined by imposing the canonical commutation relation of Bosons:
$$[\hat{a}_i, \hat{a}_i^\dagger] = |C|^2 (1 + e^{-\beta \omega_i} [\hat{i}^\dagger, \hat{i}]) = 1, \quad \quad (S.6)$$
and
$$[\hat{b}_i, \hat{b}_i^\dagger] = |C|^2 (1 + e^{-\beta \omega_i} [\hat{i}^\dagger, \hat{i}]) = 1. \quad \quad (S.7)$$
Taking $C$ to be real and positive,
$$C = (1 - e^{-\beta \omega_i})^{-\frac{1}{2}}. \quad \quad (S.8)$$
The transformations in Eqs. 30 and 31 are derived.

## S.2 Expression of Bogoliubov transformed Hamiltonian (up to quadratic)

In this section, we give detailed expression of the Bogoliubov transformed Hamiltonian defined in Eq. (46).
$$\begin{aligned} \hat{\tilde{h}}_0^0 &= E_0 + \sum_i \sinh \theta_i \sinh \theta_i^* \hat{h}_i^i \\ \hat{\tilde{h}}^1 &= \sum_i (\cosh \theta_i \hat{h}_i^i \hat{a}_i^\dagger + \sinh \theta_i \hat{h}_i^i \hat{b}_i) \\ \hat{\tilde{h}}_1 &= \sum_i (\cosh \theta_i \hat{h}_i^i \hat{a}_i + \sinh \theta_i \hat{h}_i^i \hat{b}_i^\dagger) \\ \hat{\tilde{h}}_2 &= \frac{1}{2} \sum_{ij} (\cosh \theta_i \cosh \theta_j \hat{h}_{ij}^{ai} \hat{a}_j^\dagger + \sinh \theta_i \sinh \theta_j \hat{h}_{ij}^{bi} \hat{b}_j^\dagger + \cosh \theta_i \sinh \theta_j \hat{h}_{ij}^{ai} \hat{b}_j + \sinh \theta_i \cosh \theta_j \hat{h}_{ij}^{bi} \hat{a}_j^\dagger) \\ \hat{\tilde{h}}_{\bar{2}} &= \frac{1}{2} \sum_{ij} (\cosh \theta_i \cosh \theta_j \hat{h}_{ij}^{ai} \hat{a}_j + \sinh \theta_i \sinh \theta_j \hat{h}_{ij}^{bi} \hat{b}_j + \cosh \theta_i \sinh \theta_j \hat{h}_{ij}^{ai} \hat{b}_j + \sinh \theta_i \cosh \theta_j \hat{h}_{ij}^{bi} \hat{a}_j) \\ \hat{\tilde{h}}_{\bar{1}}^i &= \sum_{ij} (\cosh \theta_i \cosh \theta_j \hat{h}_{ij}^{ai} \hat{a}_j + \sinh \theta_i \sinh \theta_j \hat{h}_{ij}^{bi} \hat{b}_j + \cosh \theta_i \sinh \theta_j \hat{h}_{ij}^{ai} \hat{b}_j + \sinh \theta_i \cosh \theta_j \hat{h}_{ij}^{bi} \hat{a}_j) \end{aligned} \quad \quad (S.9)$$

## S.3 The initial cluster amplitudes (at high T)

The $\hat{T}$ and $\hat{Z}$ operators in the Bogoliubov representation can be written as:
$$\hat{T}^{(w)} = \sum_{ij} t_{ij}^{(w)} \hat{a}_i^\dagger \hat{a}_j + \frac{1}{2} \sum_{ij} t_{ij}^{(w)} \hat{a}_i^\dagger \hat{a}_j^\dagger + \frac{1}{2} \sum_{ij} t_{ij}^{(w)} \hat{a}_i \hat{a}_j + \frac{1}{2} \sum_{ij} t_{ij}^{(w)} \hat{b}_i^\dagger \hat{b}_j + \frac{1}{2} \sum_{ij} t_{ij}^{(w)} \hat{b}_i \hat{b}_j, \quad \quad (S.10)$$
and
$$\hat{Z}_{\hat{x}}^{(w)} = z_{\hat{x}}^{(w)} + \sum_{ij} z_{ij}^{(w)} \hat{a}_i^\dagger \hat{a}_j + \frac{1}{2} \sum_{ij} z_{ij}^{(w)} \hat{a}_i^\dagger \hat{a}_j^\dagger + \frac{1}{2} \sum_{ij} z_{ij}^{(w)} \hat{a}_i \hat{a}_j + \frac{1}{2} \sum_{ij} z_{ij}^{(w)} \hat{b}_i^\dagger \hat{b}_j^\dagger + \frac{1}{2} \sum_{ij} z_{ij}^{(w)} \hat{b}_i \hat{b}_j, \quad \quad (S.11)$$
We propagate the amplitudes initializing the amplitudes one electronic surface at a time, where this initial surface label is denoted $w$. At high temperature, each of the amplitude in Eq. (S.10) and Eq. (S.11) can be mapped to the analytical expression of the thermal density matrix of D.H.O.. The explicit expression for the mapping is as follow:
$$\begin{aligned} t_{a_i a_j}^{(w)} &= \frac{X_i}{\cosh \theta_i} \delta_{ij} \\ t_{b_i b_j}^{(w)} &= -\frac{X_i}{\sinh \theta_i} \delta_{ij} \\ t_{a_i b_j}^{(w)} &= t_{b_i a_j}^{(w)} = \delta_{\lambda\bar{\lambda}} = 0 \\ t_{a_i b_j}^{(w)} &= t_{b_i a_j}^{(w)} = \delta_{\lambda\bar{\lambda}} \frac{n_i(\beta) - \cosh^2 \theta_i + 1}{\sinh \theta_i \cosh \theta_i} \end{aligned} \quad \quad (S.12)$$
where $X_i \stackrel{\text{def}}{=} \frac{\lambda_i^x}{\omega_i}$ and $n_i(\beta) \stackrel{\text{def}}{=} \frac{1}{1-e^{-\beta \omega_i}}$, and
$$\begin{aligned} z_x^{0(w)} &= \delta_{\lambda\bar{\lambda}} Q(\beta) \\ z_{a_i}^{(w)} &= z_{b_i}^{(w)} = z_{a_i a_j}^{(w)} = z_{a_i b_j}^{(w)} = z_{b_i a_j}^{(w)} = z_{b_i b_j}^{(w)} = 0 \end{aligned} \quad \quad (S.13)$$
where $Q(\beta) \stackrel{\text{def}}{=} e^{-\beta(E_0 - \sum_x \lambda_x^x)^2} \prod_i \frac{1}{1-e^{-\beta \omega_i^x}}$.

## S.4 Proof of the connectedness for the (single surface) CC-EOM

In this appendix, we give a proof the equation of motion (EOM) given in Eq. (57) is connected. The starting point is an exponential parameterization of the thermal density operator in Bogoliubov representation and the EOM is defined as:
$$\frac{d \hat{e}^{\hat{T}(\tau)}}{d\tau} |\theta(\beta)\rangle = \hat{\tilde{H}} e^{\hat{T}(\tau)} |\theta(\beta)\rangle. \quad \quad (S.14)$$
We project both hands side of the Eq. (S.14) against $\langle\theta(\beta)| e^{-\hat{T}(\tau)}$:
$$-\langle\theta(\beta)| \hat{e}^{\hat{T}(\tau)} \hat{e}^{-\hat{T}(\tau)} \frac{d\hat{T}(\tau)}{d\tau} |\theta(\beta)\rangle = \langle\theta(\beta)| e^{-\hat{T}(\tau)} \hat{\tilde{H}} e^{\hat{T}(\tau)} |\theta(\beta)\rangle. \quad \quad (S.15)$$
In Eq. (S.15), the two exponential operators cancels out and we define the similarity transformed Hamiltonian as $\hat{\tilde{H}}(\tau) \stackrel{\text{def}}{=} e^{-\hat{T}(\tau)} \hat{H} e^{\hat{T}(\tau)}$, then the projected EOM can be simplified as:
$$\frac{d\hat{\mu}(\tau)}{d\tau} = \langle\theta(\beta)| \hat{\Omega}_k^T \hat{\tilde{H}}(\tau) |\theta(\beta)\rangle . \quad \quad (S.16)$$
In Eq. (S.16), the similarity transformed Hamiltonian $\hat{\tilde{H}}(\tau)$ can be expanded applying BCH formula:
$$e^{-\hat{T}(\tau)} \hat{H} e^{\hat{T}(\tau)} = \hat{H} + [\hat{H}, \hat{T}(\tau)] + \frac{1}{2!} [[\hat{H}, \hat{T}(\tau)], \hat{T}(\tau)] + \dots \quad \quad (S.17)$$
In the BCH expansion in Eq. (S.17), the commutator can be expressed as:
$$[\hat{H}, \hat{T}(\tau)] = \hat{H} \hat{T}(\tau) - \hat{T}(\tau) \hat{H} \quad \quad (S.18)$$
and both products in the commutator can be expanded as their normal product and sum over all contraction with the respect to the reference state $|\theta(\beta)\rangle$ according to the Wick's theorem. The normal product of the two terms in Eq. (S.18) cancels out by definition $(\{\hat{H}\hat{T}(\tau)\} - \{\hat{T}(\tau)\hat{H}\} = 0)$. Thus, the only surviving terms in the commutator in Eq. (S.18) are connected terms with the Hamiltonian operator $\hat{H}$ has at least one contraction with the cluster operator $\hat{T}(\tau)$. Applying the same argument recursively, the whole BCH expansion in Eq. (S.17) consists of connected terms only:
$$e^{-\hat{T}(\tau)} \hat{H} e^{\hat{T}(\tau)} = (\hat{H} e^{\hat{T}(\tau)})_{\text{conn.}} \quad \quad (S.19)$$
Substituting Eq. (S.19) into Eq. (S.16), we prove that the CC-EOM is connected. In the derivation above, $\hat{H}$ can be replaced by any observable. Therefore
$$e^{-\hat{T}(\tau)} \hat{O} e^{\hat{T}(\tau)} = (\hat{O} e^{\hat{T}(\tau)})_{\text{conn.}} \quad \quad (S.20)$$

## S.5 Motivation of the ansatz for the vibronic problem

In analog to the vibrational problem, the most straightforward ansatz to parameterize the vibronic thermofield state, for surface $w$ is the single exponential ansatz:
$$|\phi(\tau)^{(w)}\rangle \stackrel{\text{def}}{=} e^{-\hat{T}(\tau)} |w, U\rangle = e^{\hat{T}^{(w)}(\tau)} |w, \theta(\beta)\rangle, \quad \quad (S.21)$$
where
$$\hat{T}(\tau) = \sum_{x,y} \hat{t}_{xy}(\tau) \hat{x}^\dagger \hat{y} \quad \quad (S.22)$$
and
$$\hat{T}_{xy}(\tau) = t_{xy}^{(v)}(\tau) \hat{x}^\dagger \hat{y}^\dagger + \sum_{pq} t_{xy, pq}^{(v)} \hat{x}^\dagger \hat{p}^\dagger \hat{q}^\dagger + \frac{1}{2} \sum_{pq} t_{xy, pq}^{(v)} \hat{p}^\dagger \hat{q}^\dagger + \dots \quad \quad (S.23)$$
The cluster operator $\hat{T}$ spans both electronic and vibrational degrees of freedom (DoF). Projecting the vibronic EOM by $\langle z, \theta(\beta)| \hat{\Omega}_x$ in analog to vibrational counterpart in Eq. (57), we obtain
$$-\langle z, \theta(\beta) | \sum_{xy} (\hat{e}^{-\hat{T}(\tau)}) \frac{d\hat{T}(\tau)}{d\tau} |w, \theta(\beta)\rangle = \langle z, \theta(\beta) | \sum_{wx, xy} (\hat{e}^{-\hat{T}(\tau)}) \hat{H}_{wx} (\hat{e}^{\hat{T}(\tau)})_{xy} |w, \theta(\beta)\rangle. \quad \quad (S.24)$$
However, due to the matrix nature of the cluster operator with respect to the electronic DoF, the disconnected contributions in both sides Eq. (S.24) do not cancels out:
$$\sum_{x} (e^{-\hat{T}(\tau)})_{xz} \left(\frac{d \hat{e}^{\hat{T}(\tau)}}{d\tau}\right)_{zy} \neq \sum_{xy} \left(\frac{d\hat{T}(\tau)}{d\tau}\right)_{zy} \quad \quad (S.25)$$
and
$$\sum_{w,x} (e^{-\hat{T}(\tau)})_{zw} \hat{H}_{\text{ex}}^{(e^{\hat{T}(\tau)})_{xy}} \neq \left((\hat{H} e^{\hat{T}(\tau)})_{\text{conn.}}\right)_{zy} \quad \quad (S.26)$$
To avoid this complication, we choose $\hat{T}$ to be pure vibrational and independent of electronic surface. In order to achieve this, we introduce the mixed CC/CI ansatz:
$$|\phi(\tau)^{(w)}\rangle \stackrel{\text{def}}{=} e^{-\tau \hat{T}^{(v)}} e^{\hat{T}^{(w)}(\tau)} \sum_x \hat{Z}_x^{(w)}(\tau) |x, \theta(\beta)\rangle. \quad \quad (S.27)$$
In this way, $\hat{T}$ has no electronic dependencies and the EOM becomes connected again, i.e.,
$$e^{-\hat{T}^{(w)}(\tau)} \frac{d \hat{T}^{(w)}(\tau)}{d\tau} = \frac{d \hat{T}^{(w)}(\tau)}{d\tau} \quad \quad (S.28)$$
and
$$e^{-\hat{T}^{(w)}(\tau)} \hat{H}_{xy} e^{\hat{T}^{(w)}(\tau)} = (\hat{H}_{xy} e^{\hat{T}^{(w)}(\tau)})_{\text{conn.}} \quad \quad (S.29)$$
The mixing of electronic states is now all induced by the CI operator $\hat{Z}$. Making use of Eq. (S.28) and Eq. (S.29), the EOM (Eq. (91)) for full vibronic coupling problem with mixed CC/CI ansatz can be derived.

## S.6 Alternative derivation of TFCC EOM

In Section 2.1.2, we give explicit derivation of the TFCC EOM. However, the derivation resorts to our previous work on thermal normal-ordered exponential (TNOE)$^1$ which is unnecessary. In this section, we give an alternative derivation of TFCC EOM without explicitly introducing the TNOE.

Making use of the cyclic rules, the statistical average of a physical property can be expressed as $\hat{O}$:
$$\begin{aligned} \text{Tr}(\hat{O}\hat{D}) &= \langle U|\hat{O} e^{-\tau \hat{H}} |U\rangle \\ &= \langle U|\hat{O} e^{-\hat{\beta} \hat{h}_0} e^{-\hat{\beta} \hat{h}_e} |U\rangle \\ &= \langle U|e^{-\hat{\beta} \hat{h}_0/2} \hat{O} e^{-\tau \hat{H}} e^{-\hat{\beta} \hat{h}_0/2} |U\rangle \\ &= \langle\theta(\beta)|\hat{O} e^{-\tau \hat{H}} |\theta(\beta)\rangle \end{aligned} \quad \quad (S.30)$$
From Eq. (S.30), we observe that if we define state $|\Psi(\tau)\rangle$ in the compound space as:
$$|\Psi(\tau)\rangle \stackrel{\text{def}}{=} e^{-\hat{\beta} \hat{h}_0/2} e^{-\tau \hat{H}} |\theta(\beta)\rangle, \quad \quad (S.31)$$
then the statistical average of $\hat{O}$ can be re-written as:
$$\text{Tr}(\hat{O}\hat{D}) = \langle\theta(\beta)|\hat{O} |\Psi(\tau)\rangle. \quad \quad (S.32)$$
Such a state $|\Psi(\tau)\rangle$ (if not orthogonal to $|\theta(\beta)\rangle$), can be represented as:
$$|\Psi(\tau)\rangle \stackrel{\text{def}}{=} e^{\hat{T}(\tau)} |\theta(\tau)^{(\tau)}\rangle, \quad \quad (S.33)$$
where
$$\hat{T}(\tau) = t_0(\tau) \hat{I} + \sum_\lambda t_\lambda(\tau) \hat{\Omega}_\lambda, \quad \quad (S.34)$$
where $\hat{\Omega}_\lambda$ are excitation operators in the Bogoliubov representation.
Since $\hat{T}(\tau)$ is represented in Bogoliubov representation, we should represent $\hat{H}$ also in Bogoliubov representation when deriving the EOM:
$$|\Psi(\tau)\rangle \stackrel{\text{def}}{=} e^{-\tau \hat{H}} e^{\hat{\beta} \hat{h}_0} |\theta(\beta)\rangle. \quad \quad (S.35)$$
Eq. (S.33) can make analogy to the exponential ansatz of ground state wavefunction in the conventional coupled cluster theory. To derive the TFCC EOM, we take derivative over the imaginary $\tau$ on both hands sides of Eq. (S.33) and we have:
$$e^{\hat{T}(\tau)} \frac{d\hat{T}(\tau)}{d\tau} |\theta(\beta)\rangle = -\hat{H} |\Psi(\tau)\rangle = -\hat{H} e^{\hat{T}(\tau)} |\theta(\beta)\rangle \quad \quad (S.36)$$
In Eq. (S.36), we assume that $\frac{d}{d\tau} |\theta(\beta)\rangle = 0$.
In the final step, we project both hand sides of Eq. (S.36) against $\langle\theta(\beta)| e^{-\hat{T}(\tau)}$ to obtain:
$$-\langle\theta(\beta)| \Omega_p^\dagger \frac{d\hat{T}(\tau)}{d\tau} |\theta(\beta)\rangle = \langle\theta(\beta)| \Omega_p^\dagger (\hat{H} e^{\hat{T}(\tau)})_{\text{conn.}} |\theta(\beta)\rangle \quad \quad (S.37)$$
where
$$e^{-\hat{T}(\tau)} \hat{H} e^{\hat{T}(\tau)} = (\hat{H} e^{\hat{T}(\tau)})_{\text{conn.}} \quad \quad (S.38)$$
has been used. Thus, we manage to derive that TFCC EOM.
The same TFCC EOM in Section 2.1.2 is then obtained. This concise derivation does not involve the TNOE ansatz and any presumptions.

## S.7 Detailed scheme for calculating the variance of the position (q-variance) using VE-TFCC

In this section, we give the explicit scheme to calculate the q-variance $(\Delta q)$ from the VE-TFCC approach. By definition $\Delta q_i^2 \stackrel{\text{def}}{=} \langle \hat{q}_i^2 \rangle - \langle \hat{q}_i \rangle^2$. We can use the general scheme discussed in Section 2.2.1 to calculate the individual expectation values $\langle \hat{q}_i \rangle$ and $\langle \hat{q}_i^2 \rangle$ and then the variance can be determined. We will elaborate the detailed working equations below.

Firstly we can express the $\hat{q}$ operator in Bosonic construction operators:
$$\hat{q}_i = \frac{1}{\sqrt{2}} (\hat{i} + \hat{i}^\dagger) \quad \quad (S.39)$$
Thus, we can express $\hat{q}^2$ as:
$$\begin{aligned} \hat{q}^2 &= \frac{1}{2} (\hat{i}^\dagger + \hat{i}) (\hat{i}^\dagger + \hat{i}) \\ &= \frac{1}{2} (\hat{i}^\dagger \hat{i}^\dagger + \hat{i}^\dagger \hat{i} + \hat{i} \hat{i}^\dagger + \hat{i} \hat{i}) \quad ([\hat{i}, \hat{i}^\dagger] = 1) \\ &= \frac{1}{2} \hat{i}^\dagger \hat{i}^\dagger + \hat{i}^\dagger \hat{i} + \frac{1}{2} \hat{i} \hat{i} + \frac{1}{2} \end{aligned} \quad \quad (S.40)$$
Making using of Eq. (99) and explicit equation of $\hat{q}_i$ and $\hat{q}_i^2$ in Eq. (S.39) and Eq. (S.40), we can express the $\langle \hat{q}_i \rangle$ and $\langle \hat{q}_i^2 \rangle$ in terms of the thermal physical reduced density matrices:
$$\langle \hat{q}_i(\tau) \rangle = \frac{1}{\sqrt{2}} \sum_j d_{ij}^T(\tau) + d_{ji}^*(\tau), \quad \quad (S.41)$$
$$\langle \hat{q}_i^2(\tau) \rangle = \sum_{j} \frac{1}{2} d_{ii}^{xy}(\tau) + d_{ii}^{xx}(\tau) + \frac{1}{2} d_{xx}^{yy}(\tau) + \frac{1}{2} \delta_{xx}, \quad \quad (S.42)$$
where the thermal physical reduced density matrices are defined as:
$$\begin{aligned} d_{\hat{i}\hat{i}^\dagger}^{xy}(\tau) &\stackrel{\text{def}}{=} \frac{1}{Q(\tau)} \langle x, \theta | \hat{i}^\dagger \hat{i} |\phi^{(y)}(\tau)\rangle, \\ d_{\hat{i}\hat{i}}^{xy}(\tau) &\stackrel{\text{def}}{=} \frac{1}{Q(\tau)} \langle x, \theta | \hat{i} \hat{i} |\phi^{(y)}(\tau)\rangle, \\ d_{\hat{i}^\dagger \hat{i}}^{xy}(\tau) &\stackrel{\text{def}}{=} \frac{1}{Q(\tau)} \langle x, \theta | \hat{i}^\dagger \hat{i} |\phi^{(y)}(\tau)\rangle, \\ d_{\hat{i}^\dagger \hat{i}^\dagger}^{xy}(\tau) &\stackrel{\text{def}}{=} \frac{1}{Q(\tau)} \langle x, \theta | \hat{i}^\dagger \hat{i}^\dagger |\phi^{(y)}(\tau)\rangle, \\ d_{\hat{i}\hat{i}}^{xy}(\tau) &\stackrel{\text{def}}{=} \frac{1}{Q(\tau)} \langle x, \theta | \hat{i} \hat{i} |\phi^{(y)}(\tau)\rangle. \end{aligned} \quad \quad (S.43)$$
and the canonical partition function $Q(\tau) \stackrel{\text{def}}{=} \sum_x z_x^{0(\tau)} e^{\hat{T}(\tau)}$. For simplicity, since all thermal physical reduced density matrices are temperature-dependent, we will drop the temperature $(\tau)$ of them in the rest of this section ($d_{\mu\nu}^{\alpha\beta} \stackrel{\text{def}}{=} d_{\mu\nu}^{\alpha\beta}(\tau)$).

Next, we map the thermal physical reduced density matrices from the thermal reduced density matrices in Bogoliubov representation:
$$\begin{aligned} d_{\hat{i}\hat{i}^\dagger}^{xy} &= \frac{1}{Q(\tau)} \langle x, \theta | \cosh \theta_i \hat{a}_i + \sinh \theta_i \hat{b}_i^\dagger |\phi^{(y)}\rangle = \sinh \theta_i \hat{d}_{y i}^{b \dagger}, \\ d_{\hat{i}\hat{i}}^{xy} &= \frac{1}{Q(\tau)} \langle x, \theta | \cosh \theta_i \hat{a}_i + \sinh \theta_i \hat{b}_i^\dagger |\phi^{(y)}\rangle = \cosh \theta_i \hat{d}_{y i}^{a \dagger}, \\ d_{\hat{i}\hat{i}^\dagger \hat{i}^\dagger}^{xy} &= \frac{1}{Q(\tau)} \langle x, \theta | (\cosh \theta_i \hat{a}_i + \sinh \theta_i \hat{b}_i^\dagger) (\cosh \theta_j \hat{a}_j + \sinh \theta_j \hat{b}_j^\dagger) |\phi^{(y)}\rangle = \cosh \theta_i \sinh \theta_j \hat{d}_{y i}^{a \dagger} \hat{d}_{y j}^{b \dagger}, \\ d_{\hat{i}\hat{i}}^{xy} &= \frac{1}{Q(\tau)} \langle x, \theta | (\cosh \theta_i \hat{a}_i + \sinh \theta_i \hat{b}_i^\dagger) (\cosh \theta_j \hat{a}_j + \sinh \theta_j \hat{b}_j^\dagger) |\phi^{(y)}\rangle = \cosh \theta_i \cosh \theta_j \hat{d}_{y i}^{a} \hat{d}_{y j}^{a}, \\ d_{\hat{i}\hat{i}}^{xy} &= \frac{1}{Q(\tau)} \langle x, \theta | (\cosh \theta_i \hat{a}_i + \sinh \theta_i \hat{b}_i^\dagger) (\cosh \theta_j \hat{a}_j + \sinh \theta_j \hat{b}_j^\dagger) |\phi^{(y)}\rangle \\ &= \sinh \theta_i \cosh \theta_j \hat{d}_{y j}^{b a_j} + \sinh \theta_i \sinh \theta_j \delta_{xy}, \end{aligned} \quad \quad (S.44)$$
where $\hat{d}_{\mu\nu}^{\alpha\beta}$ represent thermal reduced density matrices in Bogoliubov representation.
In the last step, we can calculate the thermal reduced density matrices in Bogoliubov representation $(\hat{d}_{\mu\nu}^{\alpha\beta})$ from the cumulant expression of the cluster amplitudes:
$$\begin{aligned} \hat{d}_{\hat{i}\hat{i}^\dagger}^{xy} &= \hat{d}_{\hat{i}^\dagger \hat{i}^\dagger}^{xy} = \hat{z}_x^{0(y)}, \\ \hat{d}_{\hat{i}\hat{i}^\dagger}^{xy} &= \hat{t}^{p(y)} z_x^{0(y)} + z_x^{p(y)}, \\ \hat{d}_{\hat{i}\hat{i}}^{xy} &= \hat{t}^{pq}(y) z_x^{0(y)} + \hat{t}^{p(y)} z_q^{0(y)} + \hat{t}^{q(y)} z_p^{0(y)} + \hat{t}^{pq}(y) \hat{t}^{pq}(y) \hat{z}_x^{0(y)} + \hat{z}_x^{pq}(y). \end{aligned} \quad \quad (S.45)$$
In this way, we complete the explicit scheme to calculate the q-variance. The procedure is in the reverse order as follow:
(1) Calculate reduced thermal density matrices in Bogoliubov representation using the cumulant expression in Eq. (S.45).
(2) Map the reduced thermal density matrices in Bogoliubov representation to physical reduced density matrices using Eq. (S.44).
(3) Calculate expectation value $\langle \hat{q}_i \rangle$ and $\langle \hat{q}_i^2 \rangle$ from physical reduced density matrices using Eq. (S.41) and Eq. (S.42).
(4) Calculate the variance using the formula $(\Delta \hat{q}_i^2)^{\text{def}} = \langle \hat{q}_i^2 \rangle - \langle \hat{q}_i \rangle^2$.

## S.8 Data for standard thermochemistry calculation

### S.8.1 Details on calculations of the reaction energy at 0 K

First, we calculated the reaction energy with the noJT approximation. The single point ground state electronic energies for $\text{CoF}_3$ ($-1680.9725264215 E_H$), $\text{F}^-$ ($-99.6938067791 E_H$) and $\text{CoF}_4$ at its $T_d$ reference structure ($-1780.8473934862 E_H$) are obtained at the GMC-QDPT/cc-pVTZ level. They are subtracted to give the electronic reaction energy ($E_{\text{el}}$) $-475.37 \text{ kJ/mol}$. $\text{CoF}_3$ has a $D_{3h}$ optimized structure with $r_{\text{CF}} = 1.71 \text{ Å}$. To maintain consistency as for $\text{CoF}_4^-$, the active space with the five $3d$-dominated orbitals and six electrons was used for $\text{CoF}_3$. The ground state of $\text{CoF}_3$ adopts a $^5A'_1$ term symbol. The occupation scheme of this state is shown in Fig. 6b.

Now, we consider the vibrational and vibronic corrections for the reaction energy. At $T = 0 \text{ K}$, there is no rotational and translational correction. Only vibrational Zero Point Energy (ZPE) and vibronic ground state energy shall be considered. The ZPE of $\text{CoF}_3$ is $16.37 \text{ kJ/mol}$ and there is no ZPE for $\text{F}^-$ anion. The vibronic ground state energies of $\text{CoF}_4^-$ calculated using the block relaxation and VE-TFCC are the essentially identical, $14.48 \text{ vs.} 14.52 \text{ kJ/mol}$, respectively. We take the VE-TFCC value to calculate the vibrational and vibronic correction of the reaction energy at $T = 0 \text{ K}$ to be $14.52 - 16.37 = -1.85 \text{ kJ/mol}$. For the noJT approximation, the subtraction of the ZPE in Eq. (107) and the ZPE of $\text{CoF}_3$ gives a $20.40 - 16.37 = 4.03 \text{ kJ/mol}$ correction to the reaction energy.

Adding up the electronic and the vibrational and vibronic contributions, the reaction energy at $T = 0 \text{ K}$ is $-477.30 \text{ kJ/mol}$ in the full JT treatment using VE-TFCC, and $-471.34 \text{ kJ/mol}$ in the noJT approximation. The $\sim 6 \text{ kJ/mol}$ difference reflects the Jahn-Teller (JT) effect in stabilizing the product and facilitating the reaction. This energy difference is non-negligible compared to the chemical accuracy of $4 \text{ kJ/mol}$. The $-477.30 \text{ kJ/mol}$ calculated reaction energy is in qualitative agreement with the experimental value of $-442.3 \pm 25 \text{ kJ/mol}^2$.

We also examined the difference between static and dynamic JT treatments in calculating the reaction energy. The VE-TFCC result above includes the full dynamic JT effect, since the vibronic Hamiltonian does not impose any restriction of $\text{CoF}_4^-$ to stay in one of the three minima on the low energy trough of the ground electronic state PES. This restriction corresponds to the static JT treatment. To calculate the reaction energy with only static JT effect, we optimized the $\text{CoF}_4^-$ structure without imposing any symmetry constraint, and calculated its vibrational frequencies at the optimized structure. All calculations were performed at the same level of theory. The optimized ground state structure is in $D_{2d}$ symmetry and has no imaginary frequency, which meets the anticipation based on the Epikernal Principal.$^3$ It is distorted from the $T_d$ reference structure along the $e_x$ bending mode shown in Fig. 7(a), with the electronic energy $-1780.8491766235 E_H$, i.e., a $4.68 \text{ kJ/mol}$ JT stabilization energy from the $T_d$ reference structure. The stable structure has a ZPE of $20.09 \text{ kJ/mol}$.

### S.8.2 Thermal internal energy corrections data

The thermal internal energy data get from the standard thermochemistry calculations is given in Table S.1.

The two columns of data are given for $\text{CoF}_4^-$ in the $T_d$ reference structure and $D_{2d}$ ground state global minimum. We emphasize again that the hessian calculation for the $T_d$ structure was performed using averaged PES of the two $^5E$ component states, which is smooth and gives normal modes that are adapted to the $T_d$ symmetry. The vibrational thermal corrections are identical (undiscernible differences after rounding) for the two structures of $\text{CoF}_4^-$, as the frequencies are only slightly different after the JT distortion, by $< 70 \text{ cm}^{-1}$ and mostly for splitting degenerate modes to non-degenerate modes due to symmetry lowering, i.e., $e$ mode to $a_1 + b_1$ modes, $t_2$ modes to $a_1 + e$ or $b_1 + e$ modes. The small differences are blurred in thermal average.

**Table S.1:** Calculated thermal internal energy corrections ($\text{kJ/mol}$) at $298\text{K}$ for the three species in Eq. 104.

| Species | $\text{CoF}_3$ | $\text{F}^-$ | $\text{CoF}_4$ ($T_d$) | $\text{CoF}_4^-$ ($D_{2d}$) |
| :--- | :--- | :--- | :--- | :--- |
| $E_{\text{trans}}$ | $3.72$ | $3.72$ | $3.72$ | $3.72$ |
| $E_{\text{rot}}$ | $3.72$ | $0.00$ | $3.72$ | $3.72$ |
| $E_{\text{vib}}$ | $21.69$ | $0.00$ | $29.60$ | $29.60$ |
| $E_{\text{tot}}$ | $29.13$ | $3.72$ | $37.03$ | $37.03$ |

### S.8.3 Entropies at 298 K

The entropy data get from the standard thermochemistry calculations is given in Table S.2. Again, two columns are given for $\text{CoF}_4^-$ for the noJT approximation ($T_d$) and static JT treatment ($D_{2d}$).

There is no electronic spin contribution to the reaction entropy. This is because the reactant side ($\text{CoF}_3 + \text{F}^-$) and the product side ($\text{CoF}_4^-$) have the same spin multiplicities of 5. Their associated entropies cancel. However, for the noJT approximation, there is an electronic contribution to entropy in the product side of $R \log 2 = 0.006 \text{ kJ/mol/K}$, due to the doubly spatial degeneracy of the $^5E$ term symbol of $\text{CoF}_4^-$. It should be added to the $S_{\text{tot}}$ of $\text{CoF}_4^-$. Such an electronic spatial degeneracy entropy is inherited by the degeneracies of the vibronic eigenstates that are derived from the JT interaction of the $^5E$ term symbol. Therefore, in our VE-TFCC calculation, we do not need to explicitly consider the entropy that stems from this degeneracy. For the static JT treatment, there are three $D_{2d}$ distorted $\text{CoF}_4^-$ configurations obtained along three directions on the $e$ mode space. Therefore, we need to add the $R \log 3 = 0.009 \text{ kJ/mol/K}$ configuration entropy to the product side.

**Table S.2:** Calculated entropies (in $\text{kJ/mol/K}$) of the species in Eq. (104) at $298\text{K}$.

| Species | $\text{CoF}_3$ | $\text{F}^-$ | $\text{CoF}_4$ ($T_d$) | $\text{CoF}_4^-$ ($D_{2d}$) |
| :--- | :--- | :--- | :--- | :--- |
| $S_{\text{trans}}$ | $0.168$ | $0.145$ | $0.170$ | $0.170$ |
| $S_{\text{rot}}$ | $0.101$ | $0.000$ | $0.106$ | $0.112$ |
| $S_{\text{vib}}$ | $0.034$ | $0.000$ | $0.054$ | $0.058$ |
| $S_{\text{tot}}$ | $0.303$ | $0.145$ | $0.330$ | $0.340$ |

***

## References

(1) Nooijen, M.; Bao, S. Normal ordered exponential approach to thermal properties and time-correlation functions: general theory and simple examples. *Molecular Physics* **2021**, *119*, e1980832.
(2) Korobov, M.; Savinova, L.; Sidorov, L. Stabilities of $\text{CoF}_4$ and $\text{CrF}_5$ in the gas phase. *The Journal of Chemical Thermodynamics* **1993**, *25*, 1161–1168.
(3) Ceulemans, A.; Vanquickenborne, L. *Stereochemistry and Bonding*; Springer: B-3030 Leuven, Belgium, 2005; pp 125-159.