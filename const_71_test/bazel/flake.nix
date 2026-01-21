{
  description = "Bazel build system with rules_nixpkgs: const 71";
  
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "bazel-71";
      
      src = pkgs.runCommand "bazel-src" {} ''
        mkdir -p $out
        
        # WORKSPACE with rules_nixpkgs
        cat > $out/WORKSPACE << 'EOF'
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "io_tweag_rules_nixpkgs",
    sha256 = "133f2c8d4d6d9e3b8e6b8e3b8e6b8e3b8e6b8e3b8e6b8e3b8e6b8e3b",
    strip_prefix = "rules_nixpkgs-0.10.0",
    urls = ["https://github.com/tweag/rules_nixpkgs/archive/v0.10.0.tar.gz"],
)

load("@io_tweag_rules_nixpkgs//nixpkgs:repositories.bzl", "rules_nixpkgs_dependencies")
rules_nixpkgs_dependencies()

load("@io_tweag_rules_nixpkgs//nixpkgs:nixpkgs.bzl", "nixpkgs_cc_configure")
nixpkgs_cc_configure(repository = "@nixpkgs")
EOF

        # BUILD file
        cat > $out/BUILD << 'EOF'
cc_binary(
    name = "const71",
    srcs = ["main.c"],
)
EOF

        # Source file
        cat > $out/main.c << 'EOF'
#include <stdio.h>
int main() {
  printf("71\n");
  return 0;
}
EOF
      '';
      
      nativeBuildInputs = [ pkgs.bazel_6 pkgs.cacert ];
      
      buildPhase = ''
        export HOME=$TMPDIR
        export USER=nixbld
        bazel build //:const71 --verbose_failures
      '';
      
      installPhase = ''
        mkdir -p $out/bin
        cp bazel-bin/const71 $out/bin/bazel-71
      '';
    };
  };
}
