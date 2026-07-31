---
date: '2025-11-14'
icon: "/images/icons/rust.svg"
---
# Rust

Rust is a statically and strongly typed systems programming language that enforces memory safety (without a garbage collector) through its ownership and borrowing model, while still being efficient in the ballpark of C or C++.

In safe Rust, the compiler prevents entire classes of bugs common in C or C++, such as use-after-free and most data races. It also forces explicit handling of optional values via Option instead of allowing null references. While this shifts many potential failures from runtime to compile time (in contrast with dynamic languages like Python or unsafe manual memory management in C) - Rust does not eliminate logic errors, panics or higher-level concurrency issues like deadlocks. These properties are the reason Rust has made its way into the Linux kernel.

Rust also shines through its ergnomic package manager cargo that provides a smooth developer UX. The only real downside of Rust is its inherent complexity (slow compile speeds and huge compilation artifacts disk size) and at times verbose / ugly syntax.

Great introduction: <https://fasterthanli.me/articles/a-half-hour-to-learn-rust>

## Borrow-Checker
A useful way to visualize Rust’s borrow checker is as a graph of ownership and temporary access. A value has one owner, while references create temporary edges pointing to that value. Ownership can move from one variable to another, but it is not duplicated unless you explicitly copy or clone the value. Borrowing means “I want access to this value without taking ownership of it,” and Rust checks that every reference remains valid for as long as it is used.

```
Allowed:                    Allowed:
   value                       value
  ▲  ▲  ▲                        ▲
  │  │  │                        │
 &a &b &c                     &mut x
 N readers                    1 writer
```

The key rule is many readers or one writer. Rust permits any number of immutable references (&T) at the same time, but if an exclusive mutable reference (&mut T) exists, no other readable or writable reference may overlap with it. You can think of this as a compile-time read/write lock. The borrow checker ensures that conflicting borrows are never active simultaneously and that no reference outlives the value it points to. This is useful even in single-threaded programs, and it also forms an important foundation for Rust’s prevention of data races in concurrent code.

The borrow-checker enforces this pattern for a variable:
`many readers → one writer → many readers → one writer → ...`

Allowed (N readers finish, then a single writer):

```rust
let mut x = 10;

let a = &x;
let b = &x;
println!("{a} {b}"); // last use of a and b

let w = &mut x;
*w += 1;                 // exclusive writer
```

This is rejected because the reader is still needed after the writer is created:

```rust
let mut x = 10;

let r = &x;
let w = &mut x;      // error: conflicting borrow (r is used after writer w)

*w += 1;
println!("{r}");         // r is still needed here
```

## Crates
### HTTP Server
- <https://github.com/actix/actix-web>
- <https://github.com/tokio-rs/axum>
- <https://github.com/poem-web/poem>

### Machine Learning
- [ndarray](https://docs.rs/ndarray/latest/ndarray/doc/ndarray_for_numpy_users/index.html): numpy equivalent
  - [loading npy files](https://docs.rs/npy/latest/npy/) (from Python numpy)
 - [ORT](https://github.com/pykeio/ort): ONNX run time

### GUI
- [Tauri](https://tauri.app/): Use Rust in backend and web stack in frontend to build desktop and mobile apps
  - [Tauri MCP Server](https://github.com/hypothesi/mcp-server-tauri)
- [GPUI](https://www.gpui.rs/): UI lib created by and used in Zed editor
  - [GPUI components](https://github.com/longbridge/gpui-component)

### Text Extraction
- [pdf_oxide](https://github.com/yfedoseev/pdf_oxide): fast PDF text extraction
- [ferrules](https://github.com/AmineDiro/ferrules/tree/main): structured text extraction
- https://github.com/kreuzberg-dev/kreuzberg
- https://github.com/yobix-ai/extractous

### Text Chunking
- https://github.com/benbrandt/text-splitter
- https://github.com/d1pankarmedhi/chunkr
- https://github.com/idleness76/wg-ragsmith

### Concurrency
- [cineyma](https://github.com/pixperk/cineyma): Erlang inspired OTP-style actor framework
- [asupersync](https://github.com/Dicklesworthstone/asupersync): Async runtime for Rust where correctness is structural: region-owned tasks, cancel-correct protocols, capability-gated effects, and deterministic replay testing

### Coding Agents
- [OpenAI Codex](https://github.com/openai/codex)
- [pi_agent_rust](https://github.com/Dicklesworthstone/pi_agent_rust): Rust port of [pi agent](https://github.com/badlogic/pi-mono)

## References
- [Error handling](https://www.howtocodeit.com/guides/the-definitive-guide-to-rust-error-handling)
- <https://jmmv.dev/2018/06/rust-review-borrow-checker.html>
- <https://doc.rust-lang.org/book/>
- <https://news.ycombinator.com/item?id=24867610>
- <https://lubeno.dev/blog/rusts-productivity-curve>

#coding
