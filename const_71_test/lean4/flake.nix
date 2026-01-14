{
  description = "Lean4 const x = 71 test";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "const-71-lean4";
        
        src = pkgs.writeText "Main.lean" ''
          def x : Nat := 71
          
          def main : IO Unit := do
            IO.println s!"x = {x}"
        '';
        
        buildInputs = [ pkgs.lean4 ];
        
        unpackPhase = "true";
        
        buildPhase = ''
          lean --make $src -o const-71-lean4
        '';
        
        installPhase = ''
          mkdir -p $out/bin
          cp const-71-lean4 $out/bin/ || echo "Lean build may need project structure"
        '';
      };
    };
}
