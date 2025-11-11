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
- Desktop app (for subagents): <https://github.com/generalaction/emdash>
- Desktop app (for subagents): <https://conductor.build/>
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

By creating a file named CLAUDE.md in ~/.claude/CLAUDE.md we can add system wide instructions (should be general workflow oriented), add CLAUDE.md also in the root of a project, to add custom prompts to providing relevant context for the project. By creating context specific CLAUDE.md files also in subdirectories, we can provide more precise context.

By creating a dir ~/.claude/commands and creating markdown files like python.md, react.md or design.md, we can call them as custom commands in claude using /python, /react or /design - this allows us to add context specific instructions to claude boosting its performance (ideally claude would automatically inject prompts based on file types it is working with but I did not get this working yet...).

Try to avoid negative rules like "do not use random data to test the environment" as current AI systems have a hard time to follow these kind of instructions. Instead formulate it positively: "use deterministic (rule based) data to the test the environment". This way the LLM will not have the thing you are trying to avoid in its context.

**General CLAUDE.md Prompt**:
```
You are an expert software architect.
Ask clarifying questions for unclear / ambiguous specs. If multiple implementations are possible, list them with up- and downsides.
Sketch out which tech stack you plan to use (Programming languages, package managers, frameworks, etc.).

Generate clean, easy to reason about, production-ready code following these patterns. Always strive to create closed loop jobs that can be verified using a test case or benchmark. Before starting to implement big features, make sure the core functionality they rely on works (verified by a real test case / benchmark).

Always use context7 when I need code generation, setup or configuration steps, or
library/API documentation. This means you should automatically use the Context7 MCP
tools to resolve library id and get library docs without me having to explicitly ask.

Use the playwright MCP tools to debug UI issues in web apps.

Use the Exa MCP to search for relevant blog articles when planning complex features.
```

**Simplification Cascade**:
``````
# Simplification Cascade Skill

## Purpose
This skill helps identify when code is unnecessarily complex and guides the refactoring process to find elegant abstractions that eliminate duplication and reduce cognitive load.

## When to Invoke
- Multiple similar implementations of the same concept exist
- Code has too many special cases and exceptions
- Complicated conditional logic with many branches
- Repetitive patterns across different parts of the codebase
- User asks to simplify, refactor, or optimize code
- Performance issues due to redundant logic
- Difficulty adding new features due to code complexity

## Core Philosophy

**The Central Question:** "What if all these seemingly different things are actually the same thing, viewed from a different perspective?"

Instead of managing multiple separate implementations, find the unified concept that encompasses them all.

## Pattern Recognition

### Red Flags Indicating Need for Simplification

1. **Multiple Implementations of Similar Concepts**
   - Three different validation functions doing similar checks
   - Separate managers for similar data types (UserManager, ProductManager, OrderManager)
   - Duplicate UI components with slight variations

2. **Special Case Proliferation**
   - Lots of `if (specialCase)` branches
   - Multiple exception handlers for similar scenarios
   - Hardcoded values for different contexts

3. **Complex Conditional Logic**
   - Deeply nested if/else statements
   - Long switch statements with similar cases
   - Boolean flags controlling behavior

