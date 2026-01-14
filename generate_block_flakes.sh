#!/usr/bin/env bash
set -euo pipefail

echo "🏗️  Generating Nix Flakes for Blockchain Blocks"
echo "================================================"

BLOCKS_DIR="blockchain_blocks"
mkdir -p "$BLOCKS_DIR"/{ethereum,solana,bitcoin}

# Generate Ethereum block flakes
generate_ethereum_block_flake() {
    local number=$1
    local hash=$2
    local txs=$3
    local gas=$4
    local timestamp=$5
    local dir="$BLOCKS_DIR/ethereum/block_$number"
    
    mkdir -p "$dir"
    
    cat > "$dir/flake.nix" << EOF
{
  description = "Ethereum block $number";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.\${system};
      
      blockData = pkgs.writeText "block.json" ''
        {
          "chain": "ethereum",
          "number": $number,
          "hash": "$hash",
          "timestamp": $timestamp,
          "transactions": $txs,
          "gas_used": $gas
        }
      '';
    in {
      packages.\${system}.default = pkgs.stdenv.mkDerivation {
        name = "ethereum-block-$number";
        
        unpackPhase = "true";
        
        installPhase = ''
          mkdir -p \$out
          cp \${blockData} \$out/block.json
          echo "$number" > \$out/block_number.txt
          echo "ethereum" > \$out/chain.txt
        '';
        
        meta = {
          description = "Ethereum block $number with $txs transactions";
        };
      };
    };
}
EOF
    
    echo "  ✓ Ethereum block $number ($txs txs)"
}

# Generate Solana block flakes
generate_solana_block_flake() {
    local slot=$1
    local hash=$2
    local txs=$3
    local cu=$4
    local timestamp=$5
    local dir="$BLOCKS_DIR/solana/slot_$slot"
    
    mkdir -p "$dir"
    
    cat > "$dir/flake.nix" << EOF
{
  description = "Solana slot $slot";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.\${system};
      
      blockData = pkgs.writeText "slot.json" ''
        {
          "chain": "solana",
          "slot": $slot,
          "blockhash": "$hash",
          "timestamp": $timestamp,
          "transactions": $txs,
          "compute_units": $cu
        }
      '';
    in {
      packages.\${system}.default = pkgs.stdenv.mkDerivation {
        name = "solana-slot-$slot";
        
        unpackPhase = "true";
        
        installPhase = ''
          mkdir -p \$out
          cp \${blockData} \$out/slot.json
          echo "$slot" > \$out/slot_number.txt
          echo "solana" > \$out/chain.txt
        '';
        
        meta = {
          description = "Solana slot $slot with $txs transactions";
        };
      };
    };
}
EOF
    
    echo "  ✓ Solana slot $slot ($txs txs)"
}

# Generate Bitcoin block flakes
generate_bitcoin_block_flake() {
    local height=$1
    local hash=$2
    local txs=$3
    local size=$4
    local timestamp=$5
    local dir="$BLOCKS_DIR/bitcoin/block_$height"
    
    mkdir -p "$dir"
    
    cat > "$dir/flake.nix" << EOF
{
  description = "Bitcoin block $height";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.\${system};
      
      blockData = pkgs.writeText "block.json" ''
        {
          "chain": "bitcoin",
          "height": $height,
          "hash": "$hash",
          "timestamp": $timestamp,
          "transactions": $txs,
          "size": $size
        }
      '';
    in {
      packages.\${system}.default = pkgs.stdenv.mkDerivation {
        name = "bitcoin-block-$height";
        
        unpackPhase = "true";
        
        installPhase = ''
          mkdir -p \$out
          cp \${blockData} \$out/block.json
          echo "$height" > \$out/block_height.txt
          echo "bitcoin" > \$out/chain.txt
        '';
        
        meta = {
          description = "Bitcoin block $height with $txs transactions";
        };
      };
    };
}
EOF
    
    echo "  ✓ Bitcoin block $height ($txs txs)"
}

# Parse JSON and generate flakes
echo ""
echo "📦 Ethereum Blocks"
if [ -f "$BLOCKS_DIR/ethereum_blocks.json" ]; then
    jq -r '.[] | "\(.number) \(.hash) \(.transactions) \(.gas_used) \(.timestamp)"' "$BLOCKS_DIR/ethereum_blocks.json" | \
    while read -r number hash txs gas timestamp; do
        generate_ethereum_block_flake "$number" "$hash" "$txs" "$gas" "$timestamp"
    done
fi

echo ""
echo "📦 Solana Slots"
if [ -f "$BLOCKS_DIR/solana_blocks.json" ]; then
    jq -r '.[] | "\(.slot) \(.blockhash) \(.transactions) \(.compute_units) \(.timestamp)"' "$BLOCKS_DIR/solana_blocks.json" | \
    while read -r slot hash txs cu timestamp; do
        generate_solana_block_flake "$slot" "$hash" "$txs" "$cu" "$timestamp"
    done
fi

echo ""
echo "📦 Bitcoin Blocks"
if [ -f "$BLOCKS_DIR/bitcoin_blocks.json" ]; then
    jq -r '.[] | "\(.height) \(.hash) \(.transactions) \(.size) \(.timestamp)"' "$BLOCKS_DIR/bitcoin_blocks.json" | \
    while read -r height hash txs size timestamp; do
        generate_bitcoin_block_flake "$height" "$hash" "$txs" "$size" "$timestamp"
    done
fi

echo ""
echo "================================================"
echo "✅ Generated block flakes in $BLOCKS_DIR/"
echo ""
echo "Test a flake:"
echo "  nix build $BLOCKS_DIR/ethereum/block_18900000#"
echo "  nix build $BLOCKS_DIR/solana/slot_250000000#"
echo "  nix build $BLOCKS_DIR/bitcoin/block_825000#"
