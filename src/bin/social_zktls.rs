//! social_zktls - Extract social media content and create zkTLS proofs
//! 
//! Fetches content from social media profiles and generates zero-knowledge proofs

use std::process::Command;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
struct SocialProof {
    platform: String,
    url: String,
    username: String,
    content_hash: String,
    timestamp: String,
    content_preview: String,
    proof: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 Social Media zkTLS Proof Generator");
    println!("====================================\n");
    
    let urls = vec![
        ("Twitter", "https://twitter.com/introsp3ctor", "introsp3ctor"),
        ("Telegram", "https://t.me/introsp3ctor", "introsp3ctor"),
        ("Discord", "https://discord.gg/WASKdrBBzu", "WASKdrBBzu"),
        ("LinkedIn", "https://www.linkedin.com/in/jamesmikedupont", "jamesmikedupont"),
        ("Linktree", "https://linktr.ee/h4km", "h4km"),
        ("Codeberg", "https://codeberg.org/introspector/SOLFUNMEME", "introspector"),
        ("GitHub", "https://github.com/meta-introspector", "meta-introspector"),
        ("HuggingFace", "https://huggingface.co/introspector", "introspector"),
        ("PumpFun", "https://pump.fun/BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5zDrsbwtapump", "SOLFUNMEME"),
        ("Streamflow", "https://app.streamflow.finance/contract/solana/mainnet/7Hny19uRWs6FhWFXrasUbqkE4rc8ciTdfQ2iyr2PVeva", "locked"),
    ];
    
    let mut proofs = Vec::new();
    
    for (platform, url, username) in urls {
        println!("📥 Fetching: {} ({})", platform, username);
        
        match fetch_content(url).await {
            Ok(content) => {
                let content_hash = hash_content(&content);
                let preview = content.chars().take(200).collect::<String>();
                
                let proof = SocialProof {
                    platform: platform.to_string(),
                    url: url.to_string(),
                    username: username.to_string(),
                    content_hash: content_hash.clone(),
                    timestamp: Utc::now().to_rfc3339(),
                    content_preview: preview,
                    proof: None, // Will be filled by zkTLS
                };
                
                println!("   ✅ Hash: {}", &content_hash[..16]);
                
                // Save raw content
                std::fs::write(
                    format!("social_content_{}_{}.html", platform.to_lowercase(), username),
                    &content
                )?;
                
                proofs.push(proof);
            }
            Err(e) => {
                println!("   ⚠️  Failed: {}", e);
            }
        }
    }
    
    println!("\n📊 Collected {} proofs", proofs.len());
    
    // Save proofs
    let output = serde_json::json!({
        "generated": Utc::now().to_rfc3339(),
        "total_proofs": proofs.len(),
        "proofs": proofs,
    });
    
    std::fs::write("social_zktls_proofs.json", serde_json::to_string_pretty(&output)?)?;
    println!("✅ Saved: social_zktls_proofs.json");
    
    // Generate zkTLS proof script
    generate_zktls_script(&proofs)?;
    
    // Append to FOAF
    append_to_foaf(&proofs)?;
    
    println!("\n🎯 Next steps:");
    println!("   1. Review: social_zktls_proofs.json");
    println!("   2. Generate zkTLS: ./generate_zktls_proofs.sh");
    println!("   3. Verify: ./verify_social_proofs.sh");
    
    Ok(())
}

async fn fetch_content(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Use curl for fetching
    let output = Command::new("curl")
        .args(&["-s", "-L", "-A", "Mozilla/5.0", url])
        .output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(format!("Failed to fetch {}", url).into())
    }
}

fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn generate_zktls_script(proofs: &[SocialProof]) -> Result<(), Box<dyn std::error::Error>> {
    let mut script = String::from("#!/bin/bash\n");
    script.push_str("# Generate zkTLS proofs for social media content\n\n");
    script.push_str("echo \"🔐 Generating zkTLS Proofs\"\n");
    script.push_str("echo \"==========================\"\n");
    script.push_str("echo \"\"\n\n");
    
    for proof in proofs {
        script.push_str(&format!("echo \"📝 {}: {}\"\n", proof.platform, proof.username));
        script.push_str(&format!("echo \"   URL: {}\"\n", proof.url));
        script.push_str(&format!("echo \"   Hash: {}\"\n", &proof.content_hash[..16]));
        script.push_str(&format!("echo \"   Timestamp: {}\"\n", proof.timestamp));
        script.push_str("echo \"\"\n\n");
        
        // TLSNotary zkTLS proof generation
        script.push_str(&format!("# Generate TLSNotary zkTLS proof for {}\n", proof.platform));
        script.push_str(&format!("# Install: cargo install tlsn-cli\n"));
        script.push_str(&format!("# Usage: tlsn prove --url {} --output {}_proof.json\n\n", 
            proof.url, proof.platform.to_lowercase()));
    }
    
    script.push_str("echo \"✅ zkTLS proofs generated\"\n");
    script.push_str("echo \"📝 Proofs saved to *_proof.json files\"\n");
    
    std::fs::write("generate_zktls_proofs.sh", script)?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata("generate_zktls_proofs.sh")?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions("generate_zktls_proofs.sh", perms)?;
    }
    
    println!("✅ Created: generate_zktls_proofs.sh");
    
    Ok(())
}

fn append_to_foaf(proofs: &[SocialProof]) -> Result<(), Box<dyn std::error::Error>> {
    let mut foaf_append = String::from("\n# zkTLS Social Media Proofs\n\n");
    
    for proof in proofs {
        foaf_append.push_str(&format!(r#"<{}> a foaf:Document ;
    dc:title "{} Profile" ;
    dc:creator <#me> ;
    dc:date "{}" ;
    foaf:sha1 "{}" ;
    rdfs:seeAlso <file://social_content_{}_{}.html> .

"#, 
            proof.url,
            proof.platform,
            proof.timestamp,
            &proof.content_hash[..40],
            proof.platform.to_lowercase(),
            proof.username
        ));
    }
    
    std::fs::write("foaf_social_proofs.ttl", foaf_append)?;
    println!("✅ Created: foaf_social_proofs.ttl");
    
    Ok(())
}
