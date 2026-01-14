{
  description = "Ethereum contract: MakerDAO_DAI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "MakerDAO_DAI";
        
        buildInputs = [ pkgs.foundry pkgs.solc ];
        
        src = pkgs.writeTextFile {
          name = "contract.sol";
          text = ''
            // SPDX-License-Identifier: MIT
            pragma solidity ^0.8.0;
            
            // MakerDAO_DAI at 0x6B175474E89094C44Da98b954EedeAC495271d0F
            // Fetch bytecode: cast code 0x6B175474E89094C44Da98b954EedeAC495271d0F --rpc-url \$ETH_RPC_URL
            
            contract MakerDAO_DAI_Stub {
                address constant MAINNET_ADDRESS = 0x6B175474E89094C44Da98b954EedeAC495271d0F;
            }
          '';
        };
        
        buildPhase = ''
          solc --bin --abi $src -o .
        '';
        
        installPhase = ''
          mkdir -p $out
          cp *.bin *.abi $out/ 2>/dev/null || true
          echo "0x6B175474E89094C44Da98b954EedeAC495271d0F" > $out/address.txt
          echo "MakerDAO_DAI" > $out/name.txt
        '';
      };
    };
}
