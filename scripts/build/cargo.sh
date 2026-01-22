#!/bin/bash
# Canonical cargo build wrapper
# All scripts should use this instead of raw "cargo build"
exec cargo build "$@"
