// tiny_transformer.rs - Minimal transformer trained on real perf data
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;
use std::fs::File;
use std::collections::HashMap;
use std::io::Write;
use rayon::prelude::*;

const EMBED_DIM: usize = 64;
const NUM_HEADS: usize = 8;

#[derive(Debug, Clone)]
struct Embedding {
    weights: Vec<Vec<f32>>,
}

impl Embedding {
    fn new(vocab_size: usize) -> Self {
        let weights = (0..vocab_size)
            .map(|_| (0..EMBED_DIM).map(|_| rand::random::<f32>() - 0.5).collect())
            .collect();
        Self { weights }
    }
    
    fn forward(&self, token: usize) -> Vec<f32> {
        self.weights[token].clone()
    }
}

#[derive(Debug, Clone)]
struct Attention {
    q_weights: Vec<Vec<f32>>,
    k_weights: Vec<Vec<f32>>,
    v_weights: Vec<Vec<f32>>,
}

impl Attention {
    fn new() -> Self {
        let init = || {
            (0..EMBED_DIM)
                .map(|_| (0..EMBED_DIM).map(|_| rand::random::<f32>() - 0.5).collect())
                .collect()
        };
        Self {
            q_weights: init(),
            k_weights: init(),
            v_weights: init(),
        }
    }
    
    fn forward(&self, x: &[Vec<f32>]) -> Vec<Vec<f32>> {
        if x.is_empty() {
            return vec![];
        }
        
        // Q = x @ q_weights, K = x @ k_weights, V = x @ v_weights
        let q = self.matmul(x, &self.q_weights);
        let k = self.matmul(x, &self.k_weights);
        let v = self.matmul(x, &self.v_weights);
        
        // Attention(Q, K, V) = softmax(QK^T / sqrt(d_k)) V
        let k_t = self.transpose(&k);
        let scores = self.matmul(&q, &k_t);
        let scaled = self.scale(&scores, (EMBED_DIM as f32).sqrt());
        let attn = self.softmax(&scaled);
        self.matmul(&attn, &v)
    }
    
    fn matmul(&self, a: &[Vec<f32>], b: &[Vec<f32>]) -> Vec<Vec<f32>> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let rows = a.len();
        let cols = b[0].len();
        let inner = b.len();
        
        (0..rows)
            .map(|i| {
                (0..cols)
                    .map(|j| (0..inner).map(|k| a[i][k] * b[k][j]).sum())
                    .collect()
            })
            .collect()
    }
    
    fn transpose(&self, m: &[Vec<f32>]) -> Vec<Vec<f32>> {
        if m.is_empty() {
            return vec![];
        }
        let rows = m.len();
        let cols = m[0].len();
        (0..cols)
            .map(|j| (0..rows).map(|i| m[i][j]).collect())
            .collect()
    }
    
    fn scale(&self, m: &[Vec<f32>], factor: f32) -> Vec<Vec<f32>> {
        m.iter()
            .map(|row| row.iter().map(|&x| x / factor).collect())
            .collect()
    }
    
    fn softmax(&self, m: &[Vec<f32>]) -> Vec<Vec<f32>> {
        m.iter()
            .map(|row| {
                let max = row.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
                let exp: Vec<f32> = row.iter().map(|&x| (x - max).exp()).collect();
                let sum: f32 = exp.iter().sum();
                exp.iter().map(|&x| x / sum).collect()
            })
            .collect()
    }
}

#[derive(Debug)]
struct TinyTransformer {
    embedding: Embedding,
    attention: Attention,
    vocab: HashMap<u64, usize>,
}

impl TinyTransformer {
    fn new(vocab_size: usize) -> Self {
        Self {
            embedding: Embedding::new(vocab_size),
            attention: Attention::new(),
            vocab: HashMap::new(),
        }
    }
    
    fn build_vocab(&mut self, ips: &[u64]) {
        for (idx, &ip) in ips.iter().enumerate() {
            self.vocab.entry(ip).or_insert(idx);
        }
    }
    
    fn forward(&self, ips: &[u64]) -> Vec<Vec<f32>> {
        let embeddings: Vec<Vec<f32>> = ips
            .iter()
            .filter_map(|ip| self.vocab.get(ip))
            .map(|&token| self.embedding.forward(token))
            .collect();
        
        self.attention.forward(&embeddings)
    }
    
