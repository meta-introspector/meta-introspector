// perf-complexity CLI: Fast complexity analysis of perf data
use perf_complexity::PerfComplexity;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(name = "perf-complexity")]
#[command(about = "Auto-label instruction data via GNU Mes bootstrap layers")]
struct Args {
    /// Perf data file to analyze
    #[arg(short, long)]
    perf_data: PathBuf,
    
    /// GNU Mes nix store path (for learning labels)
    #[arg(short, long)]
    mes_store: Option<PathBuf>,
    
    /// Output JSON file
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("🎯 Perf Complexity Analyzer");
    println!("============================");
    println!();
    
    let mut analyzer = PerfComplexity::new();
    
    // Load perf data
    analyzer.load_perf_data(&args.perf_data)?;
    
    // Learn labels from GNU Mes bootstrap if available
    if let Some(mes_store) = args.mes_store {
        analyzer.learn_from_mes_layers(&mes_store)?;
    } else {
        println!("⚠️  No mes-store provided, using default labeling");
    }
    
    println!();
    
    // Analyze and label
    let patterns = analyzer.analyze();
    
    println!();
    println!("📊 Top 10 Labeled Patterns:");
    println!("===========================");
    
    for (i, pattern) in patterns.iter().take(10).enumerate() {
        println!("{:2}. IP: 0x{:016x} (freq: {}, layer: {})", 
            i + 1, pattern.ip, pattern.frequency, pattern.layer);
        println!("    Label: {:?}", pattern.label);
    }
    
    // Output JSON if requested
    if let Some(output_path) = args.output {
        let json = serde_json::to_string_pretty(&patterns)?;
        std::fs::write(&output_path, json)?;
        println!();
        println!("✅ Saved to: {}", output_path.display());
    }
    
    println!();
    println!("🧙 Complexity analysis complete!");
    
    Ok(())
}
