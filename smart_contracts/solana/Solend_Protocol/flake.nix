{
  description = "Solana program: Solend_Protocol";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Solend_Protocol";
        
        buildInputs = [ pkgs.solana-cli ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          # Fetch program: solana program dump So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo program.so
          echo "Program: Solend_Protocol" > info.txt
          echo "Address: So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo" >> info.txt
        '';
        
        installPhase = ''
          mkdir -p $out
          echo "So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo" > $out/address.txt
          echo "Solend_Protocol" > $out/name.txt
          cp info.txt $out/
        '';
      };
    };
}
