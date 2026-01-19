#!/bin/bash
# Configure git to use local file:// for all discovered hosts

echo "🔧 Configuring git to use local file:// mirror for all hosts..."

# Get all hostnames from /mnt/data1/git
for host in /mnt/data1/git/*/; do
  hostname=$(basename "$host")
  
  # Skip special dirs
  [[ "$hostname" == "links" ]] && continue
  
  # Add file:// redirect for https://
  git config --local url."file:///mnt/data1/git/$hostname/".insteadOf "https://$hostname/"
  
  # Add for git:// protocol
  git config --local url."file:///mnt/data1/git/$hostname/".insteadOf "git://$hostname/"
  
  # Add for git@ SSH
  git config --local url."file:///mnt/data1/git/$hostname/".insteadOf "git@$hostname:"
  
  echo "  ✅ $hostname"
done

echo ""
echo "✅ Configured $(find /mnt/data1/git -maxdepth 1 -type d | wc -l) hosts"
echo ""
echo "Test: git clone https://github.com/meta-introspector/meta-introspector"
