---
title: 'React'
date: '2025-09-24'
---
React is a very popular front-end framework allowing to modularize web UI elements in components (even allowing to create mobile apps using [React Native](https://reactnative.dev/) / [Expo](https://expo.dev/)). Components render UI elements conditioned on state variables. Components are made of other components which can be passed state using so called props.

Consider Svelte over React since it is a simpler and more performant competitor. React is more popular and thus has a bigger eco-system and more experienced developers.

Only use state variables (useState) for user facing state dependent UI components. Otherwise use global vars or it will get buggy.

useEffect hook:

`useEffect(() => {   // Runs after every render });`

`useEffect(() => {   // Runs only once after initial render }, []);`

`useEffect(() => {   // Runs when count changes }, [count]);`

Zustand is a global state manager for React which can reduce state complexity in your application by reducing property drilling (passing many state vars as props through nested components). Use it with immer for sane state management.

- <https://github.com/pmndrs/zustand>

Use tanstack query to fetch data from the backend (for proper error handling, caching etc.).

- <https://tanstack.com/query/latest/docs/framework/react/overview>
- <https://ui.dev/why-react-query>

## State Management Best Practices

UI State (local): Stick with useState for ephemeral UI details (open/closed toggles, input values).

Global State (app-wide): Use context sparingly or a dedicated library like Zustand for cross-cutting concerns (auth state, theme, feature flags).

Server State (remote data): Rely on TanStack Query for fetching, caching, invalidation, and syncing with backend data. Avoid duplicating server state in global or local state.

Derive state instead of duplicating: If a state can be derived from props or other state, don’t store it separately.

## References

- <https://github.com/instructa/constructa-starter/blob/main/docs/best-practices/tanstack-start/avoid-useEffect-summary.md>
- <https://github.com/jantimon/react-hydration-rules>
- <https://dev.to/paripsky/using-effects-effectively-in-react-stop-misusing-useeffect-once-and-for-all-5fpm>
- <https://x.com/_ryannystrom/status/1970292793877643556/photo/1>

#programming
