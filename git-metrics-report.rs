use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Activity {
    commit_hash: String,
    author_name: String,
    author_email: String,
    author_date: String,
    repo_name: String,
    platform: String,
    files_changed: Option<usize>,
    insertions: Option<usize>,
    deletions: Option<usize>,
}

#[derive(Debug, Serialize, Default)]
struct AuthorMetrics {
    total_commits: usize,
    total_files: usize,
    total_insertions: usize,
    total_deletions: usize,
    repos: HashMap<String, usize>,
    platforms: HashMap<String, usize>,
    emails: Vec<String>,
}

#[derive(Debug, Serialize)]
struct MetricsReport {
    period: String,
    total_commits: usize,
    total_authors: usize,
    authors: HashMap<String, AuthorMetrics>,
    email_issues: Vec<EmailIssue>,
}

#[derive(Debug, Serialize)]
struct EmailIssue {
    author_name: String,
    email: String,
    commits: usize,
    likely_issue: String,
}

fn main() {
    let activity_dir = PathBuf::from("data/activity");
    
    let mut report = MetricsReport {
        period: "All Time".to_string(),
        total_commits: 0,
        total_authors: 0,
        authors: HashMap::new(),
        email_issues: Vec::new(),
    };

    // Recursively scan all activity JSON files
    fn scan_dir(dir: &PathBuf, report: &mut MetricsReport) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    scan_dir(&path, report);
                } else if path.extension().is_some_and(|ext| ext == "json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(activities) = serde_json::from_str::<Vec<Activity>>(&content) {
                            for activity in activities {
                                report.total_commits += 1;
                                
                                let metrics = report.authors
                                    .entry(activity.author_name.clone())
                                    .or_default();
                                
                                metrics.total_commits += 1;
                                metrics.total_files += activity.files_changed.unwrap_or(0);
                                metrics.total_insertions += activity.insertions.unwrap_or(0);
                                metrics.total_deletions += activity.deletions.unwrap_or(0);
                                
                                *metrics.repos.entry(activity.repo_name).or_insert(0) += 1;
                                *metrics.platforms.entry(activity.platform).or_insert(0) += 1;
                                
                                if !metrics.emails.contains(&activity.author_email) {
                                    metrics.emails.push(activity.author_email);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    scan_dir(&activity_dir, &mut report);

    report.total_authors = report.authors.len();

    // Detect email issues
    for (author, metrics) in &report.authors {
        for email in &metrics.emails {
            let mut issues = Vec::new();
            
            if email.contains("@solfunmeme.com") {
                issues.push("non-standard domain");
            }
            if !email.contains('@') {
                issues.push("invalid format");
            }
            if email.ends_with("@localhost") || email.ends_with("@local") {
                issues.push("local email");
            }
            
            if !issues.is_empty() {
                report.email_issues.push(EmailIssue {
                    author_name: author.clone(),
                    email: email.clone(),
                    commits: metrics.total_commits,
                    likely_issue: issues.join(", "),
                });
            }
        }
    }

    // Output report
    let json = serde_json::to_string_pretty(&report).unwrap();
    fs::write("data/git-metrics-report.json", &json).unwrap();
    
    println!("📊 Git Metrics Report");
    println!("═══════════════════════");
    println!("Total Commits: {}", report.total_commits);
    println!("Total Authors: {}", report.total_authors);
    println!("\n🔝 Top 10 Contributors:");
    
    let mut sorted: Vec<_> = report.authors.iter().collect();
    sorted.sort_by(|a, b| b.1.total_commits.cmp(&a.1.total_commits));
    
    for (i, (name, metrics)) in sorted.iter().take(10).enumerate() {
        println!("{}. {} - {} commits, {} repos, +{} -{}", 
            i + 1, name, metrics.total_commits, metrics.repos.len(),
            metrics.total_insertions, metrics.total_deletions);
    }
    
    if !report.email_issues.is_empty() {
        println!("\n⚠️  Email Attribution Issues:");
        for issue in &report.email_issues {
            println!("  {} <{}> - {} commits ({})", 
                issue.author_name, issue.email, issue.commits, issue.likely_issue);
        }
    }
    
    println!("\n✅ Full report saved to: data/git-metrics-report.json");
}
