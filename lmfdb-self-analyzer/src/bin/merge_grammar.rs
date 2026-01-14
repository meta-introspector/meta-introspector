use arrow::array::StringArray;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::collections::{HashMap, HashSet};
use std::fs::File;

#[derive(Debug)]
struct MergedGrammar {
    states: HashMap<String, StateNode>,
    start_state: String,
    accept_states: HashSet<String>,
}

#[derive(Debug)]
struct StateNode {
    transitions: HashMap<char, String>,
    sources: Vec<String>, // Which binaries contributed this state
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("nix_store_grammars.parquet")?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let mut reader = builder.build()?;
    
    let mut merged = MergedGrammar {
        states: HashMap::new(),
        start_state: "S0".to_string(),
        accept_states: HashSet::new(),
    };
    
    let mut total = 0;
    let mut state_counter = 0;
    
    println!("🔄 Merging grammars...\n");
    
    while let Some(Ok(batch)) = reader.next() {
        let name_col = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap();
        let label_col = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        
        for i in 0..batch.num_rows() {
            let name = name_col.value(i);
            let label = label_col.value(i);
            
            // Parse LMFDB label to extract pattern info
            for ch in label.chars() {
                if ch.is_alphanumeric() || ".-_".contains(ch) {
                    let state_key = format!("S{}", state_counter);
                    merged.states.entry(state_key.clone())
                        .or_insert_with(|| StateNode {
                            transitions: HashMap::new(),
                            sources: Vec::new(),
                        })
                        .sources.push(name.to_string());
                    
                    if ch != '.' {
                        merged.accept_states.insert(state_key);
                    }
                    state_counter += 1;
                }
            }
            total += 1;
        }
    }
    
    println!("✅ Merged Grammar Statistics:");
    println!("   Total grammars: {}", total);
    println!("   Unique states: {}", merged.states.len());
    println!("   Accept states: {}", merged.accept_states.len());
    println!("   Start state: {}", merged.start_state);
    
    // Find most common patterns
    let mut char_freq: HashMap<char, usize> = HashMap::new();
    for state in merged.states.values() {
        for &ch in state.transitions.keys() {
            *char_freq.entry(ch).or_insert(0) += 1;
        }
    }
    
    let mut freq_vec: Vec<_> = char_freq.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n📊 Top 20 Characters:");
    for (ch, count) in freq_vec.iter().take(20) {
        println!("   '{}': {} transitions", ch.escape_default(), count);
    }
    
    Ok(())
}
