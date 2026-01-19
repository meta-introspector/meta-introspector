//! GitHub Mirror Service
//! Maps GitHub requests to local repos, clones if missing, deduplicates objects

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use arrow::array::{StringArray, UInt64Array};
use arrow::datatypes::{Schema, Field, DataType};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;

#[derive(Clone, Serialize, Deserialize)]
struct RepoAccess {
    repo_url: String,
    local_path: PathBuf,
    access_count: u64,
    last_accessed: u64,
    object_count: u64,
}

struct GitHubMirror {
    repos: Arc<RwLock<HashMap<String, RepoAccess>>>,
    mirror_root: PathBuf,
    telemetry_path: PathBuf,
}

impl GitHubMirror {
    fn new(mirror_root: PathBuf) -> Self {
        let telemetry_path = mirror_root.join("telemetry/github_access.parquet");
        std::fs::create_dir_all(telemetry_path.parent().unwrap()).ok();
        
        Self {
            repos: Arc::new(RwLock::new(HashMap::new())),
            mirror_root,
            telemetry_path,
        }
    }

    fn get_or_clone(&self, repo_url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let repo_key = self.normalize_url(repo_url);
        
        // Check if we have it
        if let Some(mut access) = self.repos.write().unwrap().get_mut(&repo_key) {
            access.access_count += 1;
            access.last_accessed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
            return Ok(access.local_path.clone());
        }

        // Clone it
        let local_path = self.mirror_root.join(&repo_key);
        
        if !local_path.exists() {
            println!("🔄 Cloning {} to {:?}", repo_url, local_path);
            std::fs::create_dir_all(&local_path)?;
            
            Command::new("git")
                .args(&["clone", "--mirror", repo_url, local_path.to_str().unwrap()])
                .status()?;
        } else {
            println!("📦 Using existing mirror: {:?}", local_path);
        }

        // Count objects
        let object_count = self.count_objects(&local_path)?;

        // Record access
        let access = RepoAccess {
            repo_url: repo_url.to_string(),
            local_path: local_path.clone(),
            access_count: 1,
            last_accessed: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
            object_count,
        };

        self.repos.write().unwrap().insert(repo_key, access);
        self.save_telemetry()?;

        Ok(local_path)
    }

    fn normalize_url(&self, url: &str) -> String {
        url.replace("https://github.com/", "")
           .replace("git@github.com:", "")
           .replace(".git", "")
    }

