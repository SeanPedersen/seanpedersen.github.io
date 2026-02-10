# IPFS
Content based decentralized file system powered by Merkle DAG (similar to git).

**Initialize local node in working dir:**  ipfs init

**Start long running node process:** ipfs daemon

**Add file:** ipfs add $FILE_PATH

**Pin file at node:** ipfs pin $FILE_PATH

Show connected nodes: ipfs swarm peers

Discover public self IPNS content of node:
curl http://127.0.0.1:8080/ipns/$NODE_ID

Fetch file from node: ipfs get $CID

# 6. Publish to IPNS
echo "🔗 Publishing $CID via IPNS self (public discoverable)..."
ipfs name publish /ipfs/"$CID"

echo "🔗 Publishing $CID via IPNS key: $IPNS_KEY..."
ipfs name publish --key="$IPNS_KEY" /ipfs/"$CID"

# Resolve IPNS name to CID
ipfs resolve -r $IPNS_NAME

Chrome Extension:
<https://chromewebstore.google.com/detail/ipfs-companion>

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
