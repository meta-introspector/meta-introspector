{
  description = "Bitcoin block 825003";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      blockData = pkgs.writeText "block.json" ''
        {
          "chain": "bitcoin",
          "height": 825003,
          "hash": "0000000000000000000000000000000000000000000000000000000000000000",
          "timestamp": 1705236600,
          "transactions": 2300,
          "size": 1800000
        }
      '';
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "bitcoin-block-825003";
        
        unpackPhase = "true";
        
        installPhase = ''
          mkdir -p $out
          cp ${blockData} $out/block.json
          echo "825003" > $out/block_height.txt
          echo "bitcoin" > $out/chain.txt
        '';
        
        meta = {
          description = "Bitcoin block 825003 with 2300 transactions";
        };
      };
    };
}
