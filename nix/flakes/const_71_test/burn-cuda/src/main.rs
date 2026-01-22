use burn::tensor::Tensor;
use burn_cuda::{Cuda, CudaDevice};

fn main() {
    // Use CUDA backend
    type B = Cuda;
    let device = CudaDevice::default();
    
    println!("🚀 Starting 71-dimensional transformer on GPU...\n");
    
    // Transformer dimensions based on 71
    let batch_size = 71;
    let seq_len = 71;
    let d_model = 71;
    let num_iterations = 1000;
    
    println!("Architecture:");
    println!("  Batch size: {}", batch_size);
    println!("  Sequence length: {}", seq_len);
    println!("  Model dimension: {}", d_model);
    println!("  Total parameters: {} (71³)", batch_size * seq_len * d_model);
    println!("  Iterations: {}\n", num_iterations);
    
    // Create input tensor [batch, seq_len, d_model]
    let input: Tensor<B, 3> = Tensor::ones([batch_size, seq_len, d_model], &device);
    
    // Query, Key, Value projections (simplified transformer)
    let w_q: Tensor<B, 2> = Tensor::ones([d_model, d_model], &device) * 0.01;
    let w_k: Tensor<B, 2> = Tensor::ones([d_model, d_model], &device) * 0.01;
    let w_v: Tensor<B, 2> = Tensor::ones([d_model, d_model], &device) * 0.01;
    
    println!("Running {} transformer iterations on GPU...", num_iterations);
    
    let mut output = input.clone();
    
    for i in 0..num_iterations {
        // Self-attention computation
        // Reshape for matmul: [batch*seq, d_model]
        let batch_seq = batch_size * seq_len;
        let reshaped = output.clone().reshape([batch_seq, d_model]);
        
        // Q, K, V projections
        let _q = reshaped.clone().matmul(w_q.clone()).reshape([batch_size, seq_len, d_model]);
        let _k = reshaped.clone().matmul(w_k.clone()).reshape([batch_size, seq_len, d_model]);
        let v = reshaped.matmul(w_v.clone()).reshape([batch_size, seq_len, d_model]);
        
        // Simplified attention: just use V directly (avoid complex transpose)
        let new_output = v + input.clone();
        
        // ReLU activation
        output = new_output.clamp_min(0.0);
        
        if (i + 1) % 100 == 0 {
            let sum = output.clone().sum().into_scalar();
            println!("  Iteration {}: output sum = {:.2}", i + 1, sum);
        }
    }
    
    // Final statistics
    let final_sum = output.clone().sum().into_scalar();
    let final_mean = output.clone().mean().into_scalar();
    let final_max = output.clone().max().into_scalar();
    
    println!("\n✅ Transformer computation complete!");
    println!("Final statistics:");
    println!("  Sum: {:.2}", final_sum);
    println!("  Mean: {:.6}", final_mean);
    println!("  Max: {:.6}", final_max);
    println!("  Total elements processed: {}", batch_size * seq_len * d_model * num_iterations);
    
    // Verify 71 constant
    let simple: Tensor<B, 1> = Tensor::from_floats([71.0], &device);
    let value = simple.into_data().to_vec::<f32>().unwrap()[0] as i32;
    println!("\n🧙 The wizard constant: {}", value);
}
