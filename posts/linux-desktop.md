---
icon: "/images/icons/linux.webp"
---
# Linux Desktop
My choice: [CachyOS](https://cachyos.org/) ([Arch Linux](https://archlinux.org/)) with [GNOME](https://www.gnome.org/) DE

- CachyOS is tweaked for performance, has a GUI installer + easy disk encryption setup
- GNOME looks shiny and can easily be stripped down to avoid bloat

## Awesome Software
File Manager:
- [Thunar](https://wiki.archlinux.org/title/Thunar): very lightweight 50MB RAM
- [Dolphin](https://apps.kde.org/dolphin/): best feature set
  - supports showing dir size bytes
  - supports sftp (ssh remote access)
- [PCManFM](https://wiki.lxde.org/en/PCManFM)

Code Editor:
- [Zed](https://zed.dev/): fast + lightweight (similar to VS Code)
- [Marko](https://github.com/SeanPedersen/Marko): WYSIWYG markdown editor

Media Player:
- [VLC](https://www.videolan.org/vlc/)
- [mpv](https://mpv.io/)

PW Manager:
- [KeePassXC](https://keepassxc.org/)

Terminal:
- [ghostty](https://ghostty.org/): fast
- [guake](https://github.com/Guake/guake) (drop down terminal)

Shell:
- [ZSH](https://www.zsh.org/) + [ZEAL](https://github.com/SeanPedersen/zeal)

Process Monitor:
- [NeoHtop](https://github.com/Abdenasser/neohtop) / [Neop](https://github.com/SeanPedersen/neop) (my fork)
- [Mission Center](https://missioncenter.io/)
- [Glances](https://github.com/nicolargo/glances)

Useful Daemons:
- [Syncthing](https://syncthing.net/)
- [IPFS](/posts/ipfs)

## Honorable Mentions

Distros:
- [Fedora](https://www.fedoraproject.org/workstation/)
  - [Nobara](https://nobaraproject.org/) (optimized for gaming / streaming)
- [Debian](https://www.debian.org/) (solid)
- [Ubuntu Server](https://ubuntu.com/server) (solid)

Desktop Envs:
- [KDE Plasma](https://kde.org/plasma-desktop/)
- [Hyprland](https://hypr.land/)
- [XFCE](https://www.xfce.org/)

## Dishonorable Mentions
Many GNOME apps are RAM hogs while providing little functionality, I recommend to uninstall: nautilus, gnome-software, gnome-contacts, gnome-terminal
Avoid [oh-my-zsh](https://ohmyz.sh/) it is bloated and slow.
[Budgie](https://buddiesofbudgie.org/) DE is a buggy shit show.

## TO TRY
- [XFCE](https://www.xfce.org/): promising low resource DE (though no wayland support yet)
- [NixOS](https://nixos.org/)
- [Omarchy](https://omarchy.org/): new polished linux distro

Low RAM macOS config (<600MB): [Arch](https://archlinux.org/) → [Labwc](https://labwc.github.io/) → [lxqt-panel](https://github.com/lxqt/lxqt-panel) → [PCManFM-Qt](https://github.com/lxqt/pcmanfm-qt) → [mako](https://github.com/emersion/mako) → [swaybg](https://github.com/swaywm/swaybg) → dock → macOS theme.

## Tweaks & more
- [ananicy-cpp](https://gitlab.com/ananicy-cpp/ananicy-cpp): prevents system slow down on high disk usage (CachyOS ships with it)
- [OpenZFS](https://openzfs.org/wiki/Main_Page): durable file system with useful features
- Limit [systemd-coredump](https://www.freedesktop.org/software/systemd/man/latest/coredump.conf.html) via `sudoedit /etc/systemd/coredump.conf`

#coding
