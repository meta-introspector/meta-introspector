{
  description = "Gemini CLI integration for meta-introspector";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    gemini-cli.url = "github:meta-introspector/gemini-cli";
    gemini-cli.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, gemini-cli }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = gemini-cli.packages.${system}.default;
        
        apps.default = {
          type = "app";
          program = "${gemini-cli.packages.${system}.default}/bin/gemini";
        };
        
        devShells.default = pkgs.mkShell {
          buildInputs = [
            gemini-cli.packages.${system}.default
          ];
          
          shellHook = ''
            echo "🤖 Gemini CLI Integration"
            echo "========================"
            echo ""
            echo "Available commands:"
            echo "  gemini --help"
            echo "  nix run . -- 'your prompt'"
          '';
        };
      }
    );
}
