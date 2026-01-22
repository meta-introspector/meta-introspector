#!/bin/bash
# Unified driver wrapper for bash
# Replaces: jq, bash, ssh, curl, git, cargo, nix

DRIVER="cargo run --release --bin driver --"

# Create aliases
alias nix="$DRIVER nix"
alias cargo="$DRIVER cargo"
alias git="$DRIVER git"
alias jq="$DRIVER jq"
alias ssh="$DRIVER ssh"
alias curl="$DRIVER curl"
alias perf="$DRIVER perf"

# Or call directly
exec $DRIVER "$@"
