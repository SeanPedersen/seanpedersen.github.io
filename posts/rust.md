---
date: '2025-11-14'
icon: "/images/icons/rust.svg"
---
# Rust

Rust is a statically and strongly typed systems programming language that enforces memory safety (without a garbage collector) through its ownership and borrowing model, while still being efficient in the ballpark of C or C++.

In safe Rust, the compiler prevents entire classes of bugs common in C, such as use-after-free and most data races. It also forces explicit handling of optional values via Option instead of allowing null references. While this shifts many potential failures from runtime to compile time (in contrast with dynamic languages like Python or unsafe manual memory management in C) - Rust does not eliminate logic errors, panics or higher-level concurrency issues like deadlocks. These properties are the reason Rust has made its way into the Linux kernel.

Rust also shines through its ergnomic package manager cargo that provides a smooth developer UX. The only real downside of Rust is its inherent complexity and at times verbose / ugly syntax.

Great introduction: <https://fasterthanli.me/articles/a-half-hour-to-learn-rust>

## HTTP Server
- <https://github.com/actix/actix-web>
- <https://github.com/tokio-rs/axum>
- <https://github.com/poem-web/poem>

## Processing Tensors
- [ndarray](https://docs.rs/ndarray/latest/ndarray/doc/ndarray_for_numpy_users/index.html)
- [loading npy files](https://docs.rs/npy/latest/npy/) (from Python numpy)

## GUI
- [Tauri](https://tauri.app/): Use Rust in backend and web stack in frontend
  - [Tauri MCP Server](https://github.com/hypothesi/mcp-server-tauri)
- [GPUI](https://www.gpui.rs/): UI lib created by and used in Zed editor
  - [GPUI components](https://github.com/longbridge/gpui-component)

## PDF Extraction
- [pdf_oxide](https://github.com/yfedoseev/pdf_oxide): fast text extraction
- [ferrules](https://github.com/AmineDiro/ferrules/tree/main): structured text extraction

## Concurrency
- [cineyma](https://github.com/pixperk/cineyma): Erlang inspired OTP-style actor framework
- [asupersync](https://github.com/Dicklesworthstone/asupersync): Async runtime for Rust where correctness is structural: region-owned tasks, cancel-correct protocols, capability-gated effects, and deterministic replay testing

## Coding Agents
- [OpenAI Codex](https://github.com/openai/codex)
- [pi_agent_rust](https://github.com/Dicklesworthstone/pi_agent_rust): Rust port of [pi agent](https://github.com/badlogic/pi-mono)

## References
- [Error handling](https://www.howtocodeit.com/guides/the-definitive-guide-to-rust-error-handling)
- <https://jmmv.dev/2018/06/rust-review-borrow-checker.html>
- <https://doc.rust-lang.org/book/>
- <https://news.ycombinator.com/item?id=24867610>
- <https://lubeno.dev/blog/rusts-productivity-curve>

#coding
