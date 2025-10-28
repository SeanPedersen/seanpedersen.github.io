---
title: 'Javascript (TypeScript)'
date: '2025-09-18'
---
[TypeScript](https://www.typescriptlang.org/) is a superset of JavaScript that brings static typing, preventing many bugs found in vanilla JS code bases. TypeScript can both be used in the frontend as well in the backend (server) using NodeJS. JavaScript is a neccessary evil (that has many strange quirks), which TypeScript alleviates some of at least.

## Basics

- Use [pnpm](https://github.com/pnpm/pnpm) instead of npm for a faster and securer package manager.
- Use [vite](https://vite.dev/guide) for sane project bundling.

## FrontEnd Frameworks

Use [React](https://react.dev/learn) for a popular framework, with many resources (both knowledge and developers) available. Use [Svelte](https://svelte.dev/) for a smaller but better alternative. Read [this article](https://www.lorenstew.art/blog/10-kanban-boards/) for an excellent comparison of frontend state management frameworks.

## Promises (async)

JavaScript uses promises to handle asynchronous operations like API calls, file operations, or timers. A promise represents a value that may be available now, in the future or never and is returned by async functions (return values of async functions are always wrapped in a promise). The await keyword unwraps the value of the promise.

**Promise States**

A promise can be in one of three states:
- Pending: Initial state, neither fulfilled nor rejected
- Fulfilled: Operation completed successfully
- Rejected: Operation failed

**Async/Await Syntax**

Modern JavaScript provides async/await syntax for cleaner asynchronous code:
```javascript
async function fetchData() {
  try {
    const response = await fetch('/api/data');
    const data = await response.json();
    return data;
  } catch (error) {
    console.error('Error:', error);
  }
}
```

**Promise Combinators**

JavaScript provides several utility methods for working with multiple promises:
- Promise.all(): Waits for all promises to resolve
- Promise.allSettled(): Waits for all promises to settle (resolve or reject)
- Promise.race(): Returns the first promise to settle
- Promise.any(): Returns the first promise to resolve

```javascript
// Wait for all promises
const results = await Promise.all([
  fetch('/api/users'),
  fetch('/api/posts'),
  fetch('/api/comments')
]);
```

## References
- <https://www.typescriptlang.org/docs/handbook/basic-types.html>
- TypeScript performance tips: <https://github.com/microsoft/TypeScript/wiki/Performance>

#programming
