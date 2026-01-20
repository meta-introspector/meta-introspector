#!/usr/bin/env bash
set -euo pipefail

REPO_PATH="${1:-.}"
cd "$REPO_PATH"

echo "🔧 Generating self/flake.nix for: $REPO_PATH"

# Get git metadata
REMOTE=$(git remote get-url origin 2>/dev/null || echo "unknown")
BRANCH=$(git branch --show-current 2>/dev/null || echo "main")

# Parse GitHub URL
if [[ "$REMOTE" =~ github\.com[:/]([^/]+)/([^/.]+) ]]; then
    OWNER="${BASH_REMATCH[1]}"
    REPO="${BASH_REMATCH[2]}"
else
    echo "❌ Not a GitHub repo"
    exit 1
fi

# Create self directory
mkdir -p self

# Generate flake.nix from template
cat > self/flake.nix <<EOF
{
  description = "Self-building flake for $OWNER/$REPO";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    
    self-src = {
      url = "github:$OWNER/$REPO/$BRANCH";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, self-src }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      
      metadata = if builtins.pathExists "\${self-src}/zos/zos.toml"
        then builtins.fromTOML (builtins.readFile "\${self-src}/zos/zos.toml")
        else {};
      
      isRust = builtins.pathExists "\${self-src}/Cargo.toml";
      
    in {
      packages.\${system}.default = if isRust then
        pkgs.rustPlatform.buildRustPackage {
          pname = "$REPO";
          version = "git-\${builtins.substring 0 7 metadata.repo.commit or "unknown"}";
          src = self-src;
          cargoLock.lockFile = "\${self-src}/Cargo.lock";
        }
      else
        pkgs.stdenv.mkDerivation {
          name = "$REPO";
          src = self-src;
          installPhase = "mkdir -p \$out && cp -r . \$out/";
        };
      
      packages.\${system}.metadata = pkgs.writeTextFile {
        name = "$REPO-metadata";
        text = builtins.toJSON metadata;
        destination = "/metadata.json";
      };
    };
}
EOF

echo "✅ Created self/flake.nix"
echo "📍 Standard build: nix build $REPO_PATH/self#default"
echo "📍 Output: /nix/store/...-$REPO"
