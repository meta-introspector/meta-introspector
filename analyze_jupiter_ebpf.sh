#!/usr/bin/env nix
#!nix develop /mnt/data1/time-2026/01-january/18/solflake --command bash
# Trace and analyze Jupiter eBPF execution

set -e

JUPITER_SO="/mnt/data1/meta-introspector/data/solana_contracts/Jupiter/program.so"
OUTPUT_DIR="/mnt/data1/meta-introspector/data/jupiter_analysis"

mkdir -p "$OUTPUT_DIR"

echo "=== Jupiter eBPF Analysis ==="
echo ""

# 1. Extract instruction discriminators (first 8 bytes of each instruction)
echo "1. Extracting instruction discriminators..."
strings "$JUPITER_SO" | grep -E "^Instruction:" | sort -u > "$OUTPUT_DIR/instructions.txt"
echo "Found $(wc -l < $OUTPUT_DIR/instructions.txt) instructions"

# 2. Map eBPF functions to entry points
echo "2. Mapping eBPF functions..."
llvm-objdump -d "$JUPITER_SO" | grep -E "^[0-9a-f]+ <" | head -20 > "$OUTPUT_DIR/entry_points.txt"

# 3. Extract all error codes and messages
echo "3. Extracting error codes..."
strings "$JUPITER_SO" | grep -E "Error Code:|Error Message:" > "$OUTPUT_DIR/error_codes.txt"

# 4. Find account constraints
echo "4. Finding account constraints..."
strings "$JUPITER_SO" | grep -E "Constraint|constraint" > "$OUTPUT_DIR/constraints.txt"

# 5. Extract Anchor IDL hints
echo "5. Extracting Anchor IDL structure..."
strings "$JUPITER_SO" | grep -A 5 "anchor:idl" > "$OUTPUT_DIR/anchor_idl_hints.txt"

# 6. Create execution trace template
cat > "$OUTPUT_DIR/trace_template.md" << 'TRACE'
# Jupiter Execution Trace Template

## Setup
1. Get recent Jupiter transaction: `solana transaction <sig>`
2. Extract instruction data
3. Match to discriminator
4. Trace eBPF execution

## Instruction Format
```
[0-7]: Discriminator (8 bytes)
[8+]:  Instruction data
```

## Account Layout
```
Account 0: Program ID (JUP4Fb2...)
Account 1: User authority
Account 2-N: Token accounts, AMM accounts
```

## Trace Steps
1. Entry point (discriminator match)
2. Account validation (constraints)
3. Business logic (eBPF execution)
4. State updates
5. CPI calls to AMMs
6. Return/error

## Tools
- `solana transaction <sig>` - Get tx details
- `solana program dump` - Already done
- `llvm-objdump -d` - Disassemble
- eBPF debugger - Step through execution
TRACE

# 7. Create eBPF tracer script
cat > "$OUTPUT_DIR/trace_jupiter.sh" << 'TRACER'
#!/bin/bash
# Trace a Jupiter transaction

if [ -z "$1" ]; then
  echo "Usage: $0 <transaction_signature>"
  exit 1
fi

SIG="$1"
OUTPUT="jupiter_trace_${SIG:0:8}.txt"

echo "=== Tracing Jupiter Transaction ===" | tee "$OUTPUT"
echo "Signature: $SIG" | tee -a "$OUTPUT"
echo "" | tee -a "$OUTPUT"

# Get transaction details
echo "1. Fetching transaction..." | tee -a "$OUTPUT"
solana transaction "$SIG" --output json > tx.json

# Extract instruction data
echo "2. Extracting instruction data..." | tee -a "$OUTPUT"
jq '.transaction.message.instructions[] | select(.programId == "JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB")' tx.json > jupiter_ix.json

# Get discriminator
echo "3. Instruction discriminator:" | tee -a "$OUTPUT"
jq -r '.data' jupiter_ix.json | base64 -d | xxd -p -l 8 | tee -a "$OUTPUT"

# Get accounts
echo "4. Accounts involved:" | tee -a "$OUTPUT"
jq -r '.accounts[]' jupiter_ix.json | tee -a "$OUTPUT"

# Get logs
echo "5. Execution logs:" | tee -a "$OUTPUT"
jq -r '.meta.logMessages[]' tx.json | grep -A 10 "Program JUP4Fb2" | tee -a "$OUTPUT"

echo "" | tee -a "$OUTPUT"
echo "Trace saved to: $OUTPUT"
TRACER

chmod +x "$OUTPUT_DIR/trace_jupiter.sh"

# 8. Get recent Jupiter transactions
echo "6. Fetching recent Jupiter transactions..."
solana transaction-history JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB --limit 5 2>/dev/null > "$OUTPUT_DIR/recent_txs.txt" || echo "Note: Need RPC access for transaction history"

echo ""
echo "=== Analysis Complete ==="
echo "Output directory: $OUTPUT_DIR"
echo ""
echo "Files created:"
ls -lh "$OUTPUT_DIR"
echo ""
echo "Next steps:"
echo "1. Get a Jupiter transaction signature from Solscan/Solana Explorer"
echo "2. Run: $OUTPUT_DIR/trace_jupiter.sh <signature>"
echo "3. Analyze execution logs and match to eBPF disassembly"
echo "4. Map instruction flow through the program"
