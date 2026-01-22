#!/bin/bash
# Configure Git to use local file:// URLs for all hosts
# This affects nix flake update since it uses Git

echo "🔧 Configuring Git globally to use local mirror..."

# Configure for all discovered hosts
for host in /mnt/data1/git/*/; do
  hostname=$(basename "$host")
  
  # Skip non-host directories
  [[ "$hostname" == "links" ]] && continue
  [[ "$hostname" == "data" ]] && continue
  [[ "$hostname" == "file" ]] && continue
  [[ "$hostname" == "home" ]] && continue
  [[ "$hostname" == "git" ]] && continue
  [[ "$hostname" == "ssh" ]] && continue
  [[ "$hostname" =~ @ ]] && continue
  
  # Configure Git URL rewriting (global)
  git config --global url."file:///mnt/data1/git/$hostname/".insteadOf "https://$hostname/"
  git config --global url."file:///mnt/data1/git/$hostname/".insteadOf "git://$hostname/"
  git config --global url."file:///mnt/data1/git/$hostname/".insteadOf "git@$hostname:"
  
  echo "  ✅ $hostname"
done

echo ""
echo "✅ Git configured globally"
echo ""
echo "Test: nix flake update"
echo ""
echo "Verify: git config --global --get-regexp url"
echo ""
echo "To remove: git config --global --remove-section url.file:///mnt/data1/git/github.com/"
