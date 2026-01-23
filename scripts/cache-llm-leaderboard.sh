#!/usr/bin/env bash
# Cache Open LLM Leaderboard best models from HuggingFace

set -euo pipefail

OUTPUT_DIR="/mnt/data1/meta-introspector/vendor/llm-leaderboard-cache"
mkdir -p "$OUTPUT_DIR"

echo "=== Fetching Open LLM Leaderboard Best Models ==="
echo "Output: $OUTPUT_DIR"
echo ""

# Collection ID from the URL
COLLECTION_ID="open-llm-leaderboard/open-llm-leaderboard-best-models-6753b46f3e0d2e7e0e6f5c0e"

echo "Querying collection..."
curl -s "https://huggingface.co/api/collections/${COLLECTION_ID}" > "$OUTPUT_DIR/collection.json"

# Extract model IDs
jq -r '.items[].item_id' "$OUTPUT_DIR/collection.json" > "$OUTPUT_DIR/model-ids.txt" 2>/dev/null || true

MODEL_COUNT=$(wc -l < "$OUTPUT_DIR/model-ids.txt" 2>/dev/null || echo "0")
echo "Found $MODEL_COUNT models in leaderboard"

# Fetch details for each model
echo "Fetching model details..."
while IFS= read -r model_id; do
    echo "  - $model_id"
    curl -s "https://huggingface.co/api/models/$model_id" >> "$OUTPUT_DIR/models-detailed.jsonl"
    echo "" >> "$OUTPUT_DIR/models-detailed.jsonl"
done < "$OUTPUT_DIR/model-ids.txt"

# Save metadata
cat > "$OUTPUT_DIR/metadata.json" << EOF
{
  "source": "https://huggingface.co/collections/open-llm-leaderboard/open-llm-leaderboard-best-models",
  "collection_id": "$COLLECTION_ID",
  "fetched": "$(date -Iseconds)",
  "model_count": $MODEL_COUNT
}
EOF

echo ""
echo "✅ Leaderboard models cached"
echo "Files:"
echo "  - $OUTPUT_DIR/collection.json"
echo "  - $OUTPUT_DIR/model-ids.txt"
echo "  - $OUTPUT_DIR/models-detailed.jsonl"
echo "  - $OUTPUT_DIR/metadata.json"
echo ""
echo "Top models:"
head -10 "$OUTPUT_DIR/model-ids.txt"
