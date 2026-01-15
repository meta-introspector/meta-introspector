#!/bin/bash
# Save important reflog commits as branches

# Get unique commit hashes from reflog
commits=$(awk '{print $1}' reflog.txt | sort -u)

for commit in $commits; do
    # Get commit message
    msg=$(git log -1 --format=%s $commit 2>/dev/null | head -c 50 | tr ' ' '_' | tr -cd '[:alnum:]_-')
    
    if [ -n "$msg" ]; then
        branch_name="reflog/${commit:0:8}_${msg}"
        
        # Create branch if it doesn't exist
        if ! git show-ref --verify --quiet refs/heads/$branch_name; then
            git branch $branch_name $commit 2>/dev/null && echo "✅ Saved: $branch_name"
        fi
    fi
done

echo ""
echo "📊 Saved branches:"
git branch | grep "reflog/" | wc -l
