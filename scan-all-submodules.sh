#!/bin/bash
# Recursively scan all registered repos for .gitmodules and register their submodules

echo "🔍 Scanning all registered repos for submodules"

# Get all registered repo paths
./target/release/git-sources list | grep "Path:" | awk '{print $2}' | while read -r repo_path; do
    gitmodules="$repo_path/.gitmodules"
    
    if [ -f "$gitmodules" ]; then
        repo_name=$(basename "$repo_path")
        echo ""
        echo "📦 Found .gitmodules in: $repo_name"
        
        # Parse and register submodules
        awk -v base="$repo_path" '
        /^\[submodule/ { 
            if (path != "" && url != "") {
                print base "/" path "|" url
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
                print base "/" path "|" url
            }
        }
        ' "$gitmodules" | while IFS='|' read -r full_path url; do
            name=$(basename "$full_path")
            echo "  → $name ($url)"
            ./target/release/git-sources register "$name" "$full_path" 2>&1 | grep -E "Registered|Error" || true
        done
    fi
done

echo ""
echo "✅ Recursive scan complete"
echo "Total repos: $(./target/release/git-sources list | grep -c "Path:")"
