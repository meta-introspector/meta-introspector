// perf_runtime_macros - Procedural macros for performance instrumentation
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Automatic performance tracking attribute
/// Wraps function with timing instrumentation
#[proc_macro_attribute]
pub fn perf_auto(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_sig = &input.sig;
    let fn_vis = &input.vis;
    let fn_attrs = &input.attrs;

    let output = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            let _perf_start = std::time::Instant::now();
            let _perf_fn_name = stringify!(#fn_name);
            
            let result = (|| #fn_block)();
            
            let _perf_duration = _perf_start.elapsed();
            eprintln!("[PERF] {} took {:?}", _perf_fn_name, _perf_duration);
            
            result
        }
    };

    TokenStream::from(output)
}

/// Performance probe insertion attribute
/// Marks function for perf probe instrumentation
#[proc_macro_attribute]
pub fn perf_probe(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let fn_sig = &input.sig;
    let fn_vis = &input.vis;
    let fn_attrs = &input.attrs;

    let output = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
    // Use: crate::perf::record() - see src/perf/mod.rs
            eprintln!("[PROBE] Entering {}", stringify!(#fn_name));
            
            let result = (|| #fn_block)();
            
            eprintln!("[PROBE] Exiting {}", stringify!(#fn_name));
            
            result
        }
    };

    TokenStream::from(output)
}
