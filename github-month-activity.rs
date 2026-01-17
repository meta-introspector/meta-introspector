// Pull GitHub activity for a specific month using octocrab
// Usage: github-month-activity <user> <year> <month>

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommitActivity {
    commit_hash: String,
    author_name: String,
    author_email: String,
    author_date: String,
    message: String,
    repo_name: String,
    repo_url: String,
    platform: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    let user = args.get(1).map(|s| s.as_str()).unwrap_or("meta-introspector");
    let year: i32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2024);
    let month: u32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1);
    
    println!("🔍 Fetching GitHub activity for {}/{:02} - @{}", year, month, user);
    
    // Setup octocrab
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = octocrab::Octocrab::builder().personal_token(token).build()?;
    
    // Get user's repos
    println!("Fetching repositories...");
    let repos = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .type_("owner")
        .per_page(100)
        .send()
        .await?;
    
    println!("Found {} repositories", repos.items.len());
    
    let mut all_commits = Vec::new();
    
    // Check each repo for commits in the target month
    for repo in repos.items {
        let repo_name = &repo.name;
        
        // Get commits for this month
        let since = format!("{}-{:02}-01T00:00:00Z", year, month);
        let until = if month == 12 {
            format!("{:04}-01-01T00:00:00Z", year + 1)
        } else {
            format!("{:04}-{:02}-01T00:00:00Z", year, month + 1)
        };
        
        match octocrab
            .repos(user, repo_name)
            .list_commits()
            .since(since.parse()?)
            .until(until.parse()?)
            .send()
            .await
        {
            Ok(commits) => {
                if !commits.items.is_empty() {
                    println!("  {} - {} commits", repo_name, commits.items.len());
                    
                    for commit in commits.items {
                        let commit_data = &commit.commit;
                        if let Some(author) = &commit_data.author {
                            all_commits.push(CommitActivity {
                                commit_hash: commit.sha,
                                author_name: author.name.clone(),
                                author_email: author.email.clone(),
                                author_date: author.date.map(|d| d.to_string()).unwrap_or_default(),
                                message: commit_data.message.clone(),
                                repo_name: repo_name.clone(),
                                repo_url: repo.html_url.as_ref().map(|u| u.to_string()).unwrap_or_default(),
                                platform: "github".to_string(),
                            });
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("  {} - Error: {}", repo_name, e);
            }
        }
    }
    
    println!("\n📊 Summary:");
    println!("Total commits found: {}", all_commits.len());
    
    if !all_commits.is_empty() {
        // Save to file
        let output_dir = "data/my-activity".to_string();
        fs::create_dir_all(&output_dir)?;
        
        let output_file = format!("{}/github_{}_{:02}_activity.json", output_dir, year, month);
        fs::write(&output_file, serde_json::to_string_pretty(&all_commits)?)?;
        
        println!("✅ Saved to: {}", output_file);
    } else {
        println!("No commits found for this month");
    }
    
    Ok(())
}
