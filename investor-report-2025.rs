use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct Activity {
    commit_hash: String,
    author_name: String,
    author_email: String,
    author_date: String,
    repo_name: String,
    repo_url: String,
    platform: String,
    files_changed: Option<usize>,
    insertions: Option<usize>,
    deletions: Option<usize>,
}

#[derive(Debug, Serialize, Default)]
struct MonthlyStats {
    commits: usize,
    files: usize,
    insertions: usize,
    deletions: usize,
    repos: Vec<String>,
}

#[derive(Debug, Serialize)]
struct InvestorReport {
    year: u32,
    user_stats: HashMap<String, UserReport>,
    org_stats: OrgReport,
}

#[derive(Debug, Serialize)]
struct UserReport {
    total_commits: usize,
    total_files: usize,
    total_insertions: usize,
    total_deletions: usize,
    monthly: HashMap<String, MonthlyStats>,
    top_repos: Vec<(String, usize)>,
}

#[derive(Debug, Serialize)]
struct OrgReport {
    total_commits: usize,
    contributors: Vec<String>,
    repos: Vec<String>,
}

fn main() {
    let year = 2025;
    let users = vec!["jmikedupont2", "mike.dupont", "mike", "Mike DuPont"];
    
    let mut monthly_repo_matrix: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut total_commits = 0;
    let mut total_files = 0;
    let mut total_insertions = 0;
    let mut total_deletions = 0;
    let mut all_repos: HashMap<String, usize> = HashMap::new();

    // Scan all users and merge
    for user in &users {
        let user_path = format!("data/activity/github/{}/{}", user, year);
        if let Ok(entries) = fs::read_dir(&user_path) {
            for entry in entries.flatten() {
                if let Some(month) = entry.file_name().to_str() {
                    let activity_file = entry.path().join("activity.json");
                    if let Ok(content) = fs::read_to_string(&activity_file) {
                        if let Ok(activities) = serde_json::from_str::<Vec<Activity>>(&content) {
                            for activity in activities {
                                total_commits += 1;
                                total_files += activity.files_changed.unwrap_or(0);
                                total_insertions += activity.insertions.unwrap_or(0);
                                total_deletions += activity.deletions.unwrap_or(0);
                                
                                // Track month/repo matrix
                                let month_map = monthly_repo_matrix
                                    .entry(month.to_string())
                                    .or_insert_with(HashMap::new);
                                *month_map.entry(activity.repo_name.clone()).or_insert(0) += 1;
                                
                                // Track all repos
                                *all_repos.entry(activity.repo_name).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort months
    let mut months: Vec<_> = monthly_repo_matrix.keys().cloned().collect();
    months.sort();
    
    // Sort repos by total commits
    let mut repos: Vec<_> = all_repos.iter().collect();
    repos.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("📊 2025 Activity Report - Meta-Introspector");
    println!("═══════════════════════════════════════════\n");
    println!("Total: {} commits | {} files | +{} -{}\n", 
        total_commits, total_files, total_insertions, total_deletions);
    
    println!("📈 Month/Repo Matrix:");
    println!("{:<15} {}", "Month", repos.iter().take(10).map(|(r, _)| format!("{:>12}", r)).collect::<Vec<_>>().join(" "));
    println!("{}", "─".repeat(150));
    
    for month in &months {
        let month_data = &monthly_repo_matrix[month];
        print!("{:<15}", month);
        for (repo, _) in repos.iter().take(10) {
            let count = month_data.get(*repo).unwrap_or(&0);
            if *count > 0 {
                print!(" {:>12}", count);
            } else {
                print!(" {:>12}", "·");
            }
        }
        println!();
    }
    
    println!("\n🎯 Top Repositories:");
    for (i, (repo, count)) in repos.iter().take(10).enumerate() {
        println!("{}. {} - {} commits", i + 1, repo, count);
    }
    
    // Save JSON
    let report = serde_json::json!({
        "year": year,
        "total_commits": total_commits,
        "total_files": total_files,
        "total_insertions": total_insertions,
        "total_deletions": total_deletions,
        "monthly_repo_matrix": monthly_repo_matrix,
        "top_repos": repos.iter().take(20).map(|(r, c)| (r, c)).collect::<Vec<_>>()
    });
    
    fs::write("data/investor-report-2025.json", serde_json::to_string_pretty(&report).unwrap()).unwrap();
    println!("\n✅ Full report: data/investor-report-2025.json");
}
