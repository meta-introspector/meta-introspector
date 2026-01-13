use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapChain {
    pub layers: Vec<BootstrapLayer>,
    pub total_orbit_compression: f64,
    pub substrate_godel_sequence: Vec<u128>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapLayer {
    pub name: String,
    pub substrate: PrimitiveSubstrate,
    pub godel_number: u128,
    pub orbit_radius: f64,
    pub can_bootstrap_next: bool,
    pub minimal_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PrimitiveSubstrate {
    MES,        // Minimal Essential Scheme
    Scheme,     // Full Scheme
    TinyC,      // Minimal C compiler
    GCC,        // GNU Compiler Collection
    LLVM,       // LLVM IR
    OCaml,      // OCaml compiler
    Rust,       // Self-hosting Rust
}

pub struct SubstrateArchaeologist;

impl SubstrateArchaeologist {
    pub fn excavate_full_chain() -> Result<BootstrapChain> {
        let layers = vec![
            BootstrapLayer {
                name: "mes".to_string(),
                substrate: PrimitiveSubstrate::MES,
                godel_number: 2,
                orbit_radius: 1.0,
                can_bootstrap_next: true,
                minimal_code: "(define compile (lambda (s) s))".to_string(),
            },
            BootstrapLayer {
                name: "scheme".to_string(),
                substrate: PrimitiveSubstrate::Scheme,
                godel_number: 6,  // 2*3
                orbit_radius: 3.0,
                can_bootstrap_next: true,
                minimal_code: "(define (compile src) (eval (read src)))".to_string(),
            },
            BootstrapLayer {
                name: "tinyc".to_string(),
                substrate: PrimitiveSubstrate::TinyC,
                godel_number: 30, // 2*3*5
                orbit_radius: 10.0,
                can_bootstrap_next: true,
                minimal_code: "int compile(char* s) { return parse(s); }".to_string(),
            },
            BootstrapLayer {
                name: "gcc".to_string(),
                substrate: PrimitiveSubstrate::GCC,
                godel_number: 210, // 2*3*5*7
                orbit_radius: 50.0,
                can_bootstrap_next: true,
                minimal_code: "void compile() { asm(\"mov %eax, %ebx\"); }".to_string(),
            },
            BootstrapLayer {
                name: "llvm".to_string(),
                substrate: PrimitiveSubstrate::LLVM,
                godel_number: 2310, // 2*3*5*7*11
                orbit_radius: 100.0,
                can_bootstrap_next: true,
                minimal_code: "%1 = call i32 @compile(i8* %0)".to_string(),
            },
            BootstrapLayer {
                name: "ocaml".to_string(),
                substrate: PrimitiveSubstrate::OCaml,
                godel_number: 30030, // 2*3*5*7*11*13
                orbit_radius: 200.0,
                can_bootstrap_next: true,
                minimal_code: "let compile s = parse (tokenize s)".to_string(),
            },
            BootstrapLayer {
                name: "rust".to_string(),
                substrate: PrimitiveSubstrate::Rust,
                godel_number: 510510, // 2*3*5*7*11*13*17
                orbit_radius: 1000.0,
                can_bootstrap_next: false,
                minimal_code: "fn compile(s: &str) -> TokenStream { parse(s) }".to_string(),
            },
        ];
        
        let sequence = layers.iter().map(|l| l.godel_number).collect();
        let total_compression = layers.last().unwrap().orbit_radius / layers.first().unwrap().orbit_radius;
        
        Ok(BootstrapChain {
            layers,
            total_orbit_compression: total_compression,
            substrate_godel_sequence: sequence,
        })
    }
    
    pub fn generate_nix_bootstrap_chain() -> String {
        r#"
# Complete bootstrap from MES to Rust
{ pkgs ? import <nixpkgs> {} }:

let
  # Layer 0: MES (Minimal Essential Scheme)
  mes = pkgs.stdenv.mkDerivation {
    name = "mes-bootstrap";
    src = ./.;
    buildPhase = "echo '(define compile identity)' > mes.scm";
  };
  
  # Layer 1: Scheme (bootstrapped from MES)  
  scheme = pkgs.stdenv.mkDerivation {
    name = "scheme-compiler";
    buildInputs = [ mes ];
    buildPhase = "mes compile-scheme.scm";
  };
  
  # Layer 2: TinyC (compiled by Scheme)
  tinyc = pkgs.stdenv.mkDerivation {
    name = "tinyc-compiler";
    buildInputs = [ scheme ];
    buildPhase = "scheme compile-tinyc.scm";
  };
  
  # Layer 3: GCC (compiled by TinyC)
  gcc-minimal = pkgs.stdenv.mkDerivation {
    name = "gcc-minimal";
    buildInputs = [ tinyc ];
    buildPhase = "tinyc gcc-minimal.c";
  };
  
  # Layer 4: LLVM (compiled by GCC)
  llvm-minimal = pkgs.stdenv.mkDerivation {
    name = "llvm-minimal";
    buildInputs = [ gcc-minimal ];
    buildPhase = "gcc llvm-minimal.cpp";
  };
  
  # Layer 5: OCaml (compiled by GCC)
  ocaml-bootstrap = pkgs.stdenv.mkDerivation {
    name = "ocaml-bootstrap";
    buildInputs = [ gcc-minimal ];
    buildPhase = "gcc ocaml-runtime.c";
  };
  
  # Layer 6: Rust v0.1 (compiled by OCaml)
  rust-v01 = pkgs.stdenv.mkDerivation {
    name = "rust-v01";
    buildInputs = [ ocaml-bootstrap ];
    buildPhase = "ocaml rustc-v01.ml";
  };
  
  # Layer 7: Rust v1.0 (self-compiled)
  rust-v10 = pkgs.stdenv.mkDerivation {
    name = "rust-v10";
    buildInputs = [ rust-v01 ];
    buildPhase = "rustc-v01 rustc-v10.rs";
  };
  
in rust-v10
"#.to_string()
    }
    
    pub fn compute_orbit_reversal(current_godel: u128) -> Vec<u128> {
        // Reverse the prime factorization to get substrate sequence
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let mut sequence = Vec::new();
        let mut current = current_godel;
        
        // Divide by primes to get reverse sequence
        for &prime in primes.iter().rev() {
            while current % prime == 0 {
                sequence.push(current);
                current /= prime;
            }
        }
        
        sequence.reverse();
        sequence
    }
    
    pub fn minimal_substrate_code(substrate: &PrimitiveSubstrate) -> String {
        match substrate {
            PrimitiveSubstrate::MES => "(define _1 _2)".to_string(),
            PrimitiveSubstrate::Scheme => "(lambda (_1) _1)".to_string(),
            PrimitiveSubstrate::TinyC => "int _1(){return 0;}".to_string(),
            PrimitiveSubstrate::GCC => "void _1(){}".to_string(),
            PrimitiveSubstrate::LLVM => "define i32 @_1(){ret i32 0}".to_string(),
            PrimitiveSubstrate::OCaml => "let _1 = fun x -> x".to_string(),
            PrimitiveSubstrate::Rust => "fn _1()->(){()}".to_string(),
        }
    }
}
