# Making Local LLM Go Brrr

How to run your local LLM well: fast, reliable and with good quality.

Key metrics:
- Prefill speed: prompt/input tokens per second
- Decode speed: generated tokens per second
- Time to first token (latency)
- VRAM usage at target context length
- Quality at chosen model/quant/context settings
- Concurrency, if serving multiple users

## Software (Inference)

Choose the serving stack based on workload:
- [llama.cpp](https://github.com/ggml-org/llama.cpp): best general local path, especially GGUF, CPU, Apple Silicon, and mixed CPU/GPU.
  - [beelama.cpp](https://github.com/Anbeeld/beellama.cpp): DFlash & TurboQuant in llama.cpp with up to 3x faster generation and 7.5x more KV cache in same VRAM
  - [ik_llama.cpp](https://github.com/ikawrakow/ik_llama.cpp): llama.cpp fork with additional SOTA quants and improved performance
- [vLLM](https://github.com/vllm-project/vllm): strong GPU server for batching, throughput, OpenAI-compatible APIs, and production-style serving for modern GPUs.
- [SGLang](https://github.com/sgl-project/sglang): good for structured/agentic serving and high-throughput multi-call workloads for modern GPUs.
- [ZML](https://github.com/zml/zml): Zig based model run time.
- [LuceBox](https://github.com/Luce-Org/lucebox-hub): Local LLM inference server built for speed. Custom kernels, speculative prefill & decoding. (very advanced optimizations like [hot MoE VRAM cache](https://www.lucebox.com/blog/spark))
- [Uzu](https://github.com/trymirai/uzu): A high-performance inference engine for AI models (optimized for Apple hardware)

Performance checklist:
- Use the fastest supported attention kernels: FlashAttention, FlashInfer, FlashMLA, etc.
- Try speculative decoding / MTP / EAGLE-style decoding when supported, but benchmark with your actual model and sampling settings.
- Preserve prefix/KV cacheability:
  - keep the system prompt byte-identical
  - append new messages rather than rebuilding/changing history
  - avoid dynamic timestamps or changing tool schemas in the prompt prefix
  - use server-side prefix caching when available
- Tune KV cache precision:
  - for long context, test q8 KV even with q4 weights
  - aggressively quantized KV can hurt long-context coherence
- Approximate KV-cache compression for long-context or high-concurrency workloads:
  - [TurboQuant](https://github.com/0xSero/turboquant): developed by Google ([blog article](https://research.google/blog/turboquant-redefining-ai-efficiency-with-extreme-compression/))
  - [KVarN](https://github.com/huawei-csl/KVarN): claims to beat TurboQuant on all dimensions ([beellama.cpp](https://github.com/Anbeeld/beellama.cpp) supports it)

Tool-calling reliability:
- Add middleware for schema repair, retries, validation, and constrained tool loops -> https://github.com/antoinezambelli/forge

TODO:
- eval dynamic model routing based on query complexity (fast vs smart model)

## Open Models
Run LLM models locally for complete control and privacy. Open-source (reproducible training) vs open-weight (free model weights) models.

Compare model capability: [https://artificialanalysis.ai/models](https://artificialanalysis.ai/models?models=gpt-oss-20b%2Cgpt-oss-120b%2Cgpt-5-5%2Cgemini-3-1-pro-preview%2Cgemma-4-31b%2Cgemma-4-26b-a4b%2Cgemini-3-5-flash%2Cclaude-opus-4-8%2Cclaude-sonnet-4-6-adaptive%2Clfm2-5-1-2b-thinking%2Cminimax-m2-7%2Ckimi-k2-6%2Cmimo-v2-5-pro%2Cqwen3-6-35b-a3b%2Cqwen3-6-27b%2Cqwen3-7-max&intelligence=artificial-analysis-intelligence-index)
Find compatible models for your hardware: https://www.canirun.ai/ or try https://github.com/AlexsJones/llmfit -> rule of thumb: plan 70% of VRAM for the model weights (dense) and 20% for the KV-Cache.
Community benchmarks for local LLM: https://localmaxxing.com

Most models are too big for consumer GPUs, so quantized versions (compressed parameters) are used. [Mixture of Quants](https://huggingface.co/w-ahmad/Qwen3.5-9B-GGUF-MoQ) (MoQ) is a new very efficient quant variant that does not quant weights uniformly but based on importance.

**Curated open model list**:
- [Qwen3.6-35B-A3B Q4_K_XL](https://huggingface.co/unsloth/Qwen3.6-35B-A3B-MTP-GGUF): Strong MoE (3B active) model with MTP fits on 8GB VRAM GPUs
  - [Uncensored version](https://huggingface.co/LuffyTheFox/Qwen3.6-35B-A3B-Uncensored-Genesis-V2-APEX-MTP-GGUF) - [Reddit thread](https://www.reddit.com/r/LocalLLaMA/comments/1tm3toi/qwen3635ba3buncensoredgenesisapexmtp/)
  - https://x.com/witcheer/status/2053809265538678789
  - https://www.reddit.com/r/LocalLLaMA/comments/1tc132c/llamacpp_docker_images_to_run_mtp_models/
  - Custom thinking grammar (limit overthinking): https://github.com/andthattoo/structured-cot
    - TODO: find optimal thinking grammar using [GEPA](https://github.com/gepa-ai/gepa)
- [Qwen3.6 27B Q3_K_M](https://huggingface.co/unsloth/Qwen3.6-27B-GGUF) - dense model, very good (Sonnet 4.6 performance) can run on 16GB VRAM
- [Qwen3.5 9B Distilled](https://huggingface.co/mradermacher/Qwen3.5-9B-GLM5.1-Distill-v1-i1-GGUF) - small but capable agentic dense model good for <8GB VRAM
  - [MoQ](https://huggingface.co/w-ahmad/Qwen3.5-9B-GGUF-MoQ/tree/main/MoQ-Quants-Latest) variant (very efficient quantization)
- [LFM2.5-8B-A1B](https://huggingface.co/LiquidAI/LFM2.5-8B-A1B) - very fast MoE model 1.5B active + 128k context (agentic usefulness is limited though...)
- [MiniCPM5-1B](https://huggingface.co/openbmb/MiniCPM5-1B) - optimized for mobile CPU/NPU inference (32k context window)

## Hardware (GPU)
VRAM matters more than raw TFLOPs for model & context (prompt) size, but memory bandwidth and tensor cores matter for speed. Used datacenter GPUs can be good value, but check form factor, cooling, power, driver support, and PCIe vs SXM.

Interesting used GPU options:

| GPU | VRAM | Bandwidth | TDP | FP32 TFLOPS | FP16 TFLOPS | Notes |
|---|---|---|---|---|---|---|
| Tesla V100 (SXM2) | 16/32 GB HBM2 | 900 GB/s | 300W | 15.7 | 125 Tensor | Needs SXM board or riser, check cooling. |
| Tesla V100 (PCIe) | 16/32 GB HBM2 | 750 GB/s | 250W | 14.1 | 112 Tensor | Standard form factor, strong used option. |
| Tesla P40 | 24 GB GDDR5X | 346 GB/s | 250W | 12.0 | 12.0 | Lots of VRAM for cheap, no Tensor Cores. |
| Tesla P100 (PCIe) | 16 GB HBM2 | 732 GB/s | 250W | 9.5 | 19.1 | Cheap, but old — less attractive than V100/P40. |
| GTX 1080 Ti | 11 GB GDDR5X | 484 GB/s | 250W | 11.3 | 11.3 | Cheap but VRAM-limited, no Tensor Cores. |
| RTX 3090 | 24 GB GDDR6X | 936 GB/s | 350W | 35.6 | 71.2 Tensor | Often the practical local LLM sweet spot. |
| Intel Arc A770 | 16 GB GDDR6 | 560 GB/s | 225W | 19.7 | 39.3 XMX | Good llama.cpp SYCL support; get the 16 GB variant. |
| Intel Arc B580 | 12 GB GDDR6 | 456 GB/s | 190W | 14.4 | 28.8 XMX | Battlemage arch, better perf/watt than A770, solid llama.cpp support. |
| AMD BC-250 | 16 GB GDDR6 | 448 GB/s | 220W | 6.9 | 13.8 | Mining card based on PS5 APU, ROCm support varies. |

One or two used Tesla V100 16GB cards are the best bang for the buck.

TODO:
- Check current AMD ROCm support.
- Compare used datacenter GPUs against RTX 3090/4090/5090-class consumer cards.
- Benchmark watts/token, not just tokens/sec.

## References
- https://vllm.ai/blog/2026-05-11-turboquant
- https://blog.tymscar.com/posts/v100localllm/

#AI #tutorial
