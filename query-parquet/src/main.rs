// query_parquet.rs - Fast SQL queries on Parquet using DataFusion

use anyhow::Result;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: query_parquet <parquet_file> <sql_query>");
        eprintln!("\nExamples:");
        eprintln!("  query_parquet nix_build_logs.parquet 'SELECT * FROM nix_build_logs LIMIT 5'");
        eprintln!("  query_parquet nix_build_logs.parquet 'SELECT build_status, COUNT(*) as count FROM nix_build_logs GROUP BY build_status'");
        eprintln!("  query_parquet nix_build_logs.parquet 'SELECT project, exit_code FROM nix_build_logs WHERE build_status = \"failed\"'");
        std::process::exit(1);
    }
    
    let parquet_file = &args[1];
    let sql_query = &args[2];
    
    println!("🔍 Querying: {}", parquet_file);
    println!("📊 SQL: {}", sql_query);
    println!();
    
    // Create DataFusion context
    let ctx = SessionContext::new();
    
    // Register Parquet file as table
    let table_name = std::path::Path::new(parquet_file)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    
    ctx.register_parquet(table_name, parquet_file, ParquetReadOptions::default())
        .await?;
    
    // Execute query
    let df = ctx.sql(sql_query).await?;
    
    // Show results
    df.show().await?;
    
    Ok(())
}
