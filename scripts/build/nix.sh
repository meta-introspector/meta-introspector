#!/bin/bash
# Canonical nix build wrapper
# All scripts should use this instead of raw "nix build"
exec nix build "$@"
