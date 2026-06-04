# Making Local LLM Go Brrr

How to run your local LLM well: fast, reliable and with good quality.

Key metrics:
- Prefill speed: prompt/input tokens per second
- Decode speed: generated tokens per second
- Time to first token (latency)
- VRAM usage at target context length
- Quality at chosen model/quant/context settings
- Concurrency, if serving multiple users

## Software

Choose the serving stack based on workload:
- [llama.cpp](https://github.com/ggml-org/llama.cpp): best general local path, especially GGUF, CPU, Apple Silicon, and mixed CPU/GPU.
- [vLLM](https://github.com/vllm-project/vllm): strong GPU server for batching, throughput, OpenAI-compatible APIs, and production-style serving for modern GPUs.
- [SGLang](https://github.com/sgl-project/sglang): good for structured/agentic serving and high-throughput multi-call workloads for modern GPUs.

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
- Evaluate KV-cache compression such as TurboQuant for long-context or high-concurrency workloads, but treat it as experimental until benchmarked.

Performant llama.cpp fork (with advanced features): https://ikawrakow-ik_llama-cpp.mintlify.app/inference/

Tool-calling reliability:
- Add middleware for schema repair, retries, validation, and constrained tool loops -> https://github.com/antoinezambelli/forge

TODO:
- eval dynamic model routing based on query complexity (fast vs smart model)

## Open Models
Run LLM models locally for complete control and privacy. Open-source (reproducible training) vs open-weight (free model weights) models.

Compare model capability: [https://artificialanalysis.ai/models](https://artificialanalysis.ai/models?models=gpt-oss-20b%2Cgpt-oss-120b%2Cgpt-5-5%2Cgemini-3-1-pro-preview%2Cgemma-4-31b%2Cgemma-4-26b-a4b%2Cgemini-3-5-flash%2Cclaude-opus-4-8%2Cclaude-sonnet-4-6-adaptive%2Clfm2-5-1-2b-thinking%2Cminimax-m2-7%2Ckimi-k2-6%2Cmimo-v2-5-pro%2Cqwen3-6-35b-a3b%2Cqwen3-6-27b%2Cqwen3-7-max&intelligence=artificial-analysis-intelligence-index)
Find compatible models for your hardware: https://www.canirun.ai/ or try https://github.com/AlexsJones/llmfit
Community benchmarks for local LLM: https://localmaxxing.com

Curated open model list:
- [Qwen3.6-35B-A3B Q4_K_XL](https://huggingface.co/unsloth/Qwen3.6-35B-A3B-MTP-GGUF): Strong MoE (3B active) model with MTP fits on 8GB VRAM GPUs
  - [Uncensored version](https://huggingface.co/LuffyTheFox/Qwen3.6-35B-A3B-Uncensored-Genesis-V2-APEX-MTP-GGUF) - [Reddit thread](https://www.reddit.com/r/LocalLLaMA/comments/1tm3toi/qwen3635ba3buncensoredgenesisapexmtp/)
  - https://x.com/witcheer/status/2053809265538678789
  - https://www.reddit.com/r/LocalLLaMA/comments/1tc132c/llamacpp_docker_images_to_run_mtp_models/
- [Qwen3.6 27B Q3_K_M](https://huggingface.co/unsloth/Qwen3.6-27B-GGUF) - dense model, very good can run on 16GB VRAM
- [LFM2.5-8B-A1B](https://huggingface.co/LiquidAI/LFM2.5-8B-A1B) - very fast MoE model 1.5B active + 128k context (agentic usefulness is limited though...)
- [MiniCPM5-1B](https://huggingface.co/openbmb/MiniCPM5-1B) - optimized for mobile CPU/NPU inference (32k context window)

## Hardware
VRAM matters more than raw TFLOPs for model & context (prompt) size, but memory bandwidth and tensor cores matter for speed. Used datacenter GPUs can be good value, but check form factor, cooling, power, driver support, and PCIe vs SXM.

Interesting used options:
- Tesla V100 16/32GB: strong used datacenter option, but check PCIe vs SXM and cooling.
- Tesla P40 24GB: lots of VRAM for cheap, slower and no Tensor Cores.
- Pascal P100 16GB: cheap, but old and less attractive than V100/P40 depending on workload.
- GTX 1080 Ti 11GB: cheap but VRAM-limited.
- RTX 3090 24GB: often the practical local LLM sweet spot if priced well.

TODO:
- Check current Intel Arc and AMD ROCm support.
- Compare used datacenter GPUs against RTX 3090/4090/5090-class consumer cards.
- Benchmark watts/token, not just tokens/sec.

## References
- https://vllm.ai/blog/2026-05-11-turboquant
- https://blog.tymscar.com/posts/v100localllm/

#AI #tutorial
