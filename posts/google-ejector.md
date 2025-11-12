---
title: 'Google & Co Ejector'
date: '2020-12-07'
---
Is closed source, centralized, privacy invading software still part of your digital life? Eject now and start using user privacy respecting software. This is a small curated collection of awesome FOSS and privacy respecting services, capable of replacing many popular, centralized, privacy invading online services so you can sleep better at night. At least do not feed the giant data kraken like Google and Meta anymore - export your data and delete your accounts to stop feeding them your personal data.

## Awesome Alternatives

Alternatives to privacy invading data kraken services.

Google Chrome:

Firefox, Brave etc. also track their users.

- [Ungoogled Chromium](https://github.com/ungoogled-software/ungoogled-chromium) - Chromium without Google Services (spy ware)

Google Search:

- [SearX](https://searx.si/) ([Code](https://github.com/searx/searx)) - Privacy-respecting metasearch engine, self-hostable
- [StartPage](https://www.startpage.com/) - Privacy focused search engine

Google Mail / Outlook:

- [Protonmail](https://proton.me/mail) - privacy focused
- [TutaMail](https://tuta.com/) - privacy focused

ChatGPT / Claude / Gemini:

- [Local LLM Apps](https://seanpedersen.github.io/posts/local-ai-chat-apps)

Perplexity:

- [Perplexica](https://github.com/ItzCrazyKns/Perplexica) - Open source answer engine
- [Morphic](https://github.com/miurla/morphic) - Open source answer engine

WhatsApp / Telegram:

- [Signal](https://signal.org/) - Privacy focused messenger
- [Matrix](https://matrix.org/) ([Code](https://github.com/matrix-org/synapse)) - end-to-end encrypted, federated, self-hostable
- [BitChat](https://bitchat.free/) - decentralized chat app via bluetooth

Google Docs:

- [Fileverse](https://docs.fileverse.io/) ([Code](https://github.com/fileverse/fileverse-ddoc)) - end-to-end encrypted, real-time collaborative editing, self-hostable
- [Cryptpad](https://cryptpad.fr/) ([Code](https://github.com/xwiki-labs/cryptpad)) - end-to-end encrypted, real-time collaborative editing, self-hostable

Google Drive / Dropbox:

- [Syncthing](https://syncthing.net/) ([Code](https://github.com/syncthing/syncthing)) - decentralized, self-hostable, fast & unix aligned

Google Calendar / Contacts:

- [Etesync](https://www.etesync.com/) ([Code](https://github.com/etesync/server)) - end-to-end encrypted, self-hostable

Facebook / Twitter:

- [Nostr](https://nostr.com/) ([Code](https://github.com/nostr-protocol/nostr)) - decentralized, self-hostable
- [Mastodon](https://joinmastodon.org/) ([Code](https://github.com/tootsuite/mastodon)) - federated, self-hostable
- [Scuttlebutt](https://scuttlebutt.nz/) ([Code](https://github.com/ssbc/ssb-server)) - decentralized, self-hostable

YouTube:

- [Invidious](https://invidious.io/) - privacy respecting YouTube frontend
- [FreeTube](https://github.com/FreeTubeApp/FreeTube) - privacy respecting desktop client
- [PeerTube](https://joinpeertube.org/) ([Code](https://github.com/Chocobozzz/PeerTube)) - decentralized, federated, self-hostable

Spotify:

- [Funkwhale](https://funkwhale.audio/) ([Code](https://dev.funkwhale.audio/funkwhale/funkwhale)) - federated, self-hostable
- [Airsonic](https://airsonic.github.io/) ([Code](https://github.com/airsonic/airsonic)) - self-hostable

Google Authenticator:

- [Ente Auth](https://ente.io/auth/) ([Code](https://github.com/ente-io/ente)) - open source
- [Proton Authenticator](https://proton.me/authenticator) - privacy focused

HTTP(S):

- [IPFS](https://ipfs.io/) ([Code](https://github.com/ipfs/go-ipfs)) - decentralized, self-hostable

Github / Gitlab / Bitbucket:

- [Radicle](https://radicle.xyz/) ([Code](https://github.com/radicle-dev/radicle-upstream)) - decentralized, self-hostable

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

## [Export Your Twitter (X) Data](https://seanpedersen.github.io/posts/analyze-twitter-likes)

## Glossary

**End-to-end encrypted** means your data is en- & decrypted only on your machine, resulting in maximum security with maximum responsibility (password loss equals data loss).

**Self-hostable** means anyone can run the software by themselves and thus maintain control of their data independently of any central authority.

**Decentralized** means the software does not need to communicate over the internet with a central server in order to work, instead it is capable of communicating independently via peer-to-peer based networking (very useful when dealing with limited internet availability or censoring governments).

**Federated** means instances of the software build a so called "fediverse" allowing users to communicate across instances.

## More Resources

- <https://github.com/tycrek/degoogle>
- [https://news.ycombinator.com/item?id=25090218](https://news.ycombinator.com/item?id=25090218)

#tutorial #privacy
