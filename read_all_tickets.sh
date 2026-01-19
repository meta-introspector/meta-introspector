#!/bin/bash
# Read all tickets and extract unique IDs

TICKET_DIR="/mnt/data1/nix/vendor/rust/cargo2nix/ai-ml-zk-ops/documentation/art/art/memes/extracted_tickets"
OUTPUT="/mnt/data1/meta-introspector/ticket_summary.txt"

echo "Reading 194 tickets..." > "$OUTPUT"
echo "" >> "$OUTPUT"

for ticket in "$TICKET_DIR"/*.md; do
    echo "=== $(basename "$ticket") ===" >> "$OUTPUT"
    
    # Extract ID and title
    grep -E "^\*\*ID:\*\*|^\*\*Title:\*\*" "$ticket" | head -2 >> "$OUTPUT"
    
    echo "" >> "$OUTPUT"
done

# Count unique IDs
echo "" >> "$OUTPUT"
echo "=== UNIQUE IDS ===" >> "$OUTPUT"
grep "^\*\*ID:\*\*" "$OUTPUT" | sort -u | wc -l >> "$OUTPUT"

echo "Done! Summary in $OUTPUT"
