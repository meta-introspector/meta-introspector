{
  description = "Solana program: Marinade_Finance";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Marinade_Finance";
        
        buildInputs = [ pkgs.solana-cli ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          # Fetch program: solana program dump MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD program.so
          echo "Program: Marinade_Finance" > info.txt
          echo "Address: MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD" >> info.txt
        '';
        
        installPhase = ''
          mkdir -p $out
          echo "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD" > $out/address.txt
          echo "Marinade_Finance" > $out/name.txt
          cp info.txt $out/
        '';
      };
    };
}
