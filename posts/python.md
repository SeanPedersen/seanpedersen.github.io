---
title: 'Python'
date: '2025-08-19'
---
Just some notes on how to use Python effectively. Python is simple and concise, ideal for rapid software development. The average execution speed is rather slow, which is ok because it nicely interacts with C, C++ and Rust which are fast. Another major selling point is the rich ecosystem of useful packages, spanning web development, scientific computing, machine learning, data science and much more.

## Package Mangement

Use uv or pixi for fast and sane package management.

## General Tips

- Keep functions pure (no side effects) if possible
- Use early returns in functions to avoid deep nesting
- Use type hints
- Use pathlib module for dealing with file paths
- No magic numbers (use expressive variable names e.g. waiting_time_ms)
- Use f-strings for formatting
- Validate variable input types
  - try attrs and cattrs instead of pydantic
- Use caching for heavy computations
- Use pytest for unit testing

## Concurrency

Use [joblib](https://joblib.readthedocs.io/en/stable/index.html) for multi-processing.

### Global Interpreter Lock (GIL)

Python is a single threaded language. Meaning only multi-processing enables true concurrent execution. Multi-threading in Python only allows for concurrent IO operations (network / file system read and writes).

Python 3.13 has added experimental support for a no-GIL build flag, enabling true multi-threading support, which may become the default in the future.

## Async

I do not like async - I prefer multi-threading for heavy IO tasks.

## Generators

For efficient (easy on RAM) code

## SQLite

Validate / force input types before inserting or your data can get broken in strange ways since SQLite does not force consistent types on columns by default...

- <https://bigcodenerd.org/blog/sqlite-type-checking-gochas/>

## Logging

Use loguru (comes with a multi-processing queue that just works)

## CLI

- For creating CLI: https://github.com/BrianPugh/cyclopts
- For formatting console output: https://github.com/Textualize/rich
- Progress bar: tqdm

## Web Development

Use FastAPI - clean and simple way to create REST API's

#programming
