---
icon: "/images/icons/physics.svg"
---
# Physics
Physical systems naturally tend toward stable equilibrium states, generally moving toward lower free energy or greater entropy under the relevant constraints.

## Equations

**Einstein’s special relativity:** describes how measurements of space and time depend on an observer’s relative motion, while the laws of physics and the speed of light remain the same for all inertial observers; it also relates mass to energy: $E_0 = mc^2$

Intuitively, the motion of an object with mass through spacetime forms a velocity vector describing how its t,x,y,z coordinates change, whose spacetime magnitude is always c (speed of light).

**Einstein’s general relativity:** extends relativity to accelerating observers and describes gravity as spacetime curvature caused by mass-energy and momentum.

$G_{\mu\nu}+\Lambda g_{\mu\nu}=\frac{8\pi G}{c^4}T_{\mu\nu}$

**Schrödinger’s equation:** describes the time evolution of a nonrelativistic quantum wavefunction.

$i\hbar\frac{\partial}{\partial t}\lvert\psi(t)\rangle=\hat{H}\lvert\psi(t)\rangle$

**Heisenberg’s uncertainty principle:** limits how precisely certain pairs of quantum properties can be simultaneously defined and measured.

$\Delta x \cdot \Delta p \ge \frac{\hbar}{2}$

## Fundamental Interactions

**Gravity**
Gravity is the interaction of mass-energy with spacetime. In general relativity, mass-energy curves spacetime, and objects follow paths through that curved spacetime.

**Electromagnetism**
Electromagnetism acts between electrically charged particles. It causes both: attraction between opposite charges, repulsion between like charges.
Electrons have negative point charge and spin causing magnetic moment (aligned spins make a material magnetic). (attraction and repulsion)

**Strong interaction**
The strong interaction binds quarks together inside protons, neutrons and other hadrons.
A leftover effect called the residual strong force binds protons and neutrons together inside atomic nuclei, overcoming the electrical repulsion between positively charged protons at short distances. (attraction and repulsion)

**Weak interaction**
The weak interaction acts over extremely short distances and allows elementary particles to change type. For example, during beta decay, a neutron can transform into a proton while emitting an electron and an antineutrino.

## Quantum Physics

In quantum field theory, particles are understood as localized, quantized excitations of underlying quantum fields (electron, quark, photon, gluon, Higgs fields, etc.).

A quantum system is described by a quantum state, often represented by a wavefunction. The wavefunction is not itself a probability distribution; it contains probability amplitudes. Its squared magnitude, $|\psi|^2$ gives probabilities for measurement outcomes.

For a nonrelativistic isolated system, the wavefunction evolves over time according to the Schrödinger equation.

When a measurement is made, an exact outcome is observed from the range of possible outcomes (superposition). In the traditional Copenhagen interpretation, this is described as wavefunction collapse. However, whether collapse is a literal physical process depends on the interpretation of quantum mechanics.

**Quantum entanglement**, produces what Einstein famously called “spooky action at a distance”: measurements made on entangled particles can show correlations across large separations that appear to arise instantaneously, even faster than light could travel between them. However, these correlations cannot be used to transmit controllable information faster than the speed of light, because each individual measurement result is fundamentally unpredictable; only when the results are later compared through ordinary, light-speed-limited communication does the correlation become apparent. Thus, entanglement is nonlocal in its correlations but does not violate relativity or permit faster-than-light communication.

**Planck Length, Time and Quantum Unit of Action**

The Planck length $l_P$ is approximately 1.616×10−35 meters and represents an extremely small scale at which quantum effects of gravity are expected to become important. The Planck time $t_P$ is approximately 5.39×10−44 seconds. The quantum unit of action, represented by Planck’s constant $h$, is approximately 6.626×10−34 joule-seconds and relates Energy and frequency of light $E=hf$.

Fun fact: speed of light $c$ can be expressed as $c=\frac{l_P}{t_P}$, meaning light travels exactly one Planck length in one Planck time.

## Symmetry and conservation laws
Continuous symmetries correspond to conservation laws.

