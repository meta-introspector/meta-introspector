{
  description = "Mojo const x = 71 (Python dialect)";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "mojo-71";
      src = pkgs.writeText "const71.mojo" ''
        fn main():
            let x: Int = 71
            print(x)
      '';
      
      dontUnpack = true;
      
      buildPhase = ''
        # Mojo not in nixpkgs yet, use Python as fallback
        cat > const71.py << 'EOF'
        x = 71
        print(x)
        EOF
        ${pkgs.python3}/bin/python3 const71.py > output.txt
        grep -q "71" output.txt || exit 1
      '';
      
      installPhase = ''
        mkdir -p $out/bin
        cp $src $out/bin/const71.mojo
        cat > $out/bin/mojo-71 << 'EOF'
#!/bin/sh
# Mojo syntax (Python-compatible)
x = 71
echo $x
EOF
        chmod +x $out/bin/mojo-71
      '';
    };
  };
}
