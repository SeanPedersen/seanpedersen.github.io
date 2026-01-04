---

title: 'Digital Security for Everyone'  
date: '2025-05-15'

---

The internet is a war zone and you are an active participant if you like it or not - so learn to defend yourself!

## Essentials

### Password Management

Use a password manager ([KeePass](https://keepassxc.org/) or other solutions) to create and store unique, complex (random) passwords for each service / web size. Advanced: manually add another PW (that you only remember) to every stored PW when logging in - this nullifys the risk of your PW manager leaking all your stored passwords.

Enable Multi-Factor Authentication (MFA) wherever available, as it drastically reduces account takeover risk. Check if accounts including passwords of yours have been breached using [haveibeenpwned.com](https://haveibeenpwned.com/) and replace these passwords using your password manager.

### Keep Everything Updated

Those annoying update notifications? They're important. Software updates patch security vulnerabilities that hackers actively exploit. Set your devices to update automatically when possible.

### App Installation Discipline

Only install apps from official sources (App Store, Google Play, Microsoft Store). Third-party app stores and direct downloads significantly increase your risk of malware infection.

### Backup Strategy

Phones and laptops get lost, stolen, or broken more often than we would like to admit. Protect your important files by keeping three copies: one on your device, one on an external hard drive at home and one in cloud storage. This way even if disaster strikes you will still have your personal files safe and sound.

### Ad Blocking

Install [uBlock Origin](https://ublockorigin.com/) on your internet browsers. Beyond blocking annoying ads, it prevents many malicious scripts and tracking mechanisms from running on visited websites.

### Biometric Authentication Limits

Biometric authentication may be convenient for everyday use but offers limited security. Biometrics like face detection and fingerprint scans can never be changed and can be fooled by attackers accessing photos of your face or your fingerprints without your consent. Some countries have laws allowing to use force to unlock devices with biometric authentication but can not force you to enter a PIN or password.

## Security Mindset

### Phishing Awareness

Be skeptical of unexpected emails and calls: do not blindly follow instructions from unverified agents. Verify links before clicking: ensure legitimacy of domains (google.com, not go0gle.com). When in doubt, access websites directly via bookmarks or trusted search engines rather than through dubious links from f.e. phishing emails.

### Social Media Privacy

Remember that anything shared online can potentially be weaponized. Stalkers can use background details in photos to determine your location (a short window view can reveal your exact location), while hackers can gather your personal information for impersonation attacks. Set your accounts visibility to private and always be mindful and cautious about what you share.

### [Remove Data Kraken like Google and co](https://seanpedersen.github.io/posts/google-ejector)

## Advanced

### Device Encryption

Encrypt your devices to protect data if they're lost or stolen. Most smartphones are encrypted by default when you use a passcode, while computers may require setting up encryption (BitLocker on Windows, FileVault on Mac).

### Tor Browser (darknet)

Tor Browser provides anonymity by routing traffic through multiple encrypted relays. For secure use: keep it updated, avoid maximizing the window (to prevent fingerprinting), don't install additional extensions and **never log into personal accounts or share identifying information while using it**. Use HTTPS sites when possible and remember that Tor protects your connection but not the endpoints, so avoid downloading suspicious files or entering sensitive data on untrusted sites.

Download it here: <https://www.torproject.org/download/>

## Don't Waste Your Money

on digital snake-oil like...

### Anti-Virus Software

Modern operating systems have built-in protection that performs as well as or better than paid alternatives. Windows Defender, macOS security features, and mobile OS protections are mostly sufficient. Anti-virus software often slows down your computer signficantly and may introduce additional security holes. Stay away from it.

### VPN Services

For typical browsing on HTTPS-secured websites (which is most of the internet now), paid VPN services add minimal security benefit. The privacy claims in their marketing are largely exaggerated for everyday use. The core question is: do you trust your ISP less than your VPN provider? Check [this article](https://digdeeper.love/articles/vpn.xhtml) out for more info.

## Conclusion

The best digital security doesn't require expensive subscriptions or complicated setups - just consistent application of these fundamentals and developing a security aware mindset in general. How deep you want to go down the security rabbit hole should also depend on your threat model: how likely is it that you will be targeted by hackers (e.g. for profit or for political reasons).

## References

*   [https://github.com/Lissy93/personal-security-checklist/blob/HEAD/CHECKLIST.md](https://github.com/Lissy93/personal-security-checklist/blob/HEAD/CHECKLIST.md)
*   [https://arstechnica.com/tech-policy/2024/04/cops-can-force-suspect-to-unlock-phone-with-thumbprint-us-court-rules/](https://arstechnica.com/tech-policy/2024/04/cops-can-force-suspect-to-unlock-phone-with-thumbprint-us-court-rules/)
*   [https://www.datenschutz.org/bgh-urteil-polizei-darf-unter-zwang-handy-per-fingerabdruck-entsperren/](https://www.datenschutz.org/bgh-urteil-polizei-darf-unter-zwang-handy-per-fingerabdruck-entsperren/)

#privacy #tutorial