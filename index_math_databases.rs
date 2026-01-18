// Index downloaded math databases into local store
use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct MathDatabase {
    oeis: HashMap<String, OEISSequence>,
    lmfdb: HashMap<String, LMFDBObject>,
    wikidata: HashMap<String, WikidataEntity>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OEISSequence {
    id: String,
    name: String,
    sequence: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LMFDBObject {
    label: String,
    object_type: String,
    data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct WikidataEntity {
    id: String,
    label: String,
    properties: HashMap<String, String>,
}

fn index_oeis() -> HashMap<String, OEISSequence> {
    println!("📊 Indexing OEIS...");
    let mut sequences = HashMap::new();
    
    // Read sequences
    if let Ok(content) = fs::read_to_string("data/math-databases/oeis/sequences.txt") {
        for line in content.lines().take(10000) {  // First 10K sequences
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let id = parts[0].to_string();
                let seq: Vec<i64> = parts[1..].iter()
                    .filter_map(|s| s.trim_matches(',').parse().ok())
                    .collect();
                
                sequences.insert(id.clone(), OEISSequence {
                    id: id.clone(),
                    name: String::new(),
                    sequence: seq,
                });
            }
        }
    }
    
    // Add names
    if let Ok(content) = fs::read_to_string("data/math-databases/oeis/names.txt") {
        for line in content.lines().take(10000) {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let id = parts[0].to_string();
                if let Some(seq) = sequences.get_mut(&id) {
                    seq.name = parts[1].to_string();
                }
            }
        }
    }
    
    println!("  ✓ Indexed {} OEIS sequences", sequences.len());
    sequences
}

fn index_lmfdb() -> HashMap<String, LMFDBObject> {
    println!("📊 Indexing LMFDB...");
    let mut objects = HashMap::new();
    
    // Elliptic curves
    if let Ok(content) = fs::read_to_string("data/math-databases/lmfdb/elliptic_curves.json") {
        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(curves) = data.get("data").and_then(|d| d.as_array()) {
                for curve in curves {
                    if let Some(label) = curve.get("label").and_then(|l| l.as_str()) {
                        objects.insert(label.to_string(), LMFDBObject {
                            label: label.to_string(),
                            object_type: "elliptic_curve".to_string(),
                            data: curve.clone(),
                        });
                    }
                }
            }
        }
    }
    
    // Modular forms
    if let Ok(content) = fs::read_to_string("data/math-databases/lmfdb/modular_forms.json") {
        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(forms) = data.get("data").and_then(|d| d.as_array()) {
                for form in forms {
                    if let Some(label) = form.get("label").and_then(|l| l.as_str()) {
                        objects.insert(label.to_string(), LMFDBObject {
                            label: label.to_string(),
                            object_type: "modular_form".to_string(),
                            data: form.clone(),
                        });
                    }
                }
            }
        }
    }
    
    println!("  ✓ Indexed {} LMFDB objects", objects.len());
    objects
}

fn index_wikidata() -> HashMap<String, WikidataEntity> {
    println!("📊 Indexing Wikidata...");
    let entities = HashMap::new();
    
    // TODO: Parse RDF triples when downloaded
    println!("  ⚠ Wikidata indexing not yet implemented (large dataset)");
    
    entities
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗄️  MATH DATABASE INDEXER\n");
    
    // Index all databases
    let oeis = index_oeis();
    let lmfdb = index_lmfdb();
    let wikidata = index_wikidata();
    
    let db = MathDatabase {
        oeis,
        lmfdb,
        wikidata,
    };
    
    // Save indexed database
    println!("\n💾 Saving indexed database...");
    let json = serde_json::to_string(&db)?;
    fs::write("data/math_database_index.json", json)?;
    
    println!("✅ Saved data/math_database_index.json");
    println!("\n📊 Index Summary:");
    println!("  OEIS sequences: {}", db.oeis.len());
    println!("  LMFDB objects: {}", db.lmfdb.len());
    println!("  Wikidata entities: {}", db.wikidata.len());
    
    Ok(())
}
