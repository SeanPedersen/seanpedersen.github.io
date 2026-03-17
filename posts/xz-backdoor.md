# The XZ Backdoor: A Near-Catastrophic Linux Hack

A sophisticated 2024 supply-chain attack nearly gave an unknown hacker (group) access to millions of internet servers.

**The Setup**
Linux runs the world: servers, supercomputers, Android, nuclear submarines. Its security model relies on open source code being widely scrutinized ("Linus's Law"). But the ecosystem depends on thousands of tiny projects, often maintained by a single unpaid volunteer.

**The Target**
[Lasse Collin](https://github.com/Larhzu), a Finnish developer, had maintained [XZ](https://github.com/tukaani-project/xz) (a widely-used lossless compression tool) alone for 20 years. Burned out and struggling with mental health, he was vulnerable to offers of help.

**The Attack**
A persona called [Jia Tan](https://github.com/JiaT75) spent ~2.5 years:
1. Building trust as a helpful contributor to XZ
2. Taking over as maintainer
3. Secretly embedding a backdoor inside binary test blobs (invisible in normal code review)

The backdoor exploited a precise chain: XZ → dependency of OpenSSH → hijacking the RSA authentication step via IFUNC resolvers and dynamic audit hooks, creating a **master key to any server running the infected version**.

**The Near-Miss**
Microsoft engineer [Andres Freund](https://x.com/AndresFreundTec) noticed a tiny ~400ms SSH slowdown while testing an unstable Debian release - not even looking for security bugs. He kept investigating and discovered the backdoor in March 2024, just weeks before it would have shipped in major enterprise Linux releases (RHEL, Fedora, Ubuntu).

**Who Was Jia Tan?**
Almost certainly a **nation-state actor** - the operation was too patient and expensive for criminals. Timestamping and other breadcrumbs are no hard evidence, thus no one knows for certain. Jia Tan vanished immediately after discovery.

**The Bigger Lesson**
The real vulnerability wasn't the code - it was *the structure of the project*. Critical infrastructure was resting on one exhausted, unpaid volunteer. This was not the failure of the maintainer but a structural issue of how we build critical open-source software.

To future proof open-source software, we need to identify other high-impact software projects with similar vulnerable structure (few maintainers) and pour more resources into them. And also increase automated security tests for all dependencies of high-impact projects like OpenSSH.

Similar future attacks maybe even more sophisticated by taking legitimate control of criticial open-source software for years - building up trust by years of reliable and helpful work - and when needed dropping a malicious version. A more resilient structure for critical open-source projects would be decentral and select a group of verifiers randomly from a group.

## References
- [Veritasium YouTube Video](https://www.youtube.com/watch?v=aoag03mSuXQ)

#coding
