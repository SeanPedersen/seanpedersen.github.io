# Sean's Blog

## Selected Articles

- [AGI Bubble](https://seanpedersen.github.io/posts/agi-bubble)
- [Structure of Neural Embeddings](https://seanpedersen.github.io/posts/structure-of-neural-latent-space)
- [Memetics](https://seanpedersen.github.io/posts/memetics)
- [(Sub)conscious (Re)programming](https://seanpedersen.github.io/posts/conscious-reprogramming)
- [Zipf's Law](https://seanpedersen.github.io/posts/zipfs-law)
- [Kolmogorov Complexity](https://seanpedersen.github.io/posts/kolmogorov-complexity)

## Setup

Just clone this repo, create a new github project named $your_username.github.io, follow instructions below and push the modified repo up (on each push the site is getting rebuild and published).

- Posts live in ./posts/ as markdown files
- Blog name (etc) in ./website/html-templates/index.html (search heading2Xl)
- Profile pic in ./website/images/profile.webp
- Search for "https://github.com/SeanPedersen/seanpedersen.github.io" and change it to your repo so the "Edit on GitHub" link works for your blog
- Edit / remove Posthog analytics in ./website/html-templates files

### Local

- Install dependencies: `npm ci`
- [Install Rust](https://rust-lang.org/tools/install/)
- Compile: `cargo build --release`
- Build: `npm run build` (generates `out/` directory)
- Serve locally: `npm start` (http://localhost:8000)

