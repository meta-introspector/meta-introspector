#!/bin/bash
# Monitor nix2git --all progress

echo "🔍 Monitoring nix2git --all progress..."
echo ""

while true; do
    if ! pgrep -f "nix2git --all" > /dev/null; then
        echo "✅ Process completed!"
        echo ""
        if [ -f nix_store_git_repos.txt ]; then
            echo "📊 Results:"
            echo "  Total git repos: $(wc -l < nix_store_git_repos.txt)"
            echo "  Output files:"
            ls -lh nix_store_*.{json,txt} 2>/dev/null
        fi
        break
    fi
    
    clear
    echo "🔍 nix2git --all - Live Progress"
    echo "================================"
    echo ""
    tail -15 nix2git_all.log
    echo ""
    echo "Press Ctrl+C to stop monitoring (process continues in background)"
    sleep 5
done
