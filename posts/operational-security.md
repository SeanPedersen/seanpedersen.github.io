---
title: 'Operational Security'
date: '2025-11-10'
---
How to secure a VPS running in production (important service + data)?
- Use ONLY public key based AND disable password based auth (for SSH) + use non-default port
- Disable root account, disable root login via ssh -> only use user accounts with sudo
- Setup firewall (lockdown all unused ports - main: 80, 443)
- Setup fail2ban (ban IP's failing ssh login attempts)
- Use docker for your services
- Set up regular automatic updates
- Set up append only backups (whole server or DB) with regular validity tests (restore the backup)
- Advanced: Set up disk level encryption (f.e. LUKS) - in case the hard drives will be resold: customer data can not be recovered

#programming
