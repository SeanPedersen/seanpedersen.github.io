#!/bin/bash
# Fast Rust-based blog builder
# This replaces the Node.js build process for significantly faster builds

set -e

echo "🦀 Running Rust build (fast mode)..."
./build-rust/target/release/blog-builder

# Generate RSS feed (still using Node.js)
echo "📡 Generating RSS feed..."
node scripts/generate-rss.js

# Run optimization if requested
if [ "$1" == "--optimize" ] || [ "$1" == "-o" ]; then
    echo "⚡ Running optimization..."
    node scripts/optimize.js
fi

echo "✅ Build complete!"
