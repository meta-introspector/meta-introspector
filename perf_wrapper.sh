# Use: nix run ./perf-recorder#perf-build -- .#target
# See: docs/perf/README.md for canonical patterns

#!/bin/bash
# Perf wrapper that records actual command execution

PERF_OUTPUT="${PERF_OUTPUT:-/tmp/perf_wrapper_$$.data}"
REAL_CMD="$1"
shift

# Record the actual command with perf
perf record -o "$PERF_OUTPUT" -F 99 -g "$REAL_CMD" "$@"
