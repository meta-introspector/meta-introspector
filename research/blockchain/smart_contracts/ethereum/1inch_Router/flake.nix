{
  description = "Ethereum contract: 1inch_Router";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "1inch_Router";
        
        buildInputs = [ pkgs.solc ];
        
        src = pkgs.writeTextFile {
          name = "contract.sol";
          text = ''
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.0;
            
            // 1inch_Router at 0x1111111254EEB25477B68fb85Ed929f73A960582
            // Fetch bytecode: cast code 0x1111111254EEB25477B68fb85Ed929f73A960582 --rpc-url \$ETH_RPC_URL
            
            contract 1inch_Router_Stub {
                address constant MAINNET_ADDRESS = 0x1111111254EEB25477B68fb85Ed929f73A960582;
            }
          '';
        };
        
        buildPhase = ''
          solc --bin --abi $src -o .
        '';
        
        installPhase = ''
          mkdir -p $out
          cp *.bin *.abi $out/ 2>/dev/null || true
          echo "0x1111111254EEB25477B68fb85Ed929f73A960582" > $out/address.txt
          echo "1inch_Router" > $out/name.txt
        '';
      };
    };
}
