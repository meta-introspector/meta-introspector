#!/usr/bin/env bash
# Auto-fix workflow: Build, analyze errors, fix, repeat

SERVER_URL="http://127.0.0.1:3000"

echo "🔧 Starting auto-fix workflow..."

# Check if server is running
if ! curl -s $SERVER_URL/peer > /dev/null 2>&1; then
    echo "❌ Server not running. Start with: ./target/debug/minimal-build-server"
    exit 1
fi

# Build and get comprehensive error report
echo "🔨 Building git-sources..."
RESULT=$(curl -s -X POST $SERVER_URL/hot-build \
    -H "Content-Type: application/json" \
    -d '{"target":"git-sources"}')

SUCCESS=$(echo "$RESULT" | jq -r '.success')

if [ "$SUCCESS" = "true" ]; then
    echo "✅ Build successful!"
    exit 0
fi

echo "❌ Build failed. Analyzing errors..."

# Get error contexts
echo "$RESULT" | jq -r '.contexts[] | 
    "File: \(.error.file):\(.error.line)",
    "Error: \(.error.error_type) - \(.error.message)",
    "Blame: \(.blame)",
    "Status: \(.status)",
    "Lines:",
    "\(.lines)",
    "---"
' | head -50

# Try auto-fix
echo ""
echo "🔧 Attempting auto-fix..."
curl -s -X POST $SERVER_URL/fix-all | jq '.'

echo ""
echo "💡 To manually fix:"
echo "  curl -X POST $SERVER_URL/sed -d '{\"file\":\"foo.rs\",\"pattern\":\"old\",\"replacement\":\"new\"}'"
echo "  curl -X POST $SERVER_URL/hot-build -d '{\"target\":\"git-sources\"}'"
