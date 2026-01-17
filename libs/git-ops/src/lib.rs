use std::process::Command;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GitCloneRequest {
    pub url: String,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GitStatusResponse {
    pub branch: String,
    pub modified: Vec<String>,
    pub untracked: Vec<String>,
}

#[no_mangle]
pub extern "C" fn git_clone(url: *const std::os::raw::c_char, path: *const std::os::raw::c_char) -> i32 {
    let url = unsafe { std::ffi::CStr::from_ptr(url).to_str().unwrap() };
    let path = unsafe { std::ffi::CStr::from_ptr(path).to_str().unwrap() };
    
    match git_clone_rust(url, Some(path.to_string())) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

pub fn git_clone_rust(url: &str, path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new("git");
    cmd.arg("clone").arg(url);
    if let Some(p) = path {
        cmd.arg(p);
    }
    cmd.status()?;
    Ok(())
}

pub fn git_status_rust(path: &str) -> Result<GitStatusResponse, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("-C").arg(path)
        .arg("status")
        .arg("--porcelain")
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut modified = Vec::new();
    let mut untracked = Vec::new();
    
    for line in stdout.lines() {
        if line.starts_with(" M") {
            modified.push(line[3..].to_string());
        } else if line.starts_with("??") {
            untracked.push(line[3..].to_string());
        }
    }
    
    Ok(GitStatusResponse {
        branch: "main".to_string(), // Simplified
        modified,
        untracked,
    })
}
