// Example usage of perf_probe macro
use perf_macros::{perf_auto, perf_probe, probe};

#[perf_probe]
fn process_data(x: i32, y: String) -> Vec<u8> {
    // All inputs (x, y) and output captured to parquet
    vec![x as u8; y.len()]
}

#[perf_probe]
fn calculate(a: f64, b: f64) -> f64 {
    a * b + a / b
}

#[perf_auto]
fn expensive_computation(n: u64) -> u64 {
    (0..n).sum()
}

fn main() {
    println!("🔬 Perf Probe Example - Parquet Data Capture\n");
    
    // Example 1: Function with probe (captures inputs + output)
    println!("Example 1: #[perf_probe]");
    let result = process_data(42, "hello".to_string());
    println!("  Result: {:?}", result);
    println!("  (Inputs and output captured to parquet)\n");
    
    // Example 2: Math function with probe
    println!("Example 2: Math calculation");
    let result = calculate(3.14, 2.71);
    println!("  Result: {}", result);
    println!("  (Inputs and output captured to parquet)\n");
    
    // Example 3: Inline probe
    println!("Example 3: Inline probe!()");
    let x = 123;
    let y = vec![1, 2, 3, 4, 5];
    probe!(x);
    probe!(y);
    println!("  Captured x and y to parquet\n");
    
    // Example 4: Combined with perf_auto
    println!("Example 4: Combined perf_auto + probe");
    let result = expensive_computation(1_000_000);
    probe!(result);
    println!("  Result: {}", result);
    println!("  (Perf telemetry + probe capture)\n");
    
    println!("✅ Check data/probes/ for parquet files");
    println!("   Use parquet-tools or DuckDB to query:");
    println!("   duckdb -c \"SELECT * FROM 'data/probes/*.parquet'\"");
}
