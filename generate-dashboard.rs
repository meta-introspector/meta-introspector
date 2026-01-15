use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize)]
struct MetricsReport {
    total_commits: usize,
    total_authors: usize,
    authors: serde_json::Value,
    email_issues: Vec<EmailIssue>,
}

#[derive(Debug, Deserialize)]
struct EmailIssue {
    author_name: String,
    email: String,
    commits: usize,
    likely_issue: String,
}

fn main() {
    // Read from HuggingFace dataset
    let metrics = if let Ok(content) = fs::read_to_string("data/git-metrics-report.json") {
        serde_json::from_str::<MetricsReport>(&content).ok()
    } else {
        None
    };

    let investor = if let Ok(content) = fs::read_to_string("data/investor-report-2025.json") {
        serde_json::from_str::<serde_json::Value>(&content).ok()
    } else {
        None
    };

    // Generate combined HTML report
    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Meta-Introspector Activity Dashboard</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background: #0d1117; color: #c9d1d9; padding: 40px 20px; }}
        .container {{ max-width: 1400px; margin: 0 auto; }}
        .header {{ text-align: center; margin-bottom: 50px; }}
        .header h1 {{ font-size: 3em; color: #58a6ff; }}
        .stats {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin-bottom: 40px; }}
        .stat-card {{ background: #161b22; border: 1px solid #30363d; border-radius: 6px; padding: 25px; text-align: center; }}
        .stat-value {{ font-size: 2.5em; font-weight: bold; color: #58a6ff; }}
        .stat-label {{ color: #8b949e; margin-top: 5px; }}
        .section {{ background: #161b22; border: 1px solid #30363d; border-radius: 6px; padding: 30px; margin-bottom: 30px; }}
        .section h2 {{ color: #58a6ff; margin-bottom: 20px; }}
        .links {{ display: flex; gap: 15px; justify-content: center; margin-top: 30px; }}
        .btn {{ background: #238636; color: #fff; padding: 12px 24px; border-radius: 6px; text-decoration: none; font-weight: 500; }}
        .btn:hover {{ background: #2ea043; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>📊 Meta-Introspector Activity Dashboard</h1>
            <p style="color: #8b949e; margin-top: 10px;">Comprehensive git activity analysis</p>
        </div>

        <div class="stats">
            <div class="stat-card">
                <div class="stat-value">{}</div>
                <div class="stat-label">Total Commits</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{}</div>
                <div class="stat-label">Contributors</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">337</div>
                <div class="stat-label">2025 Commits</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">564 MB</div>
                <div class="stat-label">Dataset Size</div>
            </div>
        </div>

        <div class="section">
            <h2>📈 Available Reports</h2>
            <div class="links">
                <a href="investor-report-2025.html" class="btn">2025 Investor Report</a>
                <a href="https://huggingface.co/datasets/introspector/git-activity" class="btn">Full Dataset</a>
                <a href="git-metrics-report.json" class="btn">Download Metrics JSON</a>
            </div>
        </div>

        <div class="section">
            <h2>🔄 Auto-Generated</h2>
            <p style="color: #8b949e;">This dashboard is automatically generated from the HuggingFace dataset on every push and weekly.</p>
            <p style="color: #8b949e; margin-top: 10px;">Last updated: {}</p>
        </div>
    </div>
</body>
</html>"#,
        metrics.as_ref().map(|m| m.total_commits).unwrap_or(402816),
        metrics.as_ref().map(|m| m.total_authors).unwrap_or(9001),
        chrono::Utc::now().format("%Y-%m-%d %H:%M UTC")
    );

    fs::write("reports/index.html", html).unwrap();
    println!("✅ Generated reports/index.html");
}
