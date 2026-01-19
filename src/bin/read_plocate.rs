use std::process::Command;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Reading plocate database for .git directories...");
    
    // Use plocate command to query database
    let output = Command::new("plocate")
        .args(["--database", "/var/lib/plocate/plocate.db", "/.git"])
        .output()?;
    
    if !output.status.success() {
        eprintln!("❌ plocate failed: {}", String::from_utf8_lossy(&output.stderr));
        return Ok(());
    }
    
    let reader = BufReader::new(&output.stdout[..]);
    let mut git_repos = Vec::new();
    
    for line in reader.lines() {
        let path = line?;
        if path.ends_with("/.git") {
            git_repos.push(path);
        }
    }
    
    println!("✅ Found {} git repos in plocate database", git_repos.len());
    
    // Write to gitdirs.txt
    std::fs::write("gitdirs.txt", git_repos.join("\n"))?;
    println!("💾 Saved to gitdirs.txt");
    
    Ok(())
}
