#!/usr/bin/env rust-script
//! Test ZOS block collector plugin
use libloading::Library;
use std::ffi::{CStr, CString};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 Loading block-collector plugin...");
    
    let plugin = unsafe {
        Library::new("tools/so-plugins/block-collector/target/release/libblock_collector_plugin.so")?
    };
    
    // Load functions
    let register = unsafe {
        plugin.get::<unsafe extern "C" fn(*const i8) -> *const i8>(b"register_client")?
    };
    
    let submit = unsafe {
        plugin.get::<unsafe extern "C" fn(*const i8) -> *const i8>(b"submit_block")?
    };
    
    let get_contract = unsafe {
        plugin.get::<unsafe extern "C" fn() -> *const i8>(b"get_contract")?
    };
    
    // Test get_contract
    println!("\n📍 Contract:");
    let contract_ptr = unsafe { get_contract() };
    let contract = unsafe { CStr::from_ptr(contract_ptr).to_string_lossy() };
    println!("   {}", contract);
    
    // Test register_client
    println!("\n👤 Registering client...");
    let peer_id = CString::new("peer_test_123")?;
    let response_ptr = unsafe { register(peer_id.as_ptr()) };
    let response = unsafe { CStr::from_ptr(response_ptr).to_string_lossy() };
    println!("   {}", response);
    
    // Test submit_block
    println!("\n📦 Submitting block...");
    let block = serde_json::json!({
        "slot": 12345,
        "hash": "test_hash_abc",
        "transactions": ["tx1", "tx2"],
        "timestamp": 1737318000,
        "client_id": "HMEKzpgzJEfyYyqoob5uGHR9P3LF6248zbm8tWgaApim"
    });
    
    let block_json = CString::new(block.to_string())?;
    let response_ptr = unsafe { submit(block_json.as_ptr()) };
    let response = unsafe { CStr::from_ptr(response_ptr).to_string_lossy() };
    println!("   {}", response);
    
    println!("\n✅ Plugin test complete!");
    
    Ok(())
}
