{
  description = "Record actual execution for multiple languages";
  
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    
    perf_wrapper = pkgs.writeShellScriptBin "perf_wrapper" ''
      PERF_OUTPUT="''${PERF_OUTPUT:-/tmp/perf_$$.data}"
      exec ${pkgs.perf}/bin/perf record -o "$PERF_OUTPUT" -F 99 -g "$@"
    '';
    
    # Template for recording language execution
    mkLangPerf = { name, pkg, cmd, src }: pkgs.stdenv.mkDerivation {
      name = "${name}-actual-perf";
      inherit src;
      nativeBuildInputs = [ pkg pkgs.perf perf_wrapper ];
      dontUnpack = true;
      
      buildPhase = ''
        export PERF_OUTPUT=$PWD/${name}_actual.perf.data
        perf_wrapper ${cmd} || true
        echo "Recorded $(wc -c < $PERF_OUTPUT) bytes"
      '';
      
      installPhase = ''
        mkdir -p $out
        cp ${name}_actual.perf.data $out/
      '';
    };
    
  in {
    packages.${system} = {
      # Rust compilation
      rust = mkLangPerf {
        name = "rust";
        pkg = pkgs.rustc;
        cmd = "${pkgs.rustc}/bin/rustc $src -o const71";
        src = pkgs.writeText "const71.rs" "const CONST_71: i32 = 71; fn main() { println!(\"{}\", CONST_71); }";
      };
      
      # Python execution
      python = mkLangPerf {
        name = "python";
        pkg = pkgs.python3;
        cmd = "${pkgs.python3}/bin/python3 $src";
        src = pkgs.writeText "const71.py" "CONST_71 = 71\nprint(CONST_71)";
      };
      
      # Haskell compilation
      haskell = mkLangPerf {
        name = "haskell";
        pkg = pkgs.ghc;
        cmd = "${pkgs.ghc}/bin/ghc $src -o const71";
        src = pkgs.writeText "Const71.hs" "main = print 71";
      };
      
      # Coq type-checking
      coq = mkLangPerf {
        name = "coq";
        pkg = pkgs.coq;
        cmd = "${pkgs.coq}/bin/coqc $src";
        src = pkgs.writeText "Const71.v" ''
          Definition const71 : nat := 71.
          Check const71.
        '';
      };
      
      # OCaml compilation
      ocaml = mkLangPerf {
        name = "ocaml";
        pkg = pkgs.ocaml;
        cmd = "${pkgs.ocaml}/bin/ocamlopt $src -o const71";
        src = pkgs.writeText "const71.ml" "let const71 = 71 in print_int const71";
      };
      
      # Lua execution
      lua = mkLangPerf {
        name = "lua";
        pkg = pkgs.lua;
        cmd = "${pkgs.lua}/bin/lua $src";
        src = pkgs.writeText "const71.lua" "local x = 71; print(x)";
      };
      
      # Ruby execution
      ruby = mkLangPerf {
        name = "ruby";
        pkg = pkgs.ruby;
        cmd = "${pkgs.ruby}/bin/ruby $src";
        src = pkgs.writeText "const71.rb" "CONST_71 = 71; puts CONST_71";
      };
      
      # Aggregate all
      all = pkgs.runCommand "all-language-perf" {
        inherit (self.packages.${system}) rust python haskell coq ocaml lua ruby;
      } ''
        mkdir -p $out
        
        for lang in rust python haskell coq ocaml lua ruby; do
          eval "perf=\$$lang"
          cp $perf/*.perf.data $out/ || true
        done
        
        echo "🔬 Collected perf data for 7 languages" > $out/summary.txt
        ls -lh $out/*.perf.data >> $out/summary.txt
      '';
      
      default = self.packages.${system}.all;
    };
  };
}
