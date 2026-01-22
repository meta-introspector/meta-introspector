#!/bin/bash
# Test Gemini with impure flag
nix build \
  /mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/09/27/7-concepts/6-qa-testing/tests/consolidated-impure-gemini-telemetry#default \
  --impure \
  --show-trace \
  2>&1 | head -30
