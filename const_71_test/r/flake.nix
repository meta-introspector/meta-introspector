{
  description = "R const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "r-71";
      src = pkgs.writeText "const71.R" ''
        x <- 71
        cat("x =", x, "\n")
      '';
      nativeBuildInputs = [ pkgs.R ];
      buildPhase = ''
        Rscript $src > output.txt
        grep -q "x = 71" output.txt || exit 1
      '';
      installPhase = ''
        mkdir -p $out/bin
        cp $src $out/bin/const71.R
        cat > $out/bin/r-71 << 'EOF'
#!/bin/sh
${pkgs.R}/bin/Rscript $(dirname $0)/const71.R
EOF
        chmod +x $out/bin/r-71
      '';
    };
  };
}
