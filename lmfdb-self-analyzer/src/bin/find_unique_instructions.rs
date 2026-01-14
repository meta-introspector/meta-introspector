// Find unique instructions in common functions between enum and struct compilation
// This reveals the exact divergence points where enum vs struct parsing differs

use std::collections::{HashMap, HashSet};
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Finding Unique Instructions in Common Functions\n");
    
    let rustc_driver = "/nix/store/5r3salsfkfdyyl28c58dyk6sml48vklr-rust-default-1.94.0-nightly-2026-01-09/lib/librustc_driver-b3621a07141c9b94.so";
    
    // Get addresses hit during enum compilation
    let enum_addrs = get_perf_addresses("/tmp/enum_perf.data", rustc_driver)?;
    let struct_addrs = get_perf_addresses("/tmp/struct_perf.data", rustc_driver)?;
    
    println!("📊 Enum addresses: {}", enum_addrs.len());
    println!("📊 Struct addresses: {}", struct_addrs.len());
    
    // Find common and unique addresses
    let common: HashSet<_> = enum_addrs.intersection(&struct_addrs).collect();
    let enum_only: HashSet<_> = enum_addrs.difference(&struct_addrs).collect();
    let struct_only: HashSet<_> = struct_addrs.difference(&enum_addrs).collect();
    
    println!("📊 Common addresses: {}", common.len());
    println!("📊 Enum-only addresses: {}", enum_only.len());
    println!("📊 Struct-only addresses: {}", struct_only.len());
    
    // Disassemble and find unique instructions
    println!("\n🔍 Disassembling unique addresses...\n");
    
    let enum_instrs = disassemble_addresses(rustc_driver, &enum_only)?;
    let struct_instrs = disassemble_addresses(rustc_driver, &struct_only)?;
    
    println!("🎯 Enum-only instructions (first 20):\n");
    for (i, (addr, instr)) in enum_instrs.iter().take(20).enumerate() {
        println!("{:2}. 0x{:x}: {}", i+1, addr, instr);
    }
    
    println!("\n🎯 Struct-only instructions (first 20):\n");
    for (i, (addr, instr)) in struct_instrs.iter().take(20).enumerate() {
        println!("{:2}. 0x{:x}: {}", i+1, addr, instr);
    }
    
    // Look for character comparisons
    println!("\n🔤 Character comparison instructions:\n");
    
    let char_map = [
        (0x65, 'e'), (0x6e, 'n'), (0x75, 'u'), (0x6d, 'm'),  // enum
        (0x73, 's'), (0x74, 't'), (0x72, 'r'), (0x63, 'c'),  // struct
        (0x69, 'i'), (0x6d, 'm'), (0x70, 'p'), (0x6c, 'l'),  // impl
        (0x66, 'f'), (0x6e, 'n'),  // fn
    ];
    
    println!("Enum-only:");
    let mut enum_chars = Vec::new();
    for (addr, instr) in &enum_instrs {
        for (hex, ch) in &char_map {
            let hex_str = format!("0x{:x}", hex);
            if instr.contains("cmp") && instr.contains(&hex_str) {
                println!("  0x{:x}: {} <- '{}'", addr, instr, ch);
                enum_chars.push(*ch);
            }
        }
    }
    
    println!("\nStruct-only:");
    let mut struct_chars = Vec::new();
    for (addr, instr) in &struct_instrs {
        for (hex, ch) in &char_map {
            let hex_str = format!("0x{:x}", hex);
            if instr.contains("cmp") && instr.contains(&hex_str) {
                println!("  0x{:x}: {} <- '{}'", addr, instr, ch);
                struct_chars.push(*ch);
            }
        }
    }
    
    println!("\n📊 Character frequency:");
    println!("  Enum: {:?}", enum_chars);
    println!("  Struct: {:?}", struct_chars);
    
    Ok(())
}

fn get_perf_addresses(perf_file: &str, lib: &str) -> Result<HashSet<u64>, Box<dyn std::error::Error>> {
    let output = Command::new("perf")
        .args(&["script", "-i", perf_file, "-F", "ip"])
        .output()?;
    
    let mut addrs = HashSet::new();
    let script = String::from_utf8_lossy(&output.stdout);
    
    for line in script.lines() {
        if let Ok(addr) = u64::from_str_radix(line.trim().trim_start_matches("0x"), 16) {
            addrs.insert(addr);
        }
    }
    
    Ok(addrs)
}

fn disassemble_addresses(lib: &str, addrs: &HashSet<&u64>) -> Result<Vec<(u64, String)>, Box<dyn std::error::Error>> {
    let mut instrs = Vec::new();
    
    // Use grep to filter only relevant addresses - much faster
    let addr_pattern = addrs.iter()
        .map(|a| format!("{:x}", a))
        .collect::<Vec<_>>()
        .join("\\|");
    
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("objdump -d {} 2>/dev/null | grep -E '({}):'", lib, addr_pattern))
        .output()?;
    
    let disasm = String::from_utf8_lossy(&output.stdout);
    
    for line in disasm.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 2 {
            let addr_str = parts[0].trim();
            if let Ok(addr) = u64::from_str_radix(addr_str, 16) {
                if addrs.contains(&addr) {
                    instrs.push((addr, parts[1].trim().to_string()));
                }
            }
        }
    }
    
    Ok(instrs)
}
