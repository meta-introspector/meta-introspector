#!/bin/bash
# Canonical shell wrapper for perf recording
# All shell scripts should use this instead of raw perf record

# Use the nix perf-lib
exec nix run github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix#perf-build -- "$@"
