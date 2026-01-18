{
  description = "Ethereum contract: Curve_3pool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Curve_3pool";
        
        buildInputs = [ pkgs.solc ];
        
        src = pkgs.writeTextFile {
          name = "contract.sol";
          text = ''
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.0;
            
            // Curve_3pool at 0xbEbc44782C7dB0a1A60Cb6fe97d0b483032FF1C7
            // Fetch bytecode: cast code 0xbEbc44782C7dB0a1A60Cb6fe97d0b483032FF1C7 --rpc-url \$ETH_RPC_URL
            
            contract Curve_3pool_Stub {
                address constant MAINNET_ADDRESS = 0xbEbc44782C7dB0a1A60Cb6fe97d0b483032FF1C7;
            }
          '';
        };
        
        buildPhase = ''
          solc --bin --abi $src -o .
        '';
        
        installPhase = ''
          mkdir -p $out
          cp *.bin *.abi $out/ 2>/dev/null || true
          echo "0xbEbc44782C7dB0a1A60Cb6fe97d0b483032FF1C7" > $out/address.txt
          echo "Curve_3pool" > $out/name.txt
        '';
      };
    };
}
