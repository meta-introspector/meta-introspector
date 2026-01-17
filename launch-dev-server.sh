#!/bin/bash
# Single command to launch dev server with all privileges
# Usage: ./launch-dev-server.sh

echo "🚀 Launching Meta-Introspector Dev Server"
echo "=========================================="
echo ""

# Kill old server if running
OLD_PID=$(lsof -ti:3000 -sTCP:LISTEN 2>/dev/null)
if [ -n "$OLD_PID" ]; then
    OLD_NAME=$(ps -p $OLD_PID -o comm= 2>/dev/null)
    if [[ "$OLD_NAME" == *"minimal-build-server"* ]]; then
        echo "Stopping old server (PID: $OLD_PID)..."
        sudo kill $OLD_PID 2>/dev/null || kill $OLD_PID 2>/dev/null
        sleep 1
    fi
fi

# Build if needed
if [ ! -f target/release/minimal-build-server ]; then
    echo "Building server..."
    cargo build --release --bin minimal-build-server
fi

echo "Starting server on port 3000..."
echo "Access GUI at: http://localhost:3000"
echo ""

# Launch with sudo
sudo PORT=3000 ./target/release/minimal-build-server
