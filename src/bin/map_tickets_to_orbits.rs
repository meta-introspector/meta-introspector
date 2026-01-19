use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub godel_number: u64,
    pub complexity: f64,
    pub lmfdb_orbit: String,
}

pub fn read_all_tickets() -> Vec<Ticket> {
    let dir = "/mnt/data1/nix/vendor/rust/cargo2nix/ai-ml-zk-ops/documentation/art/art/memes/extracted_tickets";
    let mut tickets = Vec::new();
    
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Some(ticket) = parse_ticket(&path) {
                tickets.push(ticket);
            }
        }
    }
    
    tickets
}

fn parse_ticket(path: &Path) -> Option<Ticket> {
    let content = fs::read_to_string(path).ok()?;
    
    // Extract ID
    let id = content.lines()
        .find(|l| l.starts_with("**ID:**"))?
        .split("**ID:**")
        .nth(1)?
        .trim()
        .parse::<u64>()
        .ok()?;
    
    // Extract title
    let title = content.lines()
        .find(|l| l.starts_with("**Title:**"))
        .and_then(|l| l.split("**Title:**").nth(1))
        .unwrap_or("")
        .trim()
        .to_string();
    
    // Extract description
    let description = content.lines()
        .skip_while(|l| !l.starts_with("**Description:**"))
        .skip(1)
        .take_while(|l| !l.starts_with("**") && !l.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    
    // Calculate Gödel number (hash of content)
    let godel_number = calculate_godel(&title, &description);
    
    // Calculate complexity
    let complexity = calculate_complexity(&title, &description);
    
    // Map to LMFDB orbit
    let lmfdb_orbit = map_to_orbit(godel_number, complexity);
    
    Some(Ticket {
        id,
        title,
        description,
        godel_number,
        complexity,
        lmfdb_orbit,
    })
}

fn calculate_godel(title: &str, description: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    title.hash(&mut hasher);
    description.hash(&mut hasher);
    hasher.finish()
}

fn calculate_complexity(title: &str, description: &str) -> f64 {
    let title_len = title.len() as f64;
    let desc_len = description.len() as f64;
    let word_count = description.split_whitespace().count() as f64;
    
    (title_len + desc_len + word_count).log10()
}

fn map_to_orbit(godel: u64, complexity: f64) -> String {
    let conductor = (godel % 1000) as u32;
    let level = (complexity * 10.0) as u32;
    format!("{}.a{}", conductor, level)
}

fn main() {
    println!("📖 Reading 194 tickets...");
    
    let tickets = read_all_tickets();
    
    println!("✅ Found {} unique tickets", tickets.len());
    println!();
    
    // Show first 10
    for ticket in tickets.iter().take(10) {
        println!("🎫 Ticket {}: {}", ticket.id, ticket.title);
        println!("   Gödel: {}", ticket.godel_number);
        println!("   Complexity: {:.2}", ticket.complexity);
        println!("   Orbit: {}", ticket.lmfdb_orbit);
        println!();
    }
    
    // Export to JSON
    let json = serde_json::to_string_pretty(&tickets).unwrap();
    fs::write("/mnt/data1/meta-introspector/tickets_mapped.json", json).unwrap();
    
    println!("💾 Saved to tickets_mapped.json");
}
