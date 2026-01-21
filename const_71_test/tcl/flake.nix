{
  description = "Tcl const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "tcl-71";
      src = pkgs.writeText "const71.tcl" ''
        set x 71
        puts "x = $x"
      '';
      nativeBuildInputs = [ pkgs.tcl ];
      buildPhase = ''
        tclsh $src > output.txt
        grep -q "x = 71" output.txt || exit 1
      '';
      installPhase = ''
        mkdir -p $out/bin
        cp $src $out/bin/const71.tcl
        cat > $out/bin/tcl-71 << 'EOF'
#!/bin/sh
${pkgs.tcl}/bin/tclsh $(dirname $0)/const71.tcl
EOF
        chmod +x $out/bin/tcl-71
      '';
    };
  };
}
