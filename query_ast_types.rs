use parquet::file::reader::{FileReader, SerializedFileReader};
use std::fs::File;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("string_usage.parquet");
    let file = File::open(path)?;
    let reader = SerializedFileReader::new(file)?;
    
    let patterns = ["Expr", "Stmt", "Item", "Pat", "Ty", "Block", "Hir", "Mir", "Thir", 
                    "ast::", "hir::", "mir::", "thir::", "Body", "Def", "Res", "Node"];
    
    println!("Searching for AST/HIR/MIR type strings...\n");
    
    for i in 0..reader.num_row_groups() {
        let row_group = reader.get_row_group(i)?;
        let rows = row_group.get_row_iter(None)?;
        
        for row in rows {
            match row {
                Ok(r) => {
                    let row_str = format!("{:?}", r);
                    let lower = row_str.to_lowercase();
                    if patterns.iter().any(|p| lower.contains(&p.to_lowercase())) {
                        println!("{}", row_str);
                    }
                }
                Err(_) => continue,
            }
        }
    }
    
    Ok(())
}
