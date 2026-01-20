#!/usr/bin/env bash
set -euo pipefail

CSV_FILE="${1:-/tmp/FILE_GIT_MAPPING.csv}"
OUTPUT_DIR="${2:-data/repo-checkouts}"

mkdir -p "$OUTPUT_DIR"

echo "📊 Compiling directories per git repo from CSV..."

# Extract unique repos and their directories
awk -F',' 'NR>1 && $5 != "" {
    gsub(/"/, "", $2);  # git_repo
    gsub(/"/, "", $5);  # remote
    if ($5 != "") {
        print $5 "|" $2
    }
}' "$CSV_FILE" | sort -u > /tmp/repo_dirs.tmp

# Group by remote URL
awk -F'|' '{
    remote = $1
    dir = $2
    repos[remote] = repos[remote] dir "\n"
}
END {
    for (remote in repos) {
        # Create filename from URL
        gsub(/[^a-zA-Z0-9]/, "_", remote)
        filename = remote ".txt"
        printf "%s", repos[remote] > filename
    }
}' /tmp/repo_dirs.tmp

# Create summary
echo "📋 Summary:" > "$OUTPUT_DIR/SUMMARY.txt"
awk -F'|' '{repos[$1]++} END {for (r in repos) print repos[r], r}' /tmp/repo_dirs.tmp | sort -rn >> "$OUTPUT_DIR/SUMMARY.txt"

cat "$OUTPUT_DIR/SUMMARY.txt"

rm /tmp/repo_dirs.tmp
echo "✅ Done: $OUTPUT_DIR/"
