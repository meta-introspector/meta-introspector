// Content addressable store for source snippets
// Compressed storage in parquet with complexity ordering

use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::io::{Write, Read};
use arrow::array::{StringArray, UInt64Array, Float64Array};
use arrow::record_batch::RecordBatch;
use arrow::datatypes::{Schema, Field, DataType};
use std::sync::Arc;
use parquet::arrow::ArrowWriter;

pub struct ContentStore {
    root: PathBuf,
    index: HashMap<String, SnippetMeta>,
}

#[derive(Clone)]
pub struct SnippetMeta {
    pub hash: String,
    pub compressed_size: usize,
    pub original_size: usize,
    pub complexity: f64,
    pub refs: usize,
}

impl ContentStore {
    pub fn new(root: &str) -> Self {
        let root_path = PathBuf::from(root);
        fs::create_dir_all(&root_path).ok();
        fs::create_dir_all(root_path.join("objects")).ok();
        
        Self {
            root: root_path,
            index: HashMap::new(),
        }
    }
    
    pub fn hash_content(content: &str) -> String {
        format!("{:x}", content.len())
    }
    
    pub fn measure_complexity(content: &str) -> f64 {
        // Complexity = unique chars / total chars
        let unique: std::collections::HashSet<char> = content.chars().collect();
        unique.len() as f64 / content.len().max(1) as f64
    }
    
    pub fn compress(content: &str) -> Option<Vec<u8>> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(content.as_bytes()).ok()?;
        encoder.finish().ok()
    }
    
    pub fn decompress(data: &[u8]) -> Option<String> {
        let mut decoder = GzDecoder::new(data);
        let mut result = String::new();
        decoder.read_to_string(&mut result).ok()?;
        Some(result)
    }
    
    pub fn store(&mut self, content: &str) -> String {
        let hash = Self::hash_content(content);
        let complexity = Self::measure_complexity(content);
        
        if let Some(meta) = self.index.get_mut(&hash) {
            meta.refs += 1;
            return hash;
        }
        
        // Compress content
        if let Some(compressed) = Self::compress(content) {
            // Store by complexity level (0-9)
            let level = (complexity * 10.0).min(9.0) as usize;
            let obj_dir = self.root.join("objects").join(format!("level_{}", level));
            fs::create_dir_all(&obj_dir).ok();
            
            let obj_path = obj_dir.join(&hash);
            fs::write(&obj_path, &compressed).ok();
            
            self.index.insert(hash.clone(), SnippetMeta {
                hash: hash.clone(),
                compressed_size: compressed.len(),
                original_size: content.len(),
                complexity,
                refs: 1,
            });
        }
        
        hash
    }
    
    pub fn load(&self, hash: &str) -> Option<String> {
        let meta = self.index.get(hash)?;
        let level = (meta.complexity * 10.0).min(9.0) as usize;
        let obj_path = self.root.join("objects").join(format!("level_{}", level)).join(hash);
        let compressed = fs::read(obj_path).ok()?;
        Self::decompress(&compressed)
    }
    
    pub fn save_to_parquet(&self, path: &str) -> Result<(), String> {
        let schema = Arc::new(Schema::new(vec![
            Field::new("hash", DataType::Utf8, false),
            Field::new("compressed_size", DataType::UInt64, false),
            Field::new("original_size", DataType::UInt64, false),
            Field::new("complexity", DataType::Float64, false),
            Field::new("refs", DataType::UInt64, false),
        ]));
        
        let mut hashes = Vec::new();
        let mut compressed_sizes = Vec::new();
        let mut original_sizes = Vec::new();
        let mut complexities = Vec::new();
        let mut refs = Vec::new();
        
        for meta in self.index.values() {
            hashes.push(meta.hash.clone());
            compressed_sizes.push(meta.compressed_size as u64);
            original_sizes.push(meta.original_size as u64);
            complexities.push(meta.complexity);
            refs.push(meta.refs as u64);
        }
        
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(StringArray::from(hashes)),
                Arc::new(UInt64Array::from(compressed_sizes)),
                Arc::new(UInt64Array::from(original_sizes)),
                Arc::new(Float64Array::from(complexities)),
                Arc::new(UInt64Array::from(refs)),
            ],
        ).map_err(|e| e.to_string())?;
        
        let file = fs::File::create(path).map_err(|e| e.to_string())?;
        let mut writer = ArrowWriter::try_new(file, schema, None).map_err(|e| e.to_string())?;
        writer.write(&batch).map_err(|e| e.to_string())?;
        writer.close().map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    pub fn report(&self) {
        let total_compressed: usize = self.index.values().map(|m| m.compressed_size).sum();
        let total_original: usize = self.index.values().map(|m| m.original_size).sum();
        let total_refs: usize = self.index.values().map(|m| m.refs).sum();
        
        println!("\n📦 Content Store Report");
        println!("  Unique snippets: {}", self.index.len());
        println!("  Original size: {} bytes", total_original);
        println!("  Compressed size: {} bytes", total_compressed);
        println!("  Compression ratio: {:.2}x", total_original as f64 / total_compressed.max(1) as f64);
        println!("  Total references: {}", total_refs);
        println!("  Deduplication: {:.1}x", total_refs as f64 / self.index.len().max(1) as f64);
        
        // Report by complexity level
        let mut by_level: HashMap<usize, usize> = HashMap::new();
        for meta in self.index.values() {
            let level = (meta.complexity * 10.0).min(9.0) as usize;
            *by_level.entry(level).or_insert(0) += 1;
        }
        
        println!("\n  By complexity level:");
        for level in 0..10 {
            if let Some(count) = by_level.get(&level) {
                println!("    Level {}: {} snippets", level, count);
            }
        }
    }
}
