use std::fs;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 Loading similarity matrix...");
    
    // Read metadata to get dimensions
    let meta_str = fs::read_to_string("markov_similarity_matrix_meta.json")?;
    let n = 26383; // From metadata
    
    println!("   Matrix size: {} × {}", n, n);
    println!("   Expected bytes: {} ({:.2} GB)", n * n * 8, (n * n * 8) as f64 / (1024.0 * 1024.0 * 1024.0));
    
    println!("\n💾 Loading binary matrix...");
    let mut file = fs::File::open("markov_similarity_matrix.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    println!("   Loaded {} bytes ({:.2} GB)", buffer.len(), buffer.len() as f64 / (1024.0 * 1024.0 * 1024.0));
    
    // Convert bytes to f64 matrix
    println!("\n🔄 Converting to f64 matrix...");
    let mut matrix = vec![vec![0.0f64; n]; n];
    let mut idx = 0;
    for i in 0..n {
        for j in 0..n {
            let bytes = [
                buffer[idx], buffer[idx+1], buffer[idx+2], buffer[idx+3],
                buffer[idx+4], buffer[idx+5], buffer[idx+6], buffer[idx+7]
            ];
            matrix[i][j] = f64::from_le_bytes(bytes);
            idx += 8;
        }
        if (i + 1) % 1000 == 0 {
            println!("   Loaded {}/{} rows", i + 1, n);
        }
    }
    
    println!("\n📈 Computing statistics...");
    
    // Compute row sums (degree centrality)
    let mut row_sums: Vec<(usize, f64)> = (0..n)
        .map(|i| {
            let sum: f64 = matrix[i].iter().sum();
            (i, sum)
        })
        .collect();
    
    row_sums.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("\n🎯 Top 20 files by similarity (degree centrality):");
    for (rank, (idx, sum)) in row_sums.iter().take(20).enumerate() {
        println!("   {}. Index {}: sum = {:.2}", rank + 1, idx, sum);
    }
    
    // Power iteration for dominant eigenvector
    println!("\n🔢 Computing dominant eigenvector (power iteration)...");
    let max_iterations = 100;
    let tolerance = 1e-6;
    
    // Initialize random vector
    let mut v: Vec<f64> = (0..n).map(|i| (i as f64).sin()).collect();
    
    // Normalize
    let norm: f64 = v.iter().map(|x| x * x).sum::<f64>().sqrt();
    v.iter_mut().for_each(|x| *x /= norm);
    
    let mut eigenvalue = 0.0;
    
    for iter in 0..max_iterations {
        // Matrix-vector multiplication: v_new = A * v
        let mut v_new = vec![0.0; n];
        for i in 0..n {
            v_new[i] = matrix[i].iter().zip(&v).map(|(a, b)| a * b).sum();
        }
        
        // Compute eigenvalue (Rayleigh quotient)
        let new_eigenvalue: f64 = v_new.iter().zip(&v).map(|(a, b)| a * b).sum();
        
        // Normalize
        let norm: f64 = v_new.iter().map(|x| x * x).sum::<f64>().sqrt();
        v_new.iter_mut().for_each(|x| *x /= norm);
        
        // Check convergence
        let diff = (new_eigenvalue - eigenvalue).abs();
        if diff < tolerance {
            println!("   ✓ Converged after {} iterations", iter + 1);
            eigenvalue = new_eigenvalue;
            v = v_new;
            break;
        }
        
        eigenvalue = new_eigenvalue;
        v = v_new;
        
        if (iter + 1) % 10 == 0 {
            println!("   Iteration {}: eigenvalue = {:.6}", iter + 1, eigenvalue);
        }
    }
    
    println!("\n✅ Dominant eigenvalue: {:.6}", eigenvalue);
    
    // Find top components of eigenvector
    let mut components: Vec<(usize, f64)> = v.iter().enumerate().map(|(i, &val)| (i, val.abs())).collect();
    components.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("\n🎯 Top 20 components of dominant eigenvector:");
    for (rank, (idx, val)) in components.iter().take(20).enumerate() {
        println!("   {}. Index {}: {:.6}", rank + 1, idx, val);
    }
    
    // Save eigenvector as simple text
    let mut output = format!("Dominant eigenvalue: {:.6}\n\nTop 20 components:\n", eigenvalue);
    for (rank, (idx, val)) in components.iter().take(20).enumerate() {
        output.push_str(&format!("{}. Index {}: {:.6}\n", rank + 1, idx, val));
    }
    
    fs::write("markov_dominant_eigenvector.txt", output)?;
    println!("\n💾 Saved eigenvector to markov_dominant_eigenvector.txt");
    
    Ok(())
}
