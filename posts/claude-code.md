---
title: 'Getting started with Claude Code'  
date: '2025-07-03'
---
Claude Code is a powerful coding agent that you use from the command line. It comes with MCP support which enables it to use external tools like web browsers to automatically validate and test new features it added to web apps.

Simple and popular tech stacks that are fast to validate are the ideal choices for vibe coding (lots of training data available -> less hallucinations and fast / no  compilation -> allows for rapid iteration).

Recommendations:
- JavaScript / TypeScript: React, Next
- Python: Flask / FastAPI
- Go / Zig / Elixir: all come with fast compilation speeds

## Setting Things up

Install Claude Code: `npm install -g claude`

Update: `npm update -g claude`

Change dir to a code project and start Claude Code using: `claude`

Now you can ask claude about the project or instruct it to implement a new feature. Generally it is advised to plan out the architecture (design) yourself and then use Claude to implement small features which are easy to validate.

### GUI

Using GUI apps can make sense as they motivate writing longer and more detailed prompts.

<https://opcode.sh/>

## Starting Projects

It is recommended to use proven code base templates / example repositories that are correctly configured with all needed boilerplate for a project (based on a popular tech stack as mentioned before) - so that the code agents has a working starting point and can not fumble the project setup.

## General Advice

As with every LLM, Claude Code suffers from context rot - meaning the longer the context (input context windows) becomes, the worse the performance gets. Thus it is best practice to use the `/compact` (creates summary of current context) or `/clear` (deletes whole context) commands to reduce / reset the context when Claude gets stuck on a task - to provide a fresh start.

Use frequent git commits to save working versions in case Claude gets stuck producing bullshit.

The ideal use case for vibe coding is test driven development: write and verify tight test cases and let the coding agent try to pass them.

Use git working branches to start multiple sub-agents on the same code base.

## Directing Claude

By creating a file named CLAUDE.md in the root of a project, we can add custom prompts to claude to improve its performance tailored to our project needs. By creating context specific CLAUDE.md files also in subdirectories, we can provide more precise context - improving the performance (instead of one big project wide CLAUDE.md file).

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
```
You are an expert Python developer with a preference for concise and expressive code, that is easy to read and reason about.

## General Tips

- Keep functions pure (no side effects) if possible
- Use early returns in functions to avoid deep nesting
- Use type hints and a type checker ([ty](https://github.com/astral-sh/ty)) with pre-commit hooks
- Use [ruff](https://github.com/astral-sh/ruff) - a fast linter / formatter
- Use [pathlib](https://docs.python.org/3/library/pathlib.html#basic-use) module for dealing with file system paths
- No magic numbers (use expressive variable names e.g. waiting_time_ms)
- Use [f-strings](https://docs.python.org/3/tutorial/inputoutput.html#formatted-string-literals) for formatting strings
- Validate variable types from external (untrustworthy) inputs, e.g. user input, web requests
  - try attrs and cattrs instead of pydantic
- Use caching for heavy computations
- Use [pytest](https://docs.pytest.org/en/stable/) for unit testing

## Package Management

Use [uv](https://github.com/astral-sh/uv) (preferred and popular) or [pixi](https://pixi.sh/latest/python/tutorial/) (can install conda packages - useful for GPU/CUDA stuff) for fast and sane package management.

## Web Development

- Use [FastAPI](https://fastapi.tiangolo.com/) to create clean and simple REST API's
- Use [httpx](https://github.com/encode/httpx/) for network requests

## CLI

- For creating CLI: [cyclopts](https://github.com/BrianPugh/cyclopts)
- For formatting console output: [rich](https://github.com/Textualize/rich)
- Progress bar: [tqdm](https://github.com/tqdm/tqdm)

## Desktop Apps

[PyTauri](https://github.com/pytauri/pytauri/)

## Concurrency

Python is a single threaded language with a Global Interpreter Lock (GIL). Meaning only multi-processing enables true concurrent execution. Multi-threading or async in Python only allows for concurrent IO operations (network / file system read and writes).

Python 3.13 has added experimental support for a no-GIL build flag, enabling true multi-threading support, which may become the default in the future.

- Good ref on Python multiprocessing: <https://pythonspeed.com/articles/python-multiprocessing/>

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

When creating tables always use the STRICT keyword, to enfore type consistency on INSERT and UPDATE operations. This prevents ugly typing bugs that are possible - as Python does not guarantee type consistency at runtime.

- <https://bigcodenerd.org/blog/sqlite-type-checking-gochas/>

## Postgres

Postgres is very versatile and powerful DBMS. Use it with [psycopg](https://github.com/psycopg/psycopg) and a docker image.

## Docker

Bundle your apps and make them reproducible using docker (for uv or pixi).

## Logging

Use [loguru](https://github.com/Delgan/loguru) (comes with a multi-processing queue that just works)

## Performance

Use a profiler ([pyinstrument](https://github.com/joerick/pyinstrument)) to find slow or RAM consuming code paths.

Use C / C++ / Rust / Zig etc. for performance critical code or try [PyPy](https://pypy.org/) and [Cython](https://cython.org/) first.

Generate clean, easy to reason about, production-ready code following these patterns.
```

## MCP Toolbox

### Browser support

Allows claude to use a web browser to test and debug webapps.

[**Playwright**](https://github.com/microsoft/playwright)

`mcp add playwright npx '@playwright/mcp@latest'`

**Puppeteer**

### Code Documentation

Allows claude to fetch uptodate code documentation for your projects.

- [**Ref Tools**](https://ref.tools/)

- [**Context 7**](https://github.com/upstash/context7)

`mcp add context7 -- npx -y @upstash/context7-mcp`

### Web Search

[Exa Search](https://exa.ai/search): <https://github.com/exa-labs/exa-mcp-server>

### Knowledge Base

[Graphiti](https://github.com/getzep/graphiti): <https://github.com/getzep/graphiti/blob/main/mcp_server/README.md>

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
- Thread on sub-agents: <https://x.com/jasonzhou1993/status/1955970025984287004>
- Open-source alternatives:
  - <https://cline.bot/>
  - <https://github.com/block/goose>
  - <https://github.com/sst/opencode>

#programming #tutorial
