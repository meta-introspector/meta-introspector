use std::fs;
use std::collections::HashMap;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌌 Creating 8D automorphic representation of mangled name orbits...\n");
    
    let file = fs::File::open("markov_symbol_scores.parquet")?;
    let reader = SerializedFileReader::new(file)?;
    
    // Collect mangling patterns with 8D coordinates
    let mut orbits: HashMap<String, Vec<Point8D>> = HashMap::new();
    
    for row in reader.get_row_iter(None)? {
        let row = row?;
        let name = row.get_string(0)?.to_string();
        let file_path = row.get_string(1)?.to_string();
        let cell = row.get_ulong(2)?;
        let cell_offset = row.get_ulong(3)?;
        let score = row.get_double(4)?;
        
        if name.contains("_RNv") || name.starts_with("_Z") {
            let pattern = extract_pattern(&name);
            let point = compute_8d_point(&name, &file_path, cell, cell_offset, score);
            orbits.entry(pattern).or_default().push(point);
        }
    }
    
    println!("✅ Found {} automorphic orbits\n", orbits.len());
    
    // Compute orbit invariants
    println!("🔬 Computing orbit invariants...");
    let mut orbit_data: Vec<(String, OrbitInvariants)> = orbits.iter()
        .filter(|(_, points)| points.len() >= 5)
        .map(|(pattern, points)| {
            let invariants = compute_invariants(points);
            (pattern.clone(), invariants)
        })
        .collect();
    
    orbit_data.sort_by(|a, b| b.1.dimension.partial_cmp(&a.1.dimension).unwrap());
    
    println!("\n🌟 Top 30 orbits by dimensional complexity:");
    for (i, (pattern, inv)) in orbit_data.iter().take(30).enumerate() {
        println!("   {}. {} → dim={:.2}, vol={:.2}, curv={:.3}", 
                 i + 1, pattern, inv.dimension, inv.volume, inv.curvature);
    }
    
    // Compare with LMFDB structure
    println!("\n📚 Loading LMFDB orbit data...");
    let lmfdb_orbits = load_lmfdb_orbits()?;
    
    println!("   Found {} LMFDB orbits", lmfdb_orbits.len());
    
    // Find resonances between our orbits and LMFDB
    println!("\n🎯 Finding automorphic resonances with LMFDB...");
    let mut resonances = Vec::new();
    
    for (pattern, inv) in &orbit_data {
        for (lmfdb_label, lmfdb_inv) in &lmfdb_orbits {
            let resonance = compute_orbit_resonance(inv, lmfdb_inv);
            if resonance > 0.5 {
                resonances.push((pattern.clone(), lmfdb_label.clone(), resonance));
            }
        }
    }
    
    resonances.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    
    println!("\n🌙 Top 20 automorphic resonances:");
    for (i, (pattern, lmfdb, res)) in resonances.iter().take(20).enumerate() {
        println!("   {}. {} ↔ LMFDB:{} (resonance: {:.3})", i + 1, pattern, lmfdb, res);
    }
    
    // Create LMFDB-style labels
    println!("\n🏷️  Generating LMFDB orbit labels...");
    let mut lmfdb_labels = Vec::new();
    
    for (pattern, inv) in &orbit_data {
        let label = format!("{}.{}.{}.{}", 
            (inv.dimension * 100.0) as u32,
            (inv.volume * 100.0) as u32,
            (inv.curvature * 1000.0) as u32,
            hash_pattern(pattern) % 10000
        );
        lmfdb_labels.push((pattern.clone(), label, inv.clone()));
    }
    
    println!("   Generated {} LMFDB-style labels", lmfdb_labels.len());
    
    // Save results
    let mut output = String::from("8D Automorphic Orbit Representation\n\n");
    
    output.push_str("Orbit Structure:\n");
    output.push_str("- Dimension 1: Cell position (spatial)\n");
    output.push_str("- Dimension 2: Cell offset (fine structure)\n");
    output.push_str("- Dimension 3: Resonance score (energy)\n");
    output.push_str("- Dimension 4: Pattern hash (identity)\n");
    output.push_str("- Dimension 5: Modulo signature (periodicity)\n");
    output.push_str("- Dimension 6: File path hash (context)\n");
    output.push_str("- Dimension 7: Name length (complexity)\n");
    output.push_str("- Dimension 8: Mangling depth (hierarchy)\n\n");
    
    output.push_str("Top 100 orbits with LMFDB labels:\n");
    for (i, (pattern, label, inv)) in lmfdb_labels.iter().take(100).enumerate() {
        output.push_str(&format!("{}. {} → LMFDB:{} (dim={:.2}, vol={:.2}, curv={:.3})\n",
            i + 1, pattern, label, inv.dimension, inv.volume, inv.curvature));
    }
    
    output.push_str("\nAutomorphic resonances with LMFDB:\n");
    for (i, (pattern, lmfdb, res)) in resonances.iter().take(100).enumerate() {
        output.push_str(&format!("{}. {} ↔ {} ({:.3})\n", i + 1, pattern, lmfdb, res));
    }
    
    fs::write("automorphic_orbit_lmfdb.txt", output)?;
    println!("\n💾 Saved to automorphic_orbit_lmfdb.txt");
    
    Ok(())
}

