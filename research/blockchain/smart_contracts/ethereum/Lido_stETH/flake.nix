{
  description = "Ethereum contract: Lido_stETH";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Lido_stETH";
        
        buildInputs = [ pkgs.solc ];
        
        src = pkgs.writeTextFile {
          name = "contract.sol";
          text = ''
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.0;
            
            // Lido_stETH at 0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84
            // Fetch bytecode: cast code 0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84 --rpc-url \$ETH_RPC_URL
            
            contract Lido_stETH_Stub {
                address constant MAINNET_ADDRESS = 0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84;
            }
          '';
        };
        
        buildPhase = ''
          solc --bin --abi $src -o .
        '';
        
        installPhase = ''
          mkdir -p $out
          cp *.bin *.abi $out/ 2>/dev/null || true
          echo "0xae7ab96520DE3A18E5e111B5EaAb095312D7fE84" > $out/address.txt
          echo "Lido_stETH" > $out/name.txt
        '';
      };
    };
}
