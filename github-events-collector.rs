// Fetch GitHub events per month/user and store in canonical format
// Usage: github-events-collector <user> <start-year> <start-month> <end-year> <end-month>

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    let user = args.get(1).map(|s| s.as_str()).unwrap_or("jmikedupont2");
    let start_year: i32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2024);
    let start_month: u32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1);
    let end_year: i32 = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(2026);
    let end_month: u32 = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(1);
    
    println!("🔍 Fetching GitHub events for @{}", user);
    println!("   From: {}-{:02} to {}-{:02}", start_year, start_month, end_year, end_month);
    
    // Fetch all events first (verbatim)
    println!("\n📥 Fetching all events...");
    
    // Try user events first, fall back to org events
    let mut output = Command::new("gh")
        .args(&["api", &format!("/users/{}/events", user), "--paginate"])
        .output()?;
    
    // Check if empty array
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    if !output.status.success() || stdout_str.trim() == "[]" {
        println!("   Trying org events API...");
        output = Command::new("gh")
            .args(&["api", &format!("/orgs/{}/events", user), "--paginate"])
            .output()?;
    }
    
    if !output.status.success() {
        eprintln!("Error fetching events: {}", String::from_utf8_lossy(&output.stderr));
        return Err("Failed to fetch events".into());
    }
    
    let all_events: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout)?;
    println!("   Fetched {} total events", all_events.len());
    
    // Save raw events
    let raw_dir = PathBuf::from("data/github-activity/raw");
    fs::create_dir_all(&raw_dir)?;
    let raw_file = raw_dir.join(format!("{}_all_events.json", user));
    fs::write(&raw_file, serde_json::to_string_pretty(&all_events)?)?;
    println!("   Saved raw: {}", raw_file.display());
    
    // Split by month and save to canonical store
    println!("\n📂 Organizing by month...");
    let mut months_saved = 0;
    
    let mut year = start_year;
    let mut month = start_month;
    
    while year < end_year || (year == end_year && month <= end_month) {
        let month_str = format!("{}-{:02}", year, month);
        
        // Filter events for this month
        let month_events: Vec<&serde_json::Value> = all_events
            .iter()
            .filter(|e| {
                e["created_at"]
                    .as_str()
                    .map(|s| s.starts_with(&month_str))
                    .unwrap_or(false)
            })
            .collect();
        
        if !month_events.is_empty() {
            // Save to canonical store: data/activity/github/{user}/{year}/{month}/events.json
            let canonical_dir = PathBuf::from(format!(
                "data/activity/github/{}/{}/{:02}",
                user, year, month
            ));
            fs::create_dir_all(&canonical_dir)?;
            
            let canonical_file = canonical_dir.join("events.json");
            fs::write(&canonical_file, serde_json::to_string_pretty(&month_events)?)?;
            
            println!("   {} - {} events -> {}", month_str, month_events.len(), canonical_file.display());
            months_saved += 1;
        }
        
        // Next month
        month += 1;
        if month > 12 {
            month = 1;
            year += 1;
        }
    }
    
    println!("\n✅ Complete!");
    println!("   Months saved: {}", months_saved);
    println!("   Canonical store: data/activity/github/{user}/{year}/{month}/events.json");
    
    Ok(())
}
