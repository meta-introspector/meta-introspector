#!/usr/bin/env bash
# Build all const 71 tests in background and monitor progress

LOG_DIR="const_71_analysis/build_logs"
mkdir -p "$LOG_DIR"

LANGS=(rust gcc llvm python node ocaml haskell lean4 asm brainfuck)

echo "🚀 Starting background builds for const x=71 across 10 languages"
echo "Logs: $LOG_DIR/"
echo ""

for lang in "${LANGS[@]}"; do
  (
    echo "[$(date +%H:%M:%S)] Building $lang..." > "$LOG_DIR/${lang}.log"
    if nix build ./const_71_test/$lang# --no-link >> "$LOG_DIR/${lang}.log" 2>&1; then
      echo "[$(date +%H:%M:%S)] ✅ SUCCESS" >> "$LOG_DIR/${lang}.log"
    else
      echo "[$(date +%H:%M:%S)] ❌ FAILED" >> "$LOG_DIR/${lang}.log"
    fi
  ) &
done

echo "Monitoring builds (Ctrl+C to stop monitoring, builds continue)..."
sleep 2

# Monitor for max 1 hour
MAX_ITERATIONS=1200  # 1 hour at 3 second intervals
ITERATION=0

while [ $ITERATION -lt $MAX_ITERATIONS ]; do
  clear
  echo "🔨 Const x=71 Build Status - $(date +%H:%M:%S)"
  echo "================================================"
  
  DONE_COUNT=0
  for lang in "${LANGS[@]}"; do
    if [ -f "$LOG_DIR/${lang}.log" ]; then
      status=$(tail -1 "$LOG_DIR/${lang}.log")
      echo "$lang: $status"
      if echo "$status" | grep -q "SUCCESS\|FAILED"; then
        ((DONE_COUNT++))
      fi
    else
      echo "$lang: Waiting..."
    fi
  done
  
  # Check if all done
  if [ $DONE_COUNT -eq ${#LANGS[@]} ]; then
    echo ""
    echo "✅ All builds complete!"
    break
  fi
  
  sleep 3
  ((ITERATION++))
done

if [ $ITERATION -eq $MAX_ITERATIONS ]; then
  echo ""
  echo "⏱️  Timeout after 1 hour - some builds still running"
  echo "Check logs in: $LOG_DIR/"
fi
