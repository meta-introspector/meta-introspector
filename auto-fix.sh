#!/usr/bin/env bash
# Auto-fix common build errors

SERVER_URL="http://127.0.0.1:3000"

echo "🔧 Auto-fixing errors..."

# Get errors
ERRORS=$(curl -s $SERVER_URL/errors)

# Fix missing imports
echo "$ERRORS" | jq -r '.details[] | select(.error_type == "E0433") | .message' | while read line; do
    if [[ $line =~ "use of unresolved module or unlinked crate \`([^`]+)\`" ]]; then
        CRATE="${BASH_REMATCH[1]}"
        echo "  Adding $CRATE to Cargo.toml..."
        
        # Add to Cargo.toml
        curl -s -X POST $SERVER_URL/sed \
            -H "Content-Type: application/json" \
            -d "{\"file\":\"Cargo.toml\",\"pattern\":\"[dependencies]\",\"replacement\":\"[dependencies]\\n$CRATE = \\\"*\\\"\"}"
    fi
done

echo "✅ Auto-fix complete"
