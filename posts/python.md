---
title: 'Python'
date: '2025-08-19'
---
Just some notes on how to use Python effectively. Python is simple and concise, ideal for rapid software development. The average execution speed is rather slow, which is ok because it nicely interacts with C, C++ and Rust which are fast. Another major selling point is the rich ecosystem of useful packages, spanning web development, scientific computing, machine learning, data science and much more.

## Package Mangement

Use [uv](https://github.com/astral-sh/uv) or [pixi](https://pixi.sh/latest/python/tutorial/) for fast and sane package management.

## General Tips

- Keep functions pure (no side effects) if possible
- Use early returns in functions to avoid deep nesting
- Use type hints and a type checker ([ty](https://github.com/astral-sh/ty)) with pre-commit hooks
- Use [ruff](https://github.com/astral-sh/ruff) - a fast linter / formatter
- Use [pathlib](https://docs.python.org/3/library/pathlib.html) module for dealing with file system paths
- No magic numbers (use expressive variable names e.g. waiting_time_ms)
- Use f-strings for formatting
- Validate variable input types
  - try attrs and cattrs instead of pydantic
- Use caching for heavy computations
- Use [pytest](https://docs.pytest.org/en/stable/) for unit testing

## Web Development

- Use [FastAPI](https://fastapi.tiangolo.com/) to create clean and simple REST API's
- Use [httpx](https://github.com/encode/httpx/) for network requests

## CLI

- For creating CLI: [cyclopts](https://github.com/BrianPugh/cyclopts)
- For formatting console output: [rich](https://github.com/Textualize/rich)
- Progress bar: [tqdm](https://github.com/tqdm/tqdm)

## Concurrency

Python is a single threaded language with a Global Interpreter Lock (GIL). Meaning only multi-processing enables true concurrent execution. Multi-threading or async in Python only allows for concurrent IO operations (network / file system read and writes).

Python 3.13 has added experimental support for a no-GIL build flag, enabling true multi-threading support, which may become the default in the future.

### Multi-Processing

Use [joblib](https://joblib.readthedocs.io/en/stable/index.html) for multi-processing.

### Async

I do not like async syntax wise (just personal preference) - I prefer multi-threading to speed up heavy IO tasks.

```python
import asyncio
import httpx

async def fetch_multiple_urls(urls):
    async with httpx.AsyncClient() as client:
        tasks = [client.get(url) for url in urls]
        responses = await asyncio.gather(*tasks)
        return responses
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

When creating tables always use the STRICT keyword, to enfore type consistency on INSERT and UPDATE operations. This prevents ugly bugs that are possible - as Python does not guarantee type consistency at runtime.

- <https://bigcodenerd.org/blog/sqlite-type-checking-gochas/>

## Logging

Use loguru (comes with a multi-processing queue that just works)

## Performance

Use a profiler to find slow or RAM consuming code paths.

Use C / C++ / Rust / Zig etc. for performance critical code or try PyPy and Cython first.

#programming
