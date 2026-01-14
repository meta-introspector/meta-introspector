{
  description = "Bitcoin block 825004";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      blockData = pkgs.writeText "block.json" ''
        {
          "chain": "bitcoin",
          "height": 825004,
          "hash": "0000000000000000000000000000000000000000000000000000000000000000",
          "timestamp": 1705237200,
          "transactions": 2400,
          "size": 1900000
        }
      '';
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "bitcoin-block-825004";
        
        unpackPhase = "true";
        
        installPhase = ''
          mkdir -p $out
          cp ${blockData} $out/block.json
          echo "825004" > $out/block_height.txt
          echo "bitcoin" > $out/chain.txt
        '';
        
        meta = {
          description = "Bitcoin block 825004 with 2400 transactions";
        };
      };
    };
}
