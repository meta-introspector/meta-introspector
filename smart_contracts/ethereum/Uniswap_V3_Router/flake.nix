{
  description = "Ethereum contract: Uniswap_V3_Router";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Uniswap_V3_Router";
        
        buildInputs = [ pkgs.foundry pkgs.solc ];
        
        src = pkgs.writeTextFile {
          name = "contract.sol";
          text = ''
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.0;
            
            // Uniswap_V3_Router at 0xE592427A0AEce92De3Edee1F18E0157C05861564
            // Fetch bytecode: cast code 0xE592427A0AEce92De3Edee1F18E0157C05861564 --rpc-url \$ETH_RPC_URL
            
            contract Uniswap_V3_Router_Stub {
                address constant MAINNET_ADDRESS = 0xE592427A0AEce92De3Edee1F18E0157C05861564;
            }
          '';
        };
        
        buildPhase = ''
          solc --bin --abi $src -o .
        '';
        
        installPhase = ''
          mkdir -p $out
          cp *.bin *.abi $out/ 2>/dev/null || true
          echo "0xE592427A0AEce92De3Edee1F18E0157C05861564" > $out/address.txt
          echo "Uniswap_V3_Router" > $out/name.txt
        '';
      };
    };
}
