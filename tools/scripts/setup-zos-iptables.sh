#!/usr/bin/env bash
set -euo pipefail

CGROUP_NAME="${1:-zos-audited}"

echo "🔒 ZOS: Setting up iptables redirection for cgroup: $CGROUP_NAME"

# Create cgroup for audited processes
mkdir -p /sys/fs/cgroup/net_cls/$CGROUP_NAME
echo 0x00100001 > /sys/fs/cgroup/net_cls/$CGROUP_NAME/net_cls.classid

# Mark packets from this cgroup
iptables -t mangle -A OUTPUT -m cgroup --cgroup 0x00100001 -j MARK --set-mark 1

# Redirect only marked packets to ZOS proxies
iptables -t nat -A OUTPUT -m mark --mark 1 -p udp --dport 53 -j REDIRECT --to-port 5353
iptables -t nat -A OUTPUT -m mark --mark 1 -p tcp --dport 80 -j REDIRECT --to-port 8080
iptables -t nat -A OUTPUT -m mark --mark 1 -p tcp --dport 9418 -j REDIRECT --to-port 9418
iptables -t nat -A OUTPUT -m mark --mark 1 -p tcp --dport 443 -j REDIRECT --to-port 5000
iptables -t nat -A OUTPUT -m mark --mark 1 -p tcp --dport 11434 -j REDIRECT --to-port 11435

echo "✅ iptables rules applied for cgroup: $CGROUP_NAME"
echo "Run processes under audit: cgexec -g net_cls:$CGROUP_NAME <command>"
