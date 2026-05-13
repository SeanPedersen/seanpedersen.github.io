# Pi - Open-Source Coding Agent
An open-source coding agent rivaling claude code and openai codex.
Vanilla Pi is bare bones and should be configured via extensions - or start with a preconfigured version like [oh-my-pi](https://github.com/can1357/oh-my-pi) (though oh-my-pi can feel bloated compared to vanilla Pi with its long system prompt, many features and tools).

## Customize Pi

### Prompts

~/.pi/agent/SYSTEM.md : overwrites the global Pi system prompt (be careful)
~/.pi/agent/APPEND_SYSTEM.md : appends to the global Pi system prompt

Pi loads `AGENTS.md` or `CLAUDE.md` at startup from:
  - `~/.pi/agent/AGENTS.md` for global instructions
  - parent directories, walking up from the current working directory
  - the current directory

### Packages

Install extensions (packages) from the pi packages marketplace (review code before you use them): https://pi.dev/packages

Your custom extensions live in: $home/.pi/agent/extensions
Installed extensions (packages) are in: TODO

show complete prompt context: https://pi.dev/packages/pi-system-prompt?name=skills

### Skills
are useful to dynamically load relevant documents / tools into the prompt context to solve advanced tasks / use pre-defined complex workflows.

located in $home/.agents/skills

https://pi.dev/packages/pi-skillful?name=skills

## References
- https://pi.dev/

#coding #tutorial #AI
