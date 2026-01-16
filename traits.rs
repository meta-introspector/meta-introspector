// Traits for pluggable backends

pub trait HttpClient {
    fn get(&self, url: &str) -> Result<String, String>;
    fn post(&self, url: &str, body: &str) -> Result<String, String>;
}

pub trait GitClient {
    fn clone(&self, url: &str, path: &str) -> Result<(), String>;
}

// Stub implementations (no deps)
pub struct StubHttp;
impl HttpClient for StubHttp {
    fn get(&self, _url: &str) -> Result<String, String> {
        Err("Load libhttp.so via libnix".to_string())
    }
    fn post(&self, _url: &str, _body: &str) -> Result<String, String> {
        Err("Load libhttp.so via libnix".to_string())
    }
}

pub struct StubGit;
impl GitClient for StubGit {
    fn clone(&self, _url: &str, _path: &str) -> Result<(), String> {
        Err("Load libgit.so via libnix".to_string())
    }
}
