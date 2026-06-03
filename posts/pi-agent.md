---
icon: "/images/icons/pi.svg"
---
# Pi - Open-Source Coding Agent
An open-source coding agent rivaling claude code and openai codex.
Built to be hackable: [here](https://mariozechner.at/posts/2025-11-30-pi-coding-agent) is a great intro from the creator.
Vanilla [Pi](https://pi.dev/) is bare bones and should be configured via extensions - or start with a preconfigured version like [oh-my-pi](https://github.com/can1357/oh-my-pi) (though oh-my-pi can feel bloated compared to vanilla Pi with its long system prompt, many features and tools). Another intersting open-source coding agent that promises very efficient token usage is [Dirac](https://dirac.run/), via hash-anchored edits and AST code edits - check it out.

## Install
`$ npm install -g @earendil-works/pi-coding-agent`

Then run: `$ pi`
And add your provider with: `/login`

### LLM Providers
Best bang for the buck right now:
- OpenCode Go: https://opencode.ai/go ([pi connector](https://github.com/wahyudichrisdianto/pi-opencode-bridge))
- OpenRouter (access to all models): https://openrouter.ai/
- [Self-host](https://seanpedersen.github.io/posts/local-ai-chat-apps/#local-models) a model on GPU/s

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
- dynamic workflows (explicit subagent workflow): https://github.com/Michaelliv/pi-dynamic-workflows
- sub-agents: https://pi.dev/packages/pi-subagents
- plan mode:
  - https://pi.dev/packages/@plannotator/pi-extension
  - https://pi.dev/packages/pi-pledit
- btw command: https://github.com/dbachelder/pi-btw
- goal command: https://pi.dev/packages/@capyup/pi-goal
- beads (task management): https://pi.dev/packages/pi-beads-extension
- code index / search:
  - https://github.com/colbymchenry/codegraph
    - pi extension: https://github.com/SeanPedersen/pi-codegraph
  - https://github.com/MinishLab/semble

### Skills
Skills are useful to dynamically load relevant documents / tools into the prompt context to solve advanced tasks / use pre-defined complex workflows.

Located in $home/.agents/skills

#### Contextual Skills
An extension that selects relevant skills per project on first start of Pi (user may edit skills selection later on) - this saves context, as only skills relevant to the each project are in the agents prompt.

https://github.com/SeanPedersen/pi-context-skills

#### Other skills related packages
- https://pi.dev/packages/@kmiyh/pi-skills-menu
- https://pi.dev/packages/pi-skillful
- https://pi.dev/packages/pi-hermes-memory


## Mobile Access
https://www.pipulse.dev/
https://github.com/jademind/pi-statusbar

## References
- https://mariozechner.at/posts/2025-11-30-pi-coding-agent
-  https://pi.dev/
- https://lucumr.pocoo.org/2026/1/31/pi/

#coding #tutorial #AI
