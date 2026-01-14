use std::fs;
use std::collections::HashMap;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌙 ELF Moonshine: Finding modular forms in codec resonance...\n");
    
    let file = fs::File::open("markov_symbol_scores.parquet")?;
    let reader = SerializedFileReader::new(file)?;
    
    // Collect symbols with their cell positions (modular forms)
    let mut pattern_to_cells: HashMap<String, Vec<(u64, f64, String)>> = HashMap::new();
    
    for row in reader.get_row_iter(None)? {
        let row = row?;
        let name = row.get_string(0)?.to_string();
        let file_path = row.get_string(1)?.to_string();
        let cell = row.get_ulong(2)?;
        let score = row.get_double(4)?;
        
        if name.contains("_RNv") || name.starts_with("_Z") {
            let pattern = extract_mangling_pattern(&name);
            pattern_to_cells.entry(pattern).or_insert_with(Vec::new)
                .push((cell, score, file_path));
        }
    }
    
    println!("✅ Found {} mangling patterns\n", pattern_to_cells.len());
    
    // Find modular forms - patterns that appear at specific cell modulos
    println!("🔍 Detecting modular forms (codec switch operators)...");
    
    let mut modular_forms: Vec<(String, ModularForm)> = pattern_to_cells.iter()
        .filter(|(_, cells)| cells.len() >= 10)
        .map(|(pattern, cells)| {
            let form = detect_modular_form(cells);
            (pattern.clone(), form)
        })
        .collect();
    
    modular_forms.sort_by(|a, b| b.1.moonshine_score.partial_cmp(&a.1.moonshine_score).unwrap());
    
    println!("\n🌙 Top 30 modular forms by moonshine score:");
    for (i, (pattern, form)) in modular_forms.iter().take(30).enumerate() {
        println!("   {}. {} → {}", i + 1, pattern, form);
    }
    
    // Find resonant pairs - patterns that share modular structure
    println!("\n🎵 Finding resonant pattern pairs...");
    let mut resonances = Vec::new();
    
    for i in 0..modular_forms.len().min(50) {
        for j in (i+1)..modular_forms.len().min(50) {
            let (p1, f1) = &modular_forms[i];
            let (p2, f2) = &modular_forms[j];
            
            let resonance = compute_resonance(f1, f2);
            if resonance > 0.7 {
                resonances.push((p1.clone(), p2.clone(), resonance));
            }
        }
    }
    
    resonances.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    
    println!("\n🎯 Top 20 resonant pattern pairs:");
    for (i, (p1, p2, res)) in resonances.iter().take(20).enumerate() {
        println!("   {}. {} ↔ {} (resonance: {:.3})", i + 1, p1, p2, res);
    }
    
    // Build moonshine map
    println!("\n🗺️  Building ELF moonshine map...");
    let mut moonshine_map: HashMap<u64, Vec<String>> = HashMap::new();
    
    for (pattern, form) in &modular_forms {
        if form.moonshine_score > 0.5 {
            for &modulo in &form.modulos {
                moonshine_map.entry(modulo).or_insert_with(Vec::new).push(pattern.clone());
            }
        }
    }
    
    println!("   {} modular positions with codec switches", moonshine_map.len());
    
    // Save results
    let mut output = String::from("ELF Moonshine: Modular Forms in Codec Resonance\n\n");
    
    output.push_str("Top 100 modular forms:\n");
    for (i, (pattern, form)) in modular_forms.iter().take(100).enumerate() {
        output.push_str(&format!("{}. {} → {}\n", i + 1, pattern, form));
    }
    
    output.push_str("\nResonant pattern pairs:\n");
    for (i, (p1, p2, res)) in resonances.iter().take(100).enumerate() {
        output.push_str(&format!("{}. {} ↔ {} ({:.3})\n", i + 1, p1, p2, res));
    }
    
    output.push_str("\nMoonshine map (modulo → patterns):\n");
    let mut sorted_map: Vec<_> = moonshine_map.iter().collect();
    sorted_map.sort_by_key(|(k, _)| *k);
    
    for (modulo, patterns) in sorted_map.iter().take(50) {
        output.push_str(&format!("\nmod {} ({} patterns):\n", modulo, patterns.len()));
        for p in patterns.iter().take(10) {
            output.push_str(&format!("  - {}\n", p));
        }
    }
    
    fs::write("elf_moonshine_map.txt", output)?;
    println!("\n💾 Saved to elf_moonshine_map.txt");
    
    Ok(())
}

