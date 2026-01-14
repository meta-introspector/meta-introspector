use arrow::array::StringArray;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::collections::HashMap;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("nix_store_grammars.parquet")?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let mut reader = builder.build()?;
    
    let mut token_freq: HashMap<String, usize> = HashMap::new();
    let keywords = ["if", "else", "for", "while", "struct", "enum", "fn", "let", "const", 
                    "return", "match", "impl", "trait", "pub", "use", "mod", "type",
                    "async", "await", "loop", "break", "continue", "static", "mut"];
    
    println!("🔍 Extracting code tokens from function names...\n");
    
    while let Some(Ok(batch)) = reader.next() {
        let name_col = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap();
        
        for i in 0..batch.num_rows() {
            let name = name_col.value(i).to_lowercase();
            
            for keyword in &keywords {
                if name.contains(keyword) {
                    *token_freq.entry(keyword.to_string()).or_insert(0) += 1;
                }
            }
            
            // Extract common patterns
            for pattern in ["parse", "check", "verify", "validate", "init", "create", 
                           "get", "set", "find", "search", "hash", "encode", "decode"] {
                if name.contains(pattern) {
                    *token_freq.entry(pattern.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    
    let mut freq_vec: Vec<_> = token_freq.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("📊 Top Code Tokens Found:\n");
    for (token, count) in freq_vec.iter().take(30) {
        println!("  {}: {} occurrences", token, count);
    }
    
    Ok(())
}
