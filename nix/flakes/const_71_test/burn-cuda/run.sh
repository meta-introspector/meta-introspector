#!/bin/bash
# Run burn-cuda with proper library paths for CUDA 13
export LD_LIBRARY_PATH=/nix/store/j193mfi0f921y0kfs8vjc1znnr45ispv-glibc-2.40-66/lib:/usr/local/cuda-13.0/lib64:/usr/lib/x86_64-linux-gnu
exec "$(dirname "$0")/target/release/burn-cuda-71" "$@"
