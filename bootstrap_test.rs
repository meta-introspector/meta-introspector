// 🔥 BOOTSTRAP TEST
// Test the enhanced mkbootstrap macro with telemetry integration

use meta_introspector::*;

fn main() {
    println!("🧪 Testing Enhanced Bootstrap System");
    println!("===================================");
    
    // Test the mkbootstrap macro
    mkbootstrap!();
    
    println!("\n🔬 Testing telemetry wrapper:");
    
    // Test telemetry_wrap macro
    let result = telemetry_wrap!("test_computation", {
        std::thread::sleep(std::time::Duration::from_millis(50));
        42 * 2
    });
    
    println!("Result: {}", result);
    
    println!("\n✅ Bootstrap test completed!");
}
