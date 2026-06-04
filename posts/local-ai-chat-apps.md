---
date: '2025-04-17'
---
# Comparing Local AI Chat Apps

A short list of free AI chat apps for local LLM execution that work offline and promise privacy first (no user data collection).

| App | Type | Setup | Source | Features |
|-----|------|-------|--------|---------|
| [Digger Solo](https://solo.digger.lol) | Desktop | Easy | Closed source | RAG, privacy-first |
| [LM Studio](https://lmstudio.ai/) | Desktop | Easy | Closed source | MCP, [RAG](https://lmstudio.ai/docs/app/basics/rag), split conversations, [remote access](https://lmstudio.ai/link) |
| [Msty](https://msty.app/) | Desktop | Easy | Closed source | MCP, RAG, split conversations |
| [Deep Chat](https://github.com/thinkinaixyz/deepchat) | Desktop | Requires API access | Open source | MCP, Skills, RAG, Agents |
| [ChatBox](https://github.com/chatboxai/chatbox) | Desktop | Easy | Open source | MCP, RAG |
| [Jan](https://github.com/janhq/jan) | Desktop | Easy | Open source | MCP (broken) |
| [~~GPT4ALL~~](https://github.com/nomic-ai/gpt4all) *(discontinued)* | Desktop | Easy | Open source | RAG |
| [Onyx](https://github.com/onyx-dot-app/onyx) | Web app | Docker required | Open source | Feature rich |
| [Open WebUI](https://docs.openwebui.com/) | Web app | Complex, requires sign-up | Open source | — |
| [LibreChat](https://www.librechat.ai/) | Web app | Docker required | Open source | — |

## Conclusion

Digger Solo and LM Studio stand out. Digger Solo being a functional privacy focused chat app made for RAG, needing an API key. While LM Studio being very easy to setup (no API key required, run a local model) with also advanced features (though lacking external provider support - which Msty does though).

I recommend the Qwen family for coding and Gemma model family for local basic use.

## Local Models

For coding Qwen models are recommended and for general use Gemma models. Here is a more [detailed overview](/posts/local-llm#open-models).

## MCP Server

MCP server integration or tool use is the most important feature for any of these apps, since it allows to extend them with arbitrary functionality (access files, control programs, etc.).

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
