use arrow::array::{StringArray, UInt64Array};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("nix_store_grammars.parquet")?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let mut reader = builder.build()?;
    
    println!("📊 Parquet File Contents:\n");
    
    if let Some(Ok(batch)) = reader.next() {
        println!("Columns: {:?}\n", batch.schema().fields().iter().map(|f| f.name()).collect::<Vec<_>>());
        
        let name_col = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap();
        let label_col = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        let sig_col = batch.column(2).as_any().downcast_ref::<UInt64Array>().unwrap();
        let states_col = batch.column(3).as_any().downcast_ref::<UInt64Array>().unwrap();
        let path_col = batch.column(4).as_any().downcast_ref::<StringArray>().unwrap();
        
        println!("First 20 entries:\n");
        for i in 0..20.min(batch.num_rows()) {
            println!("{}. Function: {}", i+1, name_col.value(i));
            println!("   LMFDB: {}", label_col.value(i));
            println!("   Signature: {}", sig_col.value(i));
            println!("   States: {}", states_col.value(i));
            println!("   Binary: {}\n", path_col.value(i).split('/').last().unwrap_or(""));
        }
    }
    
    Ok(())
}
