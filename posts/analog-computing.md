---
title: 'Analog Computing - The future is continuous'
date: '2025-09-12'
---
The artificial intelligence industry faces a growing energy crisis. Training large language models consumes massive amounts of power, while deploying AI at the edge demands efficiency that traditional digital processors struggle to deliver. This challenge has sparked renewed interest in analog computing—a paradigm that performs calculations using continuous physical quantities rather than discrete digital values. Recent breakthroughs suggest **analog computing could deliver 10-40x energy efficiency improvements** over conventional digital AI accelerators while maintaining competitive accuracy for specific applications.

## Recent academic breakthroughs demonstrate practical viability

The past two years have witnessed significant progress in analog AI research, with major publications demonstrating systems that work in practice rather than just theory. IBM's team published landmark results in Nature showing their **14nm analog AI chip achieving 12.4 TOPS/W chip-sustained performance**—roughly 14x more energy efficient than comparable digital systems for speech recognition tasks. The chip integrates 35 million phase-change memory (PCM) devices across 34 tiles, successfully running a 45-million parameter speech recognition model with only minimal accuracy degradation.

Meanwhile, researchers have developed the **NeuroBench framework**, a collaborative effort from over 100 researchers across 50+ institutions to standardize neuromorphic computing benchmarks. This addresses a critical gap that has hindered fair performance comparisons across different analog and neuromorphic systems. Recent IEEE ISSCC conference presentations have showcased multiple implementations achieving **31.2-40.91 TFLOPS/W** across different analog computing architectures, with some research demonstrations reaching as high as 74 TMACS/W in specialized neuromorphic cores.

The arXiv repository shows accelerating publication rates in analog deep learning, with comprehensive reviews examining eight different analog computing methodologies. These papers focus increasingly on practical implementations rather than theoretical possibilities, suggesting the field is maturing toward commercial viability.

## Energy efficiency advantages stem from eliminating data movement

Analog computing's primary advantage lies in **eliminating the von Neumann bottleneck**—the energy-expensive separation between memory and computation that defines traditional digital architectures. In conventional AI processors, data movement between memory and compute units consumes 100-1000x more energy than the actual mathematical operations. GPUs often operate at less than 50% utilization due to memory bandwidth constraints, with external memory access dominating power consumption.

Analog systems solve this by storing neural network weights directly in memory devices that can perform computations. When IBM's PCM-based system receives input voltages, the stored conductance values multiply them automatically through Ohm's law (V×G=I), while Kirchhoff's law sums the resulting currents—effectively performing matrix-vector multiplication in a single operation. **This approach enables 35 million weight parameters to execute simultaneously**, achieving sub-microsecond inference times with minimal energy expenditure.

Measured results consistently show **10-40x energy efficiency improvements**. IBM's analog AI chip demonstrates 12.4 TOPS/W compared to roughly 1 TOPS/W for comparable digital inference systems. Research implementations have achieved even higher efficiency, with some reporting up to 397 TOPS/W for specialized workloads, though these typically operate at reduced precision.

## Digital approaches retain advantages in precision and scalability

Despite analog computing's energy benefits, digital AI accelerators maintain important advantages. **Modern GPUs and TPUs achieve higher precision** with controllable bit widths (FP32/FP16/INT8) and error-free computation, while analog systems typically operate at 3-4 bits effective precision due to device variations and noise. NVIDIA's H100 delivers ~2,000 TOPS peak theoretical performance, and while its practical inference efficiency (~30-50 TOPS/W) lags analog systems, it handles much larger models with proven reliability.

Digital systems also demonstrate **superior scalability for large models**. Current analog computing demonstrations are limited to roughly 45 million parameters, while digital systems routinely handle models with 175+ billion parameters. The software ecosystem for digital AI is mature, with established frameworks, debugging tools, and deployment pipelines that analog systems lack.

Manufacturing represents another advantage for digital approaches. Standard CMOS processes achieve >99% yield routinely, while analog precision requirements reduce effective yield and increase costs. Digital circuits tolerate process variations better than analog devices, which require tight control over materials and dimensions to maintain computational accuracy.

## Commercial ecosystem emerges with mixed results

The commercial analog AI landscape shows both promising developments and notable failures. **BrainChip leads commercial deployment** with their Akida neuromorphic processor, claiming to be the world's first commercial neuromorphic AI producer. Their development kits are available for $799, and they recently secured licensing deals for space applications with Frontgrade Gaisler.

SynSense, a Swiss-Chinese company, has raised $10 million and serves over 100 business clients with their ultra-low-power neuromorphic processors consuming less than 500μW. Their Xylo family targets smart wearables and industrial monitoring applications where battery life is critical.

However, the sector also faces significant challenges. **Rain Neuromorphics collapsed in 2024** despite $150 million in planned Series B funding and high-profile backers including Sam Altman. Mythic AI underwent major restructuring in 2023, laying off most staff before acquisition by SoftBank. These failures highlight the execution risks in translating analog computing research into viable products.

