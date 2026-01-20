use wasmtime::*;
use std::fs;

fn main() -> Result<()> {
    println!("🧪 Testing solana-p2p WASM plugin...");
    
    // Load WASM
    let wasm_path = std::env::args().nth(1)
        .unwrap_or_else(|| "/home/mdupont/zos-server/www/solana_p2p_plugin_bg.wasm".to_string());
    
    let wasm_bytes = fs::read(&wasm_path)?;
    println!("📦 Loaded WASM: {} bytes", wasm_bytes.len());
    
    // Create engine and module
    let engine = Engine::default();
    let module = Module::new(&engine, &wasm_bytes)?;
    
    println!("✅ WASM module validated");
    
    // List exports
    println!("\n📋 Exports:");
    for export in module.exports() {
        println!("  - {} ({:?})", export.name(), export.ty());
    }
    
    // Create store and instance
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    
    // Try to call get_block
    if let Some(func) = instance.get_func(&mut store, "solanap2p_get_block") {
        println!("\n🔍 Found get_block function");
        println!("   Type: {:?}", func.ty(&store));
    }
    
    println!("\n✅ WASM test complete!");
    Ok(())
}
