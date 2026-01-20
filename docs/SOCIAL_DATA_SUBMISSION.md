# Social Data Submission with Community Verification

## Flow

```
┌─────────────────────────────────────────────────────────────┐
│ 1. USER SUBMITS DATA                                        │
│                                                             │
│  User A finds viral tweet                                   │
│  → Clicks "Submit" in solfunmeme-dioxus                    │
│  → Generates zkTLS proof (optional)                        │
│  → Submits to server                                       │
│                                                             │
│  Server checks:                                             │
│  ✅ First submission? → Pay 1.0 SOL immediately            │
│  ❌ Duplicate? → No payment                                 │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. COMMUNITY VERIFIES                                       │
│                                                             │
│  Other users see pending submissions                        │
│  → Click link to verify                                     │
│  → Vote: ✅ Valid or ❌ Invalid                             │
│                                                             │
│  Consensus (3 verifications needed):                        │
│  - 2+ valid → Verified ✅                                   │
│  - 2+ invalid → Rejected ❌                                 │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. BONUS PAYMENT                                            │
│                                                             │
│  If verified:                                               │
│  → Original submitter gets +0.5 SOL bonus                  │
│  → Data added to archive                                    │
│  → DAO proposal created                                     │
│                                                             │
│  Total earnings: 1.0 + 0.5 = 1.5 SOL                       │
└─────────────────────────────────────────────────────────────┘
```

## UI Components

### Submit Form
```rust
fn SubmitSocialData(cx: Scope) -> Element {
    let url = use_state(cx, String::new);
    let data_type = use_state(cx, || "TwitterPost");
    
    cx.render(rsx! {
        div { class: "submit-form",
            h3 { "Submit Social Data" }
            
            select { 
                onchange: move |e| data_type.set(e.value.clone()),
                option { value: "TwitterPost", "Twitter Post" }
                option { value: "WhaleTransaction", "Whale Transaction" }
                option { value: "PriceAlert", "Price Alert" }
            }
            
            input {
                placeholder: "URL",
                value: "{url}",
                oninput: move |e| url.set(e.value.clone())
            }
            
            button {
                onclick: move |_| {
                    let url = url.get().clone();
                    let dt = data_type.get().clone();
                    spawn_local(async move {
                        submit_social_data(SocialData {
                            data_type: dt,
                            url,
                            content: "".to_string(),
                            timestamp: js_sys::Date::now() as u64,
                            submitter: wallet.public_key.to_string(),
                            zk_proof: None,
                        }).await;
                    });
                },
                "Submit & Earn"
            }
        }
    })
}
```

### Verification Queue
```rust
fn VerificationQueue(cx: Scope) -> Element {
    let pending = use_future(cx, (), |_| async {
        get_pending_verifications().await
    });
    
    cx.render(rsx! {
        div { class: "verification-queue",
            h3 { "Verify Submissions" }
            
            match pending.value() {
                Some(Ok(items)) => rsx! {
                    for item in items {
                        div { class: "verification-item",
                            p { "Type: {item.data_type}" }
                            a { href: "{item.url}", target: "_blank", "View Link" }
                            
                            button {
                                onclick: move |_| {
                                    verify_submission(&item.id, true).await;
                                },
                                "✅ Valid"
                            }
                            
                            button {
                                onclick: move |_| {
                                    verify_submission(&item.id, false).await;
                                },
                                "❌ Invalid"
                            }
                        }
                    }
                },
                _ => rsx! { div { "Loading..." } }
            }
        }
    })
}
```

## zkTLS Integration

```rust
// Future: Integrate with TLSNotary or similar
pub async fn generate_zk_proof(url: &str) -> Result<String, String> {
    // 1. Fetch URL via TLS
    // 2. Generate zero-knowledge proof of response
    // 3. Return proof that can be verified without revealing full content
    
    // Proof includes:
    // - URL was accessed
    // - Response contained specific keywords
    // - Timestamp of access
    // - But NOT the full response content
    
    Ok("zk_proof_hash".to_string())
}
```

## Economics

### Submission Bounties
- Twitter post: 1.0 SOL
- Whale transaction: 0.5 SOL  
- Price alert: 0.3 SOL
- Other: 0.1 SOL

### Verification Bonus
- +0.5 SOL if community verifies (2/3 consensus)

### Example Earnings
**Active submitter** (10 verified posts/day):
- Immediate: 10 × 1.0 = 10 SOL
- Bonus: 10 × 0.5 = 5 SOL
- Total: 15 SOL/day

## Anti-Gaming

1. **Content hash**: Prevents duplicate submissions
2. **Community verification**: 3 independent verifiers needed
3. **zkTLS proofs**: Cryptographic proof of data source
4. **Rate limiting**: Max 100 submissions/day per wallet
5. **Reputation**: Track verifier accuracy

## Build

```bash
# Build plugin
cd tools/so-plugins/social-data
cargo build --release

# Copy to zos-server
cp target/release/libsocial_data_plugin.so ~/zos-server/plugins/
```

## Next Steps

1. [ ] Build social-data plugin
2. [ ] Add social_data.rs to solfunmeme-dioxus
3. [ ] Create UI components
4. [ ] Test submission flow
5. [ ] Add zkTLS integration
6. [ ] Deploy

---

**Result**: Users earn SOL for submitting valuable social data, community verifies, creates curated dataset.
