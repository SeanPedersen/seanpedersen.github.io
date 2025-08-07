---
title: 'Quantum Computing'
date: '2025-05-23'
---
Quantum computing promises significant feats for heavy computations found in cryptography and machine learning but like fusion energy it is always around the corner but never arrives. So what is the state of quantum computing and what to look out for to sense real progress?

## Terminology

**Qubits** represent the fundamental unit of quantum information, existing in superposition (probability distribution between 0 and 1). However, the critical distinction lies between physical and logical qubits. Physical qubits are the actual quantum systems (atoms, photons, superconducting circuits). Logical qubits are the error-corrected, fault-tolerant units that can perform reliable computations. Current quantum computers require hundreds or thousands of physical qubits to create a single logical qubit, making this ratio a crucial metric for evaluating progress.

**Quantum decoherence** describes how quantum systems lose their quantum properties when interacting with their environment, effectively imposing a time limit on quantum computations.

**Gate fidelity** measures how accurately quantum operations are performed, with higher fidelity indicating less noise and more reliable computations.

## Applications

### Breaking Cryptography

The most immediate threat quantum computing poses is to current cryptographic systems. RSA encryption, which secures HTTPS connections and underlies much of internet security, relies on the computational difficulty of factoring large prime numbers. Shor's algorithm can factor these numbers exponentially faster on a sufficiently large quantum computer, which is the reason why many organizations are moving to post-quantum cryptographic systems.

### Machine Learning Acceleration

Quantum computers excel at linear algebra operations fundamental to machine learning: matrix inversion, eigenvalue decomposition, and solving systems of linear equations. These operations, when scaled to massive datasets, could benefit significantly from quantum acceleration. Potentially leading to significant speed ups in AI processing and thus potential massive advances in AI in general.

## Scaling Challenges

The path to practical quantum computing faces several interconnected obstacles:

**Error Correction Requirements**: Achieving fault tolerance requires quantum error correction, requiring often 1,000 physical qubits per logical qubit.

**Coherence and Connectivity**: Qubits must maintain quantum properties long enough for complex computations while remaining sufficiently connected to perform multi-qubit operations. Current systems achieve coherence times from microseconds (superconducting) to seconds (trapped ion), requiring ultra-fast quantum gates and limiting algorithmic complexity.

**Classical Integration**: Practical quantum advantage will likely emerge through hybrid quantum-classical algorithms, where quantum processors handle specific subroutines while classical computers manage the broader computation. This requires seamless integration, efficient data transfer, and sophisticated orchestration between fundamentally different computational paradigms.

**Infrastructure Requirements**: Quantum computers require enormous capital investment and specialized infrastructure. Superconducting systems need dilution refrigerators maintaining temperatures below 15 millikelvin, while trapped ion systems require complex laser arrays and ultra-high vacuum chambers.

## Recent Progress and Reality Check

The quantum computing landscape has evolved significantly, though practical breakthroughs remain limited:

**Hardware Scaling**: Leading quantum processors have grown from dozens to hundreds of qubits. IBM's roadmap targets 1,000+ qubit systems, while Google and IonQ have demonstrated impressive gate fidelities on smaller systems. However, these remain primarily physical qubits with limited error correction.

**Algorithmic Demonstrations**: Researchers have achieved quantum speedups for specialized problems like quantum sampling and certain optimization tasks. Google's 2019 quantum supremacy demonstration and subsequent experiments from IBM, IonQ, and others show quantum computers can outperform classical systems on carefully constructed problems, though these remain academic exercises rather than practical applications.

**Error Correction Progress**: Incremental improvements in qubit coherence, gate fidelity, and error correction protocols continue. Recent breakthroughs in logical qubit implementations and error syndrome detection suggest the field is progressing toward fault tolerance, though large-scale error correction remains years away.

**Industry Adoption**: Post-quantum cryptography standards are nearing finalization, with organizations beginning pilot deployments. This represents both preparation for quantum threats and recognition that cryptographically relevant quantum computers are approaching feasibility.

## Indicators of Real Progress

To distinguish genuine advancement from incremental improvements, watch for these key milestones:

**Logical Qubit Scaling**: A transition from hundreds of physical qubits to thousands of logical qubits would unlock commercially relevant applications.

**Problem-Specific Quantum Advantage**: Demonstrations of quantum speedups for commercially relevant problems are a clear sign of progress. These applications should solve real-world problems faster or more efficiently than the best classical alternatives.

**Error Correction Thresholds**: Achieving error rates below the fault tolerance threshold (approximately 0.01% for most error correction schemes) across all quantum operations would enable indefinitely long quantum computations, unlocking the full potential of quantum algorithms.

**Hybrid Algorithm Performance**: Practical quantum advantage will likely emerge through hybrid approaches. Success in accelerating machine learning training, optimization problems, or scientific simulations using quantum-classical hybrid algorithms would signal genuine progress toward commercial relevance.

## Conclusion

Quantum computing remains in its early stages, but the fundamentals are sound and progress is measurable. While real-world applications remain still years away, the field has moved beyond pure research into engineering challenges of scaling, error correction, and integration. The next five years will likely determine whether quantum computing transitions from a promising laboratory curiosity to a truly transformative technology with real impact.

## References

- Excellent in-depth intro: <https://quantum.country/>

#programming #AI
