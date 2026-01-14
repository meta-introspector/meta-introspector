#!/usr/bin/env bash
set -euo pipefail

echo "🏗️  Generating Nix Flakes for Top Smart Contracts"
echo "=================================================="

CONTRACTS_DIR="smart_contracts"
mkdir -p "$CONTRACTS_DIR"/{ethereum,solana,bitcoin}

# Generate Ethereum contract flakes
generate_ethereum_flake() {
    local name=$1
    local address=$2
    local dir="$CONTRACTS_DIR/ethereum/$name"
    
    mkdir -p "$dir"
    
    cat > "$dir/flake.nix" << EOF
{
  description = "Ethereum contract: $name";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.\${system};
    in {
      packages.\${system}.default = pkgs.stdenv.mkDerivation {
        name = "$name";
        
        buildInputs = [ pkgs.foundry pkgs.solc ];
        
        src = pkgs.writeTextFile {
          name = "contract.sol";
          text = ''
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.0;
            
            // $name at $address
            // Fetch bytecode: cast code $address --rpc-url \\\$ETH_RPC_URL
            
            contract ${name}_Stub {
                address constant MAINNET_ADDRESS = $address;
            }
          '';
        };
        
        buildPhase = ''
          solc --bin --abi \$src -o .
        '';
        
        installPhase = ''
          mkdir -p \$out
          cp *.bin *.abi \$out/ 2>/dev/null || true
          echo "$address" > \$out/address.txt
          echo "$name" > \$out/name.txt
        '';
      };
    };
}
EOF
    
    echo "  ✓ $name"
}

# Generate Solana program flakes
generate_solana_flake() {
    local name=$1
    local address=$2
    local dir="$CONTRACTS_DIR/solana/$name"
    
    mkdir -p "$dir"
    
    cat > "$dir/flake.nix" << EOF
{
  description = "Solana program: $name";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.\${system};
    in {
      packages.\${system}.default = pkgs.stdenv.mkDerivation {
        name = "$name";
        
        buildInputs = [ pkgs.solana-cli ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          # Fetch program: solana program dump $address program.so
          echo "Program: $name" > info.txt
          echo "Address: $address" >> info.txt
        '';
        
        installPhase = ''
          mkdir -p \$out
          echo "$address" > \$out/address.txt
          echo "$name" > \$out/name.txt
          cp info.txt \$out/
        '';
      };
    };
}
EOF
    
    echo "  ✓ $name"
}

# Generate Bitcoin script flakes
generate_bitcoin_flake() {
    local name=$1
    local type=$2
    local dir="$CONTRACTS_DIR/bitcoin/$name"
    
    mkdir -p "$dir"
    
    cat > "$dir/flake.nix" << EOF
{
  description = "Bitcoin script: $name ($type)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.\${system};
    in {
      packages.\${system}.default = pkgs.writeShellScriptBin "$name" ''
        #!/usr/bin/env bash
        # Bitcoin Script: $name ($type)
        
        echo "Script Type: $type"
        echo "Name: $name"
        
        # Example script pattern
        case "$type" in
          "Pay-to-PubKey-Hash")
            echo "OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG"
            ;;
          "Pay-to-Script-Hash")
            echo "OP_HASH160 <scriptHash> OP_EQUAL"
            ;;
          "SegWit v0")
            echo "OP_0 <pubKeyHash>"
            ;;
          "Taproot")
            echo "OP_1 <witnessProgram>"
            ;;
          *)
            echo "Script: $type"
            ;;
        esac
      '';
    };
}
EOF
    
    echo "  ✓ $name"
}

# Parse JSON and generate flakes
echo ""
echo "📦 Ethereum Contracts"
if [ -f "top_contracts/ethereum_contracts.json" ]; then
    jq -r '.[] | "\(.name) \(.address)"' top_contracts/ethereum_contracts.json | while read -r name address; do
        generate_ethereum_flake "$name" "$address"
    done
fi

echo ""
echo "📦 Solana Programs"
if [ -f "top_contracts/solana_programs.json" ]; then
    jq -r '.[] | "\(.name) \(.address)"' top_contracts/solana_programs.json | while read -r name address; do
        generate_solana_flake "$name" "$address"
    done
fi

echo ""
echo "📦 Bitcoin Scripts"
if [ -f "top_contracts/bitcoin_scripts.json" ]; then
    jq -r '.[] | "\(.name) \(.type)"' top_contracts/bitcoin_scripts.json | while read -r name type; do
        generate_bitcoin_flake "$name" "$type"
    done
fi

echo ""
echo "=================================================="
echo "✅ Generated flakes in $CONTRACTS_DIR/"
echo ""
echo "Test a flake:"
echo "  nix build $CONTRACTS_DIR/ethereum/Uniswap_V3_Router#"
echo "  nix build $CONTRACTS_DIR/solana/Raydium_AMM#"
echo "  nix build $CONTRACTS_DIR/bitcoin/P2PKH#"
