{
  description = "Solana program: Drift_Protocol";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "Drift_Protocol";
        
        buildInputs = [ pkgs.solana-cli ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          # Fetch program: solana program dump dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH program.so
          echo "Program: Drift_Protocol" > info.txt
          echo "Address: dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH" >> info.txt
        '';
        
        installPhase = ''
          mkdir -p $out
          echo "dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH" > $out/address.txt
          echo "Drift_Protocol" > $out/name.txt
          cp info.txt $out/
        '';
      };
    };
}
