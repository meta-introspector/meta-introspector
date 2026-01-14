{
  description = "Solana program: Mango_Markets";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Mango_Markets";
        
        buildInputs = [ pkgs.solana-cli ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          # Fetch program: solana program dump mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68 program.so
          echo "Program: Mango_Markets" > info.txt
          echo "Address: mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68" >> info.txt
        '';
        
        installPhase = ''
          mkdir -p $out
          echo "mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68" > $out/address.txt
          echo "Mango_Markets" > $out/name.txt
          cp info.txt $out/
        '';
      };
    };
}
