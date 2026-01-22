#!/bin/bash
# Setup iptables to redirect git traffic to local proxy

PROXY_PORT=8128
GIT_PORT=9418

echo "🔧 Setting up git traffic interception..."

# Redirect HTTPS git traffic (port 443) to proxy
sudo iptables -t nat -A OUTPUT -p tcp --dport 443 -m owner ! --uid-owner root -j REDIRECT --to-port $PROXY_PORT

# Redirect git protocol (port 9418)
sudo iptables -t nat -A OUTPUT -p tcp --dport 9418 -m owner ! --uid-owner root -j REDIRECT --to-port $GIT_PORT

# Redirect HTTP (port 80)
sudo iptables -t nat -A OUTPUT -p tcp --dport 80 -m owner ! --uid-owner root -j REDIRECT --to-port $PROXY_PORT

echo "✅ Traffic redirection active"
echo ""
echo "To disable:"
echo "  sudo iptables -t nat -F OUTPUT"
