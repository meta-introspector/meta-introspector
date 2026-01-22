{
  description = "Julia const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "julia-71";
      src = pkgs.writeText "const71.jl" ''
        const x = 71
        println("x = $x")
      '';
      nativeBuildInputs = [ pkgs.julia-bin ];
      dontUnpack = true;
      buildPhase = ''
        julia $src > output.txt
        grep -q "x = 71" output.txt || exit 1
      '';
      installPhase = ''
        mkdir -p $out/bin
        cp $src $out/bin/const71.jl
        cat > $out/bin/julia-71 << 'EOF'
#!/bin/sh
${pkgs.julia-bin}/bin/julia $(dirname $0)/const71.jl
EOF
        chmod +x $out/bin/julia-71
      '';
    };
  };
}
