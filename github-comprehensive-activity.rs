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
    
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN required");
    let octocrab = Octocrab::builder().personal_token(token).build()?;
    
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
        println!("  Found event from @{}: {:?}", event.actor.login, event.r#type);
        
        // Filter by date
        let event_date = event.created_at.to_string();
        if !event_date.starts_with(&format!("{}-{:02}", year, month)) {
            continue;
        }
        
        match event.r#type.as_str() {
            "PushEvent" => {
                if let Some(payload) = event.payload {
                    if let Some(commits) = payload.get("commits") {
                        if let Some(commits_array) = commits.as_array() {
                            for commit in commits_array {
                                activity.commits.push(CommitEvent {
                                    date: event_date.clone(),
                                    repo: event.repo.name.clone(),
                                    sha: commit["sha"].as_str().unwrap_or("").to_string(),
                                    message: commit["message"].as_str().unwrap_or("").to_string(),
                                });
                            }
                        }
                    }
                }
            }
            "WatchEvent" => {
                // Star event
                activity.stars.push(StarEvent {
                    date: event_date.clone(),
                    repo: event.repo.name.clone(),
                    repo_url: format!("https://github.com/{}", event.repo.name),
                });
            }
            "ForkEvent" => {
                if let Some(payload) = event.payload {
                    if let Some(forkee) = payload.get("forkee") {
                        activity.forks.push(ForkEvent {
                            date: event_date.clone(),
                            source_repo: event.repo.name.clone(),
                            forked_repo: forkee["full_name"].as_str().unwrap_or("").to_string(),
                        });
                    }
                }
            }
            "PullRequestEvent" => {
                if let Some(payload) = event.payload {
                    if let Some(pr) = payload.get("pull_request") {
                        activity.pull_requests.push(PREvent {
                            date: event_date.clone(),
                            repo: event.repo.name.clone(),
                            number: pr["number"].as_u64().unwrap_or(0),
                            title: pr["title"].as_str().unwrap_or("").to_string(),
                            state: pr["state"].as_str().unwrap_or("").to_string(),
                        });
                    }
                }
            }
            "IssuesEvent" => {
                if let Some(payload) = event.payload {
                    if let Some(issue) = payload.get("issue") {
                        activity.issues.push(IssueEvent {
                            date: event_date.clone(),
                            repo: event.repo.name.clone(),
                            number: issue["number"].as_u64().unwrap_or(0),
                            title: issue["title"].as_str().unwrap_or("").to_string(),
                            state: issue["state"].as_str().unwrap_or("").to_string(),
                        });
                    }
                }
            }
            "PullRequestReviewEvent" => {
                if let Some(payload) = event.payload {
                    if let Some(review) = payload.get("review") {
                        activity.reviews.push(ReviewEvent {
                            date: event_date.clone(),
                            repo: event.repo.name.clone(),
                            pr_number: payload["pull_request"]["number"].as_u64().unwrap_or(0),
                            state: review["state"].as_str().unwrap_or("").to_string(),
                        });
                    }
                }
            }
            _ => {}
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
