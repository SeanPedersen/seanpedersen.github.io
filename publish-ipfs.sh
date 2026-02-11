#!/usr/bin/env sh
set -e

OUT_DIR="out/"
IPNS_KEY="sean-blog"

# 1. Detect IPFS
if ! command -v ipfs >/dev/null 2>&1; then
  echo "❌ ipfs not installed"
  exit 1
fi

# 2. Ensure daemon is running
if ! ipfs id >/dev/null 2>&1; then
  echo "🚀 Starting IPFS daemon..."
  ipfs daemon --init &
  sleep 5
fi

# 3. Check out/ exists
if [ ! -d "$OUT_DIR" ]; then
  echo "❌ $OUT_DIR directory not found"
  exit 1
fi

# 4. Create IPNS key if missing
if ! ipfs key list | grep -q "$IPNS_KEY"; then
  echo "🔑 Creating IPNS key: $IPNS_KEY"
  ipfs key gen "$IPNS_KEY"
fi

# 5. Add site to IPFS
echo "📦 Adding $OUT_DIR to IPFS..."
CID=$(ipfs add -r -Q --cid-version=1 --raw-leaves "$OUT_DIR")
echo "🌐 CID IPFS link (works offline):"
echo "ipfs://$CID"

# 6. Publish to IPNS
echo "🔗 Publishing IPNS key $IPNS_KEY..."
ipfs name publish --ttl 1m --key="$IPNS_KEY" /ipfs/"$CID"

# 7. Get IPNS hash
IPNS_HASH=$(ipfs key list -l | grep "$IPNS_KEY" | awk '{print $1}')
echo "🌐 Access your blog via IPNS (stable link - works offline):"
echo "ipns://$IPNS_HASH"
echo "🌐 Access your blog via IPNS gateway (works online even without IPFS installed):"
echo "https://$IPNS_HASH.ipns.dweb.link"
