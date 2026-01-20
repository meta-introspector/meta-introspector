#!/usr/bin/env bash
set -euo pipefail

echo "🔓 ZOS: Removing iptables redirection"

# Remove all ZOS redirects
iptables -t nat -D OUTPUT -p udp --dport 53 -j REDIRECT --to-port 5353 2>/dev/null || true
iptables -t nat -D OUTPUT -p tcp --dport 80 -j REDIRECT --to-port 8080 2>/dev/null || true
iptables -t nat -D OUTPUT -p tcp --dport 9418 -j REDIRECT --to-port 9418 2>/dev/null || true
iptables -t nat -D OUTPUT -p tcp --dport 443 -d cache.nixos.org -j REDIRECT --to-port 5000 2>/dev/null || true
iptables -t nat -D OUTPUT -p tcp --dport 11434 -j REDIRECT --to-port 11435 2>/dev/null || true

echo "✅ iptables rules removed"
