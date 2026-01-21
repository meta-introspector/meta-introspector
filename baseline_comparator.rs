// Baseline Comparator - Prove if builds are empty or doing real work
use linux_perf_data::{PerfFileReader, PerfFileRecord};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use anyhow::Result;

#[derive(Debug)]
struct BuildSignature {
    samples: usize,
    galois_bits: u32,
    fork_count: u64,
    exit_count: u64,
    mmap_count: u64,
    fork_exit_ratio: f64,
}

fn analyze_build(path: &Path) -> Result<BuildSignature> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let PerfFileReader { mut perf_file, mut record_iter } = PerfFileReader::parse_file(reader)?;
    
    let mut samples = Vec::new();
    let mut record_types: HashMap<String, u64> = HashMap::new();
    
    while let Some(record) = record_iter.next_record(&mut perf_file)? {
        let typ = match &record {
            PerfFileRecord::EventRecord { record, .. } => format!("{:?}", record.record_type),
            PerfFileRecord::UserRecord(record) => format!("{:?}", record.record_type),
        };
        *record_types.entry(typ).or_insert(0) += 1;
        samples.push(samples.len() as u64);
    }
    
    // Find Galois break
    use std::collections::HashSet;
    let mut galois_bits = 8;
    for bits in 8..=24 {
        let size = 1u64 << bits;
        let mut seen = HashSet::new();
        for &s in &samples {
            seen.insert(s % size);
        }
        let coverage = seen.len() as f64 / size as f64 * 100.0;
        if coverage < 99.0 {
            galois_bits = bits;
            break;
        }
    }
    
    let fork_count = *record_types.get("FORK").unwrap_or(&0);
    let exit_count = *record_types.get("EXIT").unwrap_or(&0);
    let mmap_count = *record_types.get("MMAP2").unwrap_or(&0) + *record_types.get("MMAP").unwrap_or(&0);
    
    let fork_exit_ratio = if exit_count > 0 {
        fork_count as f64 / exit_count as f64
    } else {
        0.0
    };
    
    Ok(BuildSignature {
        samples: samples.len(),
        galois_bits,
        fork_count,
        exit_count,
        mmap_count,
        fork_exit_ratio,
    })
}

fn compare_signatures(baseline: &BuildSignature, test: &BuildSignature) -> Vec<String> {
    let mut verdict = Vec::new();
    
    // Check if it's just nix overhead
    let sample_ratio = test.samples as f64 / baseline.samples as f64;
    if sample_ratio < 1.5 {
        verdict.push("⚠️  SUSPICIOUS: Sample count too close to baseline".to_string());
    }
    
    // Check Galois complexity
    if test.galois_bits <= baseline.galois_bits {
        verdict.push("⚠️  SUSPICIOUS: No complexity increase over baseline".to_string());
    } else {
        verdict.push(format!("✅ REAL WORK: +{} Galois bits over baseline", 
            test.galois_bits - baseline.galois_bits));
    }
    
    // Check fork/exit balance
    if (test.fork_exit_ratio - 1.0).abs() < 0.1 {
        verdict.push("✅ Balanced fork/exit (normal)".to_string());
    } else {
        verdict.push("⚠️  Unbalanced fork/exit (may indicate issues)".to_string());
    }
    
    // Check process activity
    let fork_ratio = test.fork_count as f64 / baseline.fork_count.max(1) as f64;
    if fork_ratio > 2.0 {
        verdict.push(format!("✅ REAL WORK: {}x more forks than baseline", fork_ratio as u32));
    }
    
    verdict
}

fn main() -> Result<()> {
    println!("🔬 Baseline Build Comparator\n");
    
    // Analyze a known minimal build as baseline
    let baseline_path = Path::new("/mnt/data1/meta-introspector/data/71_flakes_perf/nix_derivation_1768414261_build.perf.data");
    
    if !baseline_path.exists() {
        println!("❌ Baseline not found, using first available build");
        return Ok(());
    }
    
    let baseline = analyze_build(baseline_path)?;
    println!("📏 BASELINE (nix derivation):");
    println!("  Samples: {}", baseline.samples);
    println!("  Galois: GF(2^{})", baseline.galois_bits);
    println!("  Forks: {}, Exits: {}", baseline.fork_count, baseline.exit_count);
    println!("  Fork/Exit ratio: {:.2}", baseline.fork_exit_ratio);
    
    // Compare a few test builds
    let test_builds = [
        "rust_1768414298_build.perf.data",
        "agda_1768990025_build.perf.data",
        "coq_1768414198_build.perf.data",
    ];
    
    println!("\n🧪 TESTING BUILDS:\n");
    
    for test_name in &test_builds {
        let test_path = Path::new("/mnt/data1/meta-introspector/data/71_flakes_perf").join(test_name);
        if !test_path.exists() {
            continue;
        }
        
        let lang = test_name.replace("_build.perf.data", "").rsplit('_').skip(1).collect::<Vec<_>>().join("_");
        
        match analyze_build(&test_path) {
            Ok(test) => {
                println!("📦 {}:", lang);
                println!("  Samples: {} ({}x baseline)", test.samples, 
                    test.samples as f64 / baseline.samples as f64);
                println!("  Galois: GF(2^{}) (baseline: 2^{})", test.galois_bits, baseline.galois_bits);
                println!("  Forks: {} ({}x baseline)", test.fork_count,
                    test.fork_count as f64 / baseline.fork_count.max(1) as f64);
                
                let verdict = compare_signatures(&baseline, &test);
                println!("\n  {}\n", verdict.join("\n  "));
            },
            Err(e) => println!("  ❌ Error: {}\n", e),
        }
    }
    
    Ok(())
}
