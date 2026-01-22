#!/usr/bin/env nix
#!nix develop /mnt/data1/time-2026/01-january/18/solflake --command bash
# Decompile Solana eBPF contracts to Rust

set -e

CONTRACTS_DIR="/mnt/data1/meta-introspector/data/solana_contracts"
OUTPUT_DIR="/mnt/data1/meta-introspector/data/solana_decompiled"

mkdir -p "$OUTPUT_DIR"

for contract_dir in "$CONTRACTS_DIR"/*; do
  name=$(basename "$contract_dir")
  program="$contract_dir/program.so"
  
  if [ ! -f "$program" ]; then
    continue
  fi
  
  echo "=== Decompiling $name ==="
  
  out_dir="$OUTPUT_DIR/$name"
  mkdir -p "$out_dir"
  
  # 1. Disassemble to eBPF assembly
  echo "Disassembling..."
  llvm-objdump -d "$program" > "$out_dir/disasm.s"
  
  # 2. Extract all strings (potential function names, error messages)
  echo "Extracting strings..."
  strings "$program" > "$out_dir/strings.txt"
  
  # 3. Extract Rust panic messages and source paths
  echo "Finding Rust source paths..."
  strings "$program" | grep -E "\.rs:|src/|programs/" > "$out_dir/rust_paths.txt" || true
  
  # 4. Extract Anchor IDL if present
  echo "Checking for Anchor IDL..."
  strings "$program" | grep -A 100 "anchor:idl" > "$out_dir/anchor_hints.txt" || true
  
  # 5. Symbol table
  echo "Extracting symbols..."
  llvm-nm "$program" > "$out_dir/symbols.txt" 2>/dev/null || true
  
  # 6. Section info
  echo "Getting sections..."
  llvm-objdump -h "$program" > "$out_dir/sections.txt"
  
  # 7. Try to extract function boundaries
  echo "Finding functions..."
  llvm-objdump -d "$program" | grep "^[0-9a-f]* <" > "$out_dir/functions.txt" || true
  
  echo "✅ $name decompiled to $out_dir"
  echo ""
done

echo ""
echo "=== Decompilation Summary ==="
echo "All contracts decompiled to: $OUTPUT_DIR"
echo ""
echo "Note: Full Rust reconstruction requires:"
echo "1. Anchor IDL (for interface)"
echo "2. Source code patterns (from strings)"
echo "3. Manual reconstruction from eBPF assembly"
echo ""
echo "Better approach: Match to source repos and build from source"
