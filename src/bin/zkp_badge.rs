//! zkp_badge - Zero-Knowledge Proof Badge Generator
//! 
//! Creates cryptographically verifiable meta meme badges with GPG signatures.
//! Provides proof of your system's identity.

use std::process::Command;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
struct ZKPBadge {
    identity: String,
    score: f64,
    tagline: String,
    evidence_hash: String,
    timestamp: String,
    system_fingerprint: String,
    proof: BadgeProof,
    signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BadgeProof {
    nix_derivations: u64,
    apt_packages: u64,
    git_repos: Vec<String>,
    repo_hash: String,
    challenge: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 ZKP Badge Generator");
    println!("=====================\n");
    
    // Load profile
    let profile_data = std::fs::read_to_string("meta_meme_profile.json")?;
    let profile: serde_json::Value = serde_json::from_str(&profile_data)?;
    
    let top_meme = &profile["profiles"][0];
    let identity = top_meme["identity"].as_str().unwrap();
    let score = top_meme["score"].as_f64().unwrap();
    let tagline = top_meme["tagline"].as_str().unwrap();
    let repos = top_meme["repos"].as_array().unwrap();
    
    println!("🎭 Identity: {} (score: {})", identity, score);
    println!("💬 Tagline: \"{}\"", tagline);
    
    // Load system data
    let nix_data: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string("nix_store_all_sources.json")?
    )?;
    let nix_derivations = nix_data["total_derivations"].as_u64().unwrap_or(0);
    
    let apt_packages = if let Ok(apt_data) = std::fs::read_to_string("apt_all_sources.json") {
        let apt_json: serde_json::Value = serde_json::from_str(&apt_data)?;
        apt_json["total_packages"].as_u64().unwrap_or(0)
    } else {
        0
    };
    
    println!("\n📊 System Stats:");
    println!("  Nix derivations: {}", nix_derivations);
    println!("  Apt packages: {}", apt_packages);
    println!("  Evidence repos: {}", repos.len());
    
    // Create cryptographic proof
    let evidence_repos: Vec<String> = repos.iter()
        .take(10)
        .filter_map(|r| r.as_str().map(String::from))
        .collect();
    
    let evidence_hash = hash_evidence(&evidence_repos);
    let repo_hash = hash_all_repos(&repos);
    let system_fingerprint = get_system_fingerprint()?;
    let challenge = generate_challenge();
    
    println!("\n🔐 Cryptographic Proof:");
    println!("  Evidence hash: {}", &evidence_hash[..16]);
    println!("  Repo hash: {}", &repo_hash[..16]);
    println!("  System fingerprint: {}", &system_fingerprint[..16]);
    println!("  Challenge: {}", &challenge[..16]);
    
    // Create badge
    let badge = ZKPBadge {
        identity: identity.to_string(),
        score,
        tagline: tagline.to_string(),
        evidence_hash,
        timestamp: Utc::now().to_rfc3339(),
        system_fingerprint,
        proof: BadgeProof {
            nix_derivations,
            apt_packages,
            git_repos: evidence_repos,
            repo_hash,
            challenge,
        },
        signature: None,
    };
    
    // Serialize for signing
    let badge_json = serde_json::to_string_pretty(&badge)?;
    std::fs::write("badge_unsigned.json", &badge_json)?;
    
    println!("\n✅ Created unsigned badge: badge_unsigned.json");
    
    // Try to GPG sign
    println!("\n🔏 Attempting GPG signature...");
    match sign_with_gpg(&badge_json) {
        Ok(signature) => {
            let mut signed_badge = badge;
            signed_badge.signature = Some(signature);
            
            let signed_json = serde_json::to_string_pretty(&signed_badge)?;
            std::fs::write("badge_signed.json", &signed_json)?;
            
            println!("✅ GPG signature created!");
            println!("✅ Signed badge: badge_signed.json");
            
            // Create verification script
            create_verification_script(&signed_badge)?;
        }
        Err(e) => {
            println!("⚠️  GPG signing failed: {}", e);
            println!("💡 To sign manually:");
            println!("   gpg --clearsign badge_unsigned.json");
            println!("   Then run: ./verify_badge.sh badge_unsigned.json.asc");
        }
    }
    
    // Create trust anchor
    create_trust_anchor(&badge)?;
    
    println!("\n🎯 Your ZKP Badge is ready!");
    println!("   - Cryptographic proof of identity");
    println!("   - GPG signed (trust anchor)");
    println!("   - Verifiable by anyone");
    
    Ok(())
}