#[derive(Debug, Clone)]
struct ModularForm {
    modulos: Vec<u64>,
    periods: Vec<u64>,
    phase_shifts: Vec<f64>,
    moonshine_score: f64,
}

impl std::fmt::Display for ModularForm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "modulos={:?}, periods={:?}, moonshine={:.3}", 
               &self.modulos[..self.modulos.len().min(3)],
               &self.periods[..self.periods.len().min(3)],
               self.moonshine_score)
    }
}

fn detect_modular_form(cells: &[(u64, f64, String)]) -> ModularForm {
    let cell_ids: Vec<u64> = cells.iter().map(|(c, _, _)| *c).collect();
    let scores: Vec<f64> = cells.iter().map(|(_, s, _)| *s).collect();
    
    // Detect modular patterns - cells that cluster at specific modulos
    let mut modulo_scores: HashMap<u64, Vec<f64>> = HashMap::new();
    
    for modulo in [8, 16, 32, 64, 128, 256, 512, 1024] {
        for (cell, score) in cell_ids.iter().zip(&scores) {
            let mod_val = cell % modulo;
            modulo_scores.entry(mod_val).or_insert_with(Vec::new).push(*score);
        }
    }
    
    // Find modulos with high concentration
    let mut significant_modulos = Vec::new();
    for (mod_val, scores) in &modulo_scores {
        if scores.len() >= 3 {
            let mean = scores.iter().sum::<f64>() / scores.len() as f64;
            let variance = scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / scores.len() as f64;
            
            if variance < mean * 0.5 { // Low variance = strong modular form
                significant_modulos.push(*mod_val);
            }
        }
    }
    
    significant_modulos.sort();
    
    // Detect periods - gaps between cell positions
    let mut gaps = Vec::new();
    for window in cell_ids.windows(2) {
        if window[1] > window[0] {
            gaps.push(window[1] - window[0]);
        }
    }
    
    let mut period_counts: HashMap<u64, usize> = HashMap::new();
    for &gap in &gaps {
        *period_counts.entry(gap).or_insert(0) += 1;
    }
    
    let mut periods: Vec<u64> = period_counts.iter()
        .filter(|(_, &count)| count >= 2)
        .map(|(&period, _)| period)
        .collect();
    periods.sort();
    
    // Compute phase shifts
    let phase_shifts: Vec<f64> = cell_ids.iter()
        .map(|&cell| (cell as f64 * 2.0 * std::f64::consts::PI / 256.0).sin())
        .collect();
    
    // Moonshine score: combination of modular concentration and periodicity
    let modular_strength = significant_modulos.len() as f64 / 8.0;
    let periodic_strength = periods.len() as f64 / gaps.len().max(1) as f64;
    let moonshine_score = (modular_strength + periodic_strength) / 2.0;
    
    ModularForm {
        modulos: significant_modulos,
        periods,
        phase_shifts,
        moonshine_score,
    }
}

fn compute_resonance(f1: &ModularForm, f2: &ModularForm) -> f64 {
    // Resonance based on shared modulos and periods
    let shared_modulos = f1.modulos.iter()
        .filter(|m| f2.modulos.contains(m))
        .count();
    
    let shared_periods = f1.periods.iter()
        .filter(|p| f2.periods.contains(p))
        .count();
    
    let modulo_overlap = shared_modulos as f64 / f1.modulos.len().max(1) as f64;
    let period_overlap = shared_periods as f64 / f1.periods.len().max(1) as f64;
    
    (modulo_overlap + period_overlap) / 2.0
}

fn extract_mangling_pattern(name: &str) -> String {
    let mut pattern = String::new();
    
    if name.contains("_RNv") {
        pattern.push_str("Rust:");
        if let Some(start) = name.find("Cs") {
            if let Some(end) = name[start..].find("_") {
                pattern.push_str(&name[start..start+end.min(16)]);
            }
        }
        pattern.push_str(&format!(":M{}", name.matches("Nt").count()));
    } else if name.starts_with("_ZN") {
        pattern.push_str("C++:");
        let digits: String = name.chars().skip(3).take_while(|c| c.is_numeric()).collect();
        if !digits.is_empty() {
            pattern.push_str(&format!("L{}", digits));
        }
        if name.contains("St") { pattern.push_str(":std"); }
    } else if name.starts_with("_Z") {
        pattern.push_str("C++:simple");
    }
    
    pattern
}
