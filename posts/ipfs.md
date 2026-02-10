# InterPlanetary File System

IPFS is a content based decentralized file system powered by Merkle DAG (similar to git) - allowing for local networking between nodes (separate from the internet).

Install [IPFS Desktop](https://docs.ipfs.tech/install/ipfs-desktop/#install-instructions)

Browser Extension (so ipfs.io links work):
- [Chrome](https://chromewebstore.google.com/detail/ipfs-companion)
- [Firefox](https://addons.mozilla.org/en-US/firefox/addon/ipfs-companion/)

## Publishing Static Website

Configure OUT_DIR (should contain your index.html) and IPNS_KEY (an id for your website).

```bash
#!/usr/bin/env sh
set -e

OUT_DIR="out/"
IPNS_KEY="your-website-id"

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
echo "🌐 Direct IPFS link (works offline):"
echo "https://ipfs.io/ipfs/$CID"

# 6. Publish to IPNS
echo "🔗 Publishing $CID via IPNS key: $IPNS_KEY..."
ipfs name publish --key="$IPNS_KEY" /ipfs/"$CID"

# 7. Get IPNS hash
IPNS_HASH=$(ipfs key list -l | grep "$IPNS_KEY" | awk '{print $1}')
echo "🌐 Access your blog via IPNS (stable link - works offline):"
echo "https://ipfs.io/ipns/$IPNS_HASH"
```

## Discover Self Index of Peers

This works in a local network - allowing true decentralized networking.

- Show connected nodes (returns list of $NODE_ID): `ipfs swarm peers`
- Discover public self IPNS content of node: `ipfs ls /ipns/$NODE_ID`
- Show index: `ipfs cat /ipns/$NODE_ID/index.json`

## Publish your IPNS name keys in Self Index

This creates an index of all your local IPNS keys and makes it publicly discoverable for peers in the network.

```bash
#!/usr/bin/env sh
set -e

DISCOVERY_DIR=".ipns-index"

echo "📡 Building IPNS discovery index..."

# 1. Ensure IPFS is available
command -v ipfs >/dev/null 2>&1 || {
  echo "❌ ipfs not installed"
  exit 1
}

# 2. Ensure daemon is running
if ! ipfs id >/dev/null 2>&1; then
  echo "🚀 Starting IPFS daemon..."
  ipfs daemon >/tmp/ipfs.log 2>&1 &
  until ipfs id >/dev/null 2>&1; do
    sleep 1
  done
fi

# 3. Prepare directory
rm -rf "$DISCOVERY_DIR"
mkdir -p "$DISCOVERY_DIR"

# 4. Generate index.json
echo "🧾 Generating index.json..."

echo '{ "ipns": {' > "$DISCOVERY_DIR/index.json"

FIRST=1
ipfs key list -l | while read -r KEY_ID KEY_NAME; do
  [ "$KEY_NAME" = "self" ] && continue

  if [ $FIRST -eq 0 ]; then
    echo ',' >> "$DISCOVERY_DIR/index.json"
  fi
  FIRST=0

  printf '  "%s": "%s"' "$KEY_NAME" "$KEY_ID" >> "$DISCOVERY_DIR/index.json"
done

echo '' >> "$DISCOVERY_DIR/index.json"
echo '} }' >> "$DISCOVERY_DIR/index.json"

# 5. Generate index.html
echo "🌐 Generating index.html..."

cat > "$DISCOVERY_DIR/index.html" <<'EOF'
<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <title>IPNS Index</title>
  <style>
    body { font-family: sans-serif; padding: 2rem; }
    li { margin: 0.5rem 0; }
    code { background: #eee; padding: 0.2rem 0.4rem; }
  </style>
</head>
<body>
  <h1>IPNS Index</h1>
  <ul>
EOF

ipfs key list -l | while read -r KEY_ID KEY_NAME; do
  [ "$KEY_NAME" = "self" ] && continue
  echo "    <li><a href=\"http://ipfs.io/ipns/$KEY_ID\">$KEY_NAME</a> <code>$KEY_ID</code></li>" >> "$DISCOVERY_DIR/index.html"
done

cat >> "$DISCOVERY_DIR/index.html" <<'EOF'
  </ul>
</body>
</html>
EOF

# 6. Add to IPFS
echo "📦 Adding discovery index to IPFS..."
CID=$(ipfs add -r -Q "$DISCOVERY_DIR")
echo "✅ CID: $CID"

# 7. Publish under self
echo "🔗 Publishing discovery index under self..."
ipfs name publish --lifetime=1m /ipfs/"$CID"

echo "🎉 Done!"
echo
echo "🔍 Discoverable via:"
echo "  ipfs ls /ipns/$(ipfs id -f='<id>')"
echo "  ipfs cat /ipns/$(ipfs id -f='<id>')/index.json"
echo "  http://ipfs.io/ipns/$(ipfs id -f='<id>')"
```

## More

**Initialize local node in working dir:**  ipfs init

**Start long running node process:** ipfs daemon

**Add file:** ipfs add $FILE_PATH

**Pin file at node:** ipfs pin $FILE_PATH

Show connected nodes (returns list of $NODE_ID): ipfs swarm peers

Discover public self IPNS content of node: ipfs ls /ipns/$NODE_ID

Fetch file from node: ipfs get $CID

Fetch dir from node: ipfs get -r $CID

Publish to public self IPNS: ipfs name publish /ipfs/"$CID"

Publish to IPNS key: ipfs name publish --key="$IPNS_KEY" /ipfs/"$CID"

Update self IPNS of peer: ipfs name resolve --nocache /ipns/$NODE_ID

Resolve IPNS name to CID: ipfs resolve -r $IPNS_NAME

Publish static website: <http://docs.ipfs.tech.ipns.localhost:8080/how-to/websites-on-ipfs/multipage-website>

**Node WebUI:** <http://localhost:5001/webui>

**Synchronize Pins across distributed daemons:** <https://github.com/ipfs/ipfs-cluster>

**Package Manager Integrations:** <https://github.com/ipfs/package-managers#current-ipfs-integrations>

**File encryption:**

- <https://github.com/ipfs/faq/issues/116>

## References

- [Kubo: IPFS Implementation in Go](https://github.com/ipfs/kubo)
- <https://youtu.be/HUVmypx9HGI>
- <https://wiki.archlinux.org/index.php/IPFS>
- <https://github.com/ipfs/roadmap#2019-epics>
- <https://youtu.be/GJ2980DWdyc?t=264>
- <https://decentralized.blog/ten-terrible-attempts-to-make-ipfs-human-friendly.html>

QUESTIONS:
- Can I replace SyncThing with IPFS?
- Can I run an IPFS http proxy that automatically replaces HTTP requests with local IPFS response if available (basically an IPFS cache for HTTP, so stuff works offline in my local net)? Basically I want a daemon that caches HTTP/S downloads in IPFS and will use the cache for future HTTP/S requests with same URL.
- Can we use IPFS to create a content-based package manager to lock down dependencies accurately?
- IPFS over local bluetooth / wi-fi networks?
- Can I run a website discoverable by anyone in my local network via IPFS?
