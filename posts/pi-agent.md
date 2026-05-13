---
icon: "/images/icons/pi.svg"
---
# Pi - Open-Source Coding Agent
An open-source coding agent rivaling claude code and openai codex. Built to be hackable: [here](https://mariozechner.at/posts/2025-11-30-pi-coding-agent) is a great intro from the creator.
Vanilla [Pi](https://pi.dev/) is bare bones and should be configured via extensions - or start with a preconfigured version like [oh-my-pi](https://github.com/can1357/oh-my-pi) (though oh-my-pi can feel bloated compared to vanilla Pi with its long system prompt, many features and tools).

## Install
`$ npm install -g @earendil-works/pi-coding-agent`

Then run: `$ pi`
And add your provider with: `/model`

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

- show complete prompt context: https://pi.dev/packages/pi-system-prompt
- sub-agents: https://pi.dev/packages/pi-subagents
- plan mode:
  - https://pi.dev/packages/@plannotator/pi-extension
  - https://pi.dev/packages/pi-pledit
- btw command: https://github.com/dbachelder/pi-btw
- goal command: https://pi.dev/packages/@capyup/pi-goal
- beads (task management): https://pi.dev/packages/pi-beads-extension

### Skills
Skills are useful to dynamically load relevant documents / tools into the prompt context to solve advanced tasks / use pre-defined complex workflows.

Located in $home/.agents/skills

Useful packages to manage skills:
- https://pi.dev/packages/@kmiyh/pi-skills-menu
- https://pi.dev/packages/pi-skillful
- https://pi.dev/packages/pi-hermes-memory

## References
- https://mariozechner.at/posts/2025-11-30-pi-coding-agent
-  https://pi.dev/
- https://lucumr.pocoo.org/2026/1/31/pi/

#coding #tutorial #AI
