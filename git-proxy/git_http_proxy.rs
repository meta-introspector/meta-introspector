use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri};
use std::convert::Infallible;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

struct GitHttpProxy {
    cache_root: PathBuf,
    client: Client<hyper::client::HttpConnector>,
}

impl GitHttpProxy {
    fn new(cache_root: PathBuf) -> Self {
        std::fs::create_dir_all(&cache_root).ok();
        Self {
            cache_root,
            client: Client::new(),
        }
    }

    fn parse_git_url(&self, uri: &Uri) -> Option<(String, String)> {
        let host = uri.host()?;
        let path = uri.path();
        
        if path.contains("/info/refs") || path.contains("/git-upload-pack") || path.contains("/git-receive-pack") {
            let repo_path = path.split("/info/refs").next()
                               .or_else(|| path.split("/git-upload-pack").next())
                               .or_else(|| path.split("/git-receive-pack").next())?
                               .trim_end_matches(".git");
            Some((host.to_string(), repo_path.to_string()))
        } else {
            None
        }
    }

    fn get_local_path(&self, host: &str, repo_path: &str) -> PathBuf {
        self.cache_root.join(host).join(repo_path.trim_start_matches('/'))
    }

    async fn ensure_cached(&self, host: &str, repo_path: &str) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
        let local_path = self.get_local_path(host, repo_path);
        
        if local_path.exists() {
            Command::new("git")
                .args(&["-C", local_path.to_str().unwrap(), "fetch", "--all"])
                .output()?;
            return Ok(local_path);
        }

        let url = format!("https://{}{}.git", host, repo_path);
        println!("⬇ Cloning: {}", url);
        
        std::fs::create_dir_all(local_path.parent().unwrap())?;
        Command::new("git")
            .args(&["clone", "--mirror", &url, local_path.to_str().unwrap()])
            .status()?;
        
        Ok(local_path)
    }

    async fn handle_request(&self, req: Request<Body>) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        let uri = req.uri().clone();
        
        if let Some((host, repo_path)) = self.parse_git_url(&uri) {
            if let Ok(local_path) = self.ensure_cached(&host, &repo_path).await {
                println!("✓ Serving from cache: {}{}", host, repo_path);
                return self.serve_from_cache(req, &local_path).await;
            }
        }
        
        // Fallback: proxy to upstream
        let upstream_uri = format!("https://{}{}", uri.host().unwrap_or("github.com"), uri.path_and_query().map(|x| x.as_str()).unwrap_or("/"));
        let upstream_req = Request::builder()
            .method(req.method())
            .uri(upstream_uri)
            .body(req.into_body())?;
        
        Ok(self.client.request(upstream_req).await?)
    }

    async fn serve_from_cache(&self, req: Request<Body>, repo_path: &PathBuf) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        let path = req.uri().path();
        
        if path.contains("/info/refs") {
            let service = req.uri().query().and_then(|q| {
                q.split('&').find(|s| s.starts_with("service=")).map(|s| s.split('=').nth(1).unwrap())
            }).unwrap_or("git-upload-pack");
            
            let output = Command::new("git")
                .args(&[service.trim_start_matches("git-"), "--stateless-rpc", "--advertise-refs", repo_path.to_str().unwrap()])
                .output()?;
            
            let body = format!("001e# service={}\n0000{}", service, String::from_utf8_lossy(&output.stdout));
            
            return Ok(Response::builder()
                .header("Content-Type", format!("application/x-{}-advertisement", service))
                .body(Body::from(body))?);
        }
        
        if path.contains("/git-upload-pack") {
            let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
            
            let output = Command::new("git")
                .args(&["upload-pack", "--stateless-rpc", repo_path.to_str().unwrap()])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()?
                .stdin.unwrap().write_all(&body_bytes).and_then(|_| {
                    Command::new("git")
                        .args(&["upload-pack", "--stateless-rpc", repo_path.to_str().unwrap()])
                        .output()
                })?;
            
            return Ok(Response::builder()
                .header("Content-Type", "application/x-git-upload-pack-result")
                .body(Body::from(output.stdout))?);
        }
        
        Ok(Response::builder().status(404).body(Body::from("Not found"))?)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proxy = Arc::new(GitHttpProxy::new(PathBuf::from("/mnt/data1/git")));
    
    let make_svc = make_service_fn(move |_| {
        let proxy = proxy.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let proxy = proxy.clone();
                async move {
                    proxy.handle_request(req).await.or_else(|e| {
                        eprintln!("Error: {}", e);
                        Ok::<_, Infallible>(Response::builder().status(500).body(Body::from("Error")).unwrap())
                    })
                }
            }))
        }
    });
    
    let addr = ([0, 0, 0, 0], 8128).into();
    let server = Server::bind(&addr).serve(make_svc);
    
    println!("🔀 Git HTTP proxy listening on 0.0.0.0:8128");
    println!("📁 Cache: /mnt/data1/git");
    
    server.await?;
    Ok(())
}
