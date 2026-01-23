{
  description = "Test Gemini impure build from GitHub";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    gemini-cli.url = "github:meta-introspector/gemini-cli?ref=feature/CRQ-016-nixify-2025-10-06";
  };

  outputs = { self, nixpkgs, gemini-cli }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        pname = "gemini-test-github";
        version = "1.0";
        
        src = pkgs.writeText "test" "71";
        dontUnpack = true;
        
        __impure = true;
        
        buildInputs = [ pkgs.nodejs_22 gemini-cli.packages.${system}.default ];
        
        buildPhase = ''
          echo "Testing Gemini from GitHub"
          timeout 5 ${gemini-cli.packages.${system}.default}/bin/gemini --version || true
        '';
        
        installPhase = ''
          mkdir -p $out
          echo "71" > $out/result.txt
        '';
      };
    };
}
