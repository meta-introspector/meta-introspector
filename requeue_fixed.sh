#!/bin/bash
# Re-queue 17 fixed projects

cd /mnt/data1/meta-introspector

for project in \
  "feature-19-self-source-input" \
  "feature-2-nix-base" \
  "feature-3-home-dir-creds" \
  "feature-5-oauth-creds" \
  "feature-7-telemetry-capture" \
  "feature-11-llm-output-capture" \
  "feature-13-makefile-input" \
  "feature-17-yolo-approval" \
  "log-analysis-pipeline" \
  "001_dump_nix" \
  "eval-context" \
  "gemini-prompt-flake" \
  "main" \
  "test-env-var" \
  "test-secrets-sops"
do
  flake=$(find /mnt/data1/nix/source -name "$project" -type d 2>/dev/null | head -1)
  if [ -n "$flake" ]; then
    echo "Queueing $project"
    ./nix_builder.sh queue "$flake"
  fi
done

echo ""
echo "Queued 15 projects (2 not found: nix, psyche)"
echo "Run: ./nix_builder.sh watch"
