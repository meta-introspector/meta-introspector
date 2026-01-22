#!/bin/bash
# Configure git for zos user to use local cache

set -e

echo "🔧 Configuring git for zos user..."

count=0
for host in /mnt/data1/git/*/; do
    hostname=$(basename "$host")
    
    # Skip non-host directories
    [[ "$hostname" =~ ^(links|data|file|home|git|ssh)$ ]] && continue
    [[ "$hostname" =~ @ ]] && continue
    
    # Configure git URL rewrites
    sudo -u zos git config --global url."file:///mnt/data1/git/$hostname/".insteadOf "https://$hostname/"
    sudo -u zos git config --global url."file:///mnt/data1/git/$hostname/".insteadOf "git://$hostname/"
    sudo -u zos git config --global url."file:///mnt/data1/git/$hostname/".insteadOf "git@$hostname:"
    
    ((count++))
done

echo "✅ Configured $count hosts for zos user"
echo ""
echo "Verify:"
echo "  sudo -u zos git config --global --get-regexp 'url\.'"
