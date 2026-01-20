// Server-side social data handler
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct SocialData {
    data_type: String,
    url: String,
    content: String,
    timestamp: u64,
    submitter: String,
    zk_proof: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Submission {
    id: String,
    data: SocialData,
    content_hash: String,
    verifications: Vec<Verification>,
    bounty_paid: f64,
    status: SubmissionStatus,
}

#[derive(Serialize, Deserialize)]
struct Verification {
    verifier: String,
    is_valid: bool,
    timestamp: u64,
}

#[derive(Serialize, Deserialize)]
enum SubmissionStatus {
    Pending,
    Verified,
    Rejected,
}

static mut SUBMISSIONS: Option<HashMap<String, Submission>> = None;

fn get_submissions() -> &'static mut HashMap<String, Submission> {
    unsafe {
        if SUBMISSIONS.is_none() {
            SUBMISSIONS = Some(HashMap::new());
        }
        SUBMISSIONS.as_mut().unwrap()
    }
}

#[no_mangle]
pub extern "C" fn submit_social_data(data_json: *const c_char) -> *const c_char {
    let data_str = unsafe { CStr::from_ptr(data_json).to_string_lossy() };
    let data: SocialData = match serde_json::from_str(&data_str) {
        Ok(d) => d,
        Err(e) => {
            let err = serde_json::json!({"status": "error", "message": e.to_string()});
            return CString::new(err.to_string()).unwrap().into_raw();
        }
    };
    
    // Check if first submission
    let content_hash = hash_content(&data);
    let submissions = get_submissions();
    
    if submissions.contains_key(&content_hash) {
        let response = serde_json::json!({
            "status": "duplicate",
            "first_submission": false,
            "bounty_paid": 0.0
        });
        return CString::new(response.to_string()).unwrap().into_raw();
    }
    
    // Calculate bounty
    let bounty = calculate_bounty(&data);
    
    // Create submission
    let submission = Submission {
        id: format!("sub_{}", rand_string()),
        data: data.clone(),
        content_hash: content_hash.clone(),
        verifications: vec![],
        bounty_paid: bounty,
        status: SubmissionStatus::Pending,
    };
    
    submissions.insert(content_hash, submission);
    
    // Pay bounty immediately
    pay_bounty(&data.submitter, bounty);
    
    let response = serde_json::json!({
        "status": "accepted",
        "first_submission": true,
        "bounty_paid": bounty,
        "verification_needed": true
    });
    
    CString::new(response.to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn verify_submission(
    submission_id: *const c_char,
    verifier: *const c_char,
    is_valid: bool
) -> *const c_char {
    let sub_id = unsafe { CStr::from_ptr(submission_id).to_string_lossy() };
    let verifier_str = unsafe { CStr::from_ptr(verifier).to_string_lossy() };
    
    let submissions = get_submissions();
    
    // Find submission by ID
    let submission = submissions.values_mut()
        .find(|s| s.id == sub_id.as_ref())
        .unwrap();
    
    // Add verification
    submission.verifications.push(Verification {
        verifier: verifier_str.to_string(),
        is_valid,
        timestamp: current_timestamp(),
    });
    
    // Check consensus (3 verifications needed)
    if submission.verifications.len() >= 3 {
        let valid_count = submission.verifications.iter().filter(|v| v.is_valid).count();
        
        if valid_count >= 2 {
            // Verified! Pay bonus
            submission.status = SubmissionStatus::Verified;
            pay_bonus(&submission.data.submitter, 0.5);
            
            let response = serde_json::json!({
                "status": "verified",
                "bonus_paid": 0.5
            });
            return CString::new(response.to_string()).unwrap().into_raw();
        } else {
            // Rejected
            submission.status = SubmissionStatus::Rejected;
        }
    }
    
    let response = serde_json::json!({
        "status": "verification_recorded",
        "total_verifications": submission.verifications.len()
    });
    
    CString::new(response.to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn get_pending_verifications() -> *const c_char {
    let submissions = get_submissions();
    
    let pending: Vec<_> = submissions.values()
        .filter(|s| matches!(s.status, SubmissionStatus::Pending))
        .filter(|s| s.verifications.len() < 3)
        .collect();
    
    let response = serde_json::to_string(&pending).unwrap();
    CString::new(response).unwrap().into_raw()
}

fn hash_content(data: &SocialData) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data.url.as_bytes());
    hasher.update(data.content.as_bytes());
    format!("{:x}", hasher.finalize())[..16].to_string()
}

fn calculate_bounty(data: &SocialData) -> f64 {
    match data.data_type.as_str() {
        "TwitterPost" => 1.0,
        "WhaleTransaction" => 0.5,
        "PriceAlert" => 0.3,
        _ => 0.1,
    }
}

fn pay_bounty(recipient: &str, amount: f64) {
    eprintln!("💰 Paying {} SOL to {}", amount, recipient);
}

fn pay_bonus(recipient: &str, amount: f64) {
    eprintln!("🎁 Paying bonus {} SOL to {}", amount, recipient);
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn rand_string() -> String {
    format!("{}", current_timestamp())
}
