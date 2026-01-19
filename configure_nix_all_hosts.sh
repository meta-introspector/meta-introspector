#!/bin/bash
# Configure Nix to use local git mirror for ALL discovered hosts

NIX_CONF="$HOME/.config/nix/nix.conf"
mkdir -p "$(dirname "$NIX_CONF")"

echo "🔧 Configuring Nix to use local git mirror..."

# Backup existing config
[ -f "$NIX_CONF" ] && cp "$NIX_CONF" "$NIX_CONF.backup.$(date +%s)"

# Start fresh section
cat >> "$NIX_CONF" << 'EOF'

# ===== Local Git Mirror Configuration =====
connect-timeout = 5
stalled-download-timeout = 10
url-literals = false

EOF

count=0

# Add all discovered hosts from /mnt/data1/git
for host in /mnt/data1/git/*/; do
  hostname=$(basename "$host")
  
  # Skip non-host directories
  [[ "$hostname" == "links" ]] && continue
  [[ "$hostname" == "data" ]] && continue
  [[ "$hostname" == "file" ]] && continue
  [[ "$hostname" == "home" ]] && continue
  [[ "$hostname" == "git" ]] && continue
  [[ "$hostname" == "ssh" ]] && continue
  [[ "$hostname" =~ ^git\+ssh$ ]] && continue
  [[ "$hostname" =~ ^ssh\+ ]] && continue
  [[ "$hostname" =~ @ ]] && continue
  
  # Add https:// rewrite
  echo "git-config = url.file:///mnt/data1/git/$hostname/.insteadOf=https://$hostname/" >> "$NIX_CONF"
  
  # Add git:// rewrite
  echo "git-config = url.file:///mnt/data1/git/$hostname/.insteadOf=git://$hostname/" >> "$NIX_CONF"
  
  # Add git@ SSH rewrite
  echo "git-config = url.file:///mnt/data1/git/$hostname/.insteadOf=git@$hostname:" >> "$NIX_CONF"
  
  ((count++))
done

echo "" >> "$NIX_CONF"
echo "# ===== End Local Git Mirror =====" >> "$NIX_CONF"

echo "✅ Configured $count hosts in $NIX_CONF"
echo ""
echo "Test: cd /mnt/data1/meta-introspector && nix flake update"
echo ""
echo "Restore: mv $NIX_CONF.backup.* $NIX_CONF"
