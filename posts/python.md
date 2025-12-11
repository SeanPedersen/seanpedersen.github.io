---
title: 'Python'
date: '2025-08-19'
---
Just some notes on how to use Python effectively. Python strives to be simple and concise and is a good choice for rapid software development. The average execution speed is rather slow, which is ok because it nicely interacts with C, C++ and Rust which are fast. Another major selling point is the rich ecosystem of useful packages, spanning web development, data science and much more.

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

- Use [FastAPI](https://fastapi.tiangolo.com/) to create clean and simple REST API's supporting both synchronous and [asynchronous](https://fastapi.tiangolo.com/async/) routes
- Use [httpx](https://github.com/encode/httpx/) for network requests (also supports sync and async)

## CLI

- For creating CLI: [cyclopts](https://github.com/BrianPugh/cyclopts)
- For formatting console output: [rich](https://github.com/Textualize/rich)
- Progress bar: [tqdm](https://github.com/tqdm/tqdm)

## Interactive Development

Great for prototyping, one-off analysis scripts and literate programming.

[Jupyter Notebook](https://jupyter.org/) and [Marimo](https://marimo.io/).

## Desktop Apps

[PyTauri](https://github.com/pytauri/pytauri/) / [PyLoid](https://github.com/pyloid/pyloid) / [PyWebView](https://github.com/r0x0r/pywebview)

## Concurrency

Python is a single threaded language with a Global Interpreter Lock (GIL). Meaning only multi-processing enables real parallel execution of non IO code. Multi-threading or async in Python thus only allows for concurrent IO operations (network / file system read and writes).

Python 3.13 has added experimental support for a no-GIL build flag, enabling true multi-threading support, which may become the default in the future.

- On multi-threading: <https://glyph.twistedmatrix.com/2014/02/unyielding.html>
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

### Async

I do not like async syntax wise (just personal preference) - I prefer multi-threading to speed up heavy IO tasks ([relevant HN thread](https://news.ycombinator.com/item?id=45106189)).

```python
import asyncio
import httpx

async def fetch_multiple_urls(urls):
    async with httpx.AsyncClient() as client:
        tasks = [client.get(url) for url in urls]
        responses = await asyncio.gather(*tasks)
        return responses
```

- <https://trio.readthedocs.io/en/stable/tutorial.html>
  - <https://vorpus.org/blog/notes-on-structured-concurrency-or-go-statement-considered-harmful/>
- <https://github.com/pomponchik/transfunctions>

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

SQLite3 support is built into the Python standard lib and a simple option to embed, store and analyze relational data.

When creating tables always use the STRICT keyword, to enfore type consistency on INSERT and UPDATE operations. This prevents ugly typing bugs that are possible - as Python does not guarantee type consistency at runtime.

- <https://bigcodenerd.org/blog/sqlite-type-checking-gochas/>

## Postgres

Postgres is very versatile and powerful DBMS. Use it with [psycopg](https://github.com/psycopg/psycopg) and a docker image.

```python
from contextlib import asynccontextmanager
from fastapi import Depends, FastAPI
import psycopg_pool
import psycopg

conn_string = "postgres://postgres@localhost"

pool = psycopg_pool.AsyncConnectionPool(conn_string, open=False)

@asynccontextmanager
async def lifespan(app: FastAPI):
    await pool.open()
    yield
    await pool.close()

app = FastAPI(lifespan=lifespan)

async def get_conn():
    async with pool.connection() as conn:
	    yield conn

@app.get("/visit/")
async def add_visit(conn = Depends(get_conn)):
    async with conn.cursor() as cursor:
        # Run our queries
        await cursor.execute("insert into visits(timestamp) values (now())")

    return {"message": "Visit logged"}
```

- <https://blog.danielclayton.co.uk/posts/database-connections-with-fastapi/>
- <https://spwoodcock.dev/blog/2024-10-fastapi-pydantic-psycopg/>

## Docker

Bundle your apps and make them reproducible using docker (with uv or pixi).

- Uv: <https://docs.astral.sh/uv/guides/integration/docker/#getting-started>
- Pixi: <https://github.com/prefix-dev/pixi-docker/blob/main/README.md>

## Logging

Use [loguru](https://github.com/Delgan/loguru) (comes with a multi-processing queue that just works)

## Performance

Use a profiler ([pyinstrument](https://github.com/joerick/pyinstrument)) to find slow or RAM consuming code paths.

Use C / C++ / Rust / Zig / Mojo etc. for performance critical code or try [PyPy](https://pypy.org/) and [Cython](https://cython.org/) first. Or check out a transpiler <https://github.com/py2many/py2many> / <https://github.com/paiml/depyler>.

#coding
