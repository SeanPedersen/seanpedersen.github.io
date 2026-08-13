# Messenger
Humanity is in need of a fun to use, secure, privacy-respecting and decentral messenger.

Protocol: The client owns the identity and message history. It uses end-to-end encryption for content and relationship metadata, direct peer-to-peer transport when possible, and optional relays or federation when direct delivery fails.

- User account via username (no phone number needed)
- Decentralized P2P discovery and messaging (device to device just works) + federated (optional relay servers make the network faster and more reliable)
- E2EE message content (txt, file, etc.) + metadata (who msgs whom)
- Connect (share keys) by scanning QR Code - “Invite link” fallback
- Thick client (user keeps message history - default is 2 months) + thin optional servers (just relay messages)
- Default: use internet, fallback: Wi-Fi Mesh; Bluetooth (like bitchat)

Chat Features ([Telegram](https://github.com/telegramdesktop/tdesktop) has best chat UX):

Protocol: Messages use one encrypted envelope format for text, files, and stickers. Group membership changes update the group key, while relays store and forward only encrypted data.

- group chats
- file transfer
- stickers

## Existing Chat Shitshow

Protocol: The main trade-off is where identity, routing, and metadata live. Centralized services simplify delivery, while peer-to-peer and federated protocols distribute trust across devices or servers.

Whatsapp and Telegram are security nightmares. WA being centralized and has no metadata encryption. Telegram is centralized and has no encryption by default.

| Messenger | Network | Clients / downloads |
| --- | --- | --- |
| [Jami](https://jami.net/) | P2P | [Official downloads](https://jami.net/download/) |
| [SimpleX Chat](https://simplex.chat/) | Decentralized relay network | [Official downloads](https://simplex.chat/downloads/) |
| [Berty](https://berty.tech/messenger/) | P2P, IPFS | [iOS App Store (legacy)](https://apps.apple.com/ie/app/berty-messenger/id1535500412), [official status](https://berty.tech/features) |
| [Bitchat](https://bitchat.free/) | Bluetooth mesh | [iOS](https://apps.apple.com/us/app/bitchat-mesh/id6748219622), [Android](https://play.google.com/store/apps/details?id=com.bitchat.droid), [Android releases](https://github.com/permissionlesstech/bitchat-android/releases) |
| [Matrix](https://matrix.org/) | Federated | [Official client list](https://matrix.org/ecosystem/clients/) |
| [Mindtheclub](https://www.mindtheclub.com/) | P2P, WebRTC | [Google Play beta](https://play.google.com/store/apps/details?id=com.bolimot.mindtheclub) |
| [Signal](https://signal.org/) | Centralized | [Official downloads](https://signal.org/download/) |

### Jami

Protocol: Jami uses distributed accounts based on [X.509 certificates](https://en.wikipedia.org/wiki/X.509#Certificates) and [OpenDHT](https://git.jami.net/savoirfairelinux/opendht) for peer discovery. Calls and messaging use [SIP](https://www.rfc-editor.org/info/rfc3261/) over TLS, with direct peer connections when possible.

[Jami](https://git.jami.net/savoirfairelinux) is a P2P messenger. The project website is https://jami.net/.

Jami actually seems like it has done many things right - only downside is the UI design of its clients (which can be fixed) and missing offline message delivery (can be fixed via decentral peer replication).

Clients: [official downloads](https://jami.net/download/). Repo: [jami repos](https://git.jami.net/savoirfairelinux)

### SimpleX Chat

Protocol: SimpleX uses pairs of unidirectional message queues on configurable relay servers. Connections have pairwise queue identifiers instead of global user identifiers, while [double-ratchet end-to-end encryption](https://en.wikipedia.org/wiki/Double_Ratchet_Algorithm) protects messages and relays hold them only until delivery.

[SimpleX Chat](https://simplex.chat/) is a decentralized messenger without phone numbers, usernames, or other global user identifiers. Users connect through invitation links or QR codes and can use the default relay servers, choose other operators, or host their own.

Clients: [official downloads](https://simplex.chat/downloads/) for Android, iOS, Linux, macOS, Windows, and the terminal.

### Berty

Protocol: Berty's Wesh protocol stores encrypted group events in append-only logs. It can sync those logs through IPFS or direct transports such as Bluetooth Low Energy. Berty has offline message delivery via a central replication service.

Berty is a chat app based on IPFS: https://berty.tech/messenger/ - [berty repo](https://github.com/berty/berty)

Clients: [iOS App Store (legacy)](https://apps.apple.com/ie/app/berty-messenger/id1535500412). The Android app is currently unavailable. See the [official status](https://berty.tech/features).

### Bitchat

Protocol: Bitchat sends compact packets through a Bluetooth Low Energy mesh. Private messages use the Noise Protocol Framework, with Nostr used as an internet fallback.

[Bitchat](https://bitchat.free/) is a P2P messenger that uses Bluetooth.

Clients: [iOS](https://apps.apple.com/us/app/bitchat-mesh/id6748219622), [Android](https://play.google.com/store/apps/details?id=com.bitchat.droid), and [Android releases](https://github.com/permissionlesstech/bitchat-android/releases).

### Matrix

Protocol: Matrix uses HTTPS APIs and server-to-server federation to replicate room events. Optional end-to-end encryption uses Olm for device sessions and Megolm for group messages.

[Matrix](https://matrix.org/) is federated (not P2P) and has horrible UX and no metadata encryption.

Clients: [official client list](https://matrix.org/ecosystem/clients/).

Matrix drawbacks:
- Olm/Megolm does not offer forward secrecy for group messaging
- Olm/Megolm does ensure end-to-end encryption for message data, but not for metadata.
- Federation makes it challenging to be GDPR compliant
- Synapse is very heavy, other implementations are less production ready
- For better or worse, the matrix foundation is under UK jurisdiction.

### Mindtheclub

Protocol: MindTheClub uses WebRTC to create encrypted direct connections between devices. Its group design uses gossip, so members relay messages to other members.

https://www.mindtheclub.com/white-paper.html

Client: [Google Play beta](https://play.google.com/store/apps/details?id=com.bolimot.mindtheclub).

### Signal

Protocol: Signal uses a pre-key handshake to start a session, then the Double Ratchet to derive new keys for each message. Its servers relay ciphertext and help devices find one another.

[Signal](https://signal.org/) has good encryption, but it is centralized, so users have to trust the service (single point of failure -> easy to compromise).

Clients: [official downloads](https://signal.org/download/).

## MLS (secure group messaging)

Protocol: Messaging Layer Security is a tree-based authenticated group key agreement protocol. Members commit membership changes and derive fresh epoch keys, providing forward secrecy and post-compromise security.

**MLS = a standardized group key agreement + encryption protocol**  
(RFC 9420, finalized 2023)

> Secure group messaging with **forward secrecy**, **post-compromise security**, and **efficient membership changes**.

#coding
