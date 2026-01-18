// Generate untracked files report: per repo with count, file type, and age
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use arrow::array::{Array, StringArray, BooleanArray};
use std::fs::File;
use std::collections::HashMap;
use std::path::Path;

#[derive(Default)]
struct RepoStats {
    files_by_ext: HashMap<String, usize>,
    total_untracked: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 UNTRACKED FILES REPORT BY REPOSITORY\n");
    
    let file = File::open("data/indexes/files.parquet")?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let reader = builder.build()?;
    
    let mut repo_stats: HashMap<String, RepoStats> = HashMap::new();
    let mut total_files = 0;
    let mut total_untracked = 0;
    
    for batch_result in reader {
        let batch = batch_result?;
        total_files += batch.num_rows();
        
        let file_paths = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap();
        let git_repos = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        let tracked = batch.column(6).as_any().downcast_ref::<BooleanArray>().unwrap();
        
        for i in 0..batch.num_rows() {
            if !tracked.value(i) && !git_repos.is_null(i) {
                let repo = git_repos.value(i);
                if repo.is_empty() {
                    continue;
                }
                
                // Use full repo path as key
                let file_path = file_paths.value(i);
                let ext = Path::new(file_path)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("no_ext")
                    .to_string();
                
                let stats = repo_stats.entry(repo.to_string()).or_default();
                *stats.files_by_ext.entry(ext).or_insert(0) += 1;
                stats.total_untracked += 1;
                total_untracked += 1;
            }
        }
    }
    
    println!("Total files scanned: {}", total_files);
    println!("Total untracked: {}\n", total_untracked);
    
    // Sort repos by untracked count
    let mut sorted: Vec<_> = repo_stats.iter().collect();
    sorted.sort_by(|a, b| b.1.total_untracked.cmp(&a.1.total_untracked));
    
    // Generate report
    let mut report = String::from("# Untracked Files Report by Repository\n\n");
    report.push_str(&format!("**Total files scanned**: {}\n", total_files));
    report.push_str(&format!("**Total untracked**: {}\n\n", total_untracked));
    
    for (repo, stats) in sorted.iter().take(50) {
        report.push_str(&format!("## {} ({} untracked)\n\n", repo, stats.total_untracked));
        
        // Sort extensions by count
        let mut exts: Vec<_> = stats.files_by_ext.iter().collect();
        exts.sort_by(|a, b| b.1.cmp(a.1));
        
        report.push_str("| Extension | Count | Percentage |\n");
        report.push_str("|-----------|-------|------------|\n");
        
        for (ext, count) in exts.iter().take(10) {
            let pct = (**count as f64 / stats.total_untracked as f64) * 100.0;
            report.push_str(&format!("| .{} | {} | {:.1}% |\n", ext, count, pct));
        }
        
        report.push_str("\n");
    }
    
    std::fs::write("UNTRACKED_FILES_REPORT.md", report)?;
    println!("✅ Report saved to UNTRACKED_FILES_REPORT.md");
    
    Ok(())
}
