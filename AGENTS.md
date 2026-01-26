# AGENTS.md - Agent Guidelines for SeanPedersen.github.io

## Project Overview

This is a static site generator written in Rust that builds a personal blog website. The project processes html/css/js files from the `website/` directory (organized in `index/`, `post/`, and `global/` subdirectories) and Markdown posts from the `posts/` directory to generate optimized HTML/CSS/JS output in the `out/` directory.

**Tech Stack:**
- **Language**: Rust (2021 edition)
- **Package Manager**: Cargo
- **Key Dependencies**: pulldown-cmark (Markdown), tera (templating), syntect (syntax highlighting), lightningcss/minify-html/minify-js (optimization)
- **Output**: Static HTML/CSS/JS website deployed to GitHub Pages

## Build/Lint/Test Commands

### Building
```bash
# Compile the Rust binary (optimized)
cargo build --release

# Generate the static website
./target/release/blog-builder

# Build and serve locally for development
cargo build --release && ./target/release/blog-builder && python3 -m http.server -d out
```

### Testing
```bash
# Run all tests
cargo test

# Run a specific test (by name substring)
cargo test test_name

# Run tests with verbose output
cargo test -- --nocapture

# Run tests in release mode (faster but less debug info)
cargo test --release
```

### Linting & Formatting
```bash
# Format code (rustfmt)
cargo fmt

# Check formatting without changing files
cargo fmt --check

# Run Clippy linter
cargo clippy

# Run Clippy with auto-fix suggestions
cargo clippy --fix

# Check code without building
cargo check
```

### Development Workflow
```bash
# Full development cycle: format, lint, test, build
cargo fmt && cargo clippy && cargo test && cargo build --release && ./target/release/blog-builder
```

## Code Style Guidelines

### General Principles
- **Memory Safety**: Leverage Rust's ownership system - prefer borrowing over cloning where possible
- **Error Handling**: Use `anyhow::Result` for top-level functions, `?` operator for error propagation
- **Performance**: Optimize for both compile-time and runtime performance
- **Readability**: Code should be self-documenting; use descriptive names
- **Modularity**: Keep functions focused on single responsibilities; use modules for logical grouping

### Imports and Dependencies
```rust
// Group imports by crate, then std, then external
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
```

### Naming Conventions
- **Functions**: `snake_case` (e.g., `build_index_page`, `extract_classes_from_css`)
- **Types/Structs**: `PascalCase` (e.g., `PostData`, `SiteConfig`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `const CHARS: &[u8] = b"..."`)
- **Variables**: `snake_case` (e.g., `css_content`, `output_dir`)
- **Modules**: `snake_case` (e.g., `page_generation`, `class_minifier`)

### Function Design
- **Pure Functions**: Prefer functions without side effects when possible
- **Early Returns**: Use early returns to avoid deep nesting
- **Documentation**: Document public functions with `///` comments explaining purpose and parameters
- **Parameter Types**: Use references (`&str`, `&Path`) instead of owned types when borrowing is sufficient

```rust
/// Extract all CSS class names from CSS content
fn extract_classes_from_css(css: &str) -> HashSet<String> {
    // Implementation
}
```

### Error Handling
- Use `anyhow::Result<T>` for fallible operations
- Use `?` operator for concise error propagation
- Provide meaningful error messages with context

```rust
fn process_file(path: &Path) -> Result<String> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;
    Ok(content)
}
```

### Collections and Memory
- Use `HashMap`/`HashSet` for key-based lookups
- Prefer `Vec` for ordered collections
- Use `&str` over `String` when ownership isn't needed
- Leverage iterators and functional programming patterns

```rust
let classes: HashSet<String> = css_content
    .lines()
    .flat_map(|line| extract_classes_from_line(line))
    .collect();
```

### Constants and Magic Numbers
- **Never use magic numbers** - extract to named constants
- Use descriptive constant names that explain their purpose
- Group related constants together

```rust
const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const MAX_CLASS_NAME_LENGTH: usize = 50;
```

