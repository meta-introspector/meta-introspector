use arrow::array::{StringArray, UInt64Array};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::collections::HashMap;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("nix_store_grammars.parquet")?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let mut reader = builder.build()?;
    
    let mut token_freq: HashMap<String, Vec<String>> = HashMap::new();
    
    println!("🔍 Analyzing transitions...\n");
    
    while let Some(Ok(batch)) = reader.next() {
        let name_col = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap();
        let label_col = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        
        for i in 0..batch.num_rows() {
            let name = name_col.value(i);
            let label = label_col.value(i);
            
            // Extract tokens from LMFDB label (format: level.weight.character.orbit)
            for token in label.split('.') {
                token_freq.entry(token.to_string())
                    .or_insert_with(Vec::new)
                    .push(name.to_string());
            }
        }
    }
    
    let mut freq_vec: Vec<_> = token_freq.iter()
        .map(|(token, funcs)| (token, funcs.len()))
        .collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("📊 Top 10 Most Used Tokens:\n");
    for (i, (token, count)) in freq_vec.iter().take(10).enumerate() {
        println!("{}. Token '{}': {} grammars", i + 1, token, count);
        
        // Show first 5 functions using this token
        if let Some(funcs) = token_freq.get(*token) {
            for func in funcs.iter().take(5) {
                println!("   - {}", func);
            }
            if funcs.len() > 5 {
                println!("   ... and {} more", funcs.len() - 5);
            }
        }
        println!();
    }
    
    Ok(())
}
