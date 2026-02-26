# Optimal Coding Agent

My blueprint for the optimal coding agent (team).
Keeping LLMs in the smart zone using short and relevant context.

## IDEATOR / MUSE

Agent helps you crystallize your vision and define a hard success criterion + web access.

#### DESIGNER
Uses knowledge base: tool to query existing solved solutions + web access (query tools like deepwiki.com). Designer also needs relevant context to create good architecture (blog posts, papers, books, etc)

- translates idea into objects / modules and interactions
- defines interfaces / boundaries for modules
- produces a design with specifications
- discuss best tools (tech stack) for the design specs
  
  ## GENERAL
  Create the battle plan - brief the troops.
  
  #### ORCHESTRATOR
  
  Generate the task tree from design + tech stack.

- create modular plan with task execution dependency tree
  - every task has:
    - ID
    - Problem: in- / output typed interface + internal behavior / structure
    - Criterion: success metric / test
    - Dependencies: task IDs
    - Complexity: low, mid, high
      - maps to small / big LLM
      - can assign multiple models to work on same task with different approaches
    - Context: added by INSTRUCTOR
    - State: defined, open, busy, trial, done, fail
    - StateHistory: List<(state, date, reason, author_agent)>

Refs:
- https://github.com/steveyegge/beads
- https://github.com/joshuadavidthomas/opencode-beads
- https://github.com/Dicklesworthstone/beads_viewer
- Claude Code released tasks feature (similar to beads)
  
  #### INSTRUCTOR
  
  Provides every task with perfect (relevant) Context.

- NEW: code templates for fresh projects
- relevant and version matching documentation for coding tasks
  - Coding guidelines based on programming languages / code files involved for the task
  - General use: official matching version docs / blog posts
  - Existing code: analyse package dependencies
    - exact (version) matching code documentation
    - source code -> extract public function interfaces with doc strings
      - Python: .venv/lib/python3.11/site-packges/$package_name
      - JS: node_modules/

Refs:
- https://github.com/iannuttall/librarian
- https://github.com/upstash/context7
- https://ref.tools/
- https://x.com/karpathy/status/2021633574089416993
- [CodeRLM](https://github.com/JaredStewart/coderlm/tree/main)
  
  ## EXECUTOR SWARM
  Coding agents work on tasks in parallel - working off the task tree backwards recursively (breadth first, towards the root task - the IDEA).
  
  #### CREATOR
- use LSP server for precise edits based on TASK problem and context
  - optimal code exploration
  - optimal code editing
    
    #### VALIDATOR
- validates if CREATOR solved tasks Criterion (test / metric)
  - yes: task done
  - no: retry task with modified context (add failures / learnings)
    
    #### REFLECTOR
- uses REPL / MCP to inspect live vars of code produced by CREATOR
- fixes any issues spotted by VALIDATOR
  
  ## TASK Life Cycle
  
  State: defined, open, busy, trial, done, fail

- defined: ID, Problem, Criterion, Complexity
- open: added Context by INSTRUCTOR
- busy: CREATOR is coding
- trial: CREATOR finished
- VALIDATOR is happy -> done
- VALIDATOR is unhappy -> open
- VALIDATOR gives up -> fail

Typical Project:

- task 0 (root task) is the IDEA itself and is validated at last when all subtasks are done
  the first task to tackle is the last task we planned out:
- setup coding environment for new project

User Interface:

- saves chats as text files (easy to inspect whole context)
- displays (token usage/context limit) in current session

new:
when agent sees multiple solutions explore in parallel

tool usage (MCP / CLI):
- should be contextual -> orchestrator adds only relevant tools to task context

## References

Existing Agents:
- https://www.mihaileric.com/The-Emperor-Has-No-Clothes/
  - https://news.ycombinator.com/item?id=46545620
- https://pi.dev/
  - https://mariozechner.at/posts/2025-11-30-pi-coding-agent/
  - https://news.ycombinator.com/item?id=46844822
  - https://lucumr.pocoo.org/2026/1/31/pi/
- https://github.com/openai/codex
- https://github.com/jacobsparts/agentlib - really cool project - drops the agent directly into Python repl

- replace claude code with cheaper + faster alternatives
  - https://steipete.me/posts/2025/self-hosting-ai-models
  - https://www.builder.io/blog/opencode-vs-claude-code
  - https://www.reddit.com/r/LocalLLaMA/comments/1o22iwo/ideal_cost_effective_agentic_coding_membership/
  - https://dev.to/karthidreamr/why-i-ditched-chatgpt-and-claude-for-opencode-a-smarter-cheaper-way-to-build-ai-agents-2a5h

Eval extending existing open-source coding agents TUI or GUI apps:

- https://github.com/wandb/catnip
- https://github.com/block/goose seems to be really close to my vision - check it out
- https://thebob.dev/ai/tools/productivity/2025/10/31/why-we-built-claude-os-and-what-it-actually-is/
- https://anandchowdhary.com/blog/2025/running-claude-code-in-a-loop
- https://ampcode.com/

Fast LLMs:
groq.com, https://www.inceptionlabs.ai/, https://chat.z.ai/, https://huggingface.co/chat/

- https://github.com/bmad-code-org/BMAD-METHOD
- https://xr0am.substack.com/p/what-ralph-wiggum-loops-are-missing

#coding #AI
