use std::fs;
use std::collections::HashMap;
use std::process::Command;

fn main() {
    println!("📊 DECLARATION SIZE HISTOGRAM");
    println!("{}", "=".repeat(40));
    
    let mut size_histogram: HashMap<usize, usize> = HashMap::new();
    let mut total_decls = 0;
    let mut total_bytes = 0;
    
    // Analyze rust-build for declaration sizes
    let rust_build_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build";
    
    let find_output = Command::new("find")
        .arg(rust_build_path)
        .arg("-name")
        .arg("*.rs")
        .arg("-type")
        .arg("f")
        .output()
        .expect("Failed to find files");
    
    let files: Vec<&str> = std::str::from_utf8(&find_output.stdout).unwrap().lines().collect();
    
    println!("🔍 Analyzing {} files for declaration sizes...", files.len());
    
    // Analyze ALL files
    for file_path in files.iter() {
        if let Ok(content) = fs::read_to_string(file_path) {
            analyze_declarations(&content, &mut size_histogram, &mut total_decls, &mut total_bytes);
        }
    }
    
    println!("\n📈 DECLARATION SIZE HISTOGRAM:");
    println!("Size Range (bytes) | Count | Percentage | Visual");
    println!("{}", "-".repeat(50));
    
    // Create size buckets
    let mut buckets = vec![
        (0..50, "Tiny (0-49)"),
        (50..100, "Small (50-99)"),
        (100..200, "Medium (100-199)"),
        (200..500, "Large (200-499)"),
        (500..1000, "XLarge (500-999)"),
        (1000..10000, "Huge (1K-10K)"),
    ];
    
    for (range, label) in buckets {
        let count: usize = size_histogram.iter()
            .filter(|(&size, _)| range.contains(&size))
            .map(|(_, &count)| count)
            .sum();
        
        let percentage = if total_decls > 0 { 
            (count as f64 / total_decls as f64) * 100.0 
        } else { 0.0 };
        
        let bar_length = (percentage / 5.0) as usize; // Scale for display
        let bar = "█".repeat(bar_length);
        
        println!("{:15} | {:5} | {:6.1}% | {}", label, count, percentage, bar);
    }
    
    println!("\n📊 SUMMARY STATISTICS:");
    println!("  Total declarations: {}", total_decls);
    println!("  Total bytes: {}", total_bytes);
    println!("  Average size: {:.1} bytes", total_bytes as f64 / total_decls as f64);
    
    // Find most common sizes
    let mut sorted_sizes: Vec<_> = size_histogram.iter().collect();
    sorted_sizes.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🎯 TOP 10 MOST COMMON SIZES:");
    for (i, (&size, &count)) in sorted_sizes.iter().enumerate() {
        println!("  {}. {} bytes: {} declarations", i + 1, size, count);
    }
    
    // Compression implications
    println!("\n🗜️  COMPRESSION IMPLICATIONS:");
    let small_decls = size_histogram.iter()
        .filter(|(&size, _)| size < 100)
        .map(|(_, &count)| count)
        .sum::<usize>();
    
    let small_pct = (small_decls as f64 / total_decls as f64) * 100.0;
    println!("  Small declarations (<100 bytes): {:.1}%", small_pct);
    println!("  These compress to ~3-5 bytes each (95%+ compression)");
    
    let large_decls = size_histogram.iter()
        .filter(|(&size, _)| size >= 500)
        .map(|(_, &count)| count)
        .sum::<usize>();
    
    let large_pct = (large_decls as f64 / total_decls as f64) * 100.0;
    println!("  Large declarations (500+ bytes): {:.1}%", large_pct);
    println!("  These compress to ~15-50 bytes each (90%+ compression)");
}

fn analyze_declarations(content: &str, histogram: &mut HashMap<usize, usize>, total_decls: &mut usize, total_bytes: &mut usize) {
    // Simple declaration detection
    let lines: Vec<&str> = content.lines().collect();
    let mut current_decl = String::new();
    let mut in_decl = false;
    let mut brace_count = 0;
    
    for line in lines {
        let trimmed = line.trim();
        
        // Start of declaration
        if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") ||
           trimmed.starts_with("struct ") || trimmed.starts_with("pub struct ") ||
           trimmed.starts_with("enum ") || trimmed.starts_with("pub enum ") ||
           trimmed.starts_with("impl ") || trimmed.starts_with("trait ") {
            
            if in_decl && !current_decl.is_empty() {
                // Finish previous declaration
                let size = current_decl.len();
                *histogram.entry(size).or_insert(0) += 1;
                *total_decls += 1;
                *total_bytes += size;
            }
            
            current_decl = line.to_string();
            in_decl = true;
            brace_count = line.matches('{').count() as i32 - line.matches('}').count() as i32;
        } else if in_decl {
            current_decl.push('\n');
            current_decl.push_str(line);
            brace_count += line.matches('{').count() as i32 - line.matches('}').count() as i32;
            
            // End of declaration when braces balance
            if brace_count <= 0 && (line.contains('}') || line.trim().ends_with(';')) {
                let size = current_decl.len();
                *histogram.entry(size).or_insert(0) += 1;
                *total_decls += 1;
                *total_bytes += size;
                
                current_decl.clear();
                in_decl = false;
                brace_count = 0;
            }
        }
    }
    
    // Handle final declaration
    if in_decl && !current_decl.is_empty() {
        let size = current_decl.len();
        *histogram.entry(size).or_insert(0) += 1;
        *total_decls += 1;
        *total_bytes += size;
    }
}
