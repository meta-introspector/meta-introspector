#!/bin/bash
# Single command to launch dev server with all privileges
# Usage: ./launch-dev-server.sh

echo "🚀 Launching Meta-Introspector Dev Server"
echo "=========================================="
echo ""
echo "This will start the dev server with sudo privileges"
echo "so it can create users, setup keys, and deploy services."
echo ""
echo "Access the GUI at: http://localhost:3000"
echo ""

# Build if needed
if [ ! -f target/release/minimal-build-server ]; then
    echo "Building server..."
    cargo build --release --bin minimal-build-server
fi

# Launch with sudo
sudo PORT=3000 ./target/release/minimal-build-server
