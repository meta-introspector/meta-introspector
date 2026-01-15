#!/bin/bash
# Split large files into <1MB chunks for LLM-friendly datasets

split_large_files() {
    local dir=$1
    local output_dir=$2
    
    mkdir -p "$output_dir"
    
    find "$dir" -type f -size +1M | while read file; do
        filename=$(basename "$file")
        dirname=$(dirname "$file" | sed "s|$dir||" | sed 's|^/||')
        
        # Create output directory structure
        mkdir -p "$output_dir/$dirname"
        
        # Split file into 900KB chunks (leaving room for metadata)
        split -b 900K -d -a 3 "$file" "$output_dir/$dirname/${filename}.part_"
        
        # Create manifest
        echo "Original: $file" > "$output_dir/$dirname/${filename}.manifest"
        echo "Size: $(stat -f%z "$file" 2>/dev/null || stat -c%s "$file")" >> "$output_dir/$dirname/${filename}.manifest"
        echo "Parts: $(ls "$output_dir/$dirname/${filename}.part_"* | wc -l)" >> "$output_dir/$dirname/${filename}.manifest"
        
        echo "Split: $file -> $output_dir/$dirname/${filename}.part_*"
    done
}

# Split telemetry data
echo "Splitting data-telemetry..."
split_large_files "data-telemetry" "hf-build-telemetry"

# Split markov data
echo "Splitting data-markov-analysis..."
split_large_files "data-markov-analysis" "hf-markov-analysis"

echo "Done! Ready for HuggingFace upload"
