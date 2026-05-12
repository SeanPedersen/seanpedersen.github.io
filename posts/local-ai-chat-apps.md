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

- [Gemma4](https://huggingface.co/google/gemma-4-E4B-it) - new model by Google
- [Qwen Coder Next](https://huggingface.co/Qwen/Qwen3-Coder-Next) - agentic coding model
- [Qwen3 14B](https://huggingface.co/Qwen/Qwen3-14B-GGUF) - very good can run on 16GB VRAM
- [LFM2.5](https://huggingface.co/LiquidAI/LFM2.5-1.2B-Instruct) - optimized for mobile CPU/NPU inference (32k context window)

[llama.cpp](https://github.com/ggml-org/llama.cpp) and [vLLM](https://github.com/vllm-project/vllm) are the best run time servers for local inference.

## MCP Server

MCP server integration is the most important feature for any of these apps, since it allows to extend them with arbitrary functionality (access files, control programs, etc.).

Here are some useful ones:
- [youtube-transcript-mcp](https://github.com/SeanPedersen/youtube-transcript-mcp): transcribe and summarize youtube videos
  - `uvx youtube-transcript-mcp-server`
- [ddgs free search mcp](https://github.com/deedy5/ddgs): free web search mcp for duckduckgo
  - `uv run --with 'ddgs[mcp,api]' ddgs mcp`
 - [context7](https://github.com/upstash/context7): free code package documentation
   - `npx -y @upstash/context7-mcp`

#AI #tutorial #privacy
