// git_activity_collector.rs
// Collect all git activity from the last year for investor report

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearlyGitActivity {
    pub year: u32,
    pub total_commits: usize,
    pub total_repos: usize,
    pub repos: Vec<RepoActivity>,
    pub monthly_breakdown: HashMap<String, MonthlyStats>,
    pub top_projects: Vec<ProjectSummary>,
    pub technologies: HashMap<String, TechStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoActivity {
    pub name: String,
    pub path: String,
    pub commits: usize,
    pub first_commit: String,
    pub last_commit: String,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub files_changed: usize,
    pub languages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyStats {
    pub commits: usize,
    pub repos_active: usize,
    pub lines_added: usize,
    pub lines_removed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub name: String,
    pub description: String,
    pub commits: usize,
    pub impact: String,
    pub technologies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechStats {
    pub files: usize,
    pub lines: usize,
    pub commits: usize,
}

impl YearlyGitActivity {
    pub fn collect_from_year(year: u32) -> Result<Self, Box<dyn std::error::Error>> {
        println!("📊 Collecting git activity for year {}", year);
        
        let mut activity = Self {
            year,
            total_commits: 0,
            total_repos: 0,
            repos: Vec::new(),
            monthly_breakdown: HashMap::new(),
            top_projects: Vec::new(),
            technologies: HashMap::new(),
        };
        
        // Find all git repos
        let repos = Self::find_all_repos()?;
        println!("Found {} repositories", repos.len());
        
        for repo_path in repos {
            if let Ok(repo_activity) = Self::analyze_repo(&repo_path, year) {
                if repo_activity.commits > 0 {
                    activity.total_commits += repo_activity.commits;
                    activity.repos.push(repo_activity);
                }
            }
        }
        
        activity.total_repos = activity.repos.len();
        activity.compute_monthly_breakdown();
        activity.identify_top_projects();
        activity.analyze_technologies();
        
        Ok(activity)
    }
    
    fn find_all_repos() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let output = Command::new("find")
            .args(&[
                "/mnt/data1",
                "-name", ".git",
                "-type", "d",
                "-not", "-path", "*/node_modules/*",
                "-not", "-path", "*/.cache/*"
            ])
            .output()?;
        
        let repos: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter_map(|line| {
                Path::new(line).parent().map(|p| p.to_string_lossy().to_string())
            })
            .collect();
        
        Ok(repos)
    }
    
    fn analyze_repo(repo_path: &str, year: u32) -> Result<RepoActivity, Box<dyn std::error::Error>> {
        let since = format!("{}-01-01", year);
        let until = format!("{}-12-31", year);
        
        // Get commit count
        let commits_output = Command::new("git")
            .args(&[
                "-C", repo_path,
                "log",
                "--oneline",
                &format!("--since={}", since),
                &format!("--until={}", until),
                "--author=mdupont",
            ])
            .output()?;
        
        let commits = String::from_utf8_lossy(&commits_output.stdout)
            .lines()
            .count();
        
        if commits == 0 {
            return Ok(RepoActivity {
                name: Path::new(repo_path).file_name().unwrap().to_string_lossy().to_string(),
                path: repo_path.to_string(),
                commits: 0,
                first_commit: String::new(),
                last_commit: String::new(),
                lines_added: 0,
                lines_removed: 0,
                files_changed: 0,
                languages: Vec::new(),
            });
        }
        
        // Get stats
        let stats_output = Command::new("git")
            .args(&[
                "-C", repo_path,
                "log",
                "--shortstat",
                &format!("--since={}", since),
                &format!("--until={}", until),
                "--author=mdupont",
            ])
            .output()?;
        
        let stats = String::from_utf8_lossy(&stats_output.stdout);
        let (lines_added, lines_removed, files_changed) = Self::parse_stats(&stats);
        
        // Get first and last commit
        let first_commit = Self::get_first_commit(repo_path, &since, &until)?;
        let last_commit = Self::get_last_commit(repo_path, &since, &until)?;
        
        // Detect languages
        let languages = Self::detect_languages(repo_path)?;
        
        Ok(RepoActivity {
            name: Path::new(repo_path).file_name().unwrap().to_string_lossy().to_string(),
            path: repo_path.to_string(),
            commits,
            first_commit,
            last_commit,
            lines_added,
            lines_removed,
            files_changed,
            languages,
        })
    }
    
    fn parse_stats(stats: &str) -> (usize, usize, usize) {
        let mut lines_added = 0;
        let mut lines_removed = 0;
        let mut files_changed = 0;
        
        for line in stats.lines() {
            if line.contains("insertion") || line.contains("deletion") {
                if let Some(added) = line.split("insertion").next() {
                    if let Some(num) = added.split_whitespace().last() {
                        lines_added += num.parse::<usize>().unwrap_or(0);
                    }
                }
                if let Some(removed) = line.split("deletion").next() {
                    if let Some(num) = removed.split_whitespace().last() {
                        lines_removed += num.parse::<usize>().unwrap_or(0);
                    }
                }
            }
            if line.contains("file") {
                if let Some(num) = line.split_whitespace().next() {
                    files_changed += num.parse::<usize>().unwrap_or(0);
                }
            }
        }
        
        (lines_added, lines_removed, files_changed)
    }
    
    fn get_first_commit(repo_path: &str, since: &str, until: &str) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("git")
            .args(&[
                "-C", repo_path,
                "log",
                "--reverse",
                "--format=%h %s",
                &format!("--since={}", since),
                &format!("--until={}", until),
                "--author=mdupont",
                "-1"
            ])
            .output()?;
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    
    fn get_last_commit(repo_path: &str, since: &str, until: &str) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("git")
            .args(&[
                "-C", repo_path,
                "log",
                "--format=%h %s",
                &format!("--since={}", since),
                &format!("--until={}", until),
                "--author=mdupont",
                "-1"
            ])
            .output()?;
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    
    fn detect_languages(repo_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut languages = Vec::new();
        
        let extensions = vec![
            ("rs", "Rust"),
            ("py", "Python"),
            ("js", "JavaScript"),
            ("ts", "TypeScript"),
            ("nix", "Nix"),
            ("md", "Markdown"),
        ];
        
        for (ext, lang) in extensions {
            let output = Command::new("find")
                .args(&[repo_path, "-name", &format!("*.{}", ext), "-type", "f"])
                .output()?;
            
            if !output.stdout.is_empty() {
                languages.push(lang.to_string());
            }
        }
        
        Ok(languages)
    }
    
    fn compute_monthly_breakdown(&mut self) {
        // TODO: Parse commit dates and group by month
    }
    
    fn identify_top_projects(&mut self) {
        let mut sorted = self.repos.clone();
        sorted.sort_by(|a, b| b.commits.cmp(&a.commits));
        
        for repo in sorted.iter().take(10) {
            self.top_projects.push(ProjectSummary {
                name: repo.name.clone(),
                description: format!("{} commits, {} files changed", repo.commits, repo.files_changed),
                commits: repo.commits,
                impact: Self::calculate_impact(repo),
                technologies: repo.languages.clone(),
            });
        }
    }
    
    fn calculate_impact(repo: &RepoActivity) -> String {
        let total_lines = repo.lines_added + repo.lines_removed;
        match total_lines {
            0..=1000 => "Small".to_string(),
            1001..=10000 => "Medium".to_string(),
            10001..=100000 => "Large".to_string(),
            _ => "Massive".to_string(),
        }
    }
    
    fn analyze_technologies(&mut self) {
        for repo in &self.repos {
            for lang in &repo.languages {
                let stats = self.technologies.entry(lang.clone()).or_insert(TechStats {
                    files: 0,
                    lines: 0,
                    commits: 0,
                });
                stats.commits += repo.commits;
                stats.lines += repo.lines_added;
            }
        }
    }
    
    pub fn generate_investor_report(&self) -> String {
        format!(
            "# Git Activity Report - Year {}\n\n\
             ## Executive Summary\n\
             - **Total Commits**: {:,}\n\
             - **Active Repositories**: {}\n\
             - **Lines Added**: {:,}\n\
             - **Lines Removed**: {:,}\n\n\
             ## Top 10 Projects\n{}\n\n\
             ## Technologies Used\n{}\n",
            self.year,
            self.total_commits,
            self.total_repos,
            self.repos.iter().map(|r| r.lines_added).sum::<usize>(),
            self.repos.iter().map(|r| r.lines_removed).sum::<usize>(),
            self.format_top_projects(),
            self.format_technologies()
        )
    }
    
    fn format_top_projects(&self) -> String {
        self.top_projects.iter()
            .enumerate()
            .map(|(i, p)| format!(
                "{}. **{}** - {} commits ({} impact)\n   Technologies: {}\n",
                i + 1, p.name, p.commits, p.impact, p.technologies.join(", ")
            ))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn format_technologies(&self) -> String {
        let mut techs: Vec<_> = self.technologies.iter().collect();
        techs.sort_by(|a, b| b.1.commits.cmp(&a.1.commits));
        
        techs.iter()
            .map(|(name, stats)| format!(
                "- **{}**: {} commits, {:,} lines\n",
                name, stats.commits, stats.lines
            ))
            .collect::<Vec<_>>()
            .join("")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Git Activity Collector for Investor Report\n");
    
    // Collect 2025 activity
    let activity_2025 = YearlyGitActivity::collect_from_year(2025)?;
    
    // Save JSON
    fs::create_dir_all("data/investor_reports")?;
    let json = serde_json::to_string_pretty(&activity_2025)?;
    fs::write("data/investor_reports/git_activity_2025.json", json)?;
    
    // Generate report
    let report = activity_2025.generate_investor_report();
    fs::write("data/investor_reports/INVESTOR_REPORT_2025.md", report)?;
    
    println!("\n✅ Report generated!");
    println!("📊 JSON: data/investor_reports/git_activity_2025.json");
    println!("📄 Report: data/investor_reports/INVESTOR_REPORT_2025.md");
    
    Ok(())
}
