#!/usr/bin/env bash
# Test deterministic peer behavior

echo "🧪 Testing deterministic peers..."

# Start first instance
./target/debug/minimal-build-server &
PID1=$!
sleep 2

PEER1=$(curl -s http://127.0.0.1:3000/peer | jq -r '.peer_id')
echo "Peer 1: $PEER1"

# Stop and restart
kill $PID1
sleep 1

./target/debug/minimal-build-server &
PID2=$!
sleep 2

PEER2=$(curl -s http://127.0.0.1:3000/peer | jq -r '.peer_id')
echo "Peer 2: $PEER2"

kill $PID2

if [ "$PEER1" = "$PEER2" ]; then
    echo "✅ Deterministic: Same peer ID on restart"
else
    echo "❌ Non-deterministic: Different peer IDs"
fi
