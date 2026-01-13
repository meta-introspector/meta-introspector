use std::fs;
use syn::{parse_file, Item};
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 EXTRACTING REAL FUNCTION NAMES WITH SYN");
    
    let file_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build/compiler/rustc_data_structures/src/lib.rs";
    let content = fs::read_to_string(file_path)?;
    let syntax_tree = parse_file(&content)?;
    
    println!("📋 Functions found:");
    
    for item in syntax_tree.items {
        match item {
            Item::Fn(func) => {
                println!("  fn {}", func.sig.ident);
            }
            Item::Impl(impl_block) => {
                for impl_item in impl_block.items {
                    if let syn::ImplItem::Fn(method) = impl_item {
                        println!("  method {}", method.sig.ident);
                    }
                }
            }
            Item::Struct(s) => {
                println!("  struct {}", s.ident);
            }
            Item::Enum(e) => {
                println!("  enum {}", e.ident);
            }
            Item::Mod(m) => {
                println!("  mod {}", m.ident);
            }
            Item::Use(_u) => {
                println!("  use statement");
            }
            _ => {}
        }
    }
    
    Ok(())
}