**Groq represents a hybrid approach**, raising $640 million in Series D funding with their Language Processing Units (LPUs) that combine brain-inspired architectures with digital precision. While not strictly analog, their approach demonstrates market appetite for alternative computing paradigms that promise energy efficiency improvements.

## Manufacturing and device limitations constrain near-term scaling

Analog computing faces fundamental manufacturing challenges that digital systems have largely solved. **Device variability represents the primary technical constraint**—PCM devices show ~2% read noise, while RRAM devices exhibit ~1% noise on programmed conductance values. This variability limits effective precision to 3-4 bits and requires complex calibration procedures to maintain accuracy.

**Conductance drift poses ongoing reliability challenges**, with PCM resistance values changing over time periods of days to weeks. This requires periodic recalibration and limits deployment in applications requiring long-term stability. Manufacturing yields also remain problematic, with analog systems requiring 99% programming accuracy that becomes difficult to achieve at scale.

The integration challenges are substantial. IBM's PCM process requires 14nm CMOS front-end processing combined with specialized back-end PCM integration, involving precise deposition and patterning of Ge₂Sb₂Te₅ materials. This increases cost compared to standard digital processes and limits manufacturing capacity to specialized foundries.

Current research focuses on **advanced materials and 3D integration** to address these limitations. Multi-layer RRAM demonstrations show promise for higher density, while new materials like 2D transition metal dichalcogenides offer potentially better uniformity. However, these approaches remain largely in research phases.

## Sparsity techniques amplify analog computing advantages

Recent research reveals that **network sparsity particularly benefits analog systems** by reducing both energy consumption and noise effects. NVIDIA's structured 2:4 sparsity patterns achieve 30% performance per watt improvements in digital systems, but analog implementations show even larger benefits due to their inherent parallelism.

Sparse evolutionary training techniques enable quadratic parameter reduction without accuracy loss, particularly valuable for analog systems where memory device count directly impacts chip area and power consumption. **Hardware-aware pruning methods** account for analog device characteristics, identifying which connections contribute most effectively given device noise and variability constraints.

Real-time sparsity optimization during inference represents an emerging capability unique to analog systems. Since weights are stored physically in memory devices, the system can dynamically adjust connectivity patterns based on input characteristics and power constraints—something difficult to achieve efficiently in digital architectures.

However, **attribution and interpretability remain underdeveloped** in analog systems. The inherent noise and non-idealities make it challenging to trace decision-making processes, limiting deployment in applications requiring explainable AI. Research into digital twin models for validation and interpretation shows promise but remains early-stage.

## Timeline suggests specialized deployment before mainstream adoption

Market analysis indicates analog AI will follow a **specialized-first deployment pattern** rather than attempting to replace digital systems broadly. The analog in-memory AI compute market is projected to grow from $1.45 billion in 2024 to $13.8 billion by 2033, representing 27.8% compound annual growth.

**Near-term deployment (2024-2026)** focuses on edge AI applications where energy efficiency provides clear advantages: IoT sensor processing, audio processing systems, and computer vision in controlled environments. These applications can tolerate reduced precision while benefiting significantly from energy savings.

**Medium-term expansion (2026-2028)** will likely target autonomous vehicle subsystems, advanced robotics, and scientific computing applications requiring real-time processing. Mercedes-Benz's partnership with University of Waterloo for neuromorphic computing in autonomous systems exemplifies this trajectory.

**Long-term integration (2028-2030+)** may see analog systems become standard components in hybrid architectures, handling specific computational tasks while digital systems manage precision-critical operations and complex control logic.

## Conclusion

Analog computing for AI applications has progressed from research curiosity to demonstrated commercial capability, with measurable energy efficiency improvements of 10-40x over digital alternatives. IBM's 12.4 TOPS/W chip and BrainChip's commercial Akida processor prove the technology works in practice, not just theory.

However, significant challenges remain. **Device variability limits precision to 3-4 bits**, manufacturing costs exceed digital alternatives, and software ecosystems require substantial development. The collapse of Rain Neuromorphics and restructuring of Mythic AI demonstrate execution risks in this emerging market.

The most realistic deployment scenario involves **specialized applications where energy efficiency outweighs precision limitations**: edge AI, sensor processing, and real-time systems with moderate computational complexity. Rather than replacing digital AI accelerators, analog systems will likely complement them in hybrid architectures optimized for specific workloads.

For organizations evaluating analog AI adoption, the technology offers compelling advantages for power-constrained applications, but requires careful assessment of precision requirements and software development capabilities. The 2025-2027 timeframe appears critical for establishing market position as manufacturing scales and software ecosystems mature.

## References
- <https://ioplus.nl/en/posts/watt-matters-in-ai-hardware-based-views-on-energy-efficiency>
- <https://www.nature.com/articles/s41586-025-08639-2>
- <https://research.ibm.com/blog/analog-ai-chip-low-power>
- <https://www.nature.com/articles/s44172-025-00492-5>#

#AI #programming
