#!/bin/bash
# Split ALL files over 1MB and remove originals

process_directory() {
    local src_dir=$1
    local dst_dir=$2
    
    echo "Processing $src_dir -> $dst_dir"
    
    # Copy small files directly
    find "$src_dir" -type f -size -1M -exec bash -c '
        file="$1"
        rel_path="${file#'"$src_dir"'/}"
        dst_file="'"$dst_dir"'/$rel_path"
        mkdir -p "$(dirname "$dst_file")"
        cp "$file" "$dst_file"
        echo "Copied: $rel_path"
    ' _ {} \;
    
    # Split large files
    find "$src_dir" -type f -size +1M | while read file; do
        rel_path="${file#$src_dir/}"
        dst_path="$dst_dir/$rel_path"
        mkdir -p "$(dirname "$dst_path")"
        
        # Split into 900KB chunks
        split -b 900K -d -a 4 "$file" "${dst_path}.part_"
        
        # Create manifest
        cat > "${dst_path}.manifest" <<EOF
original: $rel_path
size: $(stat -c%s "$file" 2>/dev/null || stat -f%z "$file")
parts: $(ls "${dst_path}.part_"* 2>/dev/null | wc -l)
reconstruct: cat ${rel_path}.part_* > ${rel_path}
EOF
        
        echo "Split: $rel_path ($(ls "${dst_path}.part_"* | wc -l) parts)"
    done
}

# Process markov-analysis
process_directory "data-markov-analysis" "hf-markov-analysis-upload"

echo "Done! All files <1MB"
