// Example usage of perf proc macros
use perf_macros::{perf_auto, perf};

#[perf_auto]
fn expensive_computation(n: u64) -> u64 {
    (0..n).sum()
}

#[perf_auto]
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    println!("🔥 Perf Proc Macro Example\n");
    
    // Example 1: Auto telemetry (no return change)
    println!("Example 1: #[perf_auto]");
    let result = expensive_computation(1_000_000);
    println!("  Result: {}", result);
    println!("  (Perf data sent to telemetry)\n");
    
    // Example 2: Inline perf recording
    println!("Example 2: perf!()");
    let (result, perf_data) = perf!({
        let mut sum = 0;
        for i in 0..1_000_000 {
            sum += i;
        }
        sum
    });
    println!("  Result: {}", result);
    println!("  Duration: {:.6}s", perf_data.duration_secs);
    println!("  IPC: {:.2}", perf_data.ipc());
    println!();
    
    // Example 3: Recursive function
    println!("Example 3: Recursive fibonacci");
    let result = fibonacci(20);
    println!("  Result: {}", result);
    println!("  (Perf data sent to telemetry)\n");
    
    println!("✅ Check data/telemetry/ for perf logs");
}
