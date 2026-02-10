#!/usr/bin/env sh
set -e

OUT_DIR="out"
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
  ipfs key gen "$IPNS_KEY" --type=rsa --size=2048
fi

# 5. Add site to IPFS
echo "📦 Adding $OUT_DIR to IPFS..."
CID=$(ipfs add -r -Q "$OUT_DIR")

echo "✅ CID: $CID"

# 6. Pin CID
echo "📌 Pinning CID..."
ipfs pin add "$CID"

# 7. Publish to IPNS (directory-safe)
echo "🌍 Publishing to IPNS..."
ipfs name publish \
  --key="$IPNS_KEY" \
  --allow-offline \
  "/ipfs/$CID/"

# 8. Show final URL (trailing slash ensures relative paths work)
IPNS_ID=$(ipfs key list -l | awk "/$IPNS_KEY/ {print \$1}")

echo ""
echo "🎉 Blog published!"
echo "IPNS URL (working):"
echo "  http://127.0.0.1:8080/ipns/$IPNS_ID/"
