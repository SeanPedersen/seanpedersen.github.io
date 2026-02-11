---
icon: "/images/icons/linux.webp"
---
# Linux Desktop

My choice: [CachyOS](https://cachyos.org/) ([Arch Linux](https://archlinux.org/)) with GNOME DE
- CachyOS is tweaked for performance, has a GUI installer + easy disk encryption setup
- GNOME looks shiny and can easily be stripped down to avoid bloat

## Awesome Software

File Manager:
- [Thunar](https://wiki.archlinux.org/title/Thunar): very lightweight 50MB RAM
- [Dolphin](https://apps.kde.org/dolphin/): best feature set
	- supports showing dir size bytes
	- supports sftp (ssh remote access)
- PCManFM

Code Editor:
- [Zed](https://zed.dev/): fast + lightweight (similar to VS Code)
- [MarkText](https://github.com/marktext/marktext): WYSIWYG markdown editor

Media Player:
- VLC
- mpv

PW Manager:
- KeePassXC

Terminal:
- [ghostty](https://ghostty.org/): fast
- guake (drop down terminal)

Shell:
- ZSH + [ZEAL](https://github.com/SeanPedersen/zeal)

Process Monitor:
- NeoHtop / Neop (my fork)
- [Mission Center](https://missioncenter.io/)
- [Glances](https://github.com/nicolargo/glances)

Useful Daemons:
- [Syncthing](https://syncthing.net/)
- [IPFS](/posts/ipfs)

Tweaks:
- [ananicy-cpp](https://gitlab.com/ananicy-cpp/ananicy-cpp): prevents system slow down on high disk usage (CachyOS ships with it)

## Honorable Mentions

Distros:
- [Fedora](https://www.fedoraproject.org/workstation/)
- Debian
- Ubuntu Server

Desktop Envs:
- KDE Plasma
- HyprLand
- XFCE

## Dishonorable Mentions

Many GNOME apps are RAM hogs while providing little functionality, I recommend to uninstall: nautilus, gnome-software, gnome-contacts, gnome-terminal
Avoid oh-my-zsh it is bloated and slow.
Budgie DE is a buggy shit show.

## TO TRY

- XFCE: promising low resource DE (though no wayland support yet)
- NixOS

#coding
