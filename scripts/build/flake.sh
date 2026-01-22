#!/bin/bash
# Canonical flake update wrapper
# All scripts should use this instead of raw "nix flake update"
exec nix flake update "$@"
