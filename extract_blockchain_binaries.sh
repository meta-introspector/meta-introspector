#!/bin/bash
# Extract blockchain node binaries for economic weight analysis

echo "🔗 Extracting Blockchain Node Binaries"
echo "======================================="

OUTPUT="blockchain_binaries.txt"
> "$OUTPUT"

echo ""
echo "1️⃣  Bitcoin Core..."
cd blockchain_nodes/bitcoin
BITCOIN_PATH=$(nix build --print-out-paths 2>/dev/null)
if [ -n "$BITCOIN_PATH" ]; then
    echo "   ✅ $BITCOIN_PATH"
    find "$BITCOIN_PATH" -type f -executable >> "../../$OUTPUT"
    
    # Key binaries
    echo "   Key binaries:"
    ls -lh "$BITCOIN_PATH/bin"/* | awk '{print "      " $9 " (" $5 ")"}'
else
    echo "   ⚠️  Building..."
    nix build --no-link
fi
cd ../..

echo ""
echo "2️⃣  Ethereum Geth..."
cd blockchain_nodes/ethereum
GETH_PATH=$(nix build --print-out-paths 2>/dev/null)
if [ -n "$GETH_PATH" ]; then
    echo "   ✅ $GETH_PATH"
    find "$GETH_PATH" -type f -executable >> "../../$OUTPUT"
    
    echo "   Key binaries:"
    ls -lh "$GETH_PATH/bin"/* | awk '{print "      " $9 " (" $5 ")"}'
else
    echo "   ⚠️  Building..."
    nix build --no-link
fi
cd ../..

echo ""
echo "3️⃣  Solana..."
cd blockchain_nodes/solana
SOLANA_PATH=$(nix build --print-out-paths 2>/dev/null)
if [ -n "$SOLANA_PATH" ]; then
    echo "   ✅ $SOLANA_PATH"
    find "$SOLANA_PATH" -type f -executable >> "../../$OUTPUT"
    
    echo "   Key binaries:"
    ls -lh "$SOLANA_PATH/bin"/* 2>/dev/null | head -10 | awk '{print "      " $9 " (" $5 ")"}'
else
    echo "   ⚠️  Building..."
    nix build --no-link
fi
cd ../..

echo ""
echo "======================================="
echo "📊 Summary"
echo "======================================="

if [ -s "$OUTPUT" ]; then
    TOTAL=$(wc -l < "$OUTPUT")
    echo "Total blockchain binaries: $TOTAL"
    echo ""
    echo "Saved to: $OUTPUT"
    echo ""
    echo "Next steps:"
    echo "1. Run Markov analysis:"
    echo "   cargo run --release -p markov_resonance_analyzer -- $OUTPUT"
    echo ""
    echo "2. Extract economic weights:"
    echo "   cargo run --release -p symbol_similarity --bin economic_weight"
else
    echo "⚠️  No binaries extracted yet"
    echo "Run: nix build in each blockchain_nodes/* directory"
fi
