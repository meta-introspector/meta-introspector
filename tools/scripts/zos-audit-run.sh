#!/usr/bin/env bash
set -euo pipefail

CGROUP_NAME="${1:-zos-audited}"
COMMAND="${@:2}"

if [ -z "$COMMAND" ]; then
    echo "Usage: $0 [cgroup-name] <command>"
    echo "Example: $0 zos-audited cargo build"
    exit 1
fi

echo "🔍 Running under ZOS audit: $COMMAND"

# Ensure cgroup exists
if [ ! -d "/sys/fs/cgroup/net_cls/$CGROUP_NAME" ]; then
    echo "❌ Cgroup $CGROUP_NAME not found. Run setup-zos-iptables.sh first"
    exit 1
fi

# Run command in audited cgroup
cgexec -g net_cls:$CGROUP_NAME $COMMAND
