use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

struct GitProxy {
    cache_root: PathBuf,
}

impl GitProxy {
    fn new(cache_root: PathBuf) -> Self {
        std::fs::create_dir_all(&cache_root).ok();
        Self { cache_root }
    }

    fn normalize_url(&self, url: &str) -> String {
        url.replace("https://", "")
           .replace("http://", "")
           .replace("git://", "")
           .replace("git@", "")
           .replace(":", "/")
           .replace(".git", "")
    }

    fn get_local_path(&self, url: &str) -> PathBuf {
        self.cache_root.join(self.normalize_url(url))
    }

    async fn ensure_cached(&self, url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let local_path = self.get_local_path(url);
        
        if local_path.exists() {
            println!("✓ Cache hit: {}", url);
            return Ok(local_path);
        }

        println!("⬇ Cloning: {}", url);
        std::fs::create_dir_all(local_path.parent().unwrap())?;
        
        Command::new("git")
            .args(&["clone", "--mirror", url, local_path.to_str().unwrap()])
            .status()?;
        
        Ok(local_path)
    }

    async fn handle_git_request(&self, mut client: TcpStream, target_host: &str, target_port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = vec![0u8; 4096];
        let n = client.read(&mut buffer).await?;
        let request = String::from_utf8_lossy(&buffer[..n]);
        
        // Parse git protocol: "git-upload-pack /repo.git\0host=github.com\0"
        if let Some(repo_path) = self.parse_git_request(&request) {
            let url = format!("https://{}{}", target_host, repo_path);
            
            match self.ensure_cached(&url).await {
                Ok(local_path) => {
                    // Serve from local git daemon
                    self.serve_local(&mut client, &local_path).await?;
                    return Ok(());
                }
                Err(e) => eprintln!("Cache failed: {}", e),
            }
        }
        
        // Fallback: proxy to upstream
        let mut upstream = TcpStream::connect((target_host, target_port)).await?;
        upstream.write_all(&buffer[..n]).await?;
        
        tokio::io::copy_bidirectional(&mut client, &mut upstream).await?;
        Ok(())
    }

    fn parse_git_request(&self, request: &str) -> Option<String> {
        if request.starts_with("git-upload-pack") || request.starts_with("git-receive-pack") {
            request.split_whitespace().nth(1).map(|s| s.to_string())
        } else {
            None
        }
    }

    async fn serve_local(&self, client: &mut TcpStream, repo_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // Spawn git-upload-pack and pipe to client
        let mut child = Command::new("git")
            .args(&["upload-pack", "--stateless-rpc", repo_path.to_str().unwrap()])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()?;
        
        let mut stdout = child.stdout.take().unwrap();
        let mut buf = vec![0u8; 8192];
        
        loop {
            use std::io::Read;
            match stdout.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => client.write_all(&buf[..n]).await?,
                Err(_) => break,
            }
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proxy = Arc::new(GitProxy::new(PathBuf::from("/mnt/data1/git")));
    
    // Listen on port 9418 (git protocol)
    let listener = TcpListener::bind("0.0.0.0:9418").await?;
    println!("🔀 Git proxy listening on 0.0.0.0:9418");
    println!("📁 Cache: /mnt/data1/git");
    
    loop {
        let (client, addr) = listener.accept().await?;
        let proxy = proxy.clone();
        
        tokio::spawn(async move {
            println!("📥 Connection from {}", addr);
            if let Err(e) = proxy.handle_git_request(client, "github.com", 9418).await {
                eprintln!("Error: {}", e);
            }
        });
    }
}
