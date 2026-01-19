use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn mksingularity(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let names: Vec<&str> = input_str
        .trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .map(|s| s.trim().trim_matches('"'))
        .filter(|s| !s.is_empty())
        .collect();
    
    let fields = names.iter().map(|n| match *n {
        "godel" => quote! { meta: Box<Option<Singularity>>, oracle: fn(&Self) -> bool },
        "quine" => quote! { source: &'static str },
        "escher" => quote! { level: usize },
        "bach" => quote! { voices: Vec<String> },
        "hofstadter" => quote! { analogies: Vec<(String, String)> },
        "minsky" => quote! { agents: Vec<String> },
        "stallman" => quote! { free: bool },
        "torvalds" => quote! { version: u32 },
        "satoshi" => quote! { consensus: bool },
        "eco" => quote! { signs: Vec<String> },
        _ => quote! {}
    });
    
    let methods = names.iter().map(|n| match *n {
        "godel" => quote! { pub fn prove_self(&self) -> bool { (self.oracle)(self) } },
        "quine" => quote! { pub fn print_self(&self) { println!("{}", self.source) } },
        "escher" => quote! { pub fn ascend(&mut self) { self.level += 1 } },
        "hofstadter" => quote! { pub fn find_analogy(&mut self, a: &str, b: &str) { self.analogies.push((a.to_string(), b.to_string())) } },
        "torvalds" => quote! { pub fn evolve(&mut self) { self.version += 1 } },
        "satoshi" => quote! { pub fn mine(&mut self) { self.consensus = true } },
        _ => quote! {}
    });
    
    let inits = names.iter().map(|n| match *n {
        "godel" => quote! { meta: Box::new(None), oracle: |_| true },
        "quine" => quote! { source: "mksingularity!([...])" },
        "escher" => quote! { level: 0 },
        "bach" => quote! { voices: vec![] },
        "hofstadter" => quote! { analogies: vec![] },
        "minsky" => quote! { agents: vec![] },
        "stallman" => quote! { free: true },
        "torvalds" => quote! { version: 1 },
        "satoshi" => quote! { consensus: false },
        "eco" => quote! { signs: vec![] },
        _ => quote! {}
    });
    
    let name_strs: Vec<_> = names.iter().map(|n| n.to_string()).collect();
    
    quote! {
        #[derive(Clone)]
        pub struct Singularity {
            #(#fields,)*
        }
        
        impl Singularity {
            pub fn new() -> Self {
                Self {
                    #(#inits,)*
                }
            }
            
            #(#methods)*
            
            pub fn run(&mut self) {
                println!("🌌 Singularity initialized with: {:?}", vec![#(#name_strs),*]);
            }
        }
    }.into()
}
