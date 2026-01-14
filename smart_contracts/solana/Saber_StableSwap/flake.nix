{
  description = "Solana program: Saber_StableSwap";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Saber_StableSwap";
        
        buildInputs = [ pkgs.solana-cli ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          # Fetch program: solana program dump SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ program.so
          echo "Program: Saber_StableSwap" > info.txt
          echo "Address: SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ" >> info.txt
        '';
        
        installPhase = ''
          mkdir -p $out
          echo "SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ" > $out/address.txt
          echo "Saber_StableSwap" > $out/name.txt
          cp info.txt $out/
        '';
      };
    };
}
