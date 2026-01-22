{
  description = "Bitcoin block 825000";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      blockData = pkgs.writeText "block.json" ''
        {
          "chain": "bitcoin",
          "height": 825000,
          "hash": "0000000000000000000000000000000000000000000000000000000000000000",
          "timestamp": 1705234800,
          "transactions": 2000,
          "size": 1500000
        }
      '';
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "bitcoin-block-825000";
        
        unpackPhase = "true";
        
        installPhase = ''
          mkdir -p $out
          cp ${blockData} $out/block.json
          echo "825000" > $out/block_height.txt
          echo "bitcoin" > $out/chain.txt
        '';
        
        meta = {
          description = "Bitcoin block 825000 with 2000 transactions";
        };
      };
    };
}
