use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::Field;
use std::fs::File;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("../string_usage.parquet")?;
    let reader = SerializedFileReader::new(file)?;
    
    let patterns = ["Expr", "Stmt", "Item", "Pat", "Ty", "Block", "Hir", "Mir", "Thir", 
                    "ast::", "hir::", "mir::", "thir::", "Body", "Def", "Res", "Node",
                    "Generic", "Trait", "Impl"];
    
    let mut matches: HashMap<String, Vec<String>> = HashMap::new();
    
    for i in 0..reader.num_row_groups() {
        let row_group = reader.get_row_group(i)?;
        let rows = row_group.get_row_iter(None)?;
        
        for row_result in rows {
            let row = row_result?;
            let mut string_val = String::new();
            let mut func_name = String::new();
            
            for (name, field) in row.get_column_iter() {
                if let Field::Str(s) = field {
                    if name == "string_value" {
                        string_val = s.to_string();
                    } else if name == "function_name" {
                        func_name = s.to_string();
                    }
                }
            }
            
            for pattern in &patterns {
                if func_name.contains(pattern) {
                    matches.entry(pattern.to_string())
                        .or_insert_with(Vec::new)
                        .push(format!("{} | {}", func_name, string_val));
                    break;
                }
            }
        }
    }
    
    println!("AST/HIR/MIR Type Analysis\n");
    println!("Total patterns found: {}\n", matches.len());
    
    for (pattern, funcs) in matches.iter() {
        println!("Pattern '{}': {} matches", pattern, funcs.len());
    }
    
    println!("\n--- Detailed Results ---\n");
    for (pattern, funcs) in matches.iter() {
        println!("\n=== {} ({} matches) ===", pattern, funcs.len());
        for (i, func) in funcs.iter().take(5).enumerate() {
            println!("  {}: {}", i+1, func);
        }
        if funcs.len() > 5 {
            println!("  ... and {} more", funcs.len() - 5);
        }
    }
    
    Ok(())
}
