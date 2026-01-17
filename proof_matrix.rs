// Proof matrix: syn source → rustc .so signature mapping
// Each code feature creates eigenvectors through rustc

use std::collections::HashSet;

#[derive(Clone)]
pub struct SourceSignature {
    pub source_hash: String,
    pub syn_nodes: Vec<String>,
    pub rustc_ips: HashSet<u64>,
}

#[derive(Clone)]
pub struct SoSignature {
    pub symbol: String,
    pub address: u64,
    pub size: usize,
}

pub struct ProofMatrix {
    pub rows: Vec<SourceSignature>,  // Source code
    pub cols: Vec<SoSignature>,      // .so symbols
    pub matrix: Vec<Vec<f64>>,       // Mapping strength
}

impl ProofMatrix {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            cols: Vec::new(),
            matrix: Vec::new(),
        }
    }
    
    pub fn add_source(&mut self, sig: SourceSignature) {
        self.rows.push(sig);
    }
    
    pub fn add_so_symbol(&mut self, sig: SoSignature) {
        self.cols.push(sig);
    }
    
    pub fn compute_mapping(&mut self) {
        // Build matrix: source × .so symbols
        self.matrix = vec![vec![0.0; self.cols.len()]; self.rows.len()];
        
        for (i, source) in self.rows.iter().enumerate() {
            for (j, so_sym) in self.cols.iter().enumerate() {
                // Mapping strength: IP overlap
                let overlap = source.rustc_ips.contains(&so_sym.address);
                self.matrix[i][j] = if overlap { 1.0 } else { 0.0 };
            }
        }
    }
    
    pub fn find_diagonal(&self) -> Vec<f64> {
        // Extract diagonal: natural mapping
        let mut diagonal = Vec::new();
        let size = self.matrix.len().min(self.matrix.first().map(|r| r.len()).unwrap_or(0));
        
        for i in 0..size {
            diagonal.push(self.matrix[i][i]);
        }
        
        diagonal
    }
    
    pub fn compute_eigenvectors(&self) -> Vec<Vec<f64>> {
        // Simplified eigenvector approximation
        // Real implementation would use proper linear algebra
        let mut eigenvectors = Vec::new();
        
        if self.matrix.is_empty() {
            return eigenvectors;
        }
        
        // Power iteration for dominant eigenvector
        let n = self.matrix.len();
        let m = self.matrix[0].len();
        
        let mut v = vec![1.0 / (n as f64).sqrt(); n];
        
        for _ in 0..10 {  // 10 iterations
            let mut new_v = vec![0.0; n];
            
            for i in 0..n {
                for j in 0..m.min(n) {
                    new_v[i] += self.matrix[i][j] * v[j.min(n-1)];
                }
            }
            
            // Normalize
            let norm: f64 = new_v.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm > 0.0 {
                v = new_v.iter().map(|x| x / norm).collect();
            }
        }
        
        eigenvectors.push(v);
        eigenvectors
    }
    
    pub fn report(&self) {
        println!("\n📊 Proof Matrix Report");
        println!("  Source signatures: {}", self.rows.len());
        println!("  .so symbols: {}", self.cols.len());
        println!("  Matrix size: {}×{}", self.rows.len(), self.cols.len());
        
        if !self.matrix.is_empty() {
            let total: f64 = self.matrix.iter()
                .flat_map(|row| row.iter())
                .sum();
            let density = total / (self.rows.len() * self.cols.len()) as f64;
            
            println!("  Mapping density: {:.3}", density);
            
            let diagonal = self.find_diagonal();
            if !diagonal.is_empty() {
                let diag_sum: f64 = diagonal.iter().sum();
                println!("  Diagonal strength: {:.3}", diag_sum);
            }
        }
    }
    
    pub fn save_to_parquet(&self, path: &str) -> Result<(), String> {
        use arrow::array::{StringArray, Float64Array};
        use arrow::record_batch::RecordBatch;
        use arrow::datatypes::{Schema, Field, DataType};
        use std::sync::Arc;
        use parquet::arrow::ArrowWriter;
        use std::fs;
        
        let schema = Arc::new(Schema::new(vec![
            Field::new("source_hash", DataType::Utf8, false),
            Field::new("so_symbol", DataType::Utf8, false),
            Field::new("mapping_strength", DataType::Float64, false),
        ]));
        
        let mut source_hashes = Vec::new();
        let mut so_symbols = Vec::new();
        let mut strengths = Vec::new();
        
        for (i, source) in self.rows.iter().enumerate() {
            for (j, so_sym) in self.cols.iter().enumerate() {
                if i < self.matrix.len() && j < self.matrix[i].len()
                    && self.matrix[i][j] > 0.0 {
                        source_hashes.push(source.source_hash.clone());
                        so_symbols.push(so_sym.symbol.clone());
                        strengths.push(self.matrix[i][j]);
                    }
            }
        }
        
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(StringArray::from(source_hashes)),
                Arc::new(StringArray::from(so_symbols)),
                Arc::new(Float64Array::from(strengths)),
            ],
        ).map_err(|e| e.to_string())?;
        
        let file = fs::File::create(path).map_err(|e| e.to_string())?;
        let mut writer = ArrowWriter::try_new(file, schema, None).map_err(|e| e.to_string())?;
        writer.write(&batch).map_err(|e| e.to_string())?;
        writer.close().map_err(|e| e.to_string())?;
        
        Ok(())
    }
}
