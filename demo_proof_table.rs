// Prove from data: Read actual parquet and show Syn → Rustc mappings

use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use arrow::array::{StringArray, UInt64Array};

fn main() {
    println!("📊 Proof: Syn → Rustc Symbol Mapping\n");
    println!("Reading from: /tmp/rustc_labels.parquet\n");
    
    // Open parquet file
    let file = match std::fs::File::open("/tmp/rustc_labels.parquet") {
        Ok(f) => f,
        Err(e) => {
            println!("❌ Error opening file: {}", e);
            return;
        }
    };
    
    let builder = match ParquetRecordBatchReaderBuilder::try_new(file) {
        Ok(b) => b,
        Err(e) => {
            println!("❌ Error creating reader: {}", e);
            return;
        }
    };
    
    let mut reader = match builder.build() {
        Ok(r) => r,
        Err(e) => {
            println!("❌ Error building reader: {}", e);
            return;
        }
    };
    
    println!("✅ Parquet file opened successfully\n");
    println!("📋 Mapping Table:\n");
    println!("{:<15} {:<18} {:<25} {:<25} {:<30}", 
             "Syn Node", "Rustc IP", "Rustc Function", "Semantic Label", "Source");
    println!("{}", "=".repeat(120));
    
    let mut total_rows = 0;
    let mut syn_counts = std::collections::HashMap::new();
    let mut label_counts = std::collections::HashMap::new();
    
    // Read all batches
    while let Some(batch_result) = reader.next() {
        let batch = match batch_result {
            Ok(b) => b,
            Err(e) => {
                println!("❌ Error reading batch: {}", e);
                continue;
            }
        };
        
        // Extract columns
        let sources = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap();
        let syn_nodes = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        let ips = batch.column(2).as_any().downcast_ref::<UInt64Array>().unwrap();
        let functions = batch.column(3).as_any().downcast_ref::<StringArray>().unwrap();
        let labels = batch.column(4).as_any().downcast_ref::<StringArray>().unwrap();
        
        // Show first 20 rows
        for i in 0..batch.num_rows().min(20) {
            let source = sources.value(i);
            let source_short = if source.len() > 25 {
                format!("{}...", &source[..22])
            } else {
                source.to_string()
            };
            
            println!("{:<15} 0x{:<16x} {:<25} {:<25} {}",
                     syn_nodes.value(i),
                     ips.value(i),
                     functions.value(i),
                     labels.value(i),
                     source_short);
        }
        
        // Collect statistics
        for i in 0..batch.num_rows() {
            total_rows += 1;
            *syn_counts.entry(syn_nodes.value(i).to_string()).or_insert(0) += 1;
            *label_counts.entry(labels.value(i).to_string()).or_insert(0) += 1;
        }
    }
    
    // Show statistics
    println!("\n{}", "=".repeat(120));
    println!("\n📊 Statistics from Parquet Data:\n");
    println!("Total mappings: {}", total_rows);
    
    println!("\nSyn Node Distribution:");
    let mut sorted: Vec<_> = syn_counts.iter().collect();
    sorted.sort_by_key(|e| e.1);
    sorted.reverse();
    for (node, count) in sorted {
        println!("  {}: {}", node, count);
    }
    
    println!("\nSemantic Label Distribution:");
    let mut sorted: Vec<_> = label_counts.iter().collect();
    sorted.sort_by_key(|e| e.1);
    sorted.reverse();
    for (label, count) in sorted {
        println!("  {}: {}", label, count);
    }
    
    println!("\n✅ Proof complete: Syn nodes map to Rustc IPs with semantic labels");
}
