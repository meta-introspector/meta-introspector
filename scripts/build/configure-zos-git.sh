#!/bin/bash
# Configure git for zos user to use local cache

set -e

echo "🔧 Configuring git for zos user..."

# Generate gitconfig in home directory (not /tmp which may have restrictions)
tmpfile="$HOME/.zos-gitconfig.tmp"

# Generate gitconfig as regular user (fast)
for host in /mnt/data1/git/*/; do
    hostname=$(basename "$host")
    
    # Skip non-host directories
    [[ "$hostname" =~ ^(links|data|file|home|git|ssh)$ ]] && continue
    [[ "$hostname" =~ @ ]] && continue
    
    echo "[url \"file:///mnt/data1/git/$hostname/\"]"
    echo "	insteadOf = https://$hostname/"
    echo "	insteadOf = git://$hostname/"
    echo "	insteadOf = git@$hostname:"
done > "$tmpfile"

# Copy to zos home
sudo cp "$tmpfile" /home/zos/.gitconfig
sudo chown zos:zos /home/zos/.gitconfig
rm -f "$tmpfile"

count=$(grep -c '^\[url' /home/zos/.gitconfig)
echo "✅ Configured $count hosts for zos user"
echo ""
echo "Verify:"
echo "  sudo -u zos git config --global --get-regexp 'url\.'"
