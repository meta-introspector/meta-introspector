{
  description = "Solana slot 250000003";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      blockData = pkgs.writeText "slot.json" ''
        {
          "chain": "solana",
          "slot": 250000003,
          "blockhash": "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB",
          "timestamp": 1705234801.2,
          "transactions": 2800,
          "compute_units": 48000000000
        }
      '';
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "solana-slot-250000003";
        
        unpackPhase = "true";
        
        installPhase = ''
          mkdir -p $out
          cp ${blockData} $out/slot.json
          echo "250000003" > $out/slot_number.txt
          echo "solana" > $out/chain.txt
        '';
        
        meta = {
          description = "Solana slot 250000003 with 2800 transactions";
        };
      };
    };
}
