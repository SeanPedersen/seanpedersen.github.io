---
date: '2026-07-13'
---
# The EU Parliament Just Passed Chat Control

On 9 July 2026, the [European Parliament](https://en.wikipedia.org/wiki/European_Parliament) let Chat Control 1.0 pass its second reading (enabling mass chat message surveillance legally in the EU).

The vote showed a serious flaw in the process. In all, 314 members voted to reject the Council's text. Only 276 voted against rejection. Yet rejection needed an absolute majority of 360 members, so the motion failed.

More members voted to reject the text than to keep it. The text still passed this stage.

This was not the first attempt. Six Council presidencies tried to pass the permanent Chat Control plan. Each failed to win enough support amid concerns about mass surveillance and encryption.

Parliament then rejected an extension of the temporary law in March 2026. Four months later, the Council sent the text back for a second reading. A rule that required 360 votes for rejection let it pass despite the vote.

EU lawmakers kept changing and resubmitting a surveillance plan that lacked enough support. The process followed the rules, but the result did not match the choice of most members who voted.

## What Just Passed

Chat Control 1.0 creates an exception to the [ePrivacy Directive](https://en.wikipedia.org/wiki/EPrivacy_Directive). It lets communication providers scan private messages for child sexual abuse material and grooming.

The law calls the scanning voluntary because each provider may choose whether to scan. The person whose messages it scans gets no choice. The provider needs neither a warrant nor a reason to suspect that person.

The temporary law first took effect in 2021 and expired on 3 April 2026. Parliament rejected an extension in March. The Council sent it back to Parliament for a second reading in July.

Parliament added one main limit. Its amendment excludes messages that use [end-to-end encryption](https://en.wikipedia.org/wiki/End-to-end_encryption). The Council must now accept or reject that limit.

Chat Control 1.0 differs from Chat Control 2.0. The second plan would set up a permanent system. Parliament and the Council have agreed on most of that law. They still disagree about detection orders and encrypted messages.

## What It Means for Your Messenger

An encryption label tells you little by itself. Ask whether the provider can read your message on its servers.

### Matrix

The open, decentralized [Matrix protocol](https://en.wikipedia.org/wiki/Matrix_(protocol)) lets you choose a provider or run your own homeserver. Parliament's limit should cover encrypted rooms. Homeservers can still read and scan rooms without encryption.

Self-hosting does not hide all [metadata](https://en.wikipedia.org/wiki/Metadata). Yet thousands of separate operators make one central scanning order harder to carry out across the whole network.

### Signal

[Signal](https://en.wikipedia.org/wiki/Signal) is a centralized messenger. It uses end-to-end encryption for all messages, calls, and files by default. Parliament's amendment places that content outside Chat Control 1.0.

Signal still controls the service. A government could pressure it, or an attacker could breach it. But content scanning would have to happen on the device before encryption. Experts call this method [client-side scanning](https://en.wikipedia.org/wiki/Regulation_to_Prevent_and_Combat_Child_Sexual_Abuse), and Signal has [rejected it](https://signal.org/blog/pdfs/upload-moderation.pdf).

### WhatsApp

[WhatsApp](https://en.wikipedia.org/wiki/WhatsApp) is a centralized messenger. It uses the Signal Protocol to encrypt personal messages. Chat Control 1.0 should therefore not cover their content.

Meta controls the apps, accounts, contact discovery, and related metadata. User reports can also reveal recent messages. If lawmakers remove the encryption limit, Meta could add scanning before encryption while still showing the lock icon.

### Telegram

[Telegram](https://en.wikipedia.org/wiki/Telegram) is a centralized messenger. It does not use end-to-end encryption for standard private chats, groups, or channels. Telegram therefore holds the messages and keys needed to scan most user content.

Only opt-in Secret Chats between two devices use end-to-end encryption. They do not support groups or device syncing. For most chats, Telegram can read the content. The claim that Telegram encrypts all chats is false.

## The Encryption Limit Is Not Final

Parliament's amendment protects encrypted messages, but the Council can still reject it. Talks on the permanent Chat Control 2.0 plan also continue.

Some politicians claim that providers can scan messages and still protect encryption. They cannot protect the message content from the scanner. A phone must hold readable content before it encrypts and sends that content.

Client-side scanning checks the message at that point. The encryption may still work as designed, but the scanner can read the message. A government can abuse that access, and a future government can use it for a new purpose.

Use encrypted Matrix rooms if you also want an open network and the option to host it yourself. Use Signal for simple private messages. WhatsApp encrypts personal messages, but Meta (aka the Zuck) controls the service. Do not treat standard Telegram chats as private.

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
