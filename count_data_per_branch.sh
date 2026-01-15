#!/bin/bash
# Count data files per reflog branch

echo "Branch,JSON,Parquet,CSV,Total"

for branch in $(git branch | grep "reflog/" | sed 's/^..//' | head -20); do
    json=$(git ls-tree -r $branch --name-only 2>/dev/null | grep -E '\.(json)$' | wc -l)
    parquet=$(git ls-tree -r $branch --name-only 2>/dev/null | grep -E '\.(parquet)$' | wc -l)
    csv=$(git ls-tree -r $branch --name-only 2>/dev/null | grep -E '\.(csv)$' | wc -l)
    total=$((json + parquet + csv))
    
    if [ $total -gt 0 ]; then
        echo "$branch,$json,$parquet,$csv,$total"
    fi
done
