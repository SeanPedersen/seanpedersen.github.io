---
title: 'Meta Design: Design of a Design'
date: '2019-04-20'
---
**Why even write a design?** The design process animates thinking through ideas deeply before getting lost in building them.

**What makes a successful design?**
It alone must simply suffice to realize the idea it contains.

**Some things to try**:

- Write concisely but not too short
- Use clear & vivid explanations (use pictures, graphs when appropriate)
- Avoid too long continuous text passages

## Top Down Approach

Identify distinct, preferably large, independent modules which ideally abstract away highly moving parts. Next outline interactions of the modules to map out the interaction dynamics of the system.

“We propose instead that one begins with a list of difficult design decisions or design decisions which are likely to change. Each module is then designed to hide such a decision from the others.” - <http://sunnyday.mit.edu/16.355/parnas-criteria.html>

Finally describe each module as deeply as necessary but not deeper, so that interfaces (behavior towards others) and their functionalities (inner behavior: e.g. algorithms and data structures) are clearly presented. Finally draw a clear line between what is solved, what remains unsolved and what is unknowable.

References:

- <https://learnhowtolearn.org/how-to-build-extremely-quickly/>
