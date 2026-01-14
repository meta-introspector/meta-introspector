// perf-macros/src/lib.rs
// Proc macros for wrapping any code in perf recording + parquet capture

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Expr};

/// Wrap a function in perf recording (auto telemetry, no return change)
/// 
/// # Example
/// ```
/// #[perf_auto]
/// fn my_function(x: i32) -> i32 {
///     x * 2
/// }
/// ```
#[proc_macro_attribute]
pub fn perf_auto(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();
    let fn_block = &input.block;
    let fn_sig = &input.sig;
    let fn_vis = &input.vis;
    let fn_attrs = &input.attrs;
    
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            let mut __perf_session = perf_runtime::PerfSession::start(#fn_name_str);
            let __result = #fn_block;
            let __perf_data = __perf_session.stop();
            
            // Send to telemetry server (async, non-blocking)
            perf_runtime::telemetry_send(&__perf_data);
            
            __result
        }
    };
    
    TokenStream::from(expanded)
}

/// Wrap a code block in perf recording (returns tuple with perf data)
/// 
/// # Example
/// ```
/// let (result, perf_data) = perf!({
///     expensive_computation()
/// });
/// ```
#[proc_macro]
pub fn perf(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    
    let expanded = quote! {
        {
            let mut __perf_session = perf_runtime::PerfSession::start("inline_block");
            let __result = #expr;
            let __perf_data = __perf_session.stop();
            
            // Send to telemetry server
            perf_runtime::telemetry_send(&__perf_data);
            
            (__result, __perf_data)
        }
    };
    
    TokenStream::from(expanded)
}

/// Wrap a function in perf recording (returns perf data alongside result)
/// 
/// # Example
/// ```
/// #[perf_record]
/// fn my_function(x: i32) -> i32 {
///     x * 2
/// }
/// // Returns: (i32, PerfData)
/// ```
#[proc_macro_attribute]
pub fn perf_record(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();
    let fn_block = &input.block;
    let fn_vis = &input.vis;
    let fn_attrs = &input.attrs;
    
    // Modify return type to include PerfData
    let mut fn_sig = input.sig.clone();
    let original_return = fn_sig.output.clone();
    
    fn_sig.output = syn::parse_quote! {
        -> (#original_return, perf_runtime::PerfData)
    };
    
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            let mut __perf_session = perf_runtime::PerfSession::start(#fn_name_str);
            let __result = #fn_block;
            let __perf_data = __perf_session.stop();
            
            // Send to telemetry server
            perf_runtime::telemetry_send(&__perf_data);
            
            (__result, __perf_data)
        }
    };
    
    TokenStream::from(expanded)
}

/// Capture any value or type as parquet data feed
/// 
/// # Example
/// ```
/// #[perf_probe]
/// fn process_data(x: i32, y: String) -> Vec<u8> {
///     // All inputs and output captured to parquet
///     vec![x as u8]
/// }
/// ```
#[proc_macro_attribute]
pub fn perf_probe(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();
    let fn_block = &input.block;
    let fn_sig = &input.sig;
    let fn_vis = &input.vis;
    let fn_attrs = &input.attrs;
    
    // Extract parameter names
    let param_names: Vec<_> = fn_sig.inputs.iter().filter_map(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                return Some(&pat_ident.ident);
            }
        }
        None
    }).collect();
    
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            // Start probe session
            let mut __probe_session = perf_runtime::ProbeSession::start(#fn_name_str);
            
            // Capture inputs
            #(
                __probe_session.capture_input(stringify!(#param_names), &#param_names);
            )*
            
            // Execute function
            let __result = #fn_block;
            
            // Capture output
            __probe_session.capture_output(&__result);
            
            // Write to parquet
            __probe_session.write_parquet();
            
            __result
        }
    };
    
    TokenStream::from(expanded)
}

/// Capture a value to parquet (inline)
/// 
/// # Example
/// ```
/// let x = 42;
/// probe!(x, "my_value");
/// ```
#[proc_macro]
pub fn probe(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    
    let expanded = quote! {
        {
            perf_runtime::probe_capture(stringify!(#expr), &#expr);
        }
    };
    
    TokenStream::from(expanded)
}
