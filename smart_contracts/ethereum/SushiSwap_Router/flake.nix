{
  description = "Ethereum contract: SushiSwap_Router";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "SushiSwap_Router";
        
        buildInputs = [ pkgs.foundry pkgs.solc ];
        
        src = pkgs.writeTextFile {
          name = "contract.sol";
          text = ''
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.0;
            
            // SushiSwap_Router at 0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F
            // Fetch bytecode: cast code 0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F --rpc-url \$ETH_RPC_URL
            
            contract SushiSwap_Router_Stub {
                address constant MAINNET_ADDRESS = 0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F;
            }
          '';
        };
        
        buildPhase = ''
          solc --bin --abi $src -o .
        '';
        
        installPhase = ''
          mkdir -p $out
          cp *.bin *.abi $out/ 2>/dev/null || true
          echo "0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F" > $out/address.txt
          echo "SushiSwap_Router" > $out/name.txt
        '';
      };
    };
}
