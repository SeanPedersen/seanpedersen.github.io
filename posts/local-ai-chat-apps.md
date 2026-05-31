---
date: '2025-04-17'
---
# Comparing Local AI Chat Apps

A short list of free AI chat apps for local LLM execution that work offline and promise privacy first (no user data collection).

| App | Type | Setup | Source | Features |
|-----|------|-------|--------|---------|
| [LM Studio](https://lmstudio.ai/) | Standalone | Easy | Closed source | MCP, [RAG](https://lmstudio.ai/docs/app/basics/rag), split conversations, [remote access](https://lmstudio.ai/link) |
| [Msty](https://msty.app/) | Standalone | Easy | Closed source | MCP, RAG, split conversations |
| [Deep Chat](https://github.com/thinkinaixyz/deepchat) | Standalone | Requires API access | Open source | MCP, Skills, RAG, Agents |
| [ChatBox](https://github.com/chatboxai/chatbox) | Standalone | Easy | Open source | MCP, RAG |
| [Jan](https://github.com/janhq/jan) | Standalone | Easy | Open source | MCP (broken) |
| [~~GPT4ALL~~](https://github.com/nomic-ai/gpt4all) *(discontinued)* | Standalone | Easy | Open source | RAG |
| [Onyx](https://github.com/onyx-dot-app/onyx) | Web app | Docker required | Open source | Feature rich |
| [Open WebUI](https://docs.openwebui.com/) | Web app | Complex, requires sign-up | Open source | — |
| [LibreChat](https://www.librechat.ai/) | Web app | Docker required | Open source | — |

## Conclusion

To me Deep Chat and LM Studio stand out. Deep Chat being a functional open-source chat app, that is surprisingly feature rich. While LM Studio being very easy to setup with also advanced features (though lacking external provider support - which Msty does though).

I recommend the Qwen family for coding and Gemma model family for local basic use.

## Local Models

Run LLM models locally for complete control and privacy.
Compare model capability: [https://artificialanalysis.ai/models](https://artificialanalysis.ai/models?models=gpt-oss-20b%2Cgpt-oss-120b%2Cgpt-5-5%2Cgemini-3-1-pro-preview%2Cgemma-4-31b%2Cgemma-4-26b-a4b%2Cgemini-3-5-flash%2Cclaude-opus-4-8%2Cclaude-sonnet-4-6-adaptive%2Clfm2-5-1-2b-thinking%2Cminimax-m2-7%2Ckimi-k2-6%2Cmimo-v2-5-pro%2Cqwen3-6-35b-a3b%2Cqwen3-6-27b%2Cqwen3-7-max&intelligence=artificial-analysis-intelligence-index)
Find compatible models for your hardware: https://www.canirun.ai/ or try https://github.com/AlexsJones/llmfit

Curated Model List:
- [Qwen3.6-35B-A3B Q4_K_XL](https://huggingface.co/unsloth/Qwen3.6-35B-A3B-MTP-GGUF): Strong MoE (3B active) model with MTP fits on 8GB VRAM GPUs
  - https://x.com/witcheer/status/2053809265538678789
  - https://www.reddit.com/r/LocalLLaMA/comments/1tc132c/llamacpp_docker_images_to_run_mtp_models/
- [Qwen3.6 27B Q3_K_M](https://huggingface.co/unsloth/Qwen3.6-27B-GGUF) - dense model, very good can run on 16GB VRAM
- [MiniCPM5-1B](https://huggingface.co/openbmb/MiniCPM5-1B) - optimized for mobile CPU/NPU inference (32k context window)
- [LFM2.5-8B-A1B](https://huggingface.co/LiquidAI/LFM2.5-8B-A1B) - strong MoE model 1.5B active + 128k context

[llama.cpp](https://github.com/ggml-org/llama.cpp) and [vLLM](https://github.com/vllm-project/vllm) are the best inference run time servers for consumer GPUs.

## MCP Server

MCP server integration is the most important feature for any of these apps, since it allows to extend them with arbitrary functionality (access files, control programs, etc.).

Here are some useful ones:
- [youtube-transcript-mcp](https://github.com/SeanPedersen/youtube-transcript-mcp): transcribe and summarize youtube videos
  - `uvx youtube-transcript-mcp-server`
- [ddgs free search mcp](https://github.com/deedy5/ddgs): free web search mcp for duckduckgo
  - `uv run --with 'ddgs[mcp,api]' ddgs mcp`
 - [context7](https://github.com/upstash/context7): free code package documentation
   - `npx -y @upstash/context7-mcp`

## References

- https://artificialanalysis.ai/

#AI #tutorial #privacy
