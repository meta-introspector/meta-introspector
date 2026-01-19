use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mirror_dir = Path::new("/mnt/data1/git");
    let gitdirs = fs::read_to_string("gitdirs.txt")?;
    
    let mut count = 0;
    let mut skipped = 0;
    let mut duplicates = 0;
    let mut url_counts: HashMap<String, usize> = HashMap::new();
    
    for gitdir in gitdirs.lines() {
        let repo_dir = Path::new(gitdir).parent().unwrap();
        
        // Get remote URL
        let output = Command::new("git")
            .args(["-C", repo_dir.to_str().unwrap(), "remote", "get-url", "origin"])
            .output();
        
        let remote = match output {
            Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
            _ => { skipped += 1; continue; }
        };
        
        if remote.is_empty() { skipped += 1; continue; }
        
        // Normalize URL to path
        let path = remote
            .replace("https://", "")
            .replace("http://", "")
            .replace("git://", "")
            .replace("git@", "")
            .replace(":", "/")
            .trim_end_matches(".git")
            .to_string();
        
        let target = mirror_dir.join(&path);
        
        // If target exists, create numbered link in <target>/links/N
        if target.exists() {
            let counter = url_counts.entry(path.clone()).or_insert(1);
            *counter += 1;
            
            let links_dir = target.join("links");
            fs::create_dir_all(&links_dir).ok();
            
            let link_path = links_dir.join(counter.to_string());
            
            if symlink(repo_dir, &link_path).is_ok() {
                duplicates += 1;
            }
            continue;
        }
        
        // Create parent directory
        if let Some(parent) = target.parent() {
            if let Err(_) = fs::create_dir_all(parent) {
                skipped += 1;
                continue;
            }
        }
        
        // Create primary symlink
        if symlink(repo_dir, &target).is_ok() {
            count += 1;
            if count % 100 == 0 {
                println!("  ✅ Linked {} repos...", count);
            }
        } else {
            skipped += 1;
        }
    }
    
    println!("✅ Created {} primary symlinks", count);
    println!("🔗 Created {} duplicate links in repo/links/", duplicates);
    println!("⏭️  Skipped {} (no remote or errors)", skipped);
    
    Ok(())
}
