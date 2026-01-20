use std::process::Command;

fn main() {
    println!("🔐 ZOS Level Checker");
    
    // Get current SELinux context
    let output = Command::new("id")
        .arg("-Z")
        .output()
        .expect("Failed to get SELinux context");
    
    let context = String::from_utf8_lossy(&output.stdout);
    
    let level = if context.contains("zos_level0_t") {
        0
    } else if context.contains("zos_level1_t") {
        1
    } else if context.contains("zos_level2_t") {
        2
    } else if context.contains("zos_level3_t") {
        3
    } else if context.contains("zos_level4_t") {
        4
    } else if context.contains("zos_level5_t") {
        5
    } else if context.contains("zos_level6_t") {
        6
    } else {
        println!("❌ Not running in ZOS context");
        return;
    };
    
    println!("📊 Current Level: {}", level);
    println!("📋 Context: {}", context.trim());
    
    match level {
        0 => println!("🔧 Hardware/Kernel - Lowest level"),
        1 => println!("🛡️  Hypervisor/SELinux - Security enforcement"),
        2 => println!("🌐 System Services - Proxies and logging"),
        3 => println!("📦 Build System - Nix derivations"),
        4 => println!("🦀 Language Runtime - Rust/LLVM"),
        5 => println!("💻 Application Code - User programs"),
        6 => println!("🤖 User Interface - CLI/LLM"),
        _ => unreachable!(),
    }
    
    if level > 0 {
        println!("✅ Can communicate with Level {}", level - 1);
    }
    if level < 6 {
        println!("❌ Cannot communicate with Level {}", level + 1);
    }
}
