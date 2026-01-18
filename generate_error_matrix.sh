#!/bin/bash
# Generate error matrix with short names and error types

LOGS_DIR="$HOME/.local/share/nix-builder/logs"
REPORT="NIX_ERROR_MATRIX.md"

cd /mnt/data1/meta-introspector

echo "📊 Generating error matrix..."

# Define error types
declare -A ERROR_TYPES=(
    ["missing-default"]="Missing packages.default"
    ["cannot-build"]="Cannot build derivation"
    ["jq-error"]="jq file error"
    ["assertion"]="Assertion failure"
    ["flake-url"]="Flake URL error"
    ["expect-test"]="Test failure"
    ["no-such-file"]="File not found"
)

# Classify errors
classify_error() {
    local log=$1
    if grep -q "does not provide attribute.*default" "$log"; then
        echo "missing-default"
    elif grep -q "Cannot build" "$log"; then
        echo "cannot-build"
    elif grep -q "jq: error" "$log"; then
        echo "jq-error"
    elif grep -q "Assertion" "$log"; then
        echo "assertion"
    elif grep -q "flake.*attribute 'url'" "$log"; then
        echo "flake-url"
    elif grep -q "expect test failed" "$log"; then
        echo "expect-test"
    elif grep -q "No such file" "$log"; then
        echo "no-such-file"
    else
        echo "other"
    fi
}

# Generate report
cat > "$REPORT" << 'EOF'
# Nix Build Error Matrix

**Generated**: $(date)

## Error Type Summary

| Error Type | Count | Description |
|------------|-------|-------------|
EOF

# Count by error type
declare -A type_counts
for log in "$LOGS_DIR"/*.log; do
    if grep -q "error:" "$log"; then
        type=$(classify_error "$log")
        ((type_counts[$type]++))
    fi
done

for type in "${!type_counts[@]}"; do
    desc="${ERROR_TYPES[$type]:-Unknown}"
    echo "| $type | ${type_counts[$type]} | $desc |" >> "$REPORT"
done | sort -t'|' -k3 -rn >> "$REPORT"

echo "" >> "$REPORT"
echo "## Projects by Error Type" >> "$REPORT"
echo "" >> "$REPORT"

# Group projects by error type
for error_type in "${!ERROR_TYPES[@]}"; do
    echo "### $error_type: ${ERROR_TYPES[$error_type]}" >> "$REPORT"
    echo "" >> "$REPORT"
    
    count=0
    for log in "$LOGS_DIR"/*.log; do
        if grep -q "error:" "$log"; then
            type=$(classify_error "$log")
            if [ "$type" = "$error_type" ]; then
                # Short name: remove timestamp and truncate
                name=$(basename "$log" | sed 's/_[0-9]*\.log$//' | cut -c1-40)
                echo "- $name" >> "$REPORT"
                ((count++))
            fi
        fi
    done
    
    echo "" >> "$REPORT"
    echo "**Total**: $count projects" >> "$REPORT"
    echo "" >> "$REPORT"
done

# Add matrix view
echo "## Error Matrix (Top 50 Projects)" >> "$REPORT"
echo "" >> "$REPORT"
echo "| Project | missing-default | cannot-build | jq-error | assertion | other |" >> "$REPORT"
echo "|---------|----------------|--------------|----------|-----------|-------|" >> "$REPORT"

for log in "$LOGS_DIR"/*.log; do
    if grep -q "error:" "$log"; then
        name=$(basename "$log" | sed 's/_[0-9]*\.log$//' | cut -c1-30)
        type=$(classify_error "$log")
        
        # Create row with X marks
        row="| $name |"
        for t in missing-default cannot-build jq-error assertion other; do
            if [ "$type" = "$t" ]; then
                row="$row X |"
            else
                row="$row   |"
            fi
        done
        echo "$row"
    fi
done | head -50 >> "$REPORT"

echo "" >> "$REPORT"
echo "## Statistics" >> "$REPORT"
echo "" >> "$REPORT"
echo "- **Total failed builds**: $(grep -l "error:" "$LOGS_DIR"/*.log | wc -l)" >> "$REPORT"
echo "- **Unique error types**: ${#type_counts[@]}" >> "$REPORT"
echo "- **Most common error**: $(for t in "${!type_counts[@]}"; do echo "${type_counts[$t]} $t"; done | sort -rn | head -1 | cut -d' ' -f2-)" >> "$REPORT"

echo "✅ Matrix saved to $REPORT"
cat "$REPORT" | head -100
