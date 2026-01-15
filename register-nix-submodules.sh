#!/bin/bash
# Register all submodules from ~/nix/.gitmodules into git-sources

GITMODULES="$HOME/nix/.gitmodules"
BASE_DIR="$HOME/nix"

echo "🔍 Extracting submodules from $GITMODULES"

# Parse .gitmodules and register each submodule
awk '
/^\[submodule/ { 
    if (path != "" && url != "") {
        print path "|" url
    }
    path = ""
    url = ""
}
/path = / { 
    sub(/.*path = /, "")
    gsub(/^[ \t]+|[ \t]+$/, "")
    path = $0
}
/url = / {
    sub(/.*url = /, "")
    gsub(/^[ \t]+|[ \t]+$/, "")
    url = $0
}
END {
    if (path != "" && url != "") {
        print path "|" url
    }
}
' "$GITMODULES" | while IFS='|' read -r path url; do
    full_path="$BASE_DIR/$path"
    
    if [ -d "$full_path/.git" ] || [ -f "$full_path/.git" ]; then
        name=$(basename "$path")
        echo "Registering: $name"
        ./target/release/git-sources register "$name" "$full_path" 2>&1 | grep -v "^Registered"
    else
        echo "⚠️  Skipping $path (not initialized)"
    fi
done

echo ""
echo "✅ Registration complete"
./target/release/git-sources list | head -5
