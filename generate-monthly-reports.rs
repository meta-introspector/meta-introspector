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
    files_changed: Option<usize>,
    insertions: Option<usize>,
    deletions: Option<usize>,
}

#[derive(Debug, Serialize)]
struct MonthlyReport {
    year: u32,
    month: String,
    user: String,
    commits: usize,
    files: usize,
    insertions: usize,
    deletions: usize,
    repos: HashMap<String, usize>,
}

fn main() {
    let my_users = vec!["jmikedupont2", "mike.dupont", "mike", "Mike DuPont"];
    
    println!("📊 Generating merged reports for my identities + all other users...\n");
    
    // Collect all year/month combinations from all platforms/users
    let mut year_months = std::collections::HashSet::new();
    let activity_base = "data/activity";
    
    for platform_entry in fs::read_dir(activity_base).unwrap().flatten() {
        let _platform = platform_entry.file_name().to_string_lossy().to_string();
        let platform_path = platform_entry.path();
        
        if let Ok(user_entries) = fs::read_dir(&platform_path) {
            for user_entry in user_entries.flatten() {
                let user_path = user_entry.path();
                if let Ok(year_entries) = fs::read_dir(&user_path) {
                    for year_entry in year_entries.flatten() {
                        if let Some(year_str) = year_entry.file_name().to_str() {
                            if let Ok(year) = year_str.parse::<u32>() {
                                let year_path = year_entry.path();
                                if let Ok(month_entries) = fs::read_dir(&year_path) {
                                    for month_entry in month_entries.flatten() {
                                        if let Some(month) = month_entry.file_name().to_str() {
                                            year_months.insert((year, month.to_string()));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Generate reports for each year/month
    for (year, month) in year_months {
        generate_reports_for_month(&my_users, year, &month);
    }
    
    println!("\n✅ Reports generated in reports/{{year}}/{{month}}/");
}

fn generate_reports_for_month(my_users: &[&str], year: u32, month: &str) {
    let mut my_report = MonthlyReport {
        year,
        month: month.to_string(),
        user: "mike-merged".to_string(),
        commits: 0,
        files: 0,
        insertions: 0,
        deletions: 0,
        repos: HashMap::new(),
    };
    
    let mut other_users: HashMap<String, MonthlyReport> = HashMap::new();
    
    // Scan all platforms and users
    let activity_base = "data/activity";
    for platform_entry in fs::read_dir(activity_base).unwrap().flatten() {
        let platform = platform_entry.file_name().to_string_lossy().to_string();
        let platform_path = platform_entry.path();
        
        if let Ok(user_entries) = fs::read_dir(&platform_path) {
            for user_entry in user_entries.flatten() {
                let user = user_entry.file_name().to_string_lossy().to_string();
                let activity_file = format!("{}/{}/{}/{}/{}/activity.json", activity_base, platform, user, year, month);
                
                if let Ok(content) = fs::read_to_string(&activity_file) {
                    if let Ok(activities) = serde_json::from_str::<Vec<Activity>>(&content) {
                        let is_my_user = my_users.iter().any(|&u| u == user);
                        
                        for activity in activities {
                            if is_my_user {
                                // Add to my merged report
                                my_report.commits += 1;
                                my_report.files += activity.files_changed.unwrap_or(0);
                                my_report.insertions += activity.insertions.unwrap_or(0);
                                my_report.deletions += activity.deletions.unwrap_or(0);
                                *my_report.repos.entry(activity.repo_name.clone()).or_insert(0) += 1;
                            } else {
                                // Add to other user's report
                                let report = other_users.entry(user.clone()).or_insert_with(|| MonthlyReport {
                                    year,
                                    month: month.to_string(),
                                    user: user.clone(),
                                    commits: 0,
                                    files: 0,
                                    insertions: 0,
                                    deletions: 0,
                                    repos: HashMap::new(),
                                });
                                
                                report.commits += 1;
                                report.files += activity.files_changed.unwrap_or(0);
                                report.insertions += activity.insertions.unwrap_or(0);
                                report.deletions += activity.deletions.unwrap_or(0);
                                *report.repos.entry(activity.repo_name).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Save reports
    let output_dir = format!("reports/{}/{}", year, month);
    fs::create_dir_all(&output_dir).unwrap();
    
    // Save my merged report
    if my_report.commits > 0 {
        let output_file = format!("{}/mike-merged.json", output_dir);
        let json = serde_json::to_string_pretty(&my_report).unwrap();
        fs::write(&output_file, json).unwrap();
        println!("Generated: {} ({} commits)", output_file, my_report.commits);
    }
    
    // Save other users' reports
    for (user, report) in other_users {
        let output_file = format!("{}/{}.json", output_dir, user);
        let json = serde_json::to_string_pretty(&report).unwrap();
        fs::write(&output_file, json).unwrap();
        println!("Generated: {} ({} commits)", output_file, report.commits);
    }
}