By [Noether’s theorem](https://en.wikipedia.org/wiki/Noether%27s_theorem):
time-translation symmetry → conservation of energy
spatial-translation symmetry → conservation of momentum
rotational symmetry → conservation of angular momentum
gauge symmetries → interaction structures and conserved charges

## Pauli exclusion principle
No two identical fermions can occupy the same quantum state at the same time.

Electrons are fermions, so in an atom no two electrons can have the same complete set of quantum numbers. That is why a single atomic orbital can hold at most two electrons, and those two must have opposite spin.

Important for:
atomic electron shells
chemistry
the size and rigidity of matter
electron degeneracy pressure
white dwarfs
neutron degeneracy pressure in neutron stars

## States of Matter
**Plasma**: Extremely hot, ionized matter where electrons are separated from atoms; found in stars and lightning.
**Gas**: Has no fixed shape or volume. Example: air.
**Liquid**: Has a fixed volume but takes the shape of its container. Example: water.
**Solid**: Has a fixed shape and volume. Example: rock.
**Einstein-Bose-condensate**: Matter cooled to temperatures near absolute zero, where particles occupy the same quantum state and behave collectively like a single quantum system.

## Astrophysics
The study of big structures like stars, planets and black holes.

### Black Holes
How is it made? Just mass squashed into a tiny space? Is every black hole capable to grow infinitely? Hawking radiation?

Most black holes form when a very massive star reaches the end of its life. A star normally balances two forces: gravity pulling inward and pressure from nuclear fusion pushing outward. When a massive star runs out of usable fuel, that outward pressure drops. Its core can then collapse extremely rapidly under its own gravity. If the collapsed core is massive enough, it is compressed inside a region bounded by an event horizon (where gravity is so strong that even light cannot escape). A black hole is born.

#### Schwarzschild radius
Schwarzschild radius (what a coincidence that name is!) defined as the radius matter or light can not escape the gravitational pull of a black hole.

$r_s = \frac{2GM}{c^2}$

If you compress a mass M inside that radius, an event horizon forms. For the Sun, $r_s$ is about 3 km. For Earth, it’s only about 9 mm. So if Earth was compressed into a marble, it would become a black hole.

#### Hawking radiation
Quantum effects imply that black holes are not perfectly black: they emit approximately thermal Hawking radiation with temperature $T_H=\frac{\hbar c^3}{8\pi G M k_B}$ so, most importantly, $T_H\propto\frac{1}{M}$.

Thus smaller black holes are hotter and evaporate faster, while larger black holes are colder and radiate extremely slowly. A solar-mass black hole has a temperature of only about $6\times10^{-8},\mathrm{K}$, far below the cosmic microwave background temperature of about $2.7,\mathrm{K}$, so present-day astrophysical black holes generally absorb more energy than they lose.

As a black hole loses mass, it gets hotter, causing faster radiation and accelerating evaporation. A stellar-mass black hole would take roughly $10^{67}$ years to evaporate.

Hawking radiation is a strong theoretical prediction, but it has not yet been directly observed from a real gravitational black hole. The familiar “particle-antiparticle pair” explanation is only a simplified analogy, not the full quantum-field-theory derivation.

Because of this mechanism, we expect tiny (hypothetical) black holes to evaporate extremely quickly.

## Questions
Why do electrons not repel but build bonds (in molecules)? Electrons do repel each other, but attraction to the positive nuclei and quantum effects can make the total energy of the bonded molecule lower than that of the separated atoms.

Why do protons build atomic nuclei and not repel? Protons do repel electrically, but at the tiny distances inside nuclei, the strong interaction between protons and neutrons can overcome that repulsion.

What is the max mass (atoms in a sphere) for a given element before it starts to collapse into a black hole?

Warp drive? A warp drive is a hypothetical method of faster-than-light travel based on manipulating spacetime itself rather than accelerating a spacecraft through space beyond the speed of light. In the best-known proposal, the Alcubierre drive, spacetime would contract in front of a spacecraft and expand behind it, creating a “warp bubble” that could effectively carry the craft between distant locations faster than light could travel through normal space. Because the spacecraft would remain locally at rest inside the bubble, it would not technically exceed the speed of light in its immediate surroundings, potentially avoiding a direct violation of special relativity. However, current models require exotic forms of negative energy, raise serious stability and causality problems, and have no known practical method of construction, so warp drive remains a speculative concept rather than an established technology.

## Open questions in physics
Gaps in current physics:
- No experimentally established quantum theory of gravity
- Unknown nature of dark matter
- Unknown nature of [dark energy](/posts/dark-energy)
- Matter–antimatter asymmetry
- Origin of neutrino masses
- Interpretation of quantum measurement
- Why fundamental constants have their observed values
- Whether the Standard Model is part of a deeper unified theory

#science
