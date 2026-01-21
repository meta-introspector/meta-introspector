{
  description = "Ruby const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "ruby-71";
      src = pkgs.writeText "const71.rb" ''
        x = 71
        puts "x = #{x}"
      '';
      nativeBuildInputs = [ pkgs.ruby ];
      dontUnpack = true;
      buildPhase = ''
        ruby $src > output.txt
        grep -q "x = 71" output.txt || exit 1
      '';
      installPhase = ''
        mkdir -p $out/bin
        cp $src $out/bin/const71.rb
        cat > $out/bin/ruby-71 << 'EOF'
#!/bin/sh
${pkgs.ruby}/bin/ruby $(dirname $0)/const71.rb
EOF
        chmod +x $out/bin/ruby-71
      '';
    };
  };
}
