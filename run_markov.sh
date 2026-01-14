#!/bin/bash
cd /mnt/data1/meta-introspector
cargo run --release -p markov_resonance_analyzer -- elf_files_filtered.txt > markov_output.log 2>&1
EXIT_CODE=$?
echo "Exit code: $EXIT_CODE" | tee -a markov_output.log
exit $EXIT_CODE
