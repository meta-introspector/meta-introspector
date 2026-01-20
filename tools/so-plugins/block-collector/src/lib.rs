use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use serde::{Deserialize, Serialize};

const CONTRACT: &str = "BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5zDrsbwtapump";

#[derive(Serialize, Deserialize)]
struct Block {
    slot: u64,
    hash: String,
    transactions: Vec<String>,
    timestamp: u64,
    client_id: String,
}

#[derive(Serialize, Deserialize)]
struct PaymentTx {
    from: String,
    to: String,
    amount: f64,
    signature: String,
}

#[no_mangle]
pub extern "C" fn register_client(peer_id_ptr: *const c_char) -> *const c_char {
    let peer_id = unsafe { CStr::from_ptr(peer_id_ptr).to_string_lossy() };
    
    let response = serde_json::json!({
        "status": "registered",
        "peer_id": peer_id.to_string(),
        "contract": CONTRACT
    });
    
    CString::new(response.to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn submit_block(block_json_ptr: *const c_char) -> *const c_char {
    let block_json = unsafe { CStr::from_ptr(block_json_ptr).to_string_lossy() };
    
    let block: Block = match serde_json::from_str(&block_json) {
        Ok(b) => b,
        Err(e) => {
            let err = serde_json::json!({"status": "error", "message": e.to_string()});
            return CString::new(err.to_string()).unwrap().into_raw();
        }
    };
    
    // Verify block
    if !verify_block(&block) {
        let err = serde_json::json!({"status": "error", "message": "invalid block"});
        return CString::new(err.to_string()).unwrap().into_raw();
    }
    
    // Store block
    store_block(&block);
    
    // Create Solana payment transaction
    let payment_tx = create_payment_tx(&block.client_id, 0.001);
    
    let response = serde_json::json!({
        "status": "accepted",
        "slot": block.slot,
        "payment": {
            "amount": 0.001,
            "tx_signature": payment_tx.signature,
            "from": payment_tx.from,
            "to": payment_tx.to
        }
    });
    
    CString::new(response.to_string()).unwrap().into_raw()
}

fn verify_block(block: &Block) -> bool {
    block.slot > 0 && !block.hash.is_empty()
}

fn store_block(block: &Block) {
    use std::fs::OpenOptions;
    use std::io::Write;
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("blocks.jsonl")
        .expect("Failed to open blocks.jsonl");
    
    writeln!(file, "{}", serde_json::to_string(block).unwrap()).expect("Failed to write block");
    eprintln!("📦 Stored block slot={}", block.slot);
}

fn create_payment_tx(client_pubkey: &str, amount: f64) -> PaymentTx {
    use solana_sdk::{signature::Keypair, signer::Signer, system_instruction, transaction::Transaction};
    
    let server_keypair = Keypair::new();
    let client_pubkey = solana_sdk::pubkey::Pubkey::from_str(client_pubkey)
        .unwrap_or_else(|_| solana_sdk::pubkey::Pubkey::new_unique());
    
    let lamports = (amount * 1_000_000_000.0) as u64;
    let instruction = system_instruction::transfer(&server_keypair.pubkey(), &client_pubkey, lamports);
    let recent_blockhash = solana_sdk::hash::Hash::default();
    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&server_keypair.pubkey()),
        &[&server_keypair],
        recent_blockhash,
    );
    
    eprintln!("💰 Created payment tx: {} SOL to {}", amount, client_pubkey);
    
    PaymentTx {
        from: server_keypair.pubkey().to_string(),
        to: client_pubkey.to_string(),
        amount,
        signature: tx.signatures[0].to_string(),
    }
}

fn rand_string() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{}", now.as_nanos())
}

#[no_mangle]
pub extern "C" fn get_contract() -> *const c_char {
    CString::new(CONTRACT).unwrap().into_raw()
}