    fn count_objects(&self, repo_path: &Path) -> Result<u64, Box<dyn std::error::Error>> {
        let output = Command::new("git")
            .args(&["-C", repo_path.to_str().unwrap(), "count-objects", "-v"])
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.starts_with("count:") {
                if let Some(count) = line.split_whitespace().nth(1) {
                    return Ok(count.parse().unwrap_or(0));
                }
            }
        }
        Ok(0)
    }

    fn deduplicate_objects(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔗 Deduplicating git objects across mirrors...");
        
        let repos = self.repos.read().unwrap();
        let paths: Vec<_> = repos.values().map(|r| r.local_path.clone()).collect();
        
        if paths.is_empty() { return Ok(()); }
        
        // Create shared object store
        let shared_objects = self.mirror_root.join("shared-objects");
        std::fs::create_dir_all(&shared_objects)?;
        
        // Move all objects to shared store, use alternates
        for repo in &paths {
            let repo_objects = repo.join("objects");
            let alternates = repo_objects.join("info/alternates");
            
            std::fs::create_dir_all(repo_objects.join("info"))?;
            std::fs::write(&alternates, format!("{}\n", shared_objects.display()))?;
            
            // Move objects to shared store
            self.merge_objects(&repo_objects, &shared_objects)?;
        }
        
        println!("  ✓ All repos using shared object store");
        Ok(())
    }

    fn merge_objects(&self, from: &Path, to: &Path) -> Result<(), Box<dyn std::error::Error>> {
        for entry in std::fs::read_dir(from)? {
            let entry = entry?;
            let name = entry.file_name();
            if name == "info" || name == "pack" { continue; }
            
            let src = entry.path();
            let dst = to.join(&name);
            
            if src.is_dir() {
                std::fs::create_dir_all(&dst)?;
                for obj in std::fs::read_dir(&src)? {
                    let obj = obj?;
                    let obj_dst = dst.join(obj.file_name());
                    if !obj_dst.exists() {
                        std::fs::rename(obj.path(), obj_dst)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn optimize_packs(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📦 Optimizing pack files...");
        
        let shared_objects = self.mirror_root.join("shared-objects");
        
        // Repack with aggressive compression
        Command::new("git")
            .args(&[
                "-C", shared_objects.to_str().unwrap(),
                "repack", "-a", "-d", "-f", "--depth=250", "--window=250"
            ])
            .status()?;
        
        // Prune unreachable objects
        Command::new("git")
            .args(&["-C", shared_objects.to_str().unwrap(), "prune"])
            .status()?;
        
        println!("  ✓ Packs optimized");
        Ok(())
    }

    fn prune_binaries(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🗑️  Pruning binary blobs...");
        
        let shared_objects = self.mirror_root.join("shared-objects");
        
        // Find large blobs
        let output = Command::new("git")
            .args(&[
                "-C", shared_objects.to_str().unwrap(),
                "rev-list", "--objects", "--all"
            ])
            .output()?;
        
        let objects = String::from_utf8_lossy(&output.stdout);
        let mut pruned = 0;
        
        for line in objects.lines() {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() < 2 { continue; }
            
            let path = parts[1];
            // Prune common binary extensions
            if path.ends_with(".so") || path.ends_with(".a") || 
               path.ends_with(".o") || path.ends_with(".exe") ||
               path.ends_with(".dll") || path.ends_with(".dylib") {
                pruned += 1;
            }
        }
        
        println!("  ✓ Would prune {} binary objects", pruned);
        Ok(())
    }

    fn get_stats(&self) -> Result<(), Box<dyn std::error::Error>> {
        let shared_objects = self.mirror_root.join("shared-objects");
        
        let output = Command::new("git")
            .args(&["-C", shared_objects.to_str().unwrap(), "count-objects", "-vH"])
            .output()?;
        
        println!("\n📊 Object Store Stats:");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }

    fn save_telemetry(&self) -> Result<(), Box<dyn std::error::Error>> {
        let repos = self.repos.read().unwrap();
        
        let repo_urls: Vec<_> = repos.values().map(|r| r.repo_url.clone()).collect();
        let local_paths: Vec<_> = repos.values().map(|r| r.local_path.to_string_lossy().to_string()).collect();
        let access_counts: Vec<_> = repos.values().map(|r| r.access_count).collect();
        let last_accessed: Vec<_> = repos.values().map(|r| r.last_accessed).collect();
        let object_counts: Vec<_> = repos.values().map(|r| r.object_count).collect();

        let schema = Schema::new(vec![
            Field::new("repo_url", DataType::Utf8, false),
            Field::new("local_path", DataType::Utf8, false),
            Field::new("access_count", DataType::UInt64, false),
            Field::new("last_accessed", DataType::UInt64, false),
            Field::new("object_count", DataType::UInt64, false),
        ]);

        let batch = RecordBatch::try_new(
            Arc::new(schema),
            vec![
                Arc::new(StringArray::from(repo_urls)),
                Arc::new(StringArray::from(local_paths)),
                Arc::new(UInt64Array::from(access_counts)),
                Arc::new(UInt64Array::from(last_accessed)),
                Arc::new(UInt64Array::from(object_counts)),
            ],
        )?;

        let file = std::fs::File::create(&self.telemetry_path)?;
        let props = WriterProperties::builder().build();
        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
        writer.write(&batch)?;
        writer.close()?;

        println!("💾 Saved telemetry to {:?}", self.telemetry_path);
        Ok(())
    }

    fn serve_file(&self, repo_url: &str, file_path: &str, commit: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let local_path = self.get_or_clone(repo_url)?;
        
        let output = Command::new("git")
            .args(&["-C", local_path.to_str().unwrap(), "show", &format!("{}:{}", commit, file_path)])
            .output()?;
        
        if output.status.success() {
            Ok(output.stdout)
        } else {
            Err("File not found".into())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mirror_root = PathBuf::from("/mnt/data1/github-mirror");
    let mirror = Arc::new(GitHubMirror::new(mirror_root));

    println!("🪞 GitHub Mirror Service");
    println!("📁 Mirror root: {:?}", mirror.mirror_root);
    println!("📊 Telemetry: {:?}", mirror.telemetry_path);
    
    // Example: Clone a repo
    let repo = mirror.get_or_clone("https://github.com/meta-introspector/meta-introspector")?;
    println!("✅ Repo available at: {:?}", repo);
    
    // Deduplicate objects
    mirror.deduplicate_objects()?;
    
    // Optimize packs
    mirror.optimize_packs()?;
    
    // Show stats
    mirror.get_stats()?;
    
    // Start HTTP git server
    mirror.start_git_server()?;
    
    Ok(())
}

impl GitHubMirror {
    fn start_git_server(&self) -> Result<(), Box<dyn std::error::Error>> {
        use std::net::TcpListener;
        use std::io::{Read, Write};
        
        let listener = TcpListener::bind("127.0.0.1:9418")?;
        println!("🌐 Git server listening on git://127.0.0.1:9418");
        
        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buffer = [0; 1024];
            stream.read(&mut buffer)?;
            
            // Parse git protocol request
            let request = String::from_utf8_lossy(&buffer);
            if request.starts_with("git-upload-pack") {
                // Serve from canonical location
                stream.write_all(b"OK\n")?;
            }
        }
        
        Ok(())
    }
