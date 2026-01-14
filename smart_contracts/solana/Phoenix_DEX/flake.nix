{
  description = "Solana program: Phoenix_DEX";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Phoenix_DEX";
        
        buildInputs = [ pkgs.solana-cli ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          # Fetch program: solana program dump PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY program.so
          echo "Program: Phoenix_DEX" > info.txt
          echo "Address: PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY" >> info.txt
        '';
        
        installPhase = ''
          mkdir -p $out
          echo "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY" > $out/address.txt
          echo "Phoenix_DEX" > $out/name.txt
          cp info.txt $out/
        '';
      };
    };
}
