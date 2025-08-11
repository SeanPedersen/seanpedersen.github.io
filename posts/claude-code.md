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

## Starting Projects

It is recommended to use proven code base templates / example repositories that are correctly configured with all needed boilerplate for a project (based on a popular tech stack as mentioned before) - so that the code agents has a working starting point and can not fumble the project setup.

## General Advice

As every LLM Claude Code als suffers from context rot - meaning the longer the context (input context windows) becomes the worse the performance gets. Thus it is best practice to use the /compact (creates summary of current contetx) or /clear (deletes whole context) commands to reduce / reset the context when Claude gets stuck on a task - to provide a fresh start.

Use frequent git commits to save working versions in case Claude gets stuck producing bullshit.

Use git working branches to start multiple sub-agents on the same code base.

## Directing Claude

By creating a file named CLAUDE.md in the root of a project, we can add custom prompts to claude to improve its performance tailored to our project needs. By creating context specific CLAUDE.md files also in subdirectories, we can provide more precise context - improving the performance (instead of one big project wide CLAUDE.md file).

**React Typescript Prompt**:
``````
You are an expert React TypeScript developer. Always follow these practices:

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

## Components
- Use functional components with hooks
- `useCallback` for event handlers passed to children
- `useMemo` for expensive calculations
- CSS modules: `import styles from './Button.module.css'`
- Handle loading and error states in UI components

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

- Keep functions pure (no side effects) if possible
- Use early returns to avoid deep nesting
- Use type hints
- Use pathlib when dealing with file paths
- No magic numbers (use expressive variable names e.g. waiting_time_ms)
- Use f-strings for formatting
- Validate function input types

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

[**Context 7**](https://github.com/upstash/context7)

`mcp add context7 -- npx -y @upstash/context7-mcp`

### Web Search

[Exa Search](https://exa.ai/search): <https://github.com/exa-labs/exa-mcp-server>

### Knowledge Base

[Graphiti](https://github.com/getzep/graphiti): <https://github.com/getzep/graphiti/blob/main/mcp_server/README.md>

## References

- <https://www.john-rush.com/posts/ai-20250701.html>
- <https://www.youtube.com/watch?v=eIUYSC6SilA>
- <https://www.anthropic.com/engineering/claude-code-best-practices>
- <http://www.tokenbender.com/post.html?id=how-i-bring-the-best-out-of-claude-code-part-2>
- <https://til.simonwillison.net/claude-code/playwright-mcp-claude-code>
- <https://github.com/Veraticus/nix-config/blob/main/home-manager/claude-code/CLAUDE.md>
- <https://github.com/hesreallyhim/awesome-claude-code>
- <https://github.com/wong2/awesome-mcp-servers>
- Open-source alternatives:
  - <https://cline.bot/>
  - <https://github.com/block/goose>
  - <https://github.com/sst/opencode>

#programming #tutorial
