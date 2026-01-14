{
  description = "Ethereum contract: Compound_cETH";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Compound_cETH";
        
        buildInputs = [ pkgs.foundry pkgs.solc ];
        
        src = pkgs.writeTextFile {
          name = "contract.sol";
          text = ''
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.0;
            
            // Compound_cETH at 0x4Ddc2D193948926D02f9B1fE9e1daa0718270ED5
            // Fetch bytecode: cast code 0x4Ddc2D193948926D02f9B1fE9e1daa0718270ED5 --rpc-url \$ETH_RPC_URL
            
            contract Compound_cETH_Stub {
                address constant MAINNET_ADDRESS = 0x4Ddc2D193948926D02f9B1fE9e1daa0718270ED5;
            }
          '';
        };
        
        buildPhase = ''
          solc --bin --abi $src -o .
        '';
        
        installPhase = ''
          mkdir -p $out
          cp *.bin *.abi $out/ 2>/dev/null || true
          echo "0x4Ddc2D193948926D02f9B1fE9e1daa0718270ED5" > $out/address.txt
          echo "Compound_cETH" > $out/name.txt
        '';
      };
    };
}