4. **Violation of DRY (Don't Repeat Yourself)**
   - Copy-pasted code with minor modifications
   - Similar functions with different names
   - Redundant data transformations

## The Simplification Cascade Process

### Phase 1: Discovery
**Objective:** Find all instances of the complex pattern

**Steps:**
1. Scan codebase for similar implementations
2. List all variations of the pattern
3. Note the differences between each implementation
4. Identify the context where each is used
5. Map dependencies and relationships

**Output:**
```
## Pattern Instances Found

1. **[Pattern Name 1]**
   - Location: [file:line]
   - Purpose: [what it does]
   - Complexity: [lines of code, cyclomatic complexity]
   
2. **[Pattern Name 2]**
   - Location: [file:line]
   - Purpose: [what it does]
   - Complexity: [lines of code, cyclomatic complexity]

[Continue for all instances...]
```

### Phase 2: Essence Extraction
**Objective:** Find the common core concept

**Steps:**
1. Strip away all implementation details
2. Identify the essential purpose (the "what" not the "how")
3. Find the invariants (what never changes)
4. Note the variants (what does change)
5. Look for the unifying abstraction

**Guiding Questions:**
- What is the fundamental problem being solved?
- What stays the same across all implementations?
- What varies between implementations?
- Is the variation in data, behavior, or both?
- Can the variation be represented as data/configuration?

**Output:**
```
## Essence Analysis

**Core Concept:** [The unified idea]

**Invariants (Never Changes):**
- [Thing 1]
- [Thing 2]

**Variants (Changes Between Implementations):**
- [Difference 1]: [How it varies]
- [Difference 2]: [How it varies]

**Unifying Abstraction:** [The single concept that encompasses all cases]
```

### Phase 3: Pattern Design
**Objective:** Design the simplified solution

**Steps:**
1. Create a single abstraction that handles all cases
2. Use composition over inheritance where possible
3. Make variants configurable through data/parameters
4. Design clear, minimal interfaces
5. Consider extensibility for future cases
6. Plan for backward compatibility if needed

**Design Principles:**
- **Single Responsibility:** Each component does one thing well
- **Open/Closed:** Open for extension, closed for modification
- **Dependency Inversion:** Depend on abstractions, not concretions
- **Configuration Over Code:** Move variation to data when possible

**Output:**
```
## Simplified Design

**New Abstraction:** [Name and purpose]

**Interface:**
```[language]
[Proposed interface/API]
```

**Configuration Approach:**
[How variants are handled through data/params]

**Example Usage:**
```[language]
[Code example showing simplified usage]
```

**Benefits:**
- [Benefit 1]
- [Benefit 2]
- [Lines of code reduced: X → Y]
- [Complexity reduced: X → Y]
```

### Phase 4: Transformation Plan
**Objective:** Create a safe migration path

**Steps:**
1. List all files that need changes
2. Order changes to minimize breakage
3. Identify tests that need updating
4. Plan for incremental migration
5. Note any potential risks
6. Define rollback strategy

**Migration Strategy:**
- Start with least critical usage
- Maintain backward compatibility initially
- Migrate one usage at a time
- Test thoroughly at each step
- Remove old code only after full migration

**Output:**
```
## Transformation Plan

### Phase 1: Foundation
- [ ] Create new abstraction
- [ ] Write comprehensive tests
- [ ] Verify backward compatibility

### Phase 2: Migration (Incremental)
- [ ] Migrate [Component 1] to new abstraction
- [ ] Migrate [Component 2] to new abstraction
- [ ] [Continue for all components...]

### Phase 3: Cleanup
- [ ] Remove deprecated code
- [ ] Update documentation
- [ ] Optimize new abstraction

### Risk Assessment
- **Risk 1:** [Description] → Mitigation: [Strategy]
- **Risk 2:** [Description] → Mitigation: [Strategy]

### Rollback Plan
[How to revert if issues arise]
```

## Simplification Patterns

### Common Simplification Strategies

1. **Strategy Pattern**
   - Replace conditional logic with pluggable strategies
   - Example: Multiple payment processors → Single processor with strategy

2. **Template Method**
   - Extract common algorithm, vary specific steps
   - Example: Similar workflows → Template with hooks

3. **Data-Driven Configuration**
   - Replace code with configuration
   - Example: Hardcoded rules → Rule engine with data

4. **Composition**
   - Combine small, focused pieces instead of inheritance
   - Example: Multiple managers → Composable behaviors

5. **Generalization**
   - Create generic version that handles all cases
   - Example: TypeManager → EntityManager<T>

## Metrics for Success

### Before vs. After Comparison
```
| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Lines of Code | X | Y | -Z% |
| Cyclomatic Complexity | X | Y | -Z% |
| Number of Implementations | X | 1 | -Z implementations |
| Test Coverage | X% | Y% | +Z% |
| Time to Add New Case | X hrs | Y hrs | -Z% |
```

## Output Format

For each simplification session:

```
# Simplification Cascade Report: [Area]

## Executive Summary
- **Problem:** [Brief description of complexity]
- **Impact:** [Lines of code, files affected, etc.]
- **Solution:** [One-line description of abstraction]
- **Benefit:** [Key improvement metric]

## Discovery Phase
[All pattern instances found]

## Essence Extraction
[Core concept and unified abstraction]

## Pattern Design
[New simplified design]

## Transformation Plan
[Step-by-step migration strategy]

## Next Steps
[Immediate actions to take]
```

## Important Principles

1. **Seek the Deeper Pattern:** Look beyond surface similarities to find the unifying concept
2. **Simplify, Don't Just Refactor:** Aim for conceptual clarity, not just cleaner code
3. **Measure Impact:** Quantify improvements in complexity and maintainability
4. **Migrate Safely:** Use incremental changes with rollback plans
5. **Document the Insight:** Capture the "aha moment" that led to simplification

## Questions to Guide Simplification

- If I squint, do these look the same?
- What's the one insight that would eliminate most of this complexity?
- Am I solving the right problem, or treating symptoms?
- Would a complete beginner find this obvious or confusing?
- Could I explain this to someone in one sentence?
- What's the simplest thing that could possibly work?

## Anti-Patterns to Avoid

❌ Over-abstraction: Creating abstractions too early or too complex
❌ Premature optimization: Simplifying before understanding the problem
❌ Ignoring context: Forcing unification of truly different concepts
❌ Breaking working code: Refactoring without adequate tests
❌ Analysis paralysis: Spending too long designing the perfect abstraction

## Success Criteria

A simplification is successful when:
- Code is easier to understand and explain
- New variations can be added without code changes
- Tests are simpler and more focused
- Bugs are easier to locate and fix
- Onboarding new developers is faster
- The abstraction feels "obvious" in retrospect
``````

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

**Security Analyst Prompt**:
- TODO (check for SQL injections, XSS, unsafe use of eval / pickle, etc.)

## MCP Toolbox

While MCP tools are cool they can also bloat your context (costing valuable tokens and increase context rot) as there API definitions are always in context - the github MCP for example is really bloated but the github CLI is working as well and LLM's already know how to use it properly. So only add really useful MCP servers.

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

`claude mcp add exa -e EXA_API_KEY=YOUR_API_KEY -- npx -y exa-mcp-server`

### Github

(not an MCP just a CLI tool that claude can use)

Enter into claude: `/install-github-app` and follow instructions (install and authenticate)

Now you can instruct claude to work on github issues (read or create).

### Knowledge Base

[Graphiti](https://github.com/getzep/graphiti): <https://github.com/getzep/graphiti/blob/main/mcp_server/README.md>

## TODO

- Use sub-agents with git workingtrees for rapid
- Setup custom hooks so context prompt is added based on file types being edited (Python / TypeScript / etc.)
- Analyse installed packages with version number and add correct documentation based on context (analyse code file imports)

## References

- <https://diwank.space/field-notes-from-shipping-real-code-with-claude>
- <https://every.to/source-code/my-ai-had-already-fixed-the-code-before-i-saw-it>
- <https://steipete.me/posts/just-talk-to-it>
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
  - <https://cline.bot/>
  - <https://github.com/block/goose>
  - <https://github.com/sst/opencode>

#programming #tutorial #AI
