---
date: '2025-11-10'
icon: "/images/icons/operational-security.svg"
---
# Operational Security

How to secure a VPS running in production (important service + data)?
- Use ONLY public key based AND disable password based auth (for SSH) + use non-default port
- Disable root account, disable root login via ssh -> only use user accounts with sudo
- Setup firewall (lockdown all unused ports - main: 80, 443)
- Setup fail2ban (ban IP's failing ssh login attempts)
- Use docker for your services
- Setup regular automatic updates
- Setup append only backups (whole server or DB) with regular validity tests (restore the backup)
- Setup notification (via E-Mail) critical events: high disk or RAM usage, unusual network traffic
- Advanced: Set up disk level encryption (f.e. LUKS) - in case the hard drives will be resold: customer data can not be recovered

## Limit Disk Usage

Limit system log accumulation: ```journalctl --vacuum-size=200M```

Limit docker dead containers / volumes / etc:

```bash
cat >/etc/systemd/system/docker-prune.service <<'EOF'
[Unit]
Description=Prune unused Docker data

[Service]
Type=oneshot
ExecStart=/usr/bin/docker system prune -af --volumes
EOF
```

```bash
cat >/etc/systemd/system/docker-prune.timer <<'EOF'
[Unit]
Description=Weekly Docker cleanup

[Timer]
OnCalendar=weekly
Persistent=true

[Install]
WantedBy=timers.target
EOF
```

Enable service:
```bash
systemctl daemon-reexec
systemctl enable --now docker-prune.timer
```

## References

- <https://x.com/levelsio/status/1957526292045393976>
- <https://x.com/levelsio/status/1953025356585169372>
- [Setup script for Ubuntu Server](https://github.com/AnswerDotAI/fastsetup/blob/master/ubuntu-initial.sh)
- <https://www.kkyri.com/p/how-to-secure-your-new-vps-a-step-by-step-guide>

#coding
