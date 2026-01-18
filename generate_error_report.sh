#!/bin/bash
# Generate error report from Nix builder logs

LOGS_DIR="$HOME/.local/share/nix-builder/logs"
REPORT="NIX_BUILD_ERROR_REPORT.md"

cd /mnt/data1/meta-introspector

echo "📊 Generating error report..."

cat > "$REPORT" << 'EOF'
# Nix Build Error Report

**Generated**: $(date)

## Summary

EOF

total=$(ls -1 "$LOGS_DIR"/*.log 2>/dev/null | wc -l)
errors=$(grep -l "error:" "$LOGS_DIR"/*.log 2>/dev/null | wc -l)
success=$((total - errors))

echo "- **Total builds**: $total" >> "$REPORT"
echo "- **Successful**: $success" >> "$REPORT"
echo "- **Failed**: $errors" >> "$REPORT"
echo "- **Success rate**: $(( success * 100 / total ))%" >> "$REPORT"
echo "" >> "$REPORT"

echo "## Failed Builds" >> "$REPORT"
echo "" >> "$REPORT"
echo "| Repository | Error Type | Details |" >> "$REPORT"
echo "|------------|------------|---------|" >> "$REPORT"

grep -l "error:" "$LOGS_DIR"/*.log 2>/dev/null | while read log; do
    repo=$(basename "$log" | sed 's/_[0-9]*\.log$//')
    error=$(grep "error:" "$log" | head -1 | cut -c1-80)
    echo "| $repo | Build error | \`$error\` |" >> "$REPORT"
done

echo "" >> "$REPORT"
echo "## Common Errors" >> "$REPORT"
echo "" >> "$REPORT"

grep -h "error:" "$LOGS_DIR"/*.log 2>/dev/null | \
    sed 's/^.*error: //' | \
    cut -d' ' -f1-5 | \
    sort | uniq -c | sort -rn | head -10 | \
    while read count error; do
        echo "- **$count occurrences**: $error" >> "$REPORT"
    done

echo "" >> "$REPORT"
echo "## Detailed Logs" >> "$REPORT"
echo "" >> "$REPORT"
echo "All logs available in: \`$LOGS_DIR/\`" >> "$REPORT"

echo "✅ Report saved to $REPORT"
cat "$REPORT"
