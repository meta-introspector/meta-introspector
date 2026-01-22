#!/bin/bash
# Public verification script - anyone can run this
# Verifies a byte argument using only public data

set -e

BYTE_ARG="$1"

if [ -z "$BYTE_ARG" ]; then
    echo "Usage: $0 <byte_argument.json>"
    exit 1
fi

if [ ! -f "$BYTE_ARG" ]; then
    echo "Error: File not found: $BYTE_ARG"
    exit 1
fi

# Extract fields
COMMIT=$(jq -r '.origin.git_commit' "$BYTE_ARG")
FILE=$(jq -r '.origin.file' "$BYTE_ARG")
LINE=$(jq -r '.origin.line' "$BYTE_ARG")
COL=$(jq -r '.origin.col' "$BYTE_ARG")
BYTE=$(jq -r '.byte' "$BYTE_ARG")

echo "🔍 Verifying byte argument..."
echo "   Commit: $COMMIT"
echo "   File: $FILE"
echo "   Line: $LINE, Column: $COL"
echo "   Expected byte: $BYTE"
echo ""

# 1. Verify commit exists
echo "[1/4] Verifying commit exists..."
if ! git cat-file -e "$COMMIT^{commit}" 2>/dev/null; then
    echo "❌ Commit not found"
    exit 1
fi
echo "✅ Commit exists"

# 2. Verify GPG signature
echo "[2/4] Verifying GPG signature..."
if ! git verify-commit "$COMMIT" 2>/dev/null; then
    echo "⚠️  GPG signature not verified (may not be signed)"
else
    echo "✅ GPG signature valid"
fi

# 3. Verify byte at location
echo "[3/4] Verifying byte at location..."
CONTENT=$(git show "$COMMIT:$FILE" 2>/dev/null || echo "")
if [ -z "$CONTENT" ]; then
    echo "❌ File not found in commit"
    exit 1
fi

ACTUAL_BYTE=$(echo "$CONTENT" | sed -n "$((LINE+1))p" | cut -c"$((COL+1))" | od -An -tu1 | tr -d ' ')

if [ "$ACTUAL_BYTE" != "$BYTE" ]; then
    echo "❌ Byte mismatch: expected $BYTE, got $ACTUAL_BYTE"
    exit 1
fi
echo "✅ Byte matches"

# 4. Show author info
echo "[4/4] Author information..."
AUTHOR=$(git show -s --format='%an' "$COMMIT")
DATE=$(git show -s --format='%ci' "$COMMIT")
echo "   Author: $AUTHOR"
echo "   Date: $DATE"

echo ""
echo "🎉 Byte argument verified!"
echo "   This byte provably came from commit $COMMIT"
echo "   Anyone can verify this using only public git data"
