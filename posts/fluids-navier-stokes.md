# Navier–Stokes: Physics of Fluid Motion

The **Navier–Stokes equations** describe how liquids and gases move. Developed by Claude-Louis Navier and George Gabriel Stokes in the 19th century, they apply conservation of mass and Newton's second law to a small parcel of fluid, accounting for pressure, viscosity and external forces such as gravity.

They are a continuum model: fluids are treated as smooth matter, not as individual molecules. That assumption fails at small scales (see [Limits](#limits)).

## The equations

For an incompressible, Newtonian fluid with constant density and viscosity:

$$
\rho\left(\frac{\partial \mathbf{u}}{\partial t}+(\mathbf{u}\cdot\nabla)\mathbf{u}\right)
=-\nabla p+\mu\nabla^2\mathbf{u}+\rho\mathbf{g}
$$

$$
\nabla\cdot\mathbf{u}=0
$$

- $\rho$: density
- $\mathbf{u}$: velocity field
- $t$: time
- $p$: pressure
- $\mu$: dynamic viscosity
- $\mathbf{g}$: external acceleration (e.g. gravity)

### Momentum equation

- **Inertia** $\rho\left(\partial_t \mathbf{u}+(\mathbf{u}\cdot\nabla)\mathbf{u}\right)$: the acceleration of a fluid parcel. The first term is the local change of velocity at a fixed point, the second is the convective change as the parcel moves into regions of different velocity.
- **Pressure** $-\nabla p$: pushes fluid from high towards low pressure.
- **Viscosity** $\mu\nabla^2\mathbf{u}$: diffuses momentum between neighbouring layers and smooths out velocity differences.
- **Body forces** $\rho\mathbf{g}$: external forces such as gravity.

### Continuity equation

$\nabla\cdot\mathbf{u}=0$ states that incompressible fluid is neither created nor destroyed locally: what flows into a region flows out. For compressible fluids the general form lets density vary:

$$
\frac{\partial\rho}{\partial t}+\nabla\cdot(\rho\mathbf{u})=0
$$

## Applications

- **Aerospace:** lift, drag, pressure distributions and boundary layers, used to design efficient and stable aircraft.
- **Weather and climate:** numerical weather prediction solves forms of the equations coupled with thermodynamics and moisture; climate models use them for atmospheric and ocean circulation.
- **Civil and mechanical engineering:** pipe pressure losses, pumps, turbines, HVAC, flow around bridges, dams and channels.
- **Biomedicine:** blood flow, pressure variations and the effect of vessel narrowing. Blood is non-Newtonian, so this needs additional constitutive models.
- **Computational Fluid Dynamics (CFD):** analytical solutions rarely exist for realistic geometries, so CFD discretises the domain into cells and solves numerically for velocity, pressure, temperature, turbulence and forces, replacing many expensive physical experiments.

## Connection to graph theory

A pipe network is a directed graph: junctions, reservoirs and pumps are vertices, pipes are edges carrying a flow $q_e$. Maximizing throughput from a source $S$ to a sink $T$ is the classical max-flow problem. Navier–Stokes enters through the edge capacities.

### Conservation: incidence matrix as discrete divergence

Flow conservation at every internal vertex reads $Bq=0$, where $B$ is the graph's incidence matrix. This is the discrete counterpart of $\nabla\cdot\mathbf{u}=0$. The link is exact in finite-volume methods: the divergence theorem turns mass conservation over a cell $\Omega$ into a sum of face fluxes,

$$
\int_{\partial\Omega}\mathbf{u}\cdot\mathbf{n}\,dS=0
\quad\longrightarrow\quad
\sum_{e\in\partial\Omega}q_e=0
$$

so a CFD mesh is itself a graph with cells as vertices and faces as edges. Discrete exterior calculus formalizes this correspondence.

Strictly, this is not a homomorphism but a discretization with a continuum limit, comparable to a Riemann sum converging to an integral.

### Capacity: physics instead of a fixed number

Max-flow assumes fixed capacities $0\le q_e\le c_e$. In a fluid network, flow depends on pressure, geometry, viscosity and flow regime. For steady laminar flow in a cylindrical pipe, the Hagen–Poiseuille equation (derived from Navier–Stokes) gives:

$$
Q=\frac{\pi R^4}{8\mu L}\,\Delta P
$$

Flow scales with $R^4$, so small changes in radius dominate capacity. It also mirrors Ohm's law, with $\Delta P$ as voltage, $Q$ as current and $\frac{8\mu L}{\pi R^4}$ as resistance, which is why pipe networks can be solved like electrical circuits (Kirchhoff's laws).

The throughput problem becomes: maximize outflow subject to conservation at each vertex, edge relations $q_e=f_e(\Delta p_e,R_e,L_e,\mu)$ and a pump power limit $P_\text{pump}\le P_\text{max}$.

### What does not carry over

Max-flow is a combinatorial problem. The full fluid problem is continuous, nonlinear and constrained by momentum transport, viscosity and turbulence, none of which graph flow captures. Optimizing outflow under the full equations is PDE-constrained optimization:

$$
\max_{\mathbf{u},p,\theta}\; Q_\text{out}
\quad\text{s.t.}\quad
\text{Navier–Stokes},\;\nabla\cdot\mathbf{u}=0,\;\text{boundary and design constraints}
$$

where $\theta$ are design variables such as pipe diameters, topology or pump settings. Graph models suit water distribution, vascular networks, industrial piping and microfluidics. An open question is whether max-flow solutions can initialize or approximate the optimal design of a Navier–Stokes-governed network, and whether flows on ever finer graphs converge to the continuous optimal-control problem.

## Limits

### Continuum assumption

The continuum model holds when the flow's length scale $L$ is much larger than the molecular mean free path $\lambda$. In micro- and nanochannels or rarefied gases this no longer holds. The Knudsen number quantifies it:

$$
Kn=\frac{\lambda}{L}
$$

- $Kn<0.001$: continuum mechanics is valid.
- $0.001<Kn<0.1$: slip-flow effects matter; the no-slip boundary condition becomes unreliable.
- $0.1<Kn<10$: transition regime, requires kinetic modelling.
- $Kn>10$: free-molecular flow.

When the molecular scale matters, alternatives are:

- **Molecular Dynamics (MD):** simulates individual atoms and their forces; suited to nanoscale transport.
- **Boltzmann equation:** describes the statistical distribution of molecular velocities and links molecular motion to macroscopic flow. Direct Simulation Monte Carlo (DSMC) solves it for rarefied gases.

### Turbulence

Turbulent flow has fluctuations across a wide range of spatial and temporal scales. The equations describe it, but resolving it is extremely expensive. Engineering practice trades cost against accuracy:

- **Reynolds-Averaged Navier–Stokes (RANS):** models only the averaged flow; cheapest.
- **Large Eddy Simulation (LES):** resolves large eddies, models small ones.
- **Direct Numerical Simulation (DNS):** resolves all scales; only feasible for small, low-Reynolds-number problems.

### Open mathematical problem

For 3D incompressible flow, it is unknown whether smooth initial conditions always yield smooth solutions for all time, or whether singularities can form in finite time. This is one of the Clay Institute's Millennium Prize Problems.

OpenAI claims its AI produced a proof, formalized in Lean, that finite-time singularities do form under the problem's permitted conditions. It has not yet passed independent scrutiny or been recognized by the Clay Institute, so it remains a claim, not an accepted solution. See also [Are LLMs Solving Math?](/posts/ai-math).

## Conclusion

A compact set of equations covers a vast range of phenomena, from airflow over wings to blood in arteries. Knowing where they stop applying (molecular scales, rarefied gases) and where they are merely hard to solve (turbulence, global regularity) matters as much as knowing how to use them.

## References

**Graph theory and discrete operators**

- Mohamed, Hirani, Samtaney: [Discrete exterior calculus discretization of incompressible Navier–Stokes equations over surface simplicial meshes](https://arxiv.org/abs/1508.01166)
- Nitschke, Reuther, Voigt: [Discrete exterior calculus (DEC) for the surface Navier–Stokes equation](https://arxiv.org/abs/1611.04392)
- Beltman, Anthonissen, Koren: [Mimetic staggered discretization on barycentric dual meshes (CWI)](https://ir.cwi.nl/pub/29295/29295.pdf)
- [Adaptive Hagen–Poiseuille flows on graphs](https://arxiv.org/abs/2111.15350)
- [Flow with Nonlinear Potential in General Networks: Simulation, Optimization, Control, Risk and Stability Analysis](https://arxiv.org/abs/1103.3726)
- [Minimal transport networks with general boundary conditions](https://arxiv.org/abs/1709.00694)

**PDE-constrained optimization**

- Olesen, Okkels, Bruus: [A high-level programming-language implementation of topology optimization applied to steady-state Navier–Stokes flow](https://arxiv.org/abs/physics/0410086)

**Millennium Problem**

- Tao: [Finite time blowup for an averaged three-dimensional Navier–Stokes equation](https://arxiv.org/abs/1402.0290)
- Miller: [Finite-time blowup for smooth solutions of the Navier–Stokes equations on the whole space with linear growth at infinity](https://arxiv.org/abs/2103.12237)
- OpenAI: [Navier–Stokes preprint](https://cdn.openai.com/pdf/32d9f210-8b73-45e0-91bc-82a30aef8a9a/navier-stokes.pdf)
- AI Weekly: [OpenAI claims Navier-Stokes proof as rival teams post their own](https://aiweekly.co/alerts/openai-claims-navier-stokes-proof-as-rival-teams-post-their-own)
- Emergent Mind: [Global regularity or blow-up for 3D incompressible Navier–Stokes](https://www.emergentmind.com/open-problems/navier-stokes-3d-global-regularity-clay-problem)

#math #science
