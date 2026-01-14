use arrow::array::StringArray;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::collections::HashMap;
use std::fs::File;

#[derive(Debug)]
struct MarkovNode {
    transitions: HashMap<String, (usize, f64)>,
    total: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("nix_store_grammars.parquet")?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let mut reader = builder.build()?;
    
    let mut markov: HashMap<String, MarkovNode> = HashMap::new();
    
    while let Some(Ok(batch)) = reader.next() {
        let label_col = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        
        for i in 0..batch.num_rows() {
            let label = label_col.value(i);
            let chars: Vec<String> = label.chars().map(|c| c.to_string()).collect();
            
            for window in chars.windows(2) {
                let from = &window[0];
                let to = &window[1];
                
                let node = markov.entry(from.clone()).or_insert_with(|| MarkovNode {
                    transitions: HashMap::new(),
                    total: 0,
                });
                
                let entry = node.transitions.entry(to.clone()).or_insert((0, 0.0));
                entry.0 += 1;
                node.total += 1;
            }
        }
    }
    
    for node in markov.values_mut() {
        for (_, (count, prob)) in node.transitions.iter_mut() {
            *prob = *count as f64 / node.total as f64;
        }
    }
    
    let words = ["enum", "struct", "else", "trait", "impl", "const", "match", "loop"];
    
    println!("🔍 Searching for word sequences in Markov grammar...\n");
    
    for word in &words {
        let chars: Vec<String> = word.chars().map(|c| c.to_string()).collect();
        let mut prob = 1.0;
        let mut path = String::new();
        let mut found = true;
        
        for i in 0..chars.len() - 1 {
            let from = &chars[i];
            let to = &chars[i + 1];
            
            if let Some(node) = markov.get(from) {
                if let Some((count, p)) = node.transitions.get(to) {
                    prob *= p;
                    path.push_str(&format!("{} → ", from));
                } else {
                    found = false;
                    break;
                }
            } else {
                found = false;
                break;
            }
        }
        
        if found {
            path.push_str(&chars[chars.len() - 1]);
            println!("✅ '{}': {} (p={:.6})", word, path, prob);
        } else {
            println!("❌ '{}': Not found in grammar", word);
        }
    }
    
    Ok(())
}
