// mes-transformer-gpu - Port of tiny_transformer to GPU with burn-cuda
use burn::tensor::Tensor;
use burn_cuda::{Cuda, CudaDevice};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;
use std::fs::File;
use std::collections::HashMap;

const EMBED_DIM: usize = 71;  // Use 71 for the wizard constant!
const SEQ_LEN: usize = 71;
const BATCH_SIZE: usize = 8;
const EPOCHS: usize = 100;

type B = Cuda;

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

fn build_vocab(ips: &[u64]) -> HashMap<u64, usize> {
    let mut unique: Vec<u64> = ips.to_vec();
    unique.sort_unstable();
    unique.dedup();
    
    unique.into_iter().enumerate().map(|(i, ip)| (ip, i)).collect()
}

fn train_transformer(
    vocab: &HashMap<u64, usize>, 
    sequences: &[Vec<u64>],
    device: &CudaDevice
) {
    let vocab_size = vocab.len();
    
    println!("🚀 MES Transformer GPU Training");
    println!("  Vocab size: {}", vocab_size);
    println!("  Embed dim: {}", EMBED_DIM);
    println!("  Seq len: {}", SEQ_LEN);
    println!("  Batch size: {}", BATCH_SIZE);
    println!("  Epochs: {}", EPOCHS);
    println!();
    
    // Create embedding matrix on GPU [vocab_size, EMBED_DIM]
    let embedding: Tensor<B, 2> = Tensor::random(
        [vocab_size, EMBED_DIM],
        burn::tensor::Distribution::Uniform(-0.5, 0.5),
        device
    );
    
    // Create Q, K, V weight matrices on GPU [EMBED_DIM, EMBED_DIM]
    let w_q: Tensor<B, 2> = Tensor::random(
        [EMBED_DIM, EMBED_DIM],
        burn::tensor::Distribution::Uniform(-0.1, 0.1),
        device
    );
    let w_k: Tensor<B, 2> = Tensor::random(
        [EMBED_DIM, EMBED_DIM],
        burn::tensor::Distribution::Uniform(-0.1, 0.1),
        device
    );
    let w_v: Tensor<B, 2> = Tensor::random(
        [EMBED_DIM, EMBED_DIM],
        burn::tensor::Distribution::Uniform(-0.1, 0.1),
        device
    );
    
    println!("📊 Training on {} sequences...", sequences.len());
    
    for epoch in 0..EPOCHS {
        let mut total_loss = 0.0;
        let mut num_batches = 0;
        
        for seq in sequences {
            if seq.len() < SEQ_LEN + 1 {
                continue;
            }
            
            // Convert IPs to token indices
            let tokens: Vec<usize> = seq.iter()
                .take(SEQ_LEN + 1)
                .filter_map(|ip| vocab.get(ip).copied())
                .collect();
            
            if tokens.len() < SEQ_LEN + 1 {
                continue;
            }
            
            // Input: first SEQ_LEN tokens
            let input_tokens = &tokens[..SEQ_LEN];
            
            // Create input tensor [SEQ_LEN] with token indices
            let input_data: Vec<f32> = input_tokens.iter().map(|&t| t as f32).collect();
            let input_indices: Tensor<B, 1> = Tensor::from_floats(input_data.as_slice(), device);
            
            // Lookup embeddings (simplified - using random for now)
            // Real implementation would use gather/embedding_lookup
            let embedded: Tensor<B, 2> = Tensor::random(
                [SEQ_LEN, EMBED_DIM],
                burn::tensor::Distribution::Normal(0.0, 1.0),
                device
            );
            
            // Transformer forward pass on GPU
            // Q = embedded @ W_q
            let q = embedded.clone().matmul(w_q.clone());
            
            // K = embedded @ W_k
            let k = embedded.clone().matmul(w_k.clone());
            
            // V = embedded @ W_v
            let v = embedded.matmul(w_v.clone());
            
            // Attention scores: Q @ K^T
            let k_t = k.swap_dims(0, 1);
            let scores = q.matmul(k_t);
            
            // Scale by sqrt(d_k)
            let scaled = scores / (EMBED_DIM as f32).sqrt();
            
            // Softmax
            let attn = burn::tensor::activation::softmax(scaled, 1);
            
            // Output: attn @ V
            let output = attn.matmul(v);
            
            // Compute loss (MSE with target)
            let target_token = tokens[SEQ_LEN];
            let target_emb = embedding.clone().slice([target_token..target_token+1, 0..EMBED_DIM]);
            
            // Loss: distance from last output to target
            let last_output = output.clone().slice([SEQ_LEN-1..SEQ_LEN, 0..EMBED_DIM]);
            let diff = last_output - target_emb.reshape([1, EMBED_DIM]);
            let loss = (diff.clone() * diff).sum().into_scalar();
            
            total_loss += loss;
            num_batches += 1;
        }
        
        let avg_loss = if num_batches > 0 { total_loss / num_batches as f32 } else { 0.0 };
        
        if (epoch + 1) % 10 == 0 {
            println!("  Epoch {}: loss = {:.4}", epoch + 1, avg_loss);
        }
    }
    
    println!();
    println!("✅ Training complete!");
    println!("🧙 The wizard constant: {}", EMBED_DIM);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 MES Transformer - GPU Training with Burn-CUDA\n");
    
    let device = CudaDevice::default();
    println!("🎮 Using CUDA device: {:?}\n", device);
    
    // Load perf data from 71 flakes builds
    println!("📊 Loading perf data from nix store builds...");
    
    let perf_dir = "../../data/71_flakes_perf";
    let rust_perf = format!("{}/rust_*_build.perf.data", perf_dir);
    let python_perf = format!("{}/python_*_build.perf.data", perf_dir);
    let haskell_perf = format!("{}/haskell_*_build.perf.data", perf_dir);
    
    // For now, we need to extract IPs from perf.data files
    // This requires parsing the binary perf format
    println!("⚠️  Perf data extraction not yet implemented");
    println!("    Need to parse perf.data binary format to extract instruction pointers");
    println!("    Available: {} of real nix build perf data", "4GB");
    
    return Err("Perf data extraction not implemented - need to parse perf.data format".into());
    
    println!("  Loaded {} IPs", rust_ips.len());
    
    // Build vocabulary
    println!("📚 Building vocabulary...");
    let vocab = build_vocab(&rust_ips);
    println!("  Vocab size: {}\n", vocab.len());
    
    // Create sequences
    let sequences: Vec<Vec<u64>> = rust_ips
        .chunks(SEQ_LEN + 1)
        .map(|chunk| chunk.to_vec())
        .collect();
    
    println!("  Created {} sequences\n", sequences.len());
    
    // Train on GPU
    train_transformer(&vocab, &sequences, &device);
    
    Ok(())
}