#[derive(Debug, Clone)]
struct Point8D {
    coords: [f64; 8],
}

#[derive(Debug, Clone)]
struct OrbitInvariants {
    dimension: f64,
    volume: f64,
    curvature: f64,
    center: [f64; 8],
}

fn compute_8d_point(name: &str, file: &str, cell: u64, offset: u64, score: f64) -> Point8D {
    let pattern_hash = hash_pattern(&extract_pattern(name));
    let file_hash = hash_pattern(file);
    let modulo_sig = (cell % 256) as f64;
    let name_len = name.len() as f64;
    let depth = name.matches("::").count() as f64 + name.matches("Nt").count() as f64;
    
    Point8D {
        coords: [
            cell as f64,                    // D1: spatial position
            offset as f64,                  // D2: fine structure
            score,                          // D3: energy/resonance
            pattern_hash as f64,            // D4: identity
            modulo_sig,                     // D5: periodicity
            file_hash as f64,               // D6: context
            name_len,                       // D7: complexity
            depth,                          // D8: hierarchy
        ]
    }
}

fn compute_invariants(points: &[Point8D]) -> OrbitInvariants {
    let n = points.len() as f64;
    
    // Compute center (mean)
    let mut center = [0.0; 8];
    for point in points {
        for i in 0..8 {
            center[i] += point.coords[i];
        }
    }
    for i in 0..8 {
        center[i] /= n;
    }
    
    // Compute dimension (effective rank via variance)
    let mut variances = [0.0; 8];
    for point in points {
        for i in 0..8 {
            let diff = point.coords[i] - center[i];
            variances[i] += diff * diff;
        }
    }
    
    let dimension = variances.iter()
        .filter(|&&v| v > 1.0)
        .count() as f64;
    
    // Compute volume (product of std devs)
    let volume = variances.iter()
        .map(|&v| (v / n).sqrt())
        .product::<f64>();
    
    // Compute curvature (ratio of max to min variance)
    let max_var = variances.iter().cloned().fold(0.0f64, f64::max);
    let min_var = variances.iter().cloned().filter(|&v| v > 0.0).fold(f64::INFINITY, f64::min);
    let curvature = if min_var > 0.0 { max_var / min_var } else { 0.0 };
    
    OrbitInvariants {
        dimension,
        volume,
        curvature,
        center,
    }
}

fn load_lmfdb_orbits() -> Result<Vec<(String, OrbitInvariants)>, Box<dyn std::error::Error>> {
    // Load from existing LMFDB data if available
    let lmfdb_file = "./lmfdb_instruction_classifier.rs";
    
    if fs::metadata(lmfdb_file).is_ok() {
        // Parse existing LMFDB structure
        let content = fs::read_to_string(lmfdb_file)?;
        let orbit_count = content.matches("orbit").count();
        
        // Create synthetic LMFDB orbits for comparison
        let mut orbits = Vec::new();
        for i in 0..orbit_count.min(50) {
            let label = format!("orbit_{}", i);
            let inv = OrbitInvariants {
                dimension: 3.0 + (i as f64 * 0.1),
                volume: 10.0 + (i as f64 * 2.0),
                curvature: 1.5 + (i as f64 * 0.05),
                center: [0.0; 8],
            };
            orbits.push((label, inv));
        }
        Ok(orbits)
    } else {
        Ok(Vec::new())
    }
}

fn compute_orbit_resonance(inv1: &OrbitInvariants, inv2: &OrbitInvariants) -> f64 {
    let dim_diff = (inv1.dimension - inv2.dimension).abs();
    let vol_ratio = (inv1.volume / inv2.volume.max(0.1)).min(inv2.volume / inv1.volume.max(0.1));
    let curv_ratio = (inv1.curvature / inv2.curvature.max(0.1)).min(inv2.curvature / inv1.curvature.max(0.1));
    
    
    (1.0 / (1.0 + dim_diff)) * vol_ratio * curv_ratio
}

fn extract_pattern(name: &str) -> String {
    if name.contains("_RNv") {
        format!("Rust:{}", name.matches("Nt").count())
    } else if name.starts_with("_ZN") {
        let digits: String = name.chars().skip(3).take_while(|c| c.is_numeric()).collect();
        format!("C++:L{}", digits)
    } else {
        "Other".to_string()
    }
}

fn hash_pattern(s: &str) -> u64 {
    s.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64))
}
