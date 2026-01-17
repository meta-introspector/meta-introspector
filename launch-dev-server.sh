#!/bin/bash
# Single command to launch dev server with all privileges
# Usage: ./launch-dev-server.sh

echo "🚀 Launching Meta-Introspector Dev Server"
echo "=========================================="
echo ""

# Kill old server if running on port 3000
echo "Checking for existing server on port 3000..."
OLD_PID=$(sudo lsof -ti:3000 -sTCP:LISTEN 2>/dev/null)
if [ -n "$OLD_PID" ]; then
    OLD_NAME=$(ps -p $OLD_PID -o comm= 2>/dev/null)
    if [[ "$OLD_NAME" == *"minimal-build"* ]] || [[ "$OLD_NAME" == *"minimal_build"* ]]; then
        echo "Killing old minimal-build-server (PID: $OLD_PID)..."
        sudo kill -9 $OLD_PID 2>/dev/null
        sleep 2
        echo "✓ Old server stopped"
    else
        echo "⚠ Port 3000 in use by: $OLD_NAME (PID: $OLD_PID)"
        echo "Kill it manually or use a different port"
        exit 1
    fi
else
    echo "✓ Port 3000 is free"
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
