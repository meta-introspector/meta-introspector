{
  description = "Bitcoin block 825001";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      blockData = pkgs.writeText "block.json" ''
        {
          "chain": "bitcoin",
          "height": 825001,
          "hash": "0000000000000000000000000000000000000000000000000000000000000000",
          "timestamp": 1705235400,
          "transactions": 2100,
          "size": 1600000
        }
      '';
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "bitcoin-block-825001";
        
        unpackPhase = "true";
        
        installPhase = ''
          mkdir -p $out
          cp ${blockData} $out/block.json
          echo "825001" > $out/block_height.txt
          echo "bitcoin" > $out/chain.txt
        '';
        
        meta = {
          description = "Bitcoin block 825001 with 2100 transactions";
        };
      };
    };
}
