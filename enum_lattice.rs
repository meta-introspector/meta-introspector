// Enum Lattice Generator
// Start with simplest enums, build complexity lattice through auto-discovery

use quote::quote;
use syn::{parse_quote, Item};

/// Complexity level 0: Binary enum
pub fn generate_binary_enum(name: &str) -> Item {
    let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
    parse_quote! {
        enum #ident {
            Ok,
            NotOk,
        }
    }
}

/// Complexity level 1: Ternary enum
pub fn generate_ternary_enum(name: &str) -> Item {
    let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
    parse_quote! {
        enum #ident {
            Yes,
            No,
            Maybe,
        }
    }
}

/// Complexity level N: Enum with N variants
pub fn generate_enum_n(name: &str, n: usize) -> Item {
    let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
    let variants: Vec<_> = (0..n)
        .map(|i| {
            let variant_name = format!("Variant{}", i);
            let variant_ident = syn::Ident::new(&variant_name, proc_macro2::Span::call_site());
            quote! { #variant_ident }
        })
        .collect();
    
    parse_quote! {
        enum #ident {
            #(#variants),*
        }
    }
}

/// Prime number signal: Enum with prime number of variants
pub fn generate_prime_enum(name: &str, prime: usize) -> Item {
    assert!(is_prime(prime), "Must be prime");
    generate_enum_n(name, prime)
}

fn is_prime(n: usize) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    
    let sqrt = (n as f64).sqrt() as usize;
    for i in (3..=sqrt).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

/// Combine two enums into a product type
pub fn combine_enums(name: &str, enum1: &str, enum2: &str) -> Item {
    let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
    let e1 = syn::Ident::new(enum1, proc_macro2::Span::call_site());
    let e2 = syn::Ident::new(enum2, proc_macro2::Span::call_site());
    
    parse_quote! {
        enum #ident {
            Both(#e1, #e2),
            First(#e1),
            Second(#e2),
            Neither,
        }
    }
}

/// Complexity lattice
#[derive(Debug, Clone)]
pub struct ComplexityLattice {
    pub levels: Vec<ComplexityLevel>,
}

#[derive(Debug, Clone)]
pub struct ComplexityLevel {
    pub complexity: usize,
    pub enums: Vec<String>,
    pub code: Vec<Item>,
}

impl ComplexityLattice {
    pub fn new() -> Self {
        Self { levels: Vec::new() }
    }
    
    /// Build lattice starting from simplest enums
    pub fn build(&mut self, max_complexity: usize) {
        // Level 0: Binary enums
        let mut level0 = ComplexityLevel {
            complexity: 0,
            enums: vec!["Binary".to_string()],
            code: vec![generate_binary_enum("Binary")],
        };
        self.levels.push(level0);
        
        // Level 1: Ternary enums
        let mut level1 = ComplexityLevel {
            complexity: 1,
            enums: vec!["Ternary".to_string()],
            code: vec![generate_ternary_enum("Ternary")],
        };
        self.levels.push(level1);
        
        // Level 2+: Prime enums
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
        for (i, &prime) in primes.iter().enumerate() {
            if i >= max_complexity { break; }
            
            let name = format!("Prime{}", prime);
            let mut level = ComplexityLevel {
                complexity: i + 2,
                enums: vec![name.clone()],
                code: vec![generate_prime_enum(&name, prime)],
            };
            self.levels.push(level);
        }
    }
    
    /// Auto-discover similar patterns in existing code
    pub fn discover_patterns(&mut self, source: &str) -> Vec<Pattern> {
        let mut patterns = Vec::new();
        
        // Parse source
        if let Ok(file) = syn::parse_file(source) {
            for item in file.items {
                if let Item::Enum(enum_item) = item {
                    let complexity = self.compute_complexity(&enum_item);
                    let pattern = Pattern {
                        name: enum_item.ident.to_string(),
                        complexity,
                        variants: enum_item.variants.len(),
                        source: quote!(#enum_item).to_string(),
                    };
                    patterns.push(pattern);
                }
            }
        }
        
        patterns
    }
    
    fn compute_complexity(&self, enum_item: &syn::ItemEnum) -> usize {
        let num_variants = enum_item.variants.len();
        
        // Check if it's a prime
        if is_prime(num_variants) {
            return primes_up_to(num_variants).len();
        }
        
        // Otherwise, complexity is log2 of variants
        (num_variants as f64).log2().ceil() as usize
    }
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub name: String,
    pub complexity: usize,
    pub variants: usize,
    pub source: String,
}

fn primes_up_to(n: usize) -> Vec<usize> {
    (2..=n).filter(|&x| is_prime(x)).collect()
}

/// Compress, decompress, compile cycle
pub struct EnumCycle {
    pub source: String,
    pub compressed: Vec<u8>,
    pub decompressed: String,
    pub compiled: bool,
}

impl EnumCycle {
    pub fn new(item: &Item) -> Self {
        let source = quote!(#item).to_string();
        
        Self {
            source: source.clone(),
            compressed: compress(&source),
            decompressed: String::new(),
            compiled: false,
        }
    }
    
    pub fn compress_decompress_compile(&mut self) -> Result<(), String> {
        // Decompress
        self.decompressed = decompress(&self.compressed)?;
        
        // Verify round-trip
        if self.source != self.decompressed {
            return Err("Round-trip failed".to_string());
        }
        
        // Compile
        self.compiled = compile_enum(&self.decompressed)?;
        
        Ok(())
    }
}

fn compress(source: &str) -> Vec<u8> {
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    use std::io::Write;
    
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(source.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

fn decompress(data: &[u8]) -> Result<String, String> {
    use flate2::read::ZlibDecoder;
    use std::io::Read;
    
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed)
        .map_err(|e| format!("Decompression failed: {}", e))?;
    
    Ok(decompressed)
}

fn compile_enum(source: &str) -> Result<bool, String> {
    // Try to parse as valid Rust
    syn::parse_file(source)
        .map(|_| true)
        .map_err(|e| format!("Parse failed: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_binary_enum() {
        let item = generate_binary_enum("Test");
        let code = quote!(#item).to_string();
        assert!(code.contains("enum Test"));
        assert!(code.contains("Ok"));
        assert!(code.contains("NotOk"));
    }
    
    #[test]
    fn test_prime_enum() {
        let item = generate_prime_enum("Prime7", 7);
        let code = quote!(#item).to_string();
        assert!(code.contains("enum Prime7"));
    }
    
    #[test]
    fn test_compress_cycle() {
        let item = generate_binary_enum("Test");
        let mut cycle = EnumCycle::new(&item);
        assert!(cycle.compress_decompress_compile().is_ok());
        assert!(cycle.compiled);
    }
    
    #[test]
    fn test_lattice() {
        let mut lattice = ComplexityLattice::new();
        lattice.build(5);
        assert!(lattice.levels.len() > 0);
    }
}
