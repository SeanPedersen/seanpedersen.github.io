# Messenger
Humanity is in need of a fun to use, secure, privacy-respecting and decentral messenger.

- User account via username (no phone number needed)
- Decentralized (device to device works) + federated (optional servers help make the network go fast)
- E2EE message content (msg, sticker, etc.) + metadata (who msgs whom)
- Connect (share keys) by scanning QR Code - “Invite link” fallback
- Thick client (user keeps message history - default is 2 months) + thin server (just relay messages)
- Default: use internet, fallback: Wi-Fi Mesh; Bluetooth (like bitchat)

Chat Features (Telegram has best chat UX):
- group chats
- file transfer
- stickers

## Existing Chat Shitshow

Whatsapp and Telegram are security nightmares. WA being centralized and has no metadata encryption. Telegram is centralized and has no encryption by default.

| Messenger | Network | Clients / downloads |
| --- | --- | --- |
| [Jami](https://jami.net/) | P2P | [Official downloads](https://jami.net/download/) |
| [Berty](https://berty.tech/messenger/) | P2P, IPFS | [iOS App Store (legacy)](https://apps.apple.com/ie/app/berty-messenger/id1535500412), [official status](https://berty.tech/features) |
| [Bitchat](https://bitchat.free/) | Bluetooth mesh | [iOS](https://apps.apple.com/us/app/bitchat-mesh/id6748219622), [Android](https://play.google.com/store/apps/details?id=com.bitchat.droid), [Android releases](https://github.com/permissionlesstech/bitchat-android/releases) |
| [Matrix](https://matrix.org/) | Federated | [Official client list](https://matrix.org/ecosystem/clients/) |
| [Mindtheclub](https://www.mindtheclub.com/) | P2P, WebRTC | [Google Play beta](https://play.google.com/store/apps/details?id=com.bolimot.mindtheclub) |
| [Signal](https://signal.org/) | Centralized | [Official downloads](https://signal.org/download/) |

### Jami

[Jami](https://git.jami.net/savoirfairelinux) is a P2P messenger. The project website is https://jami.net/.

Jami actually seems like it has done many things right - only downside is the design of its clients (which can be fixed).

Clients: [official downloads](https://jami.net/download/). Repo: [jami-client-qt](https://git.jami.net/savoirfairelinux/jami-client-qt)

### Berty

Berty is a chat app based on IPFS: https://berty.tech/messenger/

Clients: [iOS App Store (legacy)](https://apps.apple.com/ie/app/berty-messenger/id1535500412). The Android app is currently unavailable. See the [official status](https://berty.tech/features).

### Bitchat

[Bitchat](https://bitchat.free/) is a P2P messenger that uses Bluetooth.

Clients: [iOS](https://apps.apple.com/us/app/bitchat-mesh/id6748219622), [Android](https://play.google.com/store/apps/details?id=com.bitchat.droid), and [Android releases](https://github.com/permissionlesstech/bitchat-android/releases).

### Matrix

[Matrix](https://matrix.org/) is federated (not P2P) and has horrible UX and no metadata encryption.

Clients: [official client list](https://matrix.org/ecosystem/clients/).

Matrix drawbacks:
- Olm/Megolm does not offer forward secrecy for group messaging
- Olm/Megolm does ensure end-to-end encryption for message data, but not for metadata.
- Federation makes it challenging to be GDPR compliant
- Synapse is very heavy, other implementations are less production ready
- For better or worse, the matrix foundation is under UK jurisdiction.

### Mindtheclub

https://www.mindtheclub.com/white-paper.html

Client: [Google Play beta](https://play.google.com/store/apps/details?id=com.bolimot.mindtheclub).

### Signal

[Signal](https://signal.org/) has good encryption, but it is centralized, so users have to trust the service (single point of failure -> easy to compromise).

Clients: [official downloads](https://signal.org/download/).

## MLS (secure group messaging)

**MLS = a standardized group key agreement + encryption protocol**  
(RFC 9420, finalized 2023)

> Secure group messaging with **forward secrecy**, **post-compromise security**, and **efficient membership changes**.

#coding
