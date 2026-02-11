---
date: '2020-12-07'
icon: "/images/icons/internet-security.webp"
---
# Google & Co Ejector

Is closed source, centralized, privacy invading software still part of your digital life? Eject now and start using user privacy respecting software. This is a small curated collection of awesome FOSS and privacy respecting services, capable of replacing many popular, centralized, privacy invading software services so you can sleep better at night. Stop feeding giant invasive data kraken like Google and Meta - export your data and delete your accounts to stop feeding them your personal data.

## Awesome Software

Software that treats you as a human - respectful.

Messenger:

- [Signal](https://signal.org/) - Privacy focused messenger
- [Matrix](https://matrix.org/) - end-to-end encrypted, federated, self-hostable
  - [Clients](https://matrix.org/ecosystem/clients/): Mobile (Element X), Desktop (Nheko)
  - [Server](https://github.com/matrix-construct/tuwunel) - Rust
  - [Bot SDK](https://codeberg.org/imbev/simplematrixbotlib) - Python
- [BitChat](https://bitchat.free/) - decentralized chat app via bluetooth
- [Messenger Comparison](https://www.messenger-matrix.de/messenger-matrix.html)

Web Browser:

Firefox, Brave etc. also track their users (with default settings). Test [here](https://coveryourtracks.eff.org/) how trackable your browser is.

- [Mullvad](https://mullvad.net/en/browser) - Privacy focused Firefox fork (basically Tor browser without Tor network)
- [LibreWolf](https://librewolf.net/) - Privacy focused Firefox fork
- [Ungoogled Chromium](https://github.com/ungoogled-software/ungoogled-chromium) - Chromium without Google Services (spy ware)
- [Browser Comparison](https://digdeeper.club/articles/browsers.xhtml#fullsummary)

Search Engine:

- [SearX](https://searx.si/) ([Code](https://github.com/searx/searx)) - Privacy-respecting metasearch engine, self-hostable
- [StartPage](https://www.startpage.com/) - Privacy focused search engine
- [Search Engine Comparison](https://digdeeper.club/articles/search.xhtml#summary)

E-Mail Provider:

- [Protonmail](https://proton.me/mail) - privacy focused
- [TutaMail](https://tuta.com/) - privacy focused
- [E-Mail Comparison](https://digdeeper.club/articles/email.xhtml#Summary)

Global Maps:

- [OpenStreetMap](https://www.openstreetmap.org/) - open data, privacy focused

ChatBot:

- [Local LLM Apps](/posts/local-ai-chat-apps)
- [Perplexica](https://github.com/ItzCrazyKns/Perplexica) - Open source answer engine
- [Morphic](https://github.com/miurla/morphic) - Open source answer engine

Shared Docs:

- [Fileverse](https://docs.fileverse.io/) ([Code](https://github.com/fileverse/fileverse-ddoc)) - end-to-end encrypted, real-time collaborative editing, self-hostable
- [Cryptpad](https://cryptpad.fr/) ([Code](https://github.com/xwiki-labs/cryptpad)) - end-to-end encrypted, real-time collaborative editing, self-hostable

File Sync:

- [Syncthing](https://syncthing.net/) ([Code](https://github.com/syncthing/syncthing)) - decentralized, self-hostable, fast & unix aligned

Calendar / Contacts:

- [Etesync](https://www.etesync.com/) ([Code](https://github.com/etesync/server)) - end-to-end encrypted, self-hostable

Social Network:

- [Nostr](https://nostr.com/) ([Code](https://github.com/nostr-protocol/nostr)) - decentralized, self-hostable
- [Mastodon](https://joinmastodon.org/) ([Code](https://github.com/tootsuite/mastodon)) - federated, self-hostable
- [Scuttlebutt](https://scuttlebutt.nz/) ([Code](https://github.com/ssbc/ssb-server)) - decentralized, self-hostable

Video Platform:

- [Invidious](https://invidious.io/) - privacy respecting YouTube frontend
- [FreeTube](https://github.com/FreeTubeApp/FreeTube) - privacy respecting desktop client
- [PeerTube](https://joinpeertube.org/) ([Code](https://github.com/Chocobozzz/PeerTube)) - decentralized, federated, self-hostable

Music:

- [Funkwhale](https://funkwhale.audio/) ([Code](https://dev.funkwhale.audio/funkwhale/funkwhale)) - federated, self-hostable
- [Airsonic](https://airsonic.github.io/) ([Code](https://github.com/airsonic/airsonic)) - self-hostable

Multi-Factor Authenticator:

- [Ente Auth](https://ente.io/auth/) ([Code](https://github.com/ente-io/ente)) - open source
- [Proton Authenticator](https://proton.me/authenticator) - privacy focused

HTTP(S):

- [IPFS](/posts/ipfs) ([Code](https://github.com/ipfs/kubo)) - decentralized, self-hostable

Source Code:

- [Radicle](https://radicle.xyz/) ([Code](https://github.com/radicle-dev/radicle-upstream)) - decentralized, self-hostable

Mobile OS:

- [LineageOS](https://lineageos.org/)
- [GrapheneOS](https://grapheneos.org/)

Operating System:

- [Fedora Linux](https://fedoraproject.org/workstation/download/)
  - [Nobara](https://nobaraproject.org/) (optimized for gaming / streaming)
- [Mint](https://linuxmint.com/) (Ubuntu Linux)
- [CachyOS](https://cachyos.org/) (Arch Linux)

## Export All Your Google Data

...using <https://takeout.google.com/> (optional for Google Photos: [GooglePhotosTakeoutHelper](https://github.com/TheLastGimbus/GooglePhotosTakeoutHelper)) and delete your account.

### Converting Google Docs files to Markdown

1. Install [Pandoc](https://pandoc.org/installing.html) to convert docx to markdown
2. Change dir to your Google Drive export: Takeout/Drive
3. Create a lua script named fix_underline_links.lua with this content:

```lua
function Link(el)
  local content = el.content
  if #content == 1 and content[1].t == "Underline" then
    content = content[1].content
  end
  return pandoc.Link(content, el.target)
end
```

4. Run this bash command to convert:

`$ find . -name "*.docx" -type f -exec sh -c 'pandoc "$0" -o "${0%.docx}.md" --extract-media=./images/ --lua-filter=fix_underline_links.lua' {} \;`

5. Remove all docx files: `$ find . -type f -name "*.docx" -delete`

## [Export Your Twitter (X) Data](/posts/analyze-twitter-likes)

## TODO: Replace Instagram

How to remove your data from Zuckerberg and backup it up to host it privacy focused.

## Glossary

**End-to-end encrypted** means your data is en- & decrypted only on your machine, resulting in maximum security with maximum responsibility (password loss equals data loss).

**Self-hostable** means anyone can run the software by themselves and thus maintain control of their data independently of any central authority.

**Decentralized** means the software does not need to communicate over the internet with a central server in order to work, instead it is capable of communicating independently via peer-to-peer based networking (very useful when dealing with limited internet availability or censoring governments).

**Federated** means instances of the software build a so called "fediverse" allowing users to communicate across instances.

## More Resources

- <https://github.com/tycrek/degoogle>
- [https://news.ycombinator.com/item?id=25090218](https://news.ycombinator.com/item?id=25090218)

#tutorial #privacy
