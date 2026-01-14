#!/bin/bash
# Monitor nix build progress and update ELF file list

LOG_FILE="const71_build_full.log"
OUTPUT_DIR="const_71_analysis"
UPDATED_LIST="elf_files_updated.txt"

echo "📊 Monitoring nix build and updating ELF file list..."
echo ""

# Monitor the build
while true; do
    if [ -f "$LOG_FILE" ]; then
        # Show last few lines
        clear
        echo "=== Nix Build Monitor ==="
        echo "Time: $(date '+%H:%M:%S')"
        echo ""
        tail -15 "$LOG_FILE"
        echo ""
        echo "=== New Binaries ==="
        ls -lh "$OUTPUT_DIR"/binary_* 2>/dev/null | tail -5
        echo ""
        
        # Check if build is done
        if grep -q "Analysis Complete" "$LOG_FILE" 2>/dev/null; then
            echo "✅ Build complete!"
            break
        fi
        
        # Check if process still running
        if ! pgrep -f "build_and_analyze_const71.sh" > /dev/null; then
            echo "⚠️  Build process finished"
            break
        fi
    fi
    
    sleep 5
done

echo ""
echo "🔄 Updating ELF file list..."

# Start with existing list
cp elf_files_filtered.txt "$UPDATED_LIST"

# Add new binaries from const test
if [ -d "$OUTPUT_DIR" ]; then
    find "$OUTPUT_DIR" -name "binary_*" -type f >> "$UPDATED_LIST"
fi

# Add any new nix store binaries
find /nix/store -name "const-71-*" -type f 2>/dev/null >> "$UPDATED_LIST"

# Remove duplicates and sort
sort -u "$UPDATED_LIST" -o "$UPDATED_LIST"

OLD_COUNT=$(wc -l < elf_files_filtered.txt)
NEW_COUNT=$(wc -l < "$UPDATED_LIST")
ADDED=$((NEW_COUNT - OLD_COUNT))

echo "✅ Updated file list:"
echo "   Old: $OLD_COUNT files"
echo "   New: $NEW_COUNT files"
echo "   Added: $ADDED files"

echo ""
echo "New files:"
tail -$ADDED "$UPDATED_LIST"
