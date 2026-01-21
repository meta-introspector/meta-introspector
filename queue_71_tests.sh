#!/bin/bash
# Queue all 71 language tests for building

QUEUE_FILE="$HOME/.local/share/nix-builder/queue.txt"

echo "📋 Queueing all 71 language tests..."

# Clear old queue
> "$QUEUE_FILE"

# Add all tests
for dir in const_71_test/*/; do
    echo "$(pwd)/$dir" >> "$QUEUE_FILE"
done

echo "✅ Queued $(wc -l < $QUEUE_FILE) tests"
cat "$QUEUE_FILE"
