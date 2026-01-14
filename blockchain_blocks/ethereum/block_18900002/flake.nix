{
  description = "Ethereum block 18900002";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      blockData = pkgs.writeText "block.json" ''
        {
          "chain": "ethereum",
          "number": 18900002,
          "hash": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
          "timestamp": 1705234824,
          "transactions": 170,
          "gas_used": 17000000
        }
      '';
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "ethereum-block-18900002";
        
        unpackPhase = "true";
        
        installPhase = ''
          mkdir -p $out
          cp ${blockData} $out/block.json
          echo "18900002" > $out/block_number.txt
          echo "ethereum" > $out/chain.txt
        '';
        
        meta = {
          description = "Ethereum block 18900002 with 170 transactions";
        };
      };
    };
}
