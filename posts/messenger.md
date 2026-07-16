# Messenger
Humanity is in need of a fun to use, secure and decentral messenger.

- User identity can be tied to email (for account recovery)
- Decentralized (device to device works) + federated (optional servers help make the network go fast)
- E2EE message content (msg, sticker, etc.) + metadata (who msgs whom)
- Connect (share keys) by scanning QR Code - “Invite link” fallback
- Thick client (user keeps message history - default is 2 months) + thin server (just relay messages)
- Use Rust / Erlang for server
- Default: use internet, fallback: Wi-Fi Mesh; Bluetooth (like bitchat)

Chat Features (Telegram has best chat UX):
- group chats
- file transfer
- stickers

Account:
- public key
- username
- verified contacts

Matrix drawbacks:
- Olm/Megolm does not offer forward secrecy for group messaging
- Olm/Megolm does ensure end-to-end encryption for message data, but not for metadata.
- Federation makes it challenging to be GDPR compliant
- Synapse is very heavy, other implementations are less production ready
- For better or worse, the matrix foundation is under UK jurisdiction.

## Existing Chat Shitshow

Whatsapp and Telegram are security nightmares. WA being centralized and has no metadata encryption. Telegram is centralized and has no encryption by default.

- [Signal](https://signal.org/) has best encryption but is centralized -> trust me bro mentality.
- [Matrix](https://matrix.org/) has horrible UX and no metadata encryption.
- Berty chat based on IPFS https://berty.tech/messenger/
- [Bitchat](https://bitchat.free/): P2P bluetooth
- [Jami](https://git.jami.net/savoirfairelinux): P2P, https://jami.net/
- https://www.mindtheclub.com/white-paper.html

## MLS (secure group messaging)

**MLS = a standardized group key agreement + encryption protocol**  
(RFC 9420, finalized 2023)

> Secure group messaging with **forward secrecy**, **post-compromise security**, and **efficient membership changes**.

#coding
