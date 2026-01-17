#!/bin/bash
# Fast Rust-based blog builder
# This replaces the Node.js build process for significantly faster builds
# All optimization and RSS generation is now done in Rust!

set -e

echo "🦀 Running Rust build (fast mode)..."
./target/release/blog-builder

echo "✅ Build complete!"

