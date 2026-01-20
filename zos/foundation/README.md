# ZOS Foundation - Layered Services

ZOS runs as two systemd services: root layer (L1) and user layer (L2).

## Architecture

```
systemd (PID 1)
  ↓
├── zos-root.service (root, L1)
│     ├── SELinux enforcement
│     ├── iptables setup
│     └── cgroup management
│
└── zos-user.service (zos user, L2)
      ├── DNS proxy (5353)
      ├── File proxy (8080)
      ├── GitHub proxy (9418)
      ├── Nix proxy (5000)
      └── LLM proxy (11435)
```

## Installation

```bash
# Build
nix build .#zos-system

# Install services
sudo cp zos-root.service /etc/systemd/system/
sudo cp zos-user.service /etc/systemd/system/

# Create zos user
sudo useradd -r -s /bin/nologin zos

# Enable and start
sudo systemctl daemon-reload
sudo systemctl enable zos-root zos-user
sudo systemctl start zos-root zos-user
```

## Layer Responsibilities

### Root Layer (L1)
- **User**: root
- **SELinux**: zos_level1_t
- **Capabilities**: CAP_NET_BIND_SERVICE, CAP_NET_ADMIN
- **Tasks**:
  - Setup iptables rules
  - Create cgroups
  - Load SELinux policy
  - Manage privileged ports

### User Layer (L2)
- **User**: zos (unprivileged)
- **SELinux**: zos_level2_t
- **Restrictions**: NoNewPrivileges, PrivateTmp
- **Tasks**:
  - Run proxy services
  - Handle user requests
  - Log all traffic

## Status

```bash
# Check status
sudo systemctl status zos-root
sudo systemctl status zos-user

# View logs
journalctl -u zos-root -f
journalctl -u zos-user -f
```

## Security

- Root layer has minimal privileges (only network)
- User layer runs unprivileged
- SELinux enforces layer separation
- Cannot escalate between layers

## Future: VM Deployment

For PID 1 replacement:
```bash
# Build VM image
nix build .#zos-vm

# Run in QEMU
qemu-system-x86_64 -kernel result/kernel -initrd result/initrd
```

This keeps ZOS isolated from host systemd while maintaining full functionality.
