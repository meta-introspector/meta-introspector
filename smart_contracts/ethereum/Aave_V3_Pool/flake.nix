{
  description = "Ethereum contract: Aave_V3_Pool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Aave_V3_Pool";
        
        buildInputs = [ pkgs.foundry pkgs.solc ];
        
        src = pkgs.writeTextFile {
          name = "contract.sol";
          text = ''
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.0;
            
            // Aave_V3_Pool at 0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2
            // Fetch bytecode: cast code 0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2 --rpc-url \$ETH_RPC_URL
            
            contract Aave_V3_Pool_Stub {
                address constant MAINNET_ADDRESS = 0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2;
            }
          '';
        };
        
        buildPhase = ''
          solc --bin --abi $src -o .
        '';
        
        installPhase = ''
          mkdir -p $out
          cp *.bin *.abi $out/ 2>/dev/null || true
          echo "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2" > $out/address.txt
          echo "Aave_V3_Pool" > $out/name.txt
        '';
      };
    };
}
