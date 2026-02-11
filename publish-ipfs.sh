#!/usr/bin/env sh
set -e

OUT_DIR="out/"
IPNS_KEY="sean-blog"

# Build website
./target/release/blog-builder

# Check out/ exists
if [ ! -d "$OUT_DIR" ]; then
  echo "❌ $OUT_DIR directory not found"
  exit 1
fi

# Create IPNS key if missing
if ! ipfs key list | grep -q "$IPNS_KEY"; then
  echo "🔑 Creating IPNS key: $IPNS_KEY"
  ipfs key gen "$IPNS_KEY"
fi

# Add site to IPFS
echo "📦 Adding $OUT_DIR to IPFS..."
CID=$(ipfs add -r -Q --cid-version=1 --raw-leaves "$OUT_DIR")
echo "🌐 CID IPFS link (works offline):"
echo "ipfs://$CID"

# Publish to IPNS
echo "🔗 Publishing IPNS key $IPNS_KEY..."
ipfs name publish --ttl 1m --key="$IPNS_KEY" /ipfs/"$CID"

# Get IPNS hash
IPNS_NAME=$(ipfs key list -l | grep "$IPNS_KEY" | awk '{print $1}')
echo "🌐 Access your blog via IPNS (stable link - works offline):"
echo "ipns://$IPNS_NAME"
echo "🌐 Access your blog via IPNS gateway (works online even without IPFS installed):"
echo "https://$IPNS_NAME.ipns.dweb.link"
