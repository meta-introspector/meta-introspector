#!/bin/bash
# Cluster files by name similarity and create chunks

INPUT_FILE="elf_files_filtered.txt"
CHUNK_SIZE=1000
OUTPUT_DIR="elf_chunks"

mkdir -p "$OUTPUT_DIR"

echo "📊 Clustering files by name similarity..."

# Sort by filename (basename) to group similar files
sort -t/ -k9 "$INPUT_FILE" > "${OUTPUT_DIR}/sorted_files.txt"

# Split into chunks
cd "$OUTPUT_DIR"
split -l "$CHUNK_SIZE" -d -a 3 sorted_files.txt chunk_

# Rename to .txt
for f in chunk_*; do
    mv "$f" "${f}.txt"
done

CHUNK_COUNT=$(ls -1 chunk_*.txt | wc -l)

echo "✅ Created $CHUNK_COUNT chunks of ~$CHUNK_SIZE files each"
echo "📁 Chunks saved to: $OUTPUT_DIR/"
ls -lh chunk_*.txt | head -5
