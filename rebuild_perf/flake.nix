{
  description = "Record complete compiler execution";
  
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
    
    # Record rustc compiling a larger program
    rustc-full = pkgs.stdenv.mkDerivation {
      name = "rustc-full-perf";
      nativeBuildInputs = [ pkgs.perf pkgs.rustc ];
      dontUnpack = true;
      
      buildPhase = ''
        # Create a larger Rust program
        cat > large.rs << 'RUST'
// Large program to exercise rustc
use std::collections::HashMap;

struct Const71 {
    value: i32,
    map: HashMap<String, i32>,
}

impl Const71 {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("const".to_string(), 71);
        Self { value: 71, map }
    }
    
    fn compute(&self) -> i32 {
        self.value + self.map.get("const").unwrap_or(&0)
    }
}

fn main() {
    let c = Const71::new();
    println!("{}", c.compute());
}
RUST
        
        # Record rustc with optimizations
    # TODO: Migrate to use perf-lib.lib.perfBuild
    # See: docs/nix/PERF_FLAKE_TEMPLATE.md
        ${pkgs.perf}/bin/perf record -o rustc_full.perf.data -F 99 -g \
          ${pkgs.rustc}/bin/rustc large.rs -C opt-level=3 -o const71 2>&1 || true
        
        echo "Recorded $(wc -c < rustc_full.perf.data) bytes"
      '';
      
      installPhase = ''
        mkdir -p $out
        cp rustc_full.perf.data $out/
      '';
    };
    
  in {
    packages.${system}.default = rustc-full;
  };
}
