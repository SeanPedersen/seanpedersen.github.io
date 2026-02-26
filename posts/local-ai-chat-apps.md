---
date: '2025-04-17'
---
# Comparing Local AI Chat Apps

A short list of free AI chat apps for local LLM execution that work offline and promise privacy first (no user data collection).

**LM Studio** - <https://lmstudio.ai/>

- Standalone app: easy setup
- Closed Source
- Features: [RAG over text files](https://lmstudio.ai/docs/app/basics/rag), split (copy) conversations, [remote access](https://lmstudio.ai/link), MCP support

**Jan** - <https://github.com/janhq/jan>

- Standalone app: easy setup
- Open source
- MCP support

**Msty** - <https://msty.app/>

- Standalone app: easy setup
- Closed Source
- Features: RAG over text files, split (copy) conversations, web search (unreliable right now)

**GPT4ALL** - <https://github.com/nomic-ai/gpt4all>

- Standalone app: easy setup
- Open source
- Features: RAG over text files

**Deep Chat** - <https://deepchat.thinkinai.xyz/>

- Standalone app: requires running ollama instance
- Open source
- Features: integrated web search (Brave Search, Google), MCP support

**ChatBox** - <https://github.com/chatboxai/chatbox>

- Standalone app: easy setup
- Open source
- Features: (not yet tested)

**Onyx** - <https://github.com/onyx-dot-app/onyx>

- Web app: hard setup (requires Docker)
- Open source
- Feature rich

**Open WebUI** - <https://docs.openwebui.com/>

- Web app: hard setup (Install is complicated, requires sign-up)
- Open source

**LibreChat** - <https://www.librechat.ai/>

- Web app: hard setup (requires Docker)
- Open source

## Conclusion

To me Jan and LM Studio stand out. Jan being a functional open-source chat app, that is easy to setup. While LM Studio being also easy to setup but with more advanced features. LM Studio is the most feature rich LLM chat app.

I recommend the Llama and Gemma model family for local basic use. DeepSeek R1 for advanced reasoning and coding capabilities.

## Local Models

- [LFM2.5](https://huggingface.co/LiquidAI/LFM2.5-1.2B-Instruct) - optimized for mobile CPU/NPU inference (32k context window)
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
