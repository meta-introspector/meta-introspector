use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct MarkovMatrix {
    transitions: Vec<Vec<f64>>,
    states: Vec<String>,
}

#[derive(Debug, Serialize)]
struct Partition {
    partition_id: usize,
    states: Vec<String>,
    transitions: Vec<Vec<f64>>,
    semantic_summary: String,
}

fn main() {
    println!("🔍 Loading Markov matrix...");
    
    let matrix_file = "data-markov-analysis/matrices/markov_global_matrix.json";
    let content = fs::read_to_string(matrix_file).expect("Failed to read matrix");
    
    println!("📊 Parsing matrix...");
    let matrix: MarkovMatrix = serde_json::from_str(&content).expect("Failed to parse");
    
    let n_states = matrix.states.len();
    println!("   States: {}", n_states);
    
    // Convert to METIS graph format
    println!("🔧 Converting to METIS format...");
    let metis_file = "markov_graph.metis";
    write_metis_graph(&matrix, metis_file);
    
    // Run METIS partitioning (target ~1MB per partition)
    // Estimate: 1MB JSON ≈ 10K states, so partition into chunks of 10K
    let n_partitions = (n_states / 10000).max(1);
    println!("📦 Partitioning into {} semantic chunks...", n_partitions);
    
    let output = Command::new("gpmetis")
        .args([metis_file, &n_partitions.to_string()])
        .output();
    
    if output.is_err() {
        println!("⚠️  METIS not found, falling back to semantic clustering...");
        semantic_partition(&matrix, n_partitions);
    } else {
        println!("✅ METIS partitioning complete");
        load_metis_partitions(&matrix, metis_file, n_partitions);
    }
}

fn write_metis_graph(matrix: &MarkovMatrix, filename: &str) {
    let n = matrix.states.len();
    let mut edges = Vec::new();
    
    // Build adjacency list (only non-zero transitions)
    for i in 0..n {
        let mut adj = Vec::new();
        for j in 0..n {
            if i != j && matrix.transitions[i][j] > 0.01 {
                // Weight by transition probability
                let weight = (matrix.transitions[i][j] * 1000.0) as i32;
                adj.push(format!("{} {}", j + 1, weight)); // METIS is 1-indexed
            }
        }
        edges.push(adj);
    }
    
    // Write METIS format: n_vertices n_edges [format]
    let n_edges: usize = edges.iter().map(|e| e.len()).sum();
    let mut output = format!("{} {} 001\n", n, n_edges / 2); // 001 = weighted edges
    
    for adj in edges {
        output.push_str(&adj.join(" "));
        output.push('\n');
    }
    
    fs::write(filename, output).expect("Failed to write METIS file");
    println!("   Wrote {} vertices, {} edges", n, n_edges / 2);
}

fn semantic_partition(matrix: &MarkovMatrix, n_partitions: usize) {
    println!("🧠 Using semantic clustering fallback...");
    
    // Simple clustering by state name prefixes
    let chunk_size = matrix.states.len() / n_partitions;
    
    for i in 0..n_partitions {
        let start = i * chunk_size;
        let end = if i == n_partitions - 1 {
            matrix.states.len()
        } else {
            (i + 1) * chunk_size
        };
        
        let states = matrix.states[start..end].to_vec();
        let transitions = extract_submatrix(&matrix.transitions, start, end);
        
        let summary = analyze_partition(&states);
        
        let partition = Partition {
            partition_id: i,
            states,
            transitions,
            semantic_summary: summary,
        };
        
        let json = serde_json::to_string_pretty(&partition).unwrap();
        fs::write(format!("hf-markov-analysis-upload/partition_{:04}.json", i), json).unwrap();
        
        println!("   Partition {}: {} states", i, end - start);
    }
}

fn load_metis_partitions(matrix: &MarkovMatrix, metis_file: &str, n_partitions: usize) {
    let part_file = format!("{}.part.{}", metis_file, n_partitions);
    let assignments = fs::read_to_string(&part_file).expect("Failed to read partition file");
    
    let mut partitions: Vec<Vec<usize>> = vec![Vec::new(); n_partitions];
    
    for (idx, line) in assignments.lines().enumerate() {
        let part_id: usize = line.trim().parse().unwrap();
        partitions[part_id].push(idx);
    }
    
    for (part_id, indices) in partitions.iter().enumerate() {
        let states: Vec<String> = indices.iter().map(|&i| matrix.states[i].clone()).collect();
        let transitions = extract_submatrix_indices(&matrix.transitions, indices);
        
        let summary = analyze_partition(&states);
        
        let partition = Partition {
            partition_id: part_id,
            states,
            transitions,
            semantic_summary: summary.clone(),
        };
        
        let json = serde_json::to_string_pretty(&partition).unwrap();
        fs::write(format!("hf-markov-analysis-upload/partition_{:04}.json", part_id), json).unwrap();
        
        println!("   Partition {}: {} states - {}", part_id, indices.len(), summary);
    }
}

fn extract_submatrix(matrix: &[Vec<f64>], start: usize, end: usize) -> Vec<Vec<f64>> {
    matrix[start..end].iter()
        .map(|row| row[start..end].to_vec())
        .collect()
}

fn extract_submatrix_indices(matrix: &[Vec<f64>], indices: &[usize]) -> Vec<Vec<f64>> {
    indices.iter()
        .map(|&i| indices.iter().map(|&j| matrix[i][j]).collect())
        .collect()
}

fn analyze_partition(states: &[String]) -> String {
    // Analyze common patterns in state names
    let mut prefixes: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    
    for state in states {
        let prefix = state.split('_').next().unwrap_or(state);
        *prefixes.entry(prefix.to_string()).or_insert(0) += 1;
    }
    
    let mut sorted: Vec<_> = prefixes.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    
    let top_3: Vec<String> = sorted.iter().take(3)
        .map(|(k, v)| format!("{}({})", k, v))
        .collect();
    
    top_3.join(", ")
}
