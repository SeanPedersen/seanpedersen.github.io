---
date: '2026-07-13'
---
# The EU Parliament Just Passed Chat Control

On 9 July 2026, the [European Parliament](https://en.wikipedia.org/wiki/European_Parliament) passed Chat Control 1.0.

The vote was absurd. 314 members voted to reject it. Only 276 voted to keep it. Rejection needed an absolute majority of 360 members, so it failed.

More members voted against Chat Control than for it. Chat Control still passed the Parliament.

This was not one failed proposal revived once. Six consecutive Council presidencies pushed the permanent Chat Control plan. All six failed to secure a majority because of mass surveillance and encryption concerns.

Parliament then rejected the temporary extension in March 2026. Four months later, the same institution let it pass through a rule that ignored the majority present.

This is lawmaking by attrition. Keep rewriting, renaming, and reviving a rejected surveillance system until a procedural loophole carries it. Calling that democratic would be dishonest. It looks like institutional corruption.

## What Just Passed

Chat Control 1.0 is an exception to the [ePrivacy Directive](https://en.wikipedia.org/wiki/EPrivacy_Directive). It lets communication providers scan private messages for child sexual abuse material and grooming.

The scanning is called voluntary because providers may choose whether to do it. It is not voluntary for the person whose messages get scanned. No warrant or individual suspicion is required.

This temporary law first passed in 2021 and expired on 3 April 2026. Parliament voted against extending it in March. The Council brought it back for a second reading in July.

Parliament added one vital restriction. Communications protected by [end-to-end encryption](https://en.wikipedia.org/wiki/End-to-end_encryption) are excluded. The Council must now accept or reject that restriction.

Chat Control 1.0 is separate from Chat Control 2.0. The second proposal would create a permanent system. Most of that law is already agreed, but negotiations continue over detection orders and encrypted communication.

## What It Means for Your Messenger

The encryption label alone tells you little. The important question is whether the provider can read your message on its servers.

### Matrix

The open, decentralized [Matrix protocol](https://en.wikipedia.org/wiki/Matrix_(protocol)) lets users choose a provider or run their own homeserver. Encrypted rooms should fall under Parliament's exception, while homeservers can read and scan unencrypted rooms. Self-hosting does not hide all [metadata](https://en.wikipedia.org/wiki/Metadata), but a network spread across thousands of operators makes mass surveillance harder to impose.

### Signal

[Signal](https://en.wikipedia.org/wiki/Signal) is a centralized messenger that end-to-end encrypts all messages, calls, and files by default, placing their content outside Chat Control 1.0 under Parliament's amendment. Its central control makes it vulnerable to coercion or compromise, although scanning would require inspecting content on the device before encryption—a form of [client-side scanning](https://en.wikipedia.org/wiki/Regulation_to_Prevent_and_Combat_Child_Sexual_Abuse) that Signal has [already rejected](https://signal.org/blog/pdfs/upload-moderation.pdf).

### WhatsApp

[WhatsApp](https://en.wikipedia.org/wiki/WhatsApp) is a centralized messenger that uses the Signal Protocol to encrypt personal messages, so their content should be excluded under Parliament's amendment. Meta controls the clients, accounts, contact discovery, and surrounding metadata, while user reports can expose recent messages. If the encryption exception disappears, that central control could enable scanning on the device before encryption without removing the lock icon.

### Telegram

[Telegram](https://en.wikipedia.org/wiki/Telegram) is a centralized messenger that does not end-to-end encrypt normal private chats, groups, or channels, so it holds the messages and keys needed to scan most user content. Only opt-in Secret Chats between two devices are end-to-end encrypted, and they support neither groups nor device syncing. Calling Telegram encrypted is therefore misleading: for most conversations, Telegram remains in the middle.

## The Encryption Exception Is Not Safe Yet

Parliament's amendment is good, but it is not final. The Council can reject it. The permanent Chat Control 2.0 fight is also still open.

Politicians will keep claiming that they can scan messages while protecting encryption. They cannot. Content must exist as readable text somewhere before it gets encrypted.

Scanning that point turns every phone into a surveillance device. The encryption may remain mathematically intact, but the private communication is gone. The single biggest reason why mass surveillance is always a bad idea: it is the wet dream of any totalitarian regime and has the potential to be abused in unimaginable ways.

Use Signal for simple private messaging. Use encrypted Matrix rooms if you want secure private messaging with an open network and the option to self-host. Treat WhatsApp as encrypted but controlled by Meta (aka the Zuck). Do not treat normal Telegram chats as private.

## References

1. [European Parliament report on its 9 July 2026 Chat Control amendments](https://www.europarl.europa.eu/news/en/press-room/20260706IPR46318/combating-child-sexual-abuse-support-for-a-more-limited-eprivacy-derogation)
2. [Euronews report on how Chat Control passed Parliament](https://www.euronews.com/next/2026/07/10/chat-control-10-passed-the-european-parliament-through-the-back-door)
3. [EDRi report on six Council presidencies failing to secure a Chat Control majority](https://edri.org/our-work/16-countries-burned-polands-bridges-on-the-csa-regulation-what-now/)
4. [European Parliament procedure for the temporary ePrivacy exception](https://oeil.europarl.europa.eu/oeil/en/procedure-file?reference=2025/0429(COD))
5. [Signal technical documentation for encrypted messaging](https://signal.org/docs/)
6. [Matrix FAQ on encryption and decentralized homeservers](https://matrix.org/docs/older/faq/)
7. [WhatsApp privacy policy for users in the European region](https://www.whatsapp.com/legal/privacy-policy-eea)
8. [Telegram FAQ on cloud chats and Secret Chats](https://telegram.org/faq)

#privacy
