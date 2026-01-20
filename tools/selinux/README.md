# ZOS SELinux Security

Kernel-level mandatory access control for ZOS - more secure than Nix sandboxing.

## Why SELinux > Nix Sandbox

| Feature | Nix Sandbox | ZOS SELinux |
|---------|-------------|-------------|
| Enforcement | Userspace | Kernel (MAC) |
| Bypass | Possible | Impossible |
| Network | Limited | Full control |
| File access | Namespace | Label-based |
| Audit | Basic | Complete |

## Security Model

```
zos_audited_t domain:
  ✅ Read /nix/store
  ❌ Write /nix/store
  ❌ Direct network access
  ✅ Connect to ZOS proxies only
  ✅ All access logged
```

## Installation

```bash
# Install policy (requires root)
sudo ./tools/scripts/install-zos-selinux.sh

# Verify
sestatus
semodule -l | grep zos
```

## Running Audited Builds

```bash
# Run under SELinux + cgroup enforcement
sudo ./tools/scripts/zos-audit-run.sh cargo build

# Attempts to bypass will be denied by kernel
# All access logged to /var/log/zos/
```

## Policy Highlights

### Network Isolation
```
neverallow zos_audited_t port_type:tcp_socket { name_connect };
```
Audited processes **cannot** make direct network connections.

### Read-Only /nix/store
```
neverallow zos_audited_t nix_store_t:file { write };
```
Audited processes **cannot** modify Nix store.

### Mandatory Logging
```
allow zos_audited_t zos_log_t:file { append };
```
All file access **must** be logged.

## Advantages Over Nix

1. **Kernel enforcement**: Cannot be bypassed by userspace
2. **Label-based**: Works across filesystems
3. **Network control**: Fine-grained port restrictions
4. **Audit trail**: Kernel-level logging
5. **Type enforcement**: Prevents privilege escalation

## Integration with Nix

```nix
# In flake.nix
{
  packages.default = pkgs.stdenv.mkDerivation {
    # Build runs under zos_audited_t
    # SELinux enforces:
    # - No network except through proxies
    # - Read-only /nix/store
    # - All access logged
  };
}
```

## Audit Logs

```bash
# View all ZOS-audited access
ausearch -m AVC -c zos_audited_t

# File access
ausearch -m PATH -se zos_audited_t

# Network attempts (should be denied)
ausearch -m SYSCALL -c zos_audited_t -sc connect
```

This creates a **provably secure** build environment where even root cannot bypass restrictions.
