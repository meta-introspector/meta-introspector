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
    println!("Enum:");
    for (addr, instr) in &enum_instrs {
        if instr.contains("cmp") && (instr.contains("0x65") || instr.contains("0x6e") || 
                                     instr.contains("0x75") || instr.contains("0x6d")) {
            println!("  0x{:x}: {} <- 'e','n','u','m'", addr, instr);
        }
    }
    
    println!("\nStruct:");
    for (addr, instr) in &struct_instrs {
        if instr.contains("cmp") && (instr.contains("0x73") || instr.contains("0x74") || 
                                     instr.contains("0x72") || instr.contains("0x75")) {
            println!("  0x{:x}: {} <- 's','t','r','u'", addr, instr);
        }
    }
    
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
    
    // Get full disassembly
    let output = Command::new("objdump")
        .args(&["-d", lib])
        .output()?;
    
    let disasm = String::from_utf8_lossy(&output.stdout);
    
    for line in disasm.lines() {
        if let Some(addr_str) = line.split(':').next() {
            if let Ok(addr) = u64::from_str_radix(addr_str.trim(), 16) {
                if addrs.contains(&addr) {
                    if let Some(instr) = line.split(':').nth(1) {
                        instrs.push((addr, instr.trim().to_string()));
                    }
                }
            }
        }
    }
    
    Ok(instrs)
}
