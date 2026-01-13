// 🔥 MASTER ALL CALLS WRAPPER
// Session: allcalls_1768321896

// Include all binary wrappers:
// include!("ld_all_calls_wrapper.rs");
// include!("as_all_calls_wrapper.rs");
// include!("rustc_all_calls_wrapper.rs");
// include!("gcc_all_calls_wrapper.rs");
// include!("sh_all_calls_wrapper.rs");
// include!("cc_all_calls_wrapper.rs");
// include!("gcc_all_calls_wrapper.rs");
// include!("cc_all_calls_wrapper.rs");
// include!("ld_all_calls_wrapper.rs");
// include!("readlink_all_calls_wrapper.rs");
// include!("ld_all_calls_wrapper.rs");
// include!("cc_all_calls_wrapper.rs");
// include!("collect2_all_calls_wrapper.rs");
// include!("rustc_all_calls_wrapper.rs");

macro_rules! init_all_call_wrappers {
    () => {{
        println!("🔥 INITIALIZING ALL CALL WRAPPERS");
        println!("  1. ld ({} libs, {} syms)", 0, 0);
        println!("  2. as ({} libs, {} syms)", 2, 6);
        println!("  3. rustc ({} libs, {} syms)", 0, 0);
        println!("  4. gcc ({} libs, {} syms)", 2, 6);
        println!("  5. sh ({} libs, {} syms)", 2, 0);
        println!("  6. cc ({} libs, {} syms)", 0, 0);
        println!("  7. gcc ({} libs, {} syms)", 3, 6);
        println!("  8. cc ({} libs, {} syms)", 0, 0);
        println!("  9. ld ({} libs, {} syms)", 6, 6);
        println!("  10. readlink ({} libs, {} syms)", 5, 2);
        println!("  11. ld ({} libs, {} syms)", 2, 6);
        println!("  12. cc ({} libs, {} syms)", 0, 0);
        println!("  13. collect2 ({} libs, {} syms)", 3, 6);
        println!("  14. rustc ({} libs, {} syms)", 14, 0);
        println!("✅ All call wrappers initialized!");
    }};
}
