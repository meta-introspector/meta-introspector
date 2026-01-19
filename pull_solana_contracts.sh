#!/usr/bin/env nix
#!nix develop /mnt/data1/time-2026/01-january/18/solflake --command bash
# Pull and decompile Solana contracts

set -e

CONTRACTS=(
  "Jupiter:JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB"
  "Orca:whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"
  "Raydium:675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"
  "Phoenix:PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY"
  "Serum:9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin"
  "Solend:So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo"
  "Mango:mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68"
  "Marinade:MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD"
  "Saber:SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ"
  "Drift:dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH"
)

OUTPUT_DIR="/mnt/data1/meta-introspector/data/solana_contracts"
mkdir -p "$OUTPUT_DIR"

for contract in "${CONTRACTS[@]}"; do
  name="${contract%%:*}"
  address="${contract##*:}"
  
  echo "=== Pulling $name ($address) ==="
  
  # Create contract dir
  contract_dir="$OUTPUT_DIR/$name"
  mkdir -p "$contract_dir"
  
  # Dump program
  echo "Dumping program..."
  solana program dump "$address" "$contract_dir/program.so" || {
    echo "Failed to dump $name"
    continue
  }
  
  # Get program info
  echo "Getting program info..."
  solana program show "$address" > "$contract_dir/info.txt" || true
  
  # Decompile with objdump
  echo "Disassembling..."
  llvm-objdump -d "$contract_dir/program.so" > "$contract_dir/disasm.txt" || true
  
  # Extract strings
  echo "Extracting strings..."
  strings "$contract_dir/program.so" > "$contract_dir/strings.txt" || true
  
  # Get file info
  file "$contract_dir/program.so" > "$contract_dir/file_info.txt"
  
  echo "✅ $name complete"
  echo ""
done

echo "All contracts pulled to: $OUTPUT_DIR"
echo ""
echo "Next: Analyze strings and disassembly to find source patterns"
