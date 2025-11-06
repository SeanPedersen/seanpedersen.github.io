---
title: 'Analog Computing - The future is continuous'
date: '2025-09-12'
---
The artificial intelligence industry faces a growing energy crisis. Training large language models consumes massive amounts of power, while deploying AI at the edge demands efficiency that traditional digital processors struggle to deliver. This challenge has created interest in analog computing: which performs calculations using continuous physical quantities rather than discrete digital values. Recent breakthroughs suggest **analog computing could deliver 10-40x energy efficiency improvements** over conventional digital AI accelerators while maintaining competitive accuracy for specific applications.

## Recent academic breakthroughs

The past two years have witnessed significant progress in analog AI research, with major publications demonstrating systems that work in practice rather than just theory. IBM's team published landmark results in Nature showing their 14nm analog AI chip achieving 12.4 TOPS/W chip-sustained performance, roughly 14x more energy efficient than comparable digital systems for speech recognition tasks. The chip integrates 35 million phase-change memory (PCM) devices across 34 tiles, successfully running a 45-million parameter speech recognition model with only minimal accuracy degradation.

The arXiv repository shows accelerating publication rates in analog deep learning, with comprehensive reviews examining eight different analog computing methodologies. These papers focus increasingly on practical implementations rather than theoretical possibilities, suggesting the field is maturing toward commercial viability.

## Energy efficiency advantages

Analog computing's primary advantage lies in eliminating the von Neumann bottleneck - the energy-expensive separation between memory and computation that defines traditional digital architectures. In conventional AI processors, data movement between memory and compute units consumes 100-1000x more energy than the actual mathematical operations. GPUs often operate at less than 50% utilization due to memory bandwidth constraints, with external memory access dominating power consumption.

Analog systems solve this by storing neural network weights directly in memory devices that can perform computations. When IBM's PCM-based system receives input voltages, the stored conductance values multiply them automatically through Ohm's law (V×G=I), while Kirchhoff's law sums the resulting currents—effectively performing matrix-vector multiplication in a single operation. This approach enables 35 million weight parameters to execute simultaneously, achieving sub-microsecond inference times with minimal energy expenditure.

Measured results consistently show 10-40x energy efficiency improvements. IBM's analog AI chip demonstrates 12.4 TOPS/W compared to roughly 1 TOPS/W for comparable digital inference systems. Research implementations have achieved even higher efficiency, with some reporting up to 397 TOPS/W for specialized workloads, though these typically operate at reduced precision.

## Digital computing retains advantages

Despite analog computing's energy benefits, digital AI accelerators maintain important advantages. Modern GPUs and TPUs achieve higher precision with controllable bit widths (FP32/FP16/INT8) and error-free computation, while analog systems typically operate at 3-4 bits effective precision due to device variations and noise. NVIDIA's H100 delivers ~2,000 TOPS peak theoretical performance, and while its practical inference efficiency (~30-50 TOPS/W) lags analog systems, it handles much larger models with proven reliability.

Digital systems also demonstrate superior scalability for large models. Current analog computing demonstrations are limited to roughly 45 million parameters, while digital systems routinely handle models with 175+ billion parameters. The software ecosystem for digital AI is mature, with established frameworks, debugging tools, and deployment pipelines that analog systems lack.

Manufacturing represents another advantage for digital approaches. Standard CMOS processes achieve >99% yield routinely, while analog precision requirements reduce effective yield and increase costs. Digital circuits tolerate process variations better than analog devices, which require tight control over materials and dimensions to maintain computational accuracy.

## Commercial adoption

The commercial analog AI landscape shows both promising developments and notable failures. [BrainChip](https://brainchip.com/technology/) leads commercial deployment with their Akida neuromorphic processor, claiming to be the world's first commercial neuromorphic AI producer. Their development kits are available for $799, and they recently secured licensing deals for space applications with Frontgrade Gaisler.

[SynSense](https://www.synsense.ai/), a Swiss-Chinese company, has raised $10 million and serves over 100 business clients with their ultra-low-power neuromorphic processors consuming less than 500μW. Their Xylo family targets smart wearables and industrial monitoring applications where battery life is critical.

However, the sector also faces significant challenges. Rain Neuromorphics collapsed in 2024 despite $150 million in planned Series B funding and high-profile backers including Sam Altman. Mythic AI underwent major restructuring in 2023, laying off most staff before acquisition by SoftBank. These failures highlight the execution risks in translating analog computing research into viable products.

## Conclusion

Analog computing for AI applications has progressed from research curiosity to demonstrated commercial capability, with measurable energy efficiency improvements of 10-40x over digital alternatives. IBM's 12.4 TOPS/W chip and BrainChip's commercial Akida processor prove the technology works in practice, not just theory.

However, significant challenges remain. Device variability limits precision to 3-4 bits, manufacturing costs exceed digital alternatives and software ecosystems require substantial development.

## References
- <https://ioplus.nl/en/posts/watt-matters-in-ai-hardware-based-views-on-energy-efficiency>
- <https://www.nature.com/articles/s41586-025-08639-2>
- <https://research.ibm.com/blog/analog-ai-chip-low-power>
- <https://www.nature.com/articles/s44172-025-00492-5>
- <https://x.com/brianroemmele/status/1966473607795929527?s=46>

#AI
