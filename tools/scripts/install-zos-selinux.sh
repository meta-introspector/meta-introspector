#!/usr/bin/env bash
set -euo pipefail

echo "🔐 Installing ZOS SELinux policy"

cd "$(dirname "$0")/../selinux"

# Compile policy
checkmodule -M -m -o zos.mod zos.te
semodule_package -o zos.pp -m zos.mod

# Install policy
semodule -i zos.pp

# Label ZOS binaries
semanage fcontext -a -t zos_server_exec_t "/usr/local/bin/zos_server"
semanage fcontext -a -t zos_audited_exec_t "/usr/local/bin/zos-audit-run.sh"

# Label log directory
semanage fcontext -a -t zos_log_t "/var/log/zos(/.*)?"
mkdir -p /var/log/zos
restorecon -Rv /var/log/zos

echo "✅ SELinux policy installed"
echo "Audited processes now enforced by kernel-level MAC"
