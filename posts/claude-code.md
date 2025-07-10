---

title: 'Getting started with Claude Code'  
date: '2025-07-03'

---
(work-in-progress)
Claude Code is a powerful coding agent that you use from the command line. It comes with MCP support which enables it to use external tools like web browsers to automatically validate and test new features it added to web apps.

## Setting Things up

Install Claude Code: `npm install -g claude`

Change dir to a code project and start Claude Code using: `claude`

Now you can ask claude about the project or instruct it to implement a new feature. Generally it is advised to plan out the architecture (design) yourself and then use Claude to implement small features which are easy to validate.

## Directing Claude

By creating a file named CLAUDE.md in the root of a project, we can add custom prompts to claude to improve its performance tailored to our project needs.

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
