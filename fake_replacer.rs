use syn::{visit_mut::VisitMut, Expr, ExprLit, Lit, LitInt, LitStr, parse_quote};
use std::fs;
use std::path::Path;

struct FakeDataReplacer {
    replacements: usize,
}

impl FakeDataReplacer {
    fn new() -> Self {
        Self { replacements: 0 }
    }

    fn is_fake_int(&self, val: u64) -> bool {
        matches!(val, 42 | 123 | 999 | 1234)
    }

    fn is_fake_string(&self, s: &str) -> bool {
        let lower = s.to_lowercase();
        lower.contains("test") 
            || lower.contains("example")
            || lower.contains("placeholder")
            || lower.contains("mock")
            || lower.contains("fake")
            || lower.contains("demo")
    }
}

impl VisitMut for FakeDataReplacer {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if let Expr::Lit(ExprLit { lit: Lit::Int(lit_int), .. }) = expr {
            if let Ok(val) = lit_int.base10_parse::<u64>() {
                if self.is_fake_int(val) {
                    *expr = parse_quote! {
                        panic!("FAKE DATA DETECTED: hardcoded value {} - replace with real data source", #val)
                    };
                    self.replacements += 1;
                }
            }
        }

        if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = expr {
            let s = lit_str.value();
            if self.is_fake_string(&s) {
                *expr = parse_quote! {
                    panic!("FAKE DATA DETECTED: suspicious string {:?} - replace with real data source", #s)
                };
                self.replacements += 1;
            }
        }

        syn::visit_mut::visit_expr_mut(self, expr);
    }
}

fn replace_fake_data(file_path: &Path) -> std::io::Result<usize> {
    let content = fs::read_to_string(file_path)?;
    let mut syntax = syn::parse_file(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut replacer = FakeDataReplacer::new();
    replacer.visit_file_mut(&mut syntax);

    if replacer.replacements > 0 {
        let new_content = prettyplease::unparse(&syntax);
        fs::write(file_path, new_content)?;
        println!("✅ {}: Replaced {} fake data instances", file_path.display(), replacer.replacements);
    }

    Ok(replacer.replacements)
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: fake_replacer <file_or_dir>");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let mut total_replacements = 0;

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                match replace_fake_data(&path) {
                    Ok(n) => total_replacements += n,
                    Err(e) => eprintln!("Error processing {}: {}", path.display(), e),
                }
            }
        }
    } else {
        total_replacements = replace_fake_data(path)?;
    }

    println!("\n=== Summary ===");
    println!("Total fake data replaced: {}", total_replacements);
    
    if total_replacements > 0 {
        println!("\n⚠️  Code will now panic at runtime if fake data is accessed");
        println!("Replace panic!() calls with real data sources");
    }

    Ok(())
}
