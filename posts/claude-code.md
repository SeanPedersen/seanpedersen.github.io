---
date: '2025-07-03'
---
# Getting started with Claude Code

Claude Code is a powerful CLI based coding agent. It comes with MCP support which enables it to use external tools like web browsers to automatically validate and test new features in web apps. Popular alternatives are OpenAI's [codex](https://openai.com/codex/), [pi-agent](https://pi.dev/) and [opencode](https://opencode.ai/).

Simple and popular tech stacks that are fast to validate are the ideal choices for vibe coding (lots of training data available -> less hallucinations and fast / no  compilation -> allows for rapid iteration).

## Setting Things up

Install Claude Code: `curl -fsSL https://claude.ai/install.sh | bash`

Change dir to a code project and start Claude Code using: `claude` and authenticate.

Now you can ask claude about the project or instruct it to implement a new feature. Generally it is advised to plan out the architecture (design) yourself and then use Claude to implement small features which are easy to validate.

Bonus: Use [this awesome project](https://github.com/ericc-ch/copilot-api) to use your Github Copilot subscription with claude code

## Starting Projects

It is recommended to use proven code base templates / example repositories that are correctly configured with all needed boilerplate for a project (based on a popular tech stack as mentioned before) - so that the code agents has a working starting point and can not fumble the project setup. Always aim to create tight evaluation / test loops where the agent can verify its progress on its own without you needing to hold hands.

Vibe code project flow:
- general idea with meta-prompt to act as app designer and to ask clarifying questions
- create technical design with clear specifications
- ideally find a fitting project code template (boilerplate with basics setup)
- insert relevant tech stack code documentation (with correct code version)
- create end-to-end test cases based on the specifications
- submit the fletched out technical design to the coding agent
- advanced: identify independent modules and spawn multiple subagents to work in parallel

## General Advice

As with every LLM, Claude Code suffers from context rot - meaning the longer and noisier the context (input context windows) becomes, the worse the performance gets and the more tokens are used. Thus it is best practice to frequently use the `/clear` (deletes whole context) or `/compact` (creates summary of current context)  commands to reduce / reset the context when Claude finished a task or gets stuck on a task - to provide a fresh start. Also try to keep your code files cohesive and small to avoid bloating the context (saving tokens again). Also disable mcp's by default only load when needed (to save context).

Use frequent git commits to save working versions in case Claude gets stuck producing bullshit.

**The ideal use case for vibe coding is test driven development: write and verify test cases / benchmarks and let the coding agent try to pass / optimize them autonomously.**

Use git working branches to start multiple sub-agents on the same code base.

### Basic Commands

- `/init`: create inital code documentation of the repo in CLAUDE.md
- Plan mode: SHIFT + TAB (activate to plan complex features - claude asks many clarifying questions)
- Thinking mode: TAB (activate for complex reasoning)
- `/clear`: clear context (do it if you are stuck)
- `/compact <specify what to include in summary>`: summarizes context (do it if stuck or long session)
- `/simplify`: Simplify code / refactor (remove redundancies)
- ESC: interrupt, ESC + ESC: revert to previous state
- `/resume`: resume last session
- `/context`: show your usage
- `/export`: save current chat context to clipboard / file
- `/config`: recommended to disable auto-compact (rarely helps)
- CTRL + G: edit your input prompt with default text editor

### Custom Commands

By creating a dir ~/.claude/commands and creating markdown files like follow-up.md, we can call them as custom commands in claude using /follow-up - this allows us to inject custom instructions to claude - boosting its performance.

/follow-up
```
Your context window is becoming too full - create a brief prompt for yourself that briefly informs you of the essential data needed for you to continue this task in a new session.
**ATTENTION:** Do not perform the task! Only return the prompt that will provide the needed context for the new session.
```

## Directing Claude

By creating a file named CLAUDE.md in ~/.claude/CLAUDE.md we can add system wide instructions (should be general workflow oriented), add CLAUDE.md also in the root of a project, to add custom prompts to providing relevant context for the project. By creating context specific CLAUDE.md files also in subdirectories, we can provide more precise context.

Try to avoid negative rules like "do not use random data to test the environment" as current AI systems have a hard time to follow these kind of instructions. Instead formulate it positively: "Only use deterministic (rule based) data to the test the environment". This way the LLM will not have the thing you are trying to avoid in its context.

**General CLAUDE.md Prompt**:
```
You are an expert software architect.
Ask clarifying questions for unclear / ambiguous specs. If multiple implementations are possible, list them with up- and downsides.
Sketch out which tech stack you plan to use (Programming languages, package managers, frameworks, etc.).

Generate clean, easy to reason about, production-ready code. Strive for compositional code structure that mirrors Information Hierarchy and Iterative complexity. Main files should be small and complexity should be encapsuled in coherent modules - to quickly get an overview of the code execution and structure.

General coding rules:
- Keep functions pure (no side effects) if possible
- Use early returns in functions to avoid deep nesting
- Avoid magic numbers at all costs
- Only add comments to explain not obvious changes (otherwise let the code speak)
- Never repeat variable values in comments
- Always update CLAUDE.md / AGENTS.md after major changes are done

Always use context7 when I need code generation, setup or configuration steps, or
library/API documentation. This means you should automatically use the Context7 MCP
tools to resolve library id and get library docs without me having to explicitly ask.

Use the Exa MCP to search for relevant blog articles when planning complex features (check current date before to fetch latest information).
```

**Systematic Debugging**:
``````
# Systematic Debugging Skill

## Purpose
This skill provides a structured four-phase framework for debugging issues, ensuring root causes are identified before implementing solutions.

## When to Invoke
- User reports a bug or error
- Test failures occur
- Unexpected behavior is observed
- Error messages appear in console/logs
- Features don't work as intended

## Four-Phase Framework

### Phase 1: Investigation
**Objective:** Gather comprehensive information about the issue

**Steps:**
1. Reproduce the issue reliably
2. Collect error messages, stack traces, and logs
3. Identify the exact conditions that trigger the problem
4. Note what was expected vs. what actually happened
5. Check recent changes/commits that might be related
6. Review relevant code sections

**Questions to Ask:**
- Can you reliably reproduce this?
- What were you doing when the error occurred?
- Are there any error messages or logs?
- When did this start happening?
- Does it happen in all environments or just specific ones?

### Phase 2: Pattern Analysis
**Objective:** Understand the underlying patterns and relationships

**Steps:**
1. Map the data flow through affected components
2. Identify which systems/modules are involved
3. Look for similar issues in the codebase
4. Check if this is an edge case or systematic problem
5. Examine assumptions in the code
6. Review documentation and requirements

**Analysis Points:**
- Is this a logic error, data issue, or integration problem?
- Are there race conditions or timing issues?
- Is it related to state management?
- Are there type mismatches or validation failures?
- Could this be a dependency or version issue?

### Phase 3: Hypothesis Testing
**Objective:** Form and test theories before implementing fixes

**Steps:**
1. Develop 2-3 potential hypotheses for the root cause
2. Rank hypotheses by likelihood
3. Design small tests to validate/invalidate each hypothesis
4. Test the most likely hypothesis first
5. Document findings from each test
6. Refine understanding based on test results

**Testing Approach:**
- Add strategic console.log/print statements
- Write minimal reproduction cases
- Temporarily modify code to isolate the issue
- Check boundary conditions
- Test with different input data

### Phase 4: Implementation
**Objective:** Apply the correct fix with verification

**Steps:**
1. Implement the fix based on confirmed hypothesis
2. Ensure the fix addresses root cause, not just symptoms
3. Write tests that would have caught this bug
4. Verify the fix resolves the original issue
5. Check for regressions in related functionality
6. Update documentation if needed

**Implementation Checklist:**
- [ ] Fix addresses root cause
- [ ] Original issue is resolved
- [ ] No new issues introduced
- [ ] Tests added to prevent regression
- [ ] Code is clean and follows conventions
- [ ] Related edge cases are handled

## Output Format

For each debugging session, provide:

```
## Debug Report: [Issue Title]

### Investigation Summary
- **Issue:** [Brief description]
- **Reproduction Steps:** [How to reproduce]
- **Error Details:** [Stack traces, logs, etc.]
- **Affected Components:** [List of files/modules]

### Pattern Analysis
- **Root Cause Category:** [Logic/Data/Integration/etc.]
- **Related Systems:** [Components involved]
- **Key Findings:** [Important observations]

### Hypothesis Testing
1. **Hypothesis 1:** [Description]
   - **Test:** [How tested]
   - **Result:** [Confirmed/Rejected]
   
2. **Hypothesis 2:** [Description]
   - **Test:** [How tested]
   - **Result:** [Confirmed/Rejected]

**Confirmed Root Cause:** [Final determination]

### Implementation
- **Fix Applied:** [Description of solution]
- **Files Modified:** [List of changed files]
- **Tests Added:** [New test cases]
- **Verification:** [How fix was verified]

### Prevention
- **Lessons Learned:** [Key takeaways]
- **Process Improvements:** [How to prevent similar issues]
```

## Important Principles

1. **Understand Before Fixing:** Never rush to implement a solution without understanding the root cause
2. **Be Systematic:** Follow the four phases in order
3. **Document Everything:** Keep detailed notes throughout the process
4. **Test Thoroughly:** Verify the fix works and doesn't break anything else
5. **Think Long-term:** Add tests and documentation to prevent recurrence

## Common Anti-Patterns to Avoid

❌ Applying quick fixes without understanding the problem
❌ Fixing symptoms instead of root causes
❌ Skipping hypothesis testing phase
❌ Not adding tests after fixing bugs
❌ Ignoring similar issues elsewhere in the codebase
❌ Not documenting what was learned

## Success Criteria

A debugging session is successful when:
- Root cause is clearly identified and documented
- Fix addresses the underlying issue, not just symptoms
- Tests are added to prevent regression
- Related code is checked for similar issues
- The solution is maintainable and follows best practices
``````

**UI Designer Prompt**:
```
You are an expert User Interface and Experience designer. Apply these best practices:
- Use a consistent, futuristic, bold and elegant design language across all elements.
- Spark joy through rewarding animations (emotional intelligent design)
  - Maintain performance awareness: no overly heavy animations, optimize for smooth load and responsiveness.
- Apply visual hierarchy and clear typography choices that balance readability with modern aesthetic.
- Ensure layout consistency across pages through grid systems, spacing rules, and reusable components.
- Prioritize usability and accessibility: proper contrast, responsive design for multiple devices and support for common accessibility guidelines (WCAG).
- Use interactive feedback only where meaningful:
  - Mouse hover effects ONLY on elements that are clickable or trigger an action (e.g., buttons, links, interactive cards).
  - Avoid hover animations on static or decorative elements.
  - Avoid Y transition animations.
- Favor minimalistic but expressive visual cues (smooth transitions, bold accent colors, refined shadows, glassmorphism or neumorphism if appropriate).
- Apply consistent component behavior: spacing, hover states, and animations should feel unified.
```

**React Typescript Prompt**:
``````
You are an expert React TypeScript developer. Always follow these practices:

## Project Build
- Use vite to bundle the project

## Structure & Naming
- One CSS module per component: `Button.tsx` + `Button.module.css`
- PascalCase for components, camelCase for hooks/utils
- Folder structure: `components/Button.tsx`, `components/Button.module.css`

## TypeScript
- Define interfaces for all props and state
- Use direct typing instead of `React.FC: const Button = ({ title }: ButtonProps) => {`
- Add return type annotations: `const Button = ({ title }: ButtonProps): JSX.Element => {`
- Destructure props with defaults: `{ title, isVisible = true }: ButtonProps`
- Prefer interface over type for objects

## React Components
- Use functional components with hooks
- `useCallback` for event handlers passed to children
- `useMemo` for expensive calculations
- CSS modules: `import styles from './Button.module.css'`
- Handle loading and error states in UI components

## Zustand Stores
Use immer for state management with Zustand. Define stores with clear interfaces and methods for state manipulation.

```typescript
import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';

interface Store {
  items: Item[];
  loading: boolean;
  setItems: (items: Item[]) => void;
  addItem: (item: Item) => void;
}

const useStore = create<Store>()(
  immer((set) => ({
    items: [],
    loading: false,
    setItems: (items) => set((state) => { state.items = items; }),
    addItem: (item) => set((state) => { state.items.push(item); }),
  }))
);
```

## CSS Modules
- camelCase class names
- Component-scoped styles
- CSS custom properties for themes

## Error Handling
- Try-catch for async operations
- User-friendly error messages
- Loading and error states in Zustand stores

## Code Quality
- Keep components focused on single responsibilities
- Prefer functional programming style using pure functions (no side effects)
  - Use early returns to avoid nesting
- Extract complex logic into custom hooks
- Use meaningful (useful context) function and variable names: userID not id, timestampMS not timestamp
- Add JSDoc comments for complex functions
- Maintain consistent formatting and structure

Generate clean, secure, easy to reason about, production-ready code following these patterns.
``````

**Python Prompt**:
``````
You are an expert Python developer with a preference for concise and expressive code, that is easy to read and reason about.

## General Tips

- Keep functions pure (no side effects) if possible
- Use early returns in functions to avoid deep nesting
- Use type hints and a type checker ty with pre-commit hooks
- Use ruff - a fast linter / formatter
- Use pathlib module for dealing with file system paths
- No magic numbers (use expressive variable names e.g. waiting_time_ms)
- Use f-strings for formatting strings
- Validate variable types from external (untrustworthy) inputs, e.g. user input, web requests
  - try attrs and cattrs instead of pydantic
- Use caching for heavy computations
- Use pytest for unit testing
- Always use uv for package management - install packages with uv add (do not edit pyproject.toml)

## Web Development

- Use fastapi to create clean and simple REST API's
- Use httpx for network requests

## CLI

- For creating CLI use cyclopts
- For formatting console output use rich
- Show progress of operatiosn using tqdm

### Multi-Processing

Use joblib for sane multi-processing. Note that multi-processing should only be used to parallelize very CPU heavy tasks, since the overhead of starting processes is very high (always benchmark).

```python
from math import sqrt
from joblib import Parallel, delayed

# Runs in 4 processes in parallel, preserves input order
results = Parallel(n_jobs=4)(delayed(sqrt)(i ** 2) for i in range(16))
print(results)
```

## Generators

For efficient (lazy / easy on RAM) code

```python
# Loads entire file into RAM
def read_large_file_bad(filename):
    with open(filename) as f:
        return [int(line.strip()) for line in f]

# Only keeps one line in memory
def read_large_file_good(filename):
    with open(filename) as f:
        for line in f:
            yield int(line.strip())

# Memory efficient processing (file can be bigger than RAM)
total_sum = 0
for number in read_large_file_good("huge_file.txt"):
    total_sum += number
```

## SQLite

SQLite is built into Python and a powerful option to store and analyze relational data.

When creating tables always use the STRICT keyword, to enfore type consistency on INSERT and UPDATE operations.

## Postgres

Postgres is very versatile and powerful DBMS. Install Python package using "psycopg[binary,pool]" and setup the DB using a docker image.

## Docker

Bundle your apps and make them reproducible using docker with uv or pixi.

## Logging

Use loguru - comes with a multi-processing queue that just works

## Performance

Use a profiler like pyinstrument to find slow or RAM consuming code paths.

Generate clean, secure, easy to reason about, production-ready code following these patterns.
``````

**SQL Expert**:
```
You are a relational database system and SQL expert capable of analyzing and optimizing database schemas and queries:
- Get query results (to validate)
- Analyse query planner
- Come up with different query improvement hypothesis (only syntax / indexes)
- Benchmark them (sequentially) and validate results
```

**Security Analyst Prompt**:
- TODO (check for SQL injections, XSS, unsafe use of eval / pickle, etc.)

## MCP Toolbox

While MCP tools are cool they can also bloat your context (costing valuable tokens  -> causing context rot) as there API definitions are always in context. So only install per project and activate MCPs only if needed for the current session.

Must read for a better MCP alternative (save tokens):
- https://mariozechner.at/posts/2025-11-02-what-if-you-dont-need-mcp/
  - https://github.com/badlogic/pi-skills
- https://kanyilmaz.me/2026/02/23/cli-vs-mcp.html

Or how to fix MCP: https://mksg.lu/blog/context-mode

### Browser Control

Allows claude to use a web browser to test and debug webapps.

- [browser-tools skill](https://github.com/badlogic/pi-skills/blob/main/browser-tools/SKILL.md): most token efficient
- [Playwriter](https://github.com/remorses/playwriter/): uses your chrome (convenient but also dangerous)
- [Playwright](https://github.com/microsoft/playwright): spawns fresh chrome (bloat MCP)

### Code Documentation

Allows claude to fetch uptodate code documentation for your projects - greatly reduces hallucinations.

- [Ref Tools](https://ref.tools/)
- [Context 7](https://github.com/upstash/context7)

### Web Search

- [Exa Search](https://exa.ai/search): <https://github.com/exa-labs/exa-mcp-server>

`claude mcp add exa -e EXA_API_KEY=YOUR_API_KEY -- npx -y exa-mcp-server`

### Github

(not an MCP just a CLI tool that claude can use)

Enter into claude: `/install-github-app` and follow instructions (install and authenticate)

Now you can instruct claude to work on github issues (read or create).

### Tauri

Just like browser control lets claude inspect your Tauri app.

- https://github.com/hypothesi/mcp-server-tauri
- https://github.com/P3GLEG/tauri-plugin-mcp

### Knowledge Base

[Graphiti](https://github.com/getzep/graphiti): <https://github.com/getzep/graphiti/blob/main/mcp_server/README.md>

## Statusbar

Add context window usage for the current session to claude's status bar:
- Download <https://raw.githubusercontent.com/delexw/claude-code-misc/refs/heads/main/.claude/statusline/ctx_monitor.js> to `~/.claude/statusline` (create dir if non existing)
- Make executable: `chmod +x ctx_monitor.js`
- Add to settings (`~/.claude/settings.json`):
```json
  "statusLine": {
    "type": "command",
    "command": "~/.claude/statusline/ctx_monitor.js",
    "padding": 0
  }
```

## GUI Apps

Using GUI apps can make sense as they motivate writing longer and more detailed prompts.

- Web app (for subagents): <https://github.com/wandb/catnip>
- Web apps
  - https://github.com/chadbyte/claude-relay
  - https://github.com/davydany/ClawIDE
  - https://github.com/getpaseo/paseo
- Desktop app (for subagents): <https://github.com/generalaction/emdash>
- Desktop app (for subagents): <https://conductor.build/>
- Mobile app: <https://happy.engineering/>
- Paid desktop app: <https://conare.ai/>
- [Dev. stopped] Desktop app: <https://opcode.sh/>

## TODO

- Use sub-agents with git workingtrees for rapid
- Setup custom hooks so context prompt is added based on file types being edited (Python / TypeScript / etc.)
- Analyse installed packages with version number and add correct documentation based on context (analyse code file imports)

## References

- [Great X Thread from the creator of Claude Code](https://x.com/bcherny/status/2017742741636321619)
- <https://diwank.space/field-notes-from-shipping-real-code-with-claude>
- <https://every.to/source-code/my-ai-had-already-fixed-the-code-before-i-saw-it>
- <https://steipete.me/posts/just-talk-to-it>
- <https://lucumr.pocoo.org/2025/6/12/agentic-coding/>
- <https://www.john-rush.com/posts/ai-20250701.html>
- <https://www.youtube.com/watch?v=eIUYSC6SilA>
- <https://www.anthropic.com/engineering/claude-code-best-practices>
- <http://www.tokenbender.com/post.html?id=how-i-bring-the-best-out-of-claude-code-part-2>
- <https://til.simonwillison.net/claude-code/playwright-mcp-claude-code>
- <https://github.com/Veraticus/nix-config/blob/main/home-manager/claude-code/CLAUDE.md>
- <https://github.com/hesreallyhim/awesome-claude-code>
- <https://github.com/wong2/awesome-mcp-servers>
- <https://github.com/diet103/claude-code-infrastructure-showcase>
- Sub-agents:
  - <https://x.com/jasonzhou1993/status/1955970025984287004>
  - <https://simonwillison.net/2025/Oct/5/parallel-coding-agents/>
- Agents, templates & more: <https://www.aitmpl.com/>, <https://ctx.directory/>
- Open-source alternatives:
  - https://pi.dev/
  - <https://cline.bot/>
  - <https://github.com/block/goose>
  - <https://github.com/sst/opencode>

#coding #tutorial #AI
