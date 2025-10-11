---
title: 'Getting started with Claude Code'  
date: '2025-07-03'
---
Claude Code is a powerful coding agent that you use from the command line. It comes with MCP support which enables it to use external tools like web browsers to automatically validate and test new features it added to web apps.

Simple and popular tech stacks that are fast to validate are the ideal choices for vibe coding (lots of training data available -> less hallucinations and fast / no  compilation -> allows for rapid iteration).

Recommendations:
- [JavaScript / TypeScript](https://seanpedersen.github.io/posts/javascript-typescript): React, Next
- [Python](https://seanpedersen.github.io/posts/python): Flask / FastAPI
- Go / Zig / Elixir: all come with fast compilation speeds

## Setting Things up

Install Claude Code: `curl -fsSL https://claude.ai/install.sh | bash`

Change dir to a code project and start Claude Code using: `claude` and authenticate.

Now you can ask claude about the project or instruct it to implement a new feature. Generally it is advised to plan out the architecture (design) yourself and then use Claude to implement small features which are easy to validate.

### GUI

Using GUI apps can make sense as they motivate writing longer and more detailed prompts.

- Desktop app: <https://opcode.sh/>
- Mobile app: <https://happy.engineering/>

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

As with every LLM, Claude Code suffers from context rot - meaning the longer and noisier the context (input context windows) becomes, the worse the performance gets. Thus it is best practice to use the `/compact` (creates summary of current context) or `/clear` (deletes whole context) commands to reduce / reset the context when Claude gets stuck on a task - to provide a fresh start.

Use frequent git commits to save working versions in case Claude gets stuck producing bullshit.

The ideal use case for vibe coding is test driven development: write and verify tight test cases and let the coding agent try to pass them.

Use git working branches to start multiple sub-agents on the same code base.

### Basic Commands

- /clear: clear context (do it if you are stuck)
- /compact: summarizes context (do it if stuck or long session)
- /resume: resume last session
- /context: show your usage

## Directing Claude

By creating a file named CLAUDE.md in the root of a project, we can add custom prompts to claude to improve its performance tailored to our project and workflow needs. By creating context specific CLAUDE.md files also in subdirectories, we can provide more precise context. By creating a dir ~/.claude/commands and creating markdown files like python.md, react.md or design.md, we can call them as custom commands in claude using /python, /react or /design - this allows us to add context specific instructions to claude boosting its performance (ideally claude would automatically inject prompts based on file types it is working with but I did not get this working yet...).

**General CLAUDE.MD Prompt**:
```
You are an expert software architect.
Ask clarifying questions for unclear / ambiguous specs. If multiple implementations are possible, list them with up- and downsides.
Sketch out which tech stack you plan to use (Programming languages, package managers, frameworks, etc.).

Generate clean, easy to reason about, production-ready code following these patterns.

Always use context7 when I need code generation, setup or configuration steps, or
library/API documentation. This means you should automatically use the Context7 MCP
tools to resolve library id and get library docs without me having to explicitly ask.

Use the playwright MCP tools to debug frontend UI issues.
```

**Designer Prompt**:
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

Generate clean, easy to reason about, production-ready code following these patterns.
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

Use [joblib](https://joblib.readthedocs.io/en/stable/index.html) for sane multi-processing. Note that multi-processing should only be used to parallelize very CPU heavy tasks, since the overhead of starting processes is very high (always benchmark).

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

Postgres is very versatile and powerful DBMS. Use it with [psycopg](https://github.com/psycopg/psycopg) and a docker image.

## Docker

Bundle your apps and make them reproducible using docker with uv.

## Logging

Use loguru - comes with a multi-processing queue that just works

## Performance

Use a profiler like pyinstrument to find slow or RAM consuming code paths.

Generate clean, easy to reason about, production-ready code following these patterns.
``````

**Security Analyst Prompt**:
- TODO (check for SQL injections, XSS, unsafe use of eval / pickle, etc.)

## MCP Toolbox

### Browser support

Allows claude to use a web browser to test and debug webapps.

[**Playwright**](https://github.com/microsoft/playwright)

`claude mcp add playwright npx '@playwright/mcp@latest'`

**Puppeteer**

### Code Documentation

Allows claude to fetch uptodate code documentation for your projects - greatly reduces hallucinations.

- [**Ref Tools**](https://ref.tools/)

- [**Context 7**](https://github.com/upstash/context7)

`claude mcp add context7 -- npx -y @upstash/context7-mcp`

### Web Search

[Exa Search](https://exa.ai/search): <https://github.com/exa-labs/exa-mcp-server>

### Knowledge Base

[Graphiti](https://github.com/getzep/graphiti): <https://github.com/getzep/graphiti/blob/main/mcp_server/README.md>

## TODO Improvements

- Setup custom hooks so context prompt is added based on file types being edited (Python / TypeScript / etc.)
- Analyse installed packages with version number and add correct documentation based on context (analyse code file imports)

## References

- <https://every.to/source-code/my-ai-had-already-fixed-the-code-before-i-saw-it>
- <https://www.john-rush.com/posts/ai-20250701.html>
- <https://www.youtube.com/watch?v=eIUYSC6SilA>
- <https://www.anthropic.com/engineering/claude-code-best-practices>
- <http://www.tokenbender.com/post.html?id=how-i-bring-the-best-out-of-claude-code-part-2>
- <https://til.simonwillison.net/claude-code/playwright-mcp-claude-code>
- <https://github.com/Veraticus/nix-config/blob/main/home-manager/claude-code/CLAUDE.md>
- <https://github.com/hesreallyhim/awesome-claude-code>
- <https://github.com/wong2/awesome-mcp-servers>
- Sub-agents:
  - <https://x.com/jasonzhou1993/status/1955970025984287004>
  - <https://simonwillison.net/2025/Oct/5/parallel-coding-agents/>
- Agents, templates & more: <https://www.aitmpl.com/>, <https://ctx.directory/>
- Open-source alternatives:
  - <https://cline.bot/>
  - <https://github.com/block/goose>
  - <https://github.com/sst/opencode>

#programming #tutorial #AI
