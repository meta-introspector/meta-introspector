// Enhanced concept map with source tracking - saves to Parquet
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use rayon::prelude::*;
use arrow::array::{ArrayRef, StringArray, UInt64Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;

fn extract_tokens_with_source(path: &PathBuf) -> Vec<(String, String, usize, usize, String)> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    
    let mut tokens = Vec::new();
    let path_str = path.to_string_lossy().to_string();
    
    for (line_num, line) in content.lines().enumerate() {
        let mut col = 0;
        for word in line.split(|c: char| !c.is_alphanumeric() && c != '_') {
            if !word.is_empty() && word.len() > 2 {
                tokens.push((
                    word.to_lowercase(),
                    path_str.clone(),
                    line_num + 1,
                    col,
                    line.trim().to_string(),
                ));
            }
            col += word.len() + 1;
        }
    }
    
    tokens
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗺️  ENHANCED CONCEPT MAP WITH SOURCE TRACKING\n");
    
    // Collect all Rust files
    let files: Vec<PathBuf> = fs::read_dir(".")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
        .map(|e| e.path())
        .collect();
    
    println!("📂 Found {} Rust files", files.len());
    
    // Extract tokens with source tracking
    println!("📝 Extracting tokens with source info...");
    let all_tokens: Vec<(String, String, usize, usize, String)> = files.par_iter()
        .flat_map(|path| extract_tokens_with_source(path))
        .collect();
    
    println!("   Total tokens: {}", all_tokens.len());
    
    // Prepare data for Parquet
    println!("💾 Saving to Parquet...");
    
    let words: Vec<String> = all_tokens.iter().map(|(w, _, _, _, _)| w.clone()).collect();
    let files: Vec<String> = all_tokens.iter().map(|(_, f, _, _, _)| f.clone()).collect();
    let lines: Vec<u64> = all_tokens.iter().map(|(_, _, l, _, _)| *l as u64).collect();
    let columns: Vec<u64> = all_tokens.iter().map(|(_, _, _, c, _)| *c as u64).collect();
    let contexts: Vec<String> = all_tokens.iter().map(|(_, _, _, _, ctx)| ctx.clone()).collect();
    
    // Create schema
    let schema = Schema::new(vec![
        Field::new("word", DataType::Utf8, false),
        Field::new("file", DataType::Utf8, false),
        Field::new("line", DataType::UInt64, false),
        Field::new("column", DataType::UInt64, false),
        Field::new("context", DataType::Utf8, false),
    ]);
    
    // Create arrays
    let word_array = Arc::new(StringArray::from(words)) as ArrayRef;
    let file_array = Arc::new(StringArray::from(files)) as ArrayRef;
    let line_array = Arc::new(UInt64Array::from(lines)) as ArrayRef;
    let column_array = Arc::new(UInt64Array::from(columns)) as ArrayRef;
    let context_array = Arc::new(StringArray::from(contexts)) as ArrayRef;
    
    // Create record batch
    let batch = RecordBatch::try_new(
        Arc::new(schema.clone()),
        vec![word_array, file_array, line_array, column_array, context_array],
    )?;
    
    // Write to Parquet
    fs::create_dir_all("data")?;
    let file = fs::File::create("data/tokens_with_source.parquet")?;
    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, Arc::new(schema), Some(props))?;
    
    writer.write(&batch)?;
    writer.close()?;
    
    println!("✅ Saved data/tokens_with_source.parquet");
    
    // Build statistics
    let mut word_counts: HashMap<String, usize> = HashMap::new();
    let mut file_counts: HashMap<String, usize> = HashMap::new();
    
    for (word, file, _, _, _) in &all_tokens {
        *word_counts.entry(word.clone()).or_insert(0) += 1;
        *file_counts.entry(file.clone()).or_insert(0) += 1;
    }
    
    println!("\n📊 Statistics:");
    println!("  Total tokens: {}", all_tokens.len());
    println!("  Unique words: {}", word_counts.len());
    println!("  Files: {}", file_counts.len());
    
    // Top 10 words
    let mut top_words: Vec<_> = word_counts.iter().collect();
    top_words.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🔝 Top 10 words:");
    for (word, count) in top_words.iter().take(10) {
        println!("  {} → {}", word, count);
    }
    
    // Save word frequencies to Parquet
    let words: Vec<String> = top_words.iter().map(|(w, _)| w.to_string()).collect();
    let counts: Vec<u64> = top_words.iter().map(|(_, c)| **c as u64).collect();
    
    let freq_schema = Schema::new(vec![
        Field::new("word", DataType::Utf8, false),
        Field::new("count", DataType::UInt64, false),
    ]);
    
    let word_array = Arc::new(StringArray::from(words)) as ArrayRef;
    let count_array = Arc::new(UInt64Array::from(counts)) as ArrayRef;
    
    let freq_batch = RecordBatch::try_new(
        Arc::new(freq_schema.clone()),
        vec![word_array, count_array],
    )?;
    
    let freq_file = fs::File::create("data/word_frequencies.parquet")?;
    let mut freq_writer = ArrowWriter::try_new(freq_file, Arc::new(freq_schema), Some(props))?;
    freq_writer.write(&freq_batch)?;
    freq_writer.close()?;
    
    println!("✅ Saved data/word_frequencies.parquet");
    
    Ok(())
}