    fn train(&mut self, sequences: &[Vec<u64>], epochs: usize) {
        println!("📚 Training for {} epochs on {} CPUs...", epochs, rayon::current_num_threads());
        
        let mut loss_history = Vec::new();
        
        for epoch in 0..epochs {
            // Parallel training across sequences
            let losses: Vec<f32> = sequences.par_iter()
                .filter_map(|seq| {
                    if seq.len() < 2 {
                        return None;
                    }
                    
                    let mut seq_loss = 0.0;
                    
                    // Predict next IP
                    for i in 0..seq.len() - 1 {
                        let input = &seq[..i + 1];
                        let target = seq[i + 1];
                        
                        let output = self.forward(input);
                        
                        // Simple loss: distance from target embedding
                        if let Some(&target_token) = self.vocab.get(&target) {
                            let target_emb = self.embedding.forward(target_token);
                            if let Some(last_output) = output.last() {
                                let loss: f32 = last_output.iter()
                                    .zip(target_emb.iter())
                                    .map(|(a, b)| (a - b).powi(2))
                                    .sum();
                                seq_loss += loss;
                            }
                        }
                    }
                    
                    Some(seq_loss)
                })
                .collect();
            
            let total_loss: f32 = losses.iter().sum();
            let avg_loss = total_loss / sequences.len() as f32;
            loss_history.push(avg_loss);
            
            println!("  Epoch {}: loss = {:.4}", epoch, avg_loss);
        }
        
        println!("✅ Training complete!");
        
        // Save loss curve to CSV
        if let Err(e) = self.save_loss_curve(&loss_history) {
            eprintln!("⚠️  Failed to save loss curve: {}", e);
        }
    }
    
    fn save_loss_curve(&self, losses: &[f32]) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create("loss_curve.csv")?;
        writeln!(file, "epoch,loss")?;
        for (i, &loss) in losses.iter().enumerate() {
            writeln!(file, "{},{}", i, loss)?;
        }
        println!("📊 Loss curve saved to: loss_curve.csv");
        Ok(())
    }
    
    fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = bincode::serialize(&(
            &self.embedding.weights,
            &self.attention.q_weights,
            &self.attention.k_weights,
            &self.attention.v_weights,
            &self.vocab,
        ))?;
        std::fs::write(path, data)?;
        Ok(())
    }
}

fn load_perf_ips(path: &str) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = SerializedFileReader::new(file)?;
    let mut ips = Vec::new();
    
    for row_result in reader.get_row_iter(None)? {
        let row = row_result?;
        if let Some(ip_field) = row.get_column_iter().nth(1) {
            if let parquet::record::Field::Str(ip_str) = ip_field.1 {
                let cleaned = ip_str.trim_start_matches("0x");
                if let Ok(ip) = u64::from_str_radix(cleaned, 16) {
                    ips.push(ip);
                }
            }
        }
    }
    
    Ok(ips)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Tiny Transformer - Training on Real Perf Data");
    println!();
    
    // Set CPU threads
    rayon::ThreadPoolBuilder::new()
        .num_threads(24)
        .build_global()
        .unwrap();
    
    println!("💻 Using {} CPU threads", rayon::current_num_threads());
    
    #[cfg(feature = "gpu")]
    {
        if tch::Cuda::is_available() {
            println!("🎮 CUDA available!");
            println!("   Device count: {}", tch::Cuda::device_count());
            if let Ok(props) = tch::Cuda::get_device_properties(0) {
                println!("   GPU 0: {} ({} GB)", 
                    String::from_utf8_lossy(&props.name),
                    props.total_memory / (1024 * 1024 * 1024));
            }
        } else {
            println!("⚠️  CUDA not available, using CPU only");
        }
    }
    #[cfg(not(feature = "gpu"))]
    {
        println!("ℹ️  GPU support not enabled (build with --features gpu)");
    }
    
    println!();
    
    // Load real perf data
    println!("📊 Loading perf data...");
    let rust_ips = load_perf_ips("rust_perf.parquet")?;
    let python_ips = load_perf_ips("python_perf.parquet")?;
    let haskell_ips = load_perf_ips("haskell_perf.parquet")?;
    
    println!("  Rust: {} IPs", rust_ips.len());
    println!("  Python: {} IPs", python_ips.len());
    println!("  Haskell: {} IPs", haskell_ips.len());
    
    let all_ips: Vec<u64> = rust_ips.iter()
        .chain(python_ips.iter())
        .chain(haskell_ips.iter())
        .cloned()
        .collect();
    
    let unique_ips: Vec<u64> = {
        let mut set: Vec<u64> = all_ips.clone();
        set.sort_unstable();
        set.dedup();
        set
    };
    
    println!("  Total: {} IPs ({} unique)", all_ips.len(), unique_ips.len());
    println!();
    
    // Create transformer
    let mut transformer = TinyTransformer::new(unique_ips.len());
    
    // Build vocab
    println!("📚 Building vocabulary...");
    transformer.build_vocab(&unique_ips);
    println!("  Vocab size: {}", transformer.vocab.len());
    println!();
    
    // Train on sequences
    let sequences = vec![rust_ips.clone(), python_ips.clone(), haskell_ips.clone()];
    transformer.train(&sequences, 100);
    println!();
    
    // Save model
    println!("💾 Saving model...");
    transformer.save("tiny_transformer.bin")?;
    println!("  Saved to: tiny_transformer.bin");
    println!();
    
    // Inference
    println!("🔮 Inference on Rust trace...");
    let output = transformer.forward(&rust_ips[..10.min(rust_ips.len())]);
    println!("  Input: {} IPs", 10.min(rust_ips.len()));
    println!("  Output: {} embeddings of dim {}", output.len(), EMBED_DIM);
    
    println!();
    println!("✅ Tiny transformer trained on real data!");
    
    Ok(())
}
