#!/bin/bash
# 🔥 STRUCTURED STRACE CAPTURE: Save build data for telemetry system

TIMESTAMP=$(date +%s)
OUTPUT_DIR="/mnt/data1/meta-introspector/data/build_analysis"
SESSION_ID="real_build_${TIMESTAMP}"

echo "🔥 STRUCTURED STRACE CAPTURE"
echo "============================"
echo "📊 Session: $SESSION_ID"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# 1. Save executed binaries as JSON
echo "📋 Saving executed binaries..."
cat > "$OUTPUT_DIR/${SESSION_ID}_binaries.json" << EOF
{
  "session_id": "$SESSION_ID",
  "timestamp": $TIMESTAMP,
  "source": "strace_nix_build",
  "total_binaries": $(wc -l < /tmp/executed_binaries.txt),
  "binaries": [
$(while IFS= read -r binary; do
    echo "    \"$binary\","
done < /tmp/executed_binaries.txt | sed '$ s/,$//')
  ]
}
EOF

# 2. Save opened libraries as JSON
echo "📚 Saving opened libraries..."
cat > "$OUTPUT_DIR/${SESSION_ID}_libraries.json" << EOF
{
  "session_id": "$SESSION_ID", 
  "timestamp": $TIMESTAMP,
  "source": "strace_openat",
  "total_libraries": $(wc -l < /tmp/opened_libs.txt),
  "libraries": [
$(while IFS= read -r lib; do
    echo "    \"$lib\","
done < /tmp/opened_libs.txt | sed '$ s/,$//')
  ]
}
EOF

# 3. Save ldd dependencies as JSON
echo "🔍 Saving ldd dependencies..."
cat > "$OUTPUT_DIR/${SESSION_ID}_ldd_deps.json" << EOF
{
  "session_id": "$SESSION_ID",
  "timestamp": $TIMESTAMP, 
  "source": "ldd_analysis",
  "total_dependencies": $(wc -l < /tmp/ldd_libs.txt),
  "dependencies": [
$(while IFS= read -r dep; do
    echo "    \"$dep\","
done < /tmp/ldd_libs.txt | sed '$ s/,$//')
  ]
}
EOF

# 4. Save combined analysis
echo "📊 Creating combined analysis..."
BINARIES=$(wc -l < /tmp/executed_binaries.txt)
OPENED_LIBS=$(wc -l < /tmp/opened_libs.txt)
LDD_LIBS=$(wc -l < /tmp/ldd_libs.txt)
ALL_LIBS=$(wc -l < /tmp/all_libs.txt)

cat > "$OUTPUT_DIR/${SESSION_ID}_analysis.json" << EOF
{
  "session_id": "$SESSION_ID",
  "timestamp": $TIMESTAMP,
  "build_type": "nix_rust_overlay",
  "analysis_summary": {
    "binaries_executed": $BINARIES,
    "libraries_opened": $OPENED_LIBS,
    "ldd_dependencies": $LDD_LIBS,
    "total_unique_libraries": $ALL_LIBS
  },
  "data_files": {
    "binaries": "${SESSION_ID}_binaries.json",
    "libraries": "${SESSION_ID}_libraries.json", 
    "ldd_dependencies": "${SESSION_ID}_ldd_deps.json",
    "strace_log": "/tmp/nix_strace.log"
  },
  "next_phase": {
    "action": "update_telemetry_system",
    "description": "Use this data to replace old frontrun results with real build data",
    "input_files": [
      "${SESSION_ID}_binaries.json",
      "${SESSION_ID}_libraries.json"
    ]
  }
}
EOF

# 5. Copy strace log to permanent storage
echo "💾 Archiving strace log..."
cp /tmp/nix_strace.log "$OUTPUT_DIR/${SESSION_ID}_strace.log"

# 6. Create documentation
echo "📝 Creating documentation..."
cat > "$OUTPUT_DIR/${SESSION_ID}_README.md" << EOF
# Real Nix Build Analysis - $SESSION_ID

## Overview
Complete analysis of actual nix build process captured via strace on $(date).

## Key Findings
- **$BINARIES binaries executed** (vs 14 in old telemetry)
- **$OPENED_LIBS .so files opened** during build
- **$LDD_LIBS ldd dependencies** from executed binaries  
- **$ALL_LIBS total unique libraries**

## Data Files
- \`${SESSION_ID}_binaries.json\` - All executed binaries
- \`${SESSION_ID}_libraries.json\` - All opened .so files
- \`${SESSION_ID}_ldd_deps.json\` - All ldd dependencies
- \`${SESSION_ID}_analysis.json\` - Combined analysis
- \`${SESSION_ID}_strace.log\` - Raw strace output

## Next Phase
Update telemetry system (\`ldd2wrap_all_calls.rs\`) to use real binary list instead of old frontrun results.

## Usage
\`\`\`bash
# Use binaries.json as input for telemetry generation
cargo run --bin ldd2wrap_all_calls --input ${SESSION_ID}_binaries.json
\`\`\`
EOF

echo ""
echo "✅ STRUCTURED CAPTURE COMPLETE"
echo "=============================="
echo "📁 Output directory: $OUTPUT_DIR"
echo "🔧 Binaries: $BINARIES"
echo "📚 Libraries: $ALL_LIBS"
echo "📄 Session: $SESSION_ID"
echo ""
echo "🎯 NEXT PHASE:"
echo "   Update telemetry system with real build data"
echo "   Input: $OUTPUT_DIR/${SESSION_ID}_binaries.json"
