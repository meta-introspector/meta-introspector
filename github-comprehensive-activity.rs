// Comprehensive GitHub Activity Tracker
// Tracks: commits, stars, forks, PRs, issues, reviews
// Stores in: data/activity/github/{user}/{year}/{month}/

use octocrab::Octocrab;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GitHubActivity {
    year: i32,
    month: u32,
    user: String,
    commits: Vec<CommitEvent>,
    stars: Vec<StarEvent>,
    forks: Vec<ForkEvent>,
    pull_requests: Vec<PREvent>,
    issues: Vec<IssueEvent>,
    reviews: Vec<ReviewEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CommitEvent {
    date: String,
    repo: String,
    sha: String,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StarEvent {
    date: String,
    repo: String,
    repo_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ForkEvent {
    date: String,
    source_repo: String,
    forked_repo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PREvent {
    date: String,
    repo: String,
    number: u64,
    title: String,
    state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IssueEvent {
    date: String,
    repo: String,
    number: u64,
    title: String,
    state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReviewEvent {
    date: String,
    repo: String,
    pr_number: u64,
    state: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    let user = args.get(1).map(|s| s.as_str()).unwrap_or("meta-introspector");
    let year: i32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(2024);
    let month: u32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(1);
    
    println!("🔍 Fetching comprehensive GitHub activity for {}/{:02} - @{}", year, month, user);
    
    let octocrab = if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        println!("Using authenticated access");
        Octocrab::builder().personal_token(token).build()?
    } else {
        println!("⚠️  No GITHUB_TOKEN - using unauthenticated (rate limited to 60/hour)");
        Octocrab::builder().build()?
    };
    
    let mut activity = GitHubActivity {
        year,
        month,
        user: user.to_string(),
        commits: Vec::new(),
        stars: Vec::new(),
        forks: Vec::new(),
        pull_requests: Vec::new(),
        issues: Vec::new(),
        reviews: Vec::new(),
    };
    
    // Get events (public events, last 90 days max)
    println!("Fetching public events...");
    let response = octocrab
        .events()
        .per_page(100)
        .send()
        .await?;
    
    let page = response.value.ok_or("No events returned")?;
    println!("Processing {} events...", page.items.len());
    
    for event in page.items {
        // Check user (actor)
        if event.actor.login != user {
            println!("  Skipping event from @{} (looking for @{})", event.actor.login, user);
            continue;
        }
        
        println!("  ✓ Event from @{}: {:?} at {}", 
            event.actor.login, 
            event.r#type, 
            event.created_at
        );
        println!("    Repo: {}", event.repo.name);
        
        // TODO_PARSE_EVENT_TYPE! - Match on EventType enum variants
        // TODO_EXTRACT_COMMITS! - Get commits from PushEvent payload
        // TODO_EXTRACT_STARS! - Get starred repo from WatchEvent
        // TODO_EXTRACT_FORKS! - Get fork info from ForkEvent payload
        // TODO_EXTRACT_PRS! - Get PR details from PullRequestEvent payload
        // TODO_EXTRACT_ISSUES! - Get issue details from IssuesEvent payload
        // TODO_EXTRACT_REVIEWS! - Get review details from PullRequestReviewEvent payload
        
        // For now, just count by type
        match event.r#type {
            octocrab::models::events::EventType::PushEvent => {
                activity.commits.push(CommitEvent {
                    date: event.created_at.to_string(),
                    repo: event.repo.name.clone(),
                    sha: "TODO".to_string(), // TODO_EXTRACT_COMMITS!
                    message: "TODO".to_string(),
                });
            }
            octocrab::models::events::EventType::WatchEvent => {
                activity.stars.push(StarEvent {
                    date: event.created_at.to_string(),
                    repo: event.repo.name.clone(),
                    repo_url: format!("https://github.com/{}", event.repo.name),
                });
            }
            octocrab::models::events::EventType::ForkEvent => {
                activity.forks.push(ForkEvent {
                    date: event.created_at.to_string(),
                    source_repo: event.repo.name.clone(),
                    forked_repo: "TODO".to_string(), // TODO_EXTRACT_FORKS!
                });
            }
            _ => {
                // TODO_HANDLE_OTHER_EVENTS! - PRs, Issues, Reviews
            }
        }
    }
    
    // Print summary
    println!("\n📊 Activity Summary for {}/{:02}", year, month);
    println!("  Commits: {}", activity.commits.len());
    println!("  Stars: {}", activity.stars.len());
    println!("  Forks: {}", activity.forks.len());
    println!("  Pull Requests: {}", activity.pull_requests.len());
    println!("  Issues: {}", activity.issues.len());
    println!("  Reviews: {}", activity.reviews.len());
    
    // Save
    let output_dir = format!("data/activity/github/{}/{}/{:02}", user, year, month);
    fs::create_dir_all(&output_dir)?;
    
    let output_file = format!("{}/github_activity.json", output_dir);
    fs::write(&output_file, serde_json::to_string_pretty(&activity)?)?;
    
    println!("\n✅ Saved to: {}", output_file);
    
    Ok(())
}
