---
date: '2026-07-07'
icon: "/images/icons/go.svg"
---
# Go

[Go](https://en.wikipedia.org/wiki/Go_(programming_language)) is a statically typed, compiled programming language with built-in [garbage collection](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)), designed at Google for simple, reliable, and efficient software. It often compiles to a single, easy-to-deploy binary and builds quickly, which makes it a natural fit for command line tools and cloud infrastructure. [Docker](https://en.wikipedia.org/wiki/Docker_(software)), [Kubernetes](https://en.wikipedia.org/wiki/Kubernetes), and [Terraform](https://en.wikipedia.org/wiki/Terraform_(software)) are all written in Go.

Go's biggest strength is its standard library and fast compilation times. The stdlib-first culture means you reach for net/http, testing, and log/slog before any third-party package. Concurrency is built into the language through goroutines and channels, influenced by [Communicating Sequential Processes](https://en.wikipedia.org/wiki/Communicating_sequential_processes). This makes lightweight concurrent work easy to express, though it is not inherently simpler than [Erlang](/posts/erlang)’s [actor model](https://en.wikipedia.org/wiki/Actor_model); Erlang emphasizes isolated processes that communicate by message passing, while Go combines message passing with shared-memory concurrency. Compared with [Rust](/posts/rust)’s ownership-based approach, Go generally has a lower upfront learning curve but fewer compile-time guarantees.

The main downsides are verbosity, error handling, and limited type-system expressiveness. The repeated `if err != nil` pattern is noisy. There are no [algebraic data types](https://en.wikipedia.org/wiki/Algebraic_data_type), nil still needs care, data races are possible at runtime, and generics exist since Go 1.18 but remain intentionally limited compared with Rust or TypeScript. Go modules work fine, but lack the smooth experience of Cargo.

Choose Go when you want simple deployment, fast builds, strong standard-library defaults, and readable service code. It is weaker when your domain needs highly expressive types, deep compile-time invariants, or sophisticated fault-tolerant concurrency abstractions.

Great introduction: <https://go.dev/tour/>

## HTTP Server
Default to [net/http](https://pkg.go.dev/net/http). Since Go 1.22, the standard `ServeMux` supports method matching and wildcard path patterns, which covers many small and medium APIs without a framework.

- [chi](https://github.com/go-chi/chi): thin, idiomatic router that stays compatible with net/http middleware
- [Gin](https://github.com/gin-gonic/gin): popular, fast, established framework with more batteries included
- [Echo](https://github.com/labstack/echo): well-documented framework with a clean API
- [Fiber](https://github.com/gofiber/fiber): [Express](https://en.wikipedia.org/wiki/Express.js)-inspired and fast, but built on fasthttp rather than net/http

## CLI
- [cobra](https://github.com/spf13/cobra): the standard for CLI apps with subcommands, flags, and auto-help (used by kubectl and helm)
- [urfave/cli](https://github.com/urfave/cli): simple and easy to learn for small CLI tools

## TUI
- [bubbletea](https://github.com/charmbracelet/bubbletea): modern TUI framework for interactive terminal apps

## Desktop GUI
- [Wails](https://wails.io/): Go backend with a web frontend (like [Tauri](https://tauri.app/) for [Rust](/posts/rust)) to build desktop apps
- [Fyne](https://fyne.io/): pure Go, native cross-platform GUI toolkit

## Database
- [database/sql](https://pkg.go.dev/database/sql): generic stdlib interface around SQL drivers
- [pgx](https://github.com/jackc/pgx): first-class [Postgres](/posts/postgres) driver and toolkit
- [sqlc](https://github.com/sqlc-dev/sqlc): generates type-safe Go code from hand-written SQL
- [sqlx](https://github.com/jmoiron/sqlx): quality-of-life layer on top of database/sql
- [ent](https://github.com/ent/ent): schema-first code generator that maps graph schemas to Go types

## Machine Learning
Go is not the best ecosystem for model training or research. Use it mostly for numeric utilities, data processing, and deploying exported models inside Go services. There is no mature drop-in equivalent to NumPy's ndarray ecosystem.

- [gonum](https://www.gonum.org/): third-party numerical computing stack and closest Go analogue to NumPy/SciPy for linear algebra, statistics, probability, optimization, integration, and graphs
- [gonum/mat](https://pkg.go.dev/gonum.org/v1/gonum/mat): NumPy-like dense matrices and linear algebra
- [gonum/stat](https://pkg.go.dev/gonum.org/v1/gonum/stat): statistics helpers for distributions, regression, distances, and summary statistics
- [gonum/optimize](https://pkg.go.dev/gonum.org/v1/gonum/optimize): SciPy-like numerical optimization
- [GoMLX](https://github.com/gomlx/gomlx): active ML framework with tensors and accelerator-backed computation; closer to PyTorch/JAX than NumPy
- [onnxruntime_go](https://github.com/yalue/onnxruntime_go): Go wrapper around ONNX Runtime for running exported neural networks; requires the ONNX Runtime shared library

## Logging
- [log/slog](https://pkg.go.dev/log/slog): structured logging in the standard library since Go 1.21
- [zap](https://github.com/uber-go/zap): high-performance structured logging
- [zerolog](https://github.com/rs/zerolog): zero-allocation JSON logger

## Testing
- [testing](https://pkg.go.dev/testing): the built-in package for unit tests, benchmarks, and fuzzing
- [testify](https://github.com/stretchr/testify): extends stdlib testing with clean assertions and mocks
- [gomock](https://github.com/uber-go/mock): maintained mocking framework for interfaces and external services

## Linting
- [golangci-lint](https://golangci-lint.run/): the standard all-in-one linter runner for CI and local dev
- [staticcheck](https://staticcheck.dev/): advanced Go static analysis

## Concurrency
- [sync](https://pkg.go.dev/sync): stdlib primitives (Mutex, WaitGroup, Once, Map)
- [errgroup](https://pkg.go.dev/golang.org/x/sync/errgroup): run goroutines as a group and cancel on first error
- [ergo](https://github.com/ergo-services/ergo): Erlang/OTP-inspired actor framework for distributed systems
- [hollywood](https://github.com/anthdm/hollywood): actor framework for building concurrent Go applications
- [protoactor-go](https://github.com/asynkron/protoactor-go): actor model framework with local and distributed actors

## References
- [The Go Ecosystem in 2025 (JetBrains)](https://blog.jetbrains.com/go/2025/11/10/go-language-trends-ecosystem-2025/)
- [A Tour of Go](https://go.dev/tour/)
- [Effective Go](https://go.dev/doc/effective_go)
- [awesome-go](https://github.com/avelino/awesome-go)
- [Go by Example](https://gobyexample.com/)

#coding