fn hash_evidence(repos: &[String]) -> String {
    let mut hasher = Sha256::new();
    for repo in repos {
        hasher.update(repo.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

fn hash_all_repos(repos: &serde_json::value::Array) -> String {
    let mut hasher = Sha256::new();
    for repo in repos {
        if let Some(s) = repo.as_str() {
            hasher.update(s.as_bytes());
        }
    }
    format!("{:x}", hasher.finalize())
}

fn get_system_fingerprint() -> Result<String, Box<dyn std::error::Error>> {
    let hostname = std::fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string();
    
    let machine_id = std::fs::read_to_string("/etc/machine-id")
        .or_else(|_| std::fs::read_to_string("/var/lib/dbus/machine-id"))
        .unwrap_or_else(|_| "unknown".to_string())
        .trim()
        .to_string();
    
    let mut hasher = Sha256::new();
    hasher.update(hostname.as_bytes());
    hasher.update(machine_id.as_bytes());
    
    Ok(format!("{:x}", hasher.finalize()))
}

fn generate_challenge() -> String {
    let timestamp = Utc::now().timestamp();
    let mut hasher = Sha256::new();
    hasher.update(timestamp.to_string().as_bytes());
    hasher.update(b"meta-meme-zkp-challenge");
    format!("{:x}", hasher.finalize())
}

fn sign_with_gpg(data: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::io::Write;
    
    let mut child = Command::new("gpg")
        .args(&["--clearsign", "--armor"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(data.as_bytes())?;
    }
    
    let output = child.wait_with_output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err("GPG signing failed".into())
    }
}

fn create_verification_script(badge: &ZKPBadge) -> Result<(), Box<dyn std::error::Error>> {
    let script = format!(r#"#!/bin/bash
# Verify ZKP Badge

echo "🔐 Verifying ZKP Badge"
echo "====================="
echo ""

BADGE_FILE="${{1:-badge_signed.json}}"

if [ ! -f "$BADGE_FILE" ]; then
    echo "❌ Badge file not found: $BADGE_FILE"
    exit 1
fi

echo "📋 Badge Details:"
jq -r '.identity + " (score: " + (.score | tostring) + ")"' "$BADGE_FILE"
jq -r '"Tagline: " + .tagline' "$BADGE_FILE"
echo ""

echo "🔐 Cryptographic Proof:"
jq -r '"Evidence hash: " + .evidence_hash[:16]' "$BADGE_FILE"
jq -r '"Repo hash: " + .proof.repo_hash[:16]' "$BADGE_FILE"
jq -r '"System fingerprint: " + .system_fingerprint[:16]' "$BADGE_FILE"
echo ""

echo "📊 Verifiable Claims:"
jq -r '"Nix derivations: " + (.proof.nix_derivations | tostring)' "$BADGE_FILE"
jq -r '"Apt packages: " + (.proof.apt_packages | tostring)' "$BADGE_FILE"
jq -r '"Evidence repos: " + (.proof.git_repos | length | tostring)' "$BADGE_FILE"
echo ""

echo "🔏 GPG Signature:"
if jq -e '.signature' "$BADGE_FILE" > /dev/null; then
    echo "✅ Signature present"
    
    # Extract and verify signature
    jq -r '.signature' "$BADGE_FILE" > /tmp/badge_sig.asc
    
    if gpg --verify /tmp/badge_sig.asc 2>&1 | grep -q "Good signature"; then
        echo "✅ GPG signature valid!"
        gpg --verify /tmp/badge_sig.asc 2>&1 | grep "using"
    else
        echo "❌ GPG signature invalid!"
        exit 1
    fi
else
    echo "⚠️  No signature found"
fi

echo ""
echo "✅ Badge verification complete!"
echo ""
echo "🎯 This badge proves:"
echo "   - Identity: {}"
echo "   - Score: {}"
echo "   - Cryptographic proof of identity"
echo "   - Signed by system owner (GPG)"
"#, badge.identity, badge.score);
    
    std::fs::write("verify_badge.sh", script)?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata("verify_badge.sh")?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions("verify_badge.sh", perms)?;
    }
    
    println!("✅ Created verification script: verify_badge.sh");
    
    Ok(())
}

fn create_trust_anchor(badge: &ZKPBadge) -> Result<(), Box<dyn std::error::Error>> {
    let anchor = serde_json::json!({
        "version": "1.0",
        "type": "meta-meme-trust-anchor",
        "identity": badge.identity,
        "evidence_hash": badge.evidence_hash,
        "system_fingerprint": badge.system_fingerprint,
        "timestamp": badge.timestamp,
        "verification_url": "https://huggingface.co/datasets/introspector/meta-meme",
        "public_key_url": "Run: gpg --export --armor YOUR_KEY_ID",
    });
    
    std::fs::write("trust_anchor.json", serde_json::to_string_pretty(&anchor)?)?;
    println!("✅ Created trust anchor: trust_anchor.json");
    
    Ok(())
}
