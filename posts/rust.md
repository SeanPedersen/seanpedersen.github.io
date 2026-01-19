---
date: '2025-11-14'
---
# Rust

Rust is a statically and strongly typed systems programming language that enforces memory safety (without a garbage collector) through its ownership and borrowing model, while still
being efficient in the ballpark of C or C++.

In safe Rust, the compiler prevents entire classes of bugs common in C, such as use-after-free and most data races. It also forces explicit handling of optional values via Option instead of allowing null references. While this shifts many potential failures from runtime to compile time (in contrast with dynamic languages like Python or unsafe manual memory management in C) - Rust does not eliminate logic errors, panics or higher-level concurrency issues like deadlocks.

Rust also shines through its ergnomic package manager cargo that provides a smooth developer UX. The only real downside of Rust is its inherent complexity and at times verbose / ugly syntax.

Great introduction: <https://fasterthanli.me/articles/a-half-hour-to-learn-rust>

## References
- [Error handling](https://www.howtocodeit.com/guides/the-definitive-guide-to-rust-error-handling)
- [Erlang OTP-style actor framework](https://github.com/pixperk/cineyma)
- <https://jmmv.dev/2018/06/rust-review-borrow-checker.html>
- <https://doc.rust-lang.org/book/>
- <https://news.ycombinator.com/item?id=24867610>
- <https://lubeno.dev/blog/rusts-productivity-curve>

#coding
