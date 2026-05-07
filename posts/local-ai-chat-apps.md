---
date: '2025-04-17'
---
# Comparing Local AI Chat Apps

A short list of free AI chat apps for local LLM execution that work offline and promise privacy first (no user data collection).

| App | Type | Setup | Source | Features |
|-----|------|-------|--------|---------|
| [Jan](https://github.com/janhq/jan) | Standalone | Easy | Open source | MCP |
| [LM Studio](https://lmstudio.ai/) | Standalone | Easy | Closed source | [RAG](https://lmstudio.ai/docs/app/basics/rag), split conversations, [remote access](https://lmstudio.ai/link), MCP |
| [Msty](https://msty.app/) | Standalone | Easy | Closed source | RAG, split conversations, web search (unreliable) |
| [Deep Chat](https://deepchat.thinkinai.xyz/) | Standalone | Requires API access | Open source | Web search (Brave, Google), MCP, Skills |
| [ChatBox](https://github.com/chatboxai/chatbox) | Standalone | Easy | Open source | (not yet tested) |
| [~~GPT4ALL~~](https://github.com/nomic-ai/gpt4all) *(discontinued)* | Standalone | Easy | Open source | RAG |
| [Onyx](https://github.com/onyx-dot-app/onyx) | Web app | Docker required | Open source | Feature rich |
| [Open WebUI](https://docs.openwebui.com/) | Web app | Complex, requires sign-up | Open source | — |
| [LibreChat](https://www.librechat.ai/) | Web app | Docker required | Open source | — |

## Conclusion

To me Jan and LM Studio stand out. Jan being a functional open-source chat app, that is easy to setup. While LM Studio being also easy to setup but with more advanced features. LM Studio is the most feature rich LLM chat app.

I recommend the Qwen family for coding and Gemma model family for local basic use.

## Local Models

- [LFM2.5](https://huggingface.co/LiquidAI/LFM2.5-1.2B-Instruct) - optimized for mobile CPU/NPU inference (32k context window)
- Gemma4
- [Qwen Coder Next](https://huggingface.co/Qwen/Qwen3-Coder-Next) - agentic coding model

## MCP Server

MCP server integration is the most important feature for any of these apps, since it allows to extend them with arbitrary functionality (access files, control programs, etc.).

## LLM CLI Tool Integration

Set in Msty options under section "Local AI", the service endpoint (which uses Ollama) to <http://localhost:11434> (the default port for Ollama) or just install and run [Ollama](https://ollama.com/). Next install the CLI tool [llm](https://github.com/simonw/llm?tab=readme-ov-file#installation), which lets you interact with LLM's from the CLI and the Ollama plugin: `$ llm install llm-ollama`.

Now you can access (while Msty or Ollama is running), your local LLM models from the CLI using f.e.: `$ llm -m gemma3 "Tell me a joke about two sausages in a bar."` (specify the model you installed with -m option - list intalled models like so: `$ curl http://localhost:11434/api/tags | jq .`)

And the cool thing about llm is, you can pipe into it: `$ echo "omnia mirari, gaudium explorandi" | llm -m gemma3 -s "Translate into English:"`

Enabling following use case: summarize youtube video transcripts (install f.e. using pipx <https://pypi.org/project/youtube-transcript-api/> and <https://pypi.org/project/streamdown/0.18.0/>) and use this bash function:

```bash
yt-summarize () {
 local video_id
 if [[ "$1" == http* ]]
 then
  video_id=$(echo "$1" | sed -E 's/.*[?&]v=([^&]+).*/\1/; s/.*youtu\.be\/([^?&]+).*/\1/')
 else
  video_id="$1"
 fi
 youtube_transcript_api --format text $video_id | llm -m gemma3 -s "Summarize this transcript. Exclude any mentions of sponsors, advertisements, product placements, or promotional content (e.g., 'This video is sponsored by', 'free trial', '30-day trial', 'promo code', 'Use code', 'Check out', 'Brilliant.org', 'Brilliant', 'NordVPN'). Only include the main educational or informational content and remove the rest:" | streamdown
}
```

#AI #tutorial #privacy
