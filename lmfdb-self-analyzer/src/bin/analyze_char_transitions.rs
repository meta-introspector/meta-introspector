use arrow::array::StringArray;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::collections::HashMap;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("nix_store_grammars.parquet")?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let mut reader = builder.build()?;
    
    let mut transition_freq: HashMap<String, Vec<String>> = HashMap::new();
    
    println!("🔍 Analyzing character transitions...\n");
    
    while let Some(Ok(batch)) = reader.next() {
        let name_col = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap();
        let label_col = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        
        for i in 0..batch.num_rows() {
            let name = name_col.value(i);
            let label = label_col.value(i);
            
            // Extract character-to-character transitions
            let chars: Vec<char> = label.chars().collect();
            for window in chars.windows(2) {
                let transition = format!("{} → {}", window[0], window[1]);
                transition_freq.entry(transition)
                    .or_insert_with(Vec::new)
                    .push(name.to_string());
            }
        }
    }
    
    let mut freq_vec: Vec<_> = transition_freq.iter()
        .map(|(trans, funcs)| (trans, funcs.len()))
        .collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("📊 Top 10 Most Used Transitions:\n");
    for (i, (trans, count)) in freq_vec.iter().take(10).enumerate() {
        println!("{}. Transition '{}': {} grammars", i + 1, trans, count);
        
        if let Some(funcs) = transition_freq.get(*trans) {
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
