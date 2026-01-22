#!/bin/bash
# Lift Python to Rust using Gemini

PROMPT_FILE="$1"
OUTPUT_DIR="${2:-data/rust_generated}"

mkdir -p "$OUTPUT_DIR"

echo "🚀 Lifting to Rust via Gemini..."
echo "   Prompt: $PROMPT_FILE"

# Call Gemini
node ~/nix/vendor/external/gemini-cli/bundle/gemini.js \
  -p "$(cat "$PROMPT_FILE" | jq -r .prompt)" \
  --output-format json \
  --model gemini-2.5-flash \
  > "$OUTPUT_DIR/response.json"

# Extract Rust code
jq -r '.rust_code // .code // .content' "$OUTPUT_DIR/response.json" > "$OUTPUT_DIR/generated.rs"

# Extract proof
jq -r '.equivalence_proof // .proof' "$OUTPUT_DIR/response.json" > "$OUTPUT_DIR/proof.md"

echo "✅ Generated:"
echo "   Rust: $OUTPUT_DIR/generated.rs"
echo "   Proof: $OUTPUT_DIR/proof.md"
