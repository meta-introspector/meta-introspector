{
  description = "Bitcoin block 825002";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      blockData = pkgs.writeText "block.json" ''
        {
          "chain": "bitcoin",
          "height": 825002,
          "hash": "0000000000000000000000000000000000000000000000000000000000000000",
          "timestamp": 1705236000,
          "transactions": 2200,
          "size": 1700000
        }
      '';
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "bitcoin-block-825002";
        
        unpackPhase = "true";
        
        installPhase = ''
          mkdir -p $out
          cp ${blockData} $out/block.json
          echo "825002" > $out/block_height.txt
          echo "bitcoin" > $out/chain.txt
        '';
        
        meta = {
          description = "Bitcoin block 825002 with 2200 transactions";
        };
      };
    };
}
