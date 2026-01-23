#!/usr/bin/env bash
set -euo pipefail

# Test Gemini impure build from GitHub
# Logs verbose output for documentation

TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_DIR="/mnt/data1/meta-introspector/logs/gemini-tests"
LOG_FILE="$LOG_DIR/gemini-github-test-$TIMESTAMP.log"

mkdir -p "$LOG_DIR"

echo "=== Gemini GitHub Impure Build Test ===" | tee "$LOG_FILE"
echo "Timestamp: $(date -Iseconds)" | tee -a "$LOG_FILE"
echo "Log file: $LOG_FILE" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

cd /tmp

echo "Building from GitHub with verbose output..." | tee -a "$LOG_FILE"
echo "Command: nix build github:meta-introspector/meta-introspector/feature/CRQ-002-zos-ai-ticket#gemini-github-test --impure -vvv" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

nix build \
  github:meta-introspector/meta-introspector/feature/CRQ-002-zos-ai-ticket#gemini-github-test \
  --impure \
  --extra-experimental-features "impure-derivations" \
  -vvv \
  2>&1 | tee -a "$LOG_FILE"

EXIT_CODE=${PIPESTATUS[0]}

echo "" | tee -a "$LOG_FILE"
echo "=== Build Complete ===" | tee -a "$LOG_FILE"
echo "Exit code: $EXIT_CODE" | tee -a "$LOG_FILE"

if [ $EXIT_CODE -eq 0 ]; then
  echo "✅ Build succeeded" | tee -a "$LOG_FILE"
  if [ -L result ]; then
    echo "Result: $(readlink result)" | tee -a "$LOG_FILE"
    cat result/result.txt | tee -a "$LOG_FILE"
  fi
else
  echo "❌ Build failed" | tee -a "$LOG_FILE"
fi

echo "" | tee -a "$LOG_FILE"
echo "Full log saved to: $LOG_FILE"

exit $EXIT_CODE
