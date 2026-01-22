# Tiered Access Model

## Free Tier

**Orbits 0-7: Free for everyone**

```
/nix/store/orbit_0/*  ⚪ FREE - Trivial functions
/nix/store/orbit_1/*  🔢 FREE - Simple arithmetic
/nix/store/orbit_7/*  📝 FREE - Parsers, basic compilers
```

No authentication, no limits, public access.

## Paid Tiers

### Orbit 42: Pay-per-use
```
/nix/store/orbit_42/* 🔐 $0.01 per execution
```
- Crypto functions
- ML models
- Heavy computation

### Orbit 71: Subscription
```
/nix/store/orbit_71/* 🌙 $10/month unlimited
```
- Full compilers
- Advanced optimizers
- Moonshine functions

## Access Control

```rust
fn check_access(user: &User, orbit: u32) -> Result<(), String> {
    match orbit {
        0..=7 => Ok(()),  // Free tier
        
        8..=41 => {
            // Moderate tier - rate limited
            if user.requests_today < 1000 {
                Ok(())
            } else {
                Err("Rate limit exceeded. Upgrade to paid tier.".to_string())
            }
        }
        
        42 => {
            // Pay-per-use
            if user.balance >= 0.01 {
                user.balance -= 0.01;
                Ok(())
            } else {
                Err("Insufficient balance".to_string())
            }
        }
        
        43..=70 => {
            // Premium tier
            if user.subscription.is_active() {
                Ok(())
            } else {
                Err("Subscription required".to_string())
            }
        }
        
        71 => {
            // Moonshine tier
            if user.subscription == Tier::Moonshine {
                Ok(())
            } else {
                Err("Moonshine subscription required".to_string())
            }
        }
        
        _ => Err("Invalid orbit".to_string())
    }
}
```

## Pricing Table

| Orbit | Complexity | Access | Cost |
|-------|-----------|--------|------|
| 0 | Trivial | FREE | $0 |
| 1 | Simple | FREE | $0 |
| 7 | Moderate | FREE | $0 |
| 8-41 | Moderate+ | Rate limited | 1000/day free |
| 42 | Complex | Pay-per-use | $0.01/exec |
| 43-70 | Premium | Subscription | $10/month |
| 71 | Moonshine | Subscription | $100/month |

## Benefits

### Free Tier (0-7)
- Learn and experiment
- Build basic apps
- No barriers to entry
- Public good

### Rate Limited (8-41)
- Generous free quota
- Upgrade if needed
- Fair usage

### Paid Tiers (42+)
- Support infrastructure
- Priority execution
- Advanced features

## Implementation

```rust
#[derive(Debug)]
enum Tier {
    Free,           // Orbits 0-7
    RateLimited,    // Orbits 8-41
    PayPerUse,      // Orbit 42
    Premium,        // Orbits 43-70
    Moonshine,      // Orbit 71
}

impl Tier {
    fn from_orbit(orbit: u32) -> Self {
        match orbit {
            0..=7 => Tier::Free,
            8..=41 => Tier::RateLimited,
            42 => Tier::PayPerUse,
            43..=70 => Tier::Premium,
            71 => Tier::Moonshine,
            _ => Tier::Free,
        }
    }
    
    fn cost(&self) -> f64 {
        match self {
            Tier::Free => 0.0,
            Tier::RateLimited => 0.0,
            Tier::PayPerUse => 0.01,
            Tier::Premium => 10.0,
            Tier::Moonshine => 100.0,
        }
    }
}
```

## Server Integration

```rust
async fn compile(Json(req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    let target = req["target"].as_str().unwrap();
    let user = authenticate(&req);
    
    // Classify complexity
    let orbit = classify_target_orbit(target);
    let tier = Tier::from_orbit(orbit);
    
    // Check access
    if let Err(e) = check_access(&user, orbit) {
        return Json(serde_json::json!({
            "success": false,
            "error": e,
            "orbit": orbit,
            "tier": format!("{:?}", tier),
            "upgrade_url": "/pricing"
        }));
    }
    
    // Proceed with compilation
    // ...
}
```

## Why This Works

### Free Tier is Generous
- 80% of use cases covered
- Simple functions are cheap to run
- Public good / education

### Paid Tiers are Fair
- Complex functions cost more to run
- Users pay for what they use
- Sustainable infrastructure

### Progressive Pricing
- Start free
- Pay as you grow
- Predictable costs

## Examples

### Free: Learning Rust
```bash
# All free - orbit 1-7
compile("hello-world")
compile("simple-parser")
compile("basic-web-server")
```

### Rate Limited: Small Projects
```bash
# 1000/day free - orbit 8-41
compile("medium-app")  # Uses moderate complexity functions
```

### Paid: Production Apps
```bash
# $0.01 per compile - orbit 42
compile("crypto-service")  # Uses SHA256, AES
```

### Moonshine: Full Compiler
```bash
# $100/month unlimited - orbit 71
compile("rustc-fork")  # Full compiler
```

## The Vision

Make simple and moderate functions (orbits 0-7) **completely free**.

This enables:
- Education
- Experimentation
- Small projects
- Public good

While complex functions (42+) support infrastructure through fair pricing.

**80% free, 20% paid = sustainable + accessible**