### String Handling
- Use `&str` for string slices when possible
- Use `String` when ownership is required
- Prefer string interpolation with `format!` over concatenation
- Use raw strings (`r#"..."#`) for regex patterns and complex strings

### Regex Usage
- Compile regex patterns once and reuse them
- Use raw strings for regex patterns to avoid excessive escaping
- Add comments explaining complex regex patterns

```rust
// Match class selectors: .classname followed by selector boundaries
let class_re = Regex::new(r"\.([a-zA-Z_][a-zA-Z0-9_-]*)").unwrap();
```

### Performance Considerations
- Use `rayon` for parallel processing of independent tasks
- Prefer stack allocation over heap allocation when possible
- Use `once_cell` for lazy static initialization
- Profile and optimize bottlenecks identified through benchmarking

### Testing
- Write unit tests for complex logic functions
- Use descriptive test names that explain the scenario being tested
- Test both success and failure cases
- Use `#[cfg(test)]` modules for test-specific code

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_short_name_basic() {
        assert_eq!(generate_short_name(0), "a");
        assert_eq!(generate_short_name(25), "z");
        assert_eq!(generate_short_name(26), "A");
    }
}
```

### File Organization
- Keep modules focused and cohesive
- Use clear module boundaries with well-defined interfaces
- Place related functionality together
- Separate concerns: parsing, generation, optimization

### Asset Optimization
- Minify CSS, HTML, and JavaScript for production builds
- Use efficient algorithms for text processing
- Cache expensive computations when possible
- Optimize for both size and loading performance

## Project Structure

```
├── src/
│   ├── main.rs              # Entry point and build orchestration
│   ├── page_generation.rs   # Post data extraction and processing
│   ├── index_generation.rs  # Homepage generation
│   ├── post_generation.rs   # Individual post page generation
│   ├── global_generation.rs # Global HTML page generation
│   ├── rss_generation.rs    # RSS feed generation
│   ├── class_minifier.rs    # CSS class name minification
│   ├── optimization.rs      # Asset optimization (CSS/JS minification)
│   └── syntaxes/            # Custom syntax definitions for syntax highlighting
├── posts/                   # Markdown blog posts
├── website/
│   ├── index/               # Files for index page building (HTML template, CSS, JS)
│   ├── post/                # Files for post pages building (HTML template, CSS, JS)
│   ├── global/              # Global files (CSS, JS, HTML templates, favicons processed to out/)
│   └── images/              # Image assets
├── out/                    # Generated static site (gitignored)
├── Cargo.toml              # Rust dependencies and config
└── .github/workflows/      # GitHub Actions deployment
```

## Development Workflow

1. **Make Changes**: Edit Rust source files in `src/`
2. **Format**: `cargo fmt` to ensure consistent formatting
3. **Lint**: `cargo clippy` to catch potential issues
4. **Test**: `cargo test` to verify functionality
5. **Build**: `cargo build --release && ./target/release/blog-builder`
6. **Verify**: Check `out/` directory and serve locally if needed
7. **Deploy**: Push to `main` branch (GitHub Actions handles deployment)

## GitHub Actions Integration

The project uses GitHub Actions for automated deployment:
- Builds on every push to `main` branch
- Caches Rust dependencies and binary for faster builds
- Deploys generated `out/` directory to GitHub Pages

## Common Tasks

- **Add new post**: Create Markdown file in `posts/` with frontmatter
- **Add post icon**: Set optional `icon` in post frontmatter with inline SVG (rendered in index and post title)
- **Modify templates**: Edit files in `website/index/`, `website/post/`, or `website/global/`
- **Update styles**: Modify CSS in `website/index/`, `website/post/`, or `website/global/`
- **Add functionality**: Implement in appropriate `src/*.rs` module
- **Optimize assets**: Enhance `optimization.rs` or CSS/JS minification logic

## Security Considerations

- No user input processing in the static site generator
- All content is pre-processed Markdown (trusted input)
- Generated HTML is safe for static hosting
- No database or external service dependencies
