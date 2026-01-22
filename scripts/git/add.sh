#!/bin/bash
# Canonical git add wrapper
# All scripts should use this instead of raw "git add"
exec git add "$@"
