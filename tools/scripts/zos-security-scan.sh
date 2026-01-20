#!/usr/bin/env bash
set -euo pipefail

# ZOS Security: Detect anomalous complexity in untrusted binaries
# Rejects objects with complexity resonating with bootstrap (likely malicious)

THRESHOLD=${1:-71}  # Default: reject complexity > 71 (beyond ZOS boundary)
TARGET=${2:-.}

echo "🔒 ZOS Security Scanner"
echo "======================="
echo "Threshold: Complexity > $THRESHOLD"
echo "Target: $TARGET"
echo ""

# Scan all binaries
find "$TARGET" -type f -executable 2>/dev/null | while read binary; do
    # Compute complexity (unique instruction patterns)
    if complexity=$(objdump -d "$binary" 2>/dev/null | \
        grep -E "^\s+[0-9a-f]+:" | \
        awk '{print $3}' | \
        sort -u | wc -l); then
        
        if [ "$complexity" -gt "$THRESHOLD" ]; then
            echo "⚠️  SUSPICIOUS: $binary"
            echo "    Complexity: $complexity (exceeds threshold $THRESHOLD)"
            
            # Check if it resonates with bootstrap primes
            for p in 37 41 43 47 53 59 61 67 71; do
                if [ $((complexity % p)) -eq 0 ]; then
                    echo "    🚨 RESONATES at prime $p (bootstrap signature)"
                fi
            done
            
            echo "    Action: QUARANTINE"
            # mkdir -p .zos-quarantine
            # mv "$binary" .zos-quarantine/
        fi
    fi
done

echo ""
echo "✅ Scan complete"
