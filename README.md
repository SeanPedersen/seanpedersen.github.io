# Sean's Blog

Custom static site generator using Rust - builds an optimized build in ~300ms.

## Selected Articles

- [AGI Bubble](https://seanpedersen.github.io/posts/agi-bubble)
- [Structure of Neural Embeddings](https://seanpedersen.github.io/posts/structure-of-neural-latent-space)
- [Memetics](https://seanpedersen.github.io/posts/memetics)
- [(Sub)conscious (Re)programming](https://seanpedersen.github.io/posts/conscious-reprogramming)
- [Zipf's Law](https://seanpedersen.github.io/posts/zipfs-law)
- [Kolmogorov Complexity](https://seanpedersen.github.io/posts/kolmogorov-complexity)

## Setup

Just clone this repo, create a new github project named $your_username.github.io, follow instructions below and push the modified repo up (on each push the site is getting rebuild and published via Github pages).

- Posts live in ./posts/ as normal markdown files
  - date can be added at top as metadata or not (falls back to git first added)
  - tags can be specified at file end: #tag1 #tag2
- Blog name (etc) in ./website/index/index.html (search heading2Xl)
- Profile pic in ./website/images/profile.webp
- Search for "https://github.com/SeanPedersen/seanpedersen.github.io" and change it to your repo so the "Edit on GitHub" link works for your blog
- Edit / remove Posthog analytics in all html files in ./website

### Local

- [Install Rust](https://rust-lang.org/tools/install/)
- Compile: `cargo build --release`
- Build: `./target/release/blog-builder` (generates `out/` directory)
- Serve locally: `python3 -m http.server -d out` (http://localhost:8000)
