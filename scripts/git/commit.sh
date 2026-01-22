#!/bin/bash
# Canonical git commit wrapper
# All scripts should use this instead of raw "git commit"
exec git commit "$@"
