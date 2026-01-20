use std::collections::HashMap;
use parquet::arrow::ParquetRecordBatchReaderBuilder;
use std::fs::File;

fn main() {
    println!("🔑 Using 37/genus-2 as key to find LMFDB code");
    
    // Load 3M file index
    let files = load_file_index("indexes/files.parquet");
    println!("📊 Loaded {} files", files.len());
    
    // Define search patterns from our theory
    let patterns = vec![
        // Prime 37 patterns
        "37", "genus", "modular", "X_0", "X0",
        
        // LMFDB specific
        "lmfdb", "elliptic", "curve", "conductor",
        
        // Genus 2 patterns
        "genus_2", "genus==2", "genus = 2",
        
        // Irregular prime
        "irregular", "kummer", "class_number",
        
        // Postgres/Python patterns
        "postgres", "psycopg", "sqlalchemy",
        "elliptic_curves", "modular_forms",
    ];
    
    // Search for matches
    let matches = find_pattern_matches(&files, &patterns);
    println!("✅ Found {} matching files", matches.len());
    
    // Analyze matches
    analyze_matches(&matches);
    
    // Save results
    save_matches(&matches, "lmfdb_matches.parquet");
}

fn load_file_index(path: &str) -> Vec<FileInfo> {
    let mut files = Vec::new();
    
    let file = File::open(path).expect("Failed to open file index");
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
    let reader = builder.build().unwrap();
    
    for batch in reader.flatten() {
        // Extract file paths and metadata
    }
    
    files
}

fn find_pattern_matches(files: &[FileInfo], patterns: &[&str]) -> Vec<Match> {
    let mut matches = Vec::new();
    
    for file in files {
        // Skip non-code files
        if !is_code_file(&file.path) {
            continue;
        }
        
        if let Ok(content) = std::fs::read_to_string(&file.path) {
            let mut pattern_hits = HashMap::new();
            
            for pattern in patterns {
                let count = content.matches(pattern).count();
                if count > 0 {
                    pattern_hits.insert(pattern.to_string(), count);
                }
            }
            
            if !pattern_hits.is_empty() {
                matches.push(Match {
                    file_path: file.path.clone(),
                    git_repo: file.git_repo.clone(),
                    patterns: pattern_hits,
                    score: calculate_score(&pattern_hits),
                    language: detect_language(&file.path),
                });
            }
        }
    }
    
    matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    matches
}

fn is_code_file(path: &str) -> bool {
    path.ends_with(".py") || 
    path.ends_with(".rs") || 
    path.ends_with(".sql") ||
    path.ends_with(".sage")
}

fn calculate_score(patterns: &HashMap<String, usize>) -> f64 {
    let mut score = 0.0;
    
    // High value patterns
    if patterns.contains_key("37") { score += 10.0; }
    if patterns.contains_key("genus") { score += 5.0; }
    if patterns.contains_key("lmfdb") { score += 20.0; }
    if patterns.contains_key("elliptic") { score += 5.0; }
    if patterns.contains_key("irregular") { score += 10.0; }
    
    // Add counts
    for count in patterns.values() {
        score += *count as f64;
    }
    
    score
}

fn detect_language(path: &str) -> String {
    if path.ends_with(".py") { "python".to_string() }
    else if path.ends_with(".rs") { "rust".to_string() }
    else if path.ends_with(".sql") { "sql".to_string() }
    else if path.ends_with(".sage") { "sage".to_string() }
    else { "unknown".to_string() }
}

fn analyze_matches(matches: &[Match]) {
    println!("\n📊 Analysis:");
    
    // Group by language
    let mut by_lang: HashMap<String, usize> = HashMap::new();
    for m in matches {
        *by_lang.entry(m.language.clone()).or_insert(0) += 1;
    }
    
    println!("  By language:");
    for (lang, count) in by_lang {
        println!("    {}: {}", lang, count);
    }
    
    // Group by repo
    let mut by_repo: HashMap<String, usize> = HashMap::new();
    for m in matches {
        *by_repo.entry(m.git_repo.clone()).or_insert(0) += 1;
    }
    
    println!("\n  Top repos:");
    let mut repos: Vec<_> = by_repo.iter().collect();
    repos.sort_by(|a, b| b.1.cmp(a.1));
    for (repo, count) in repos.iter().take(10) {
        println!("    {}: {}", repo, count);
    }
    
    // Top matches
    println!("\n  Top matches:");
    for m in matches.iter().take(10) {
        println!("    {} (score: {:.1})", m.file_path, m.score);
        for (pattern, count) in &m.patterns {
            println!("      {}: {}", pattern, count);
        }
    }
}

fn save_matches(matches: &[Match], output: &str) {
    use arrow::array::{Float64Array, StringArray};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use parquet::arrow::ArrowWriter;
    use std::sync::Arc;
    
    let schema = Schema::new(vec![
        Field::new("file_path", DataType::Utf8, false),
        Field::new("git_repo", DataType::Utf8, false),
        Field::new("language", DataType::Utf8, false),
        Field::new("score", DataType::Float64, false),
    ]);
    
    let file_paths: Vec<String> = matches.iter().map(|m| m.file_path.clone()).collect();
    let git_repos: Vec<String> = matches.iter().map(|m| m.git_repo.clone()).collect();
    let languages: Vec<String> = matches.iter().map(|m| m.language.clone()).collect();
    let scores: Vec<f64> = matches.iter().map(|m| m.score).collect();
    
    let batch = RecordBatch::try_new(
        Arc::new(schema.clone()),
        vec![
            Arc::new(StringArray::from(file_paths)),
            Arc::new(StringArray::from(git_repos)),
            Arc::new(StringArray::from(languages)),
            Arc::new(Float64Array::from(scores)),
        ],
    ).unwrap();
    
    let file = File::create(output).unwrap();
    let mut writer = ArrowWriter::try_new(file, Arc::new(schema), None).unwrap();
    writer.write(&batch).unwrap();
    writer.close().unwrap();
    
    println!("\n💾 Saved {} matches to {}", matches.len(), output);
}

#[derive(Debug)]
struct FileInfo {
    path: String,
    git_repo: String,
}

#[derive(Debug)]
struct Match {
    file_path: String,
    git_repo: String,
    patterns: HashMap<String, usize>,
    score: f64,
    language: String,
}
