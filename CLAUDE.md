# Project Overview

Static site generator for the blog `seanpedersen.github.io` written in Rust.

It processes:
- Markdown posts from `posts/`
- HTML/CSS/JS templates and assets from `website/`
- Generated output into `out/`

The site is deployed to GitHub Pages.

## Tech Stack

- Rust 2021
- Cargo

## Common Commands

```bash
cargo fmt
cargo clippy
cargo test
cargo build --release
./target/release/blog-builder
```

Serve generated output locally:

```bash
python3 -m http.server -d out
```

## Related Posts Behavior

- `--smart-similar` is **not** enabled by default.
- Default related-post selection uses tag-based fallback logic.
- Enable embedding-based similarity explicitly with:

```bash
./target/release/blog-builder --smart-similar
```

## Typography & Layout

Both knobs live as CSS variables in `website/global/global.css` (`:root`):

- `--font-sans` — system-font stack, no webfont files or external requests.
- `--article-measure` — width of the article's text column (644px); `--article-width`
  adds the article padding/border on top and is what `article` / `.postContainer` set
  their `max-width` to.

## Project Conventions

- Prefer small, focused Rust modules and functions.
- Use `anyhow::Result` and `?` for fallible top-level workflows.
- Prefer borrowing over cloning where practical.
- Keep templates/assets under `website/`, posts under `posts/`, generated files under `out/`.
