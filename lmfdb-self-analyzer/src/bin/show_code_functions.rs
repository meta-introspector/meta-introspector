use arrow::array::StringArray;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::collections::HashMap;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("nix_store_grammars.parquet")?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let mut reader = builder.build()?;
    
    let mut token_examples: HashMap<String, Vec<String>> = HashMap::new();
    let keywords = ["else", "struct", "enum", "trait", "impl", "match", "const", "static"];
    
    println!("🔍 Finding functions with code keywords...\n");
    
    while let Some(Ok(batch)) = reader.next() {
        let name_col = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap();
        
        for i in 0..batch.num_rows() {
            let name = name_col.value(i);
            let lower = name.to_lowercase();
            
            for keyword in &keywords {
                if lower.contains(keyword) {
                    token_examples.entry(keyword.to_string())
                        .or_insert_with(Vec::new)
                        .push(name.to_string());
                }
            }
        }
    }
    
    for keyword in &keywords {
        if let Some(examples) = token_examples.get(*keyword) {
            println!("📌 '{}' - {} functions:", keyword, examples.len());
            for func in examples.iter().take(10) {
                println!("   {}", func);
            }
            if examples.len() > 10 {
                println!("   ... and {} more\n", examples.len() - 10);
            } else {
                println!();
            }
        }
    }
    
    Ok(())
}
