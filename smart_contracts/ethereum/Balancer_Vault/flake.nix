{
  description = "Ethereum contract: Balancer_Vault";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Balancer_Vault";
        
        buildInputs = [ pkgs.foundry pkgs.solc ];
        
        src = pkgs.writeTextFile {
          name = "contract.sol";
          text = ''
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.0;
            
            // Balancer_Vault at 0xBA12222222228d8Ba445958a75a0704d566BF2C8
            // Fetch bytecode: cast code 0xBA12222222228d8Ba445958a75a0704d566BF2C8 --rpc-url \$ETH_RPC_URL
            
            contract Balancer_Vault_Stub {
                address constant MAINNET_ADDRESS = 0xBA12222222228d8Ba445958a75a0704d566BF2C8;
            }
          '';
        };
        
        buildPhase = ''
          solc --bin --abi $src -o .
        '';
        
        installPhase = ''
          mkdir -p $out
          cp *.bin *.abi $out/ 2>/dev/null || true
          echo "0xBA12222222228d8Ba445958a75a0704d566BF2C8" > $out/address.txt
          echo "Balancer_Vault" > $out/name.txt
        '';
      };
    };
}
