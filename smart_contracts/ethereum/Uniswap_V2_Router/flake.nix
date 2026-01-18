{
  description = "Ethereum contract: Uniswap_V2_Router";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Uniswap_V2_Router";
        
        buildInputs = [ pkgs.solc ];
        
        src = pkgs.writeTextFile {
          name = "contract.sol";
          text = ''
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.0;
            
            // Uniswap_V2_Router at 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
            // Fetch bytecode: cast code 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D --rpc-url \$ETH_RPC_URL
            
            contract Uniswap_V2_Router_Stub {
                address constant MAINNET_ADDRESS = 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D;
            }
          '';
        };
        
        buildPhase = ''
          solc --bin --abi $src -o .
        '';
        
        installPhase = ''
          mkdir -p $out
          cp *.bin *.abi $out/ 2>/dev/null || true
          echo "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D" > $out/address.txt
          echo "Uniswap_V2_Router" > $out/name.txt
        '';
      };
    };
}
