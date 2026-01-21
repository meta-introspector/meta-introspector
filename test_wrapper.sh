#!/bin/bash
# Test the wrapper

export PERF_OUTPUT="test_agda.perf.data"
./perf_wrapper.sh agda --version

echo ""
echo "Recorded to: $PERF_OUTPUT"
ls -lh "$PERF_OUTPUT"
