// Auto-label rustc code: Syn spectrum → Rustc IPs → Parquet database
// Store all mappings for semantic understanding

use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use arrow::array::{StringArray, UInt64Array, ArrayRef};
use arrow::datatypes::{Schema, Field, DataType};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RustcLabel {
    pub source_snippet: String,
    pub syn_node_type: String,
    pub rustc_ip: u64,
    pub rustc_function: String,
    pub semantic_label: String,
}

pub struct RustcAutoLabeler {
    pub labels: Vec<RustcLabel>,
    pub ip_to_label: HashMap<u64, String>,
}

impl RustcAutoLabeler {
    pub fn new() -> Self {
        Self {
            labels: Vec::new(),
            ip_to_label: HashMap::new(),
        }
    }
    
    /// Learn mapping: Syn node → Rustc IP → Semantic label
    pub fn learn_mapping(&mut self, source: &str, syn_node: &str, rustc_ips: &[u64]) {
        // Infer semantic label from syn node type
        let semantic_label = match syn_node {
            "Fn" => "function_definition",
            "Struct" => "type_definition",
            "Impl" => "trait_implementation",
            "Trait" => "trait_definition",
            "Enum" => "enum_definition",
            "Mod" => "module_definition",
            "Use" => "import_statement",
            _ => "unknown",
        };
        
        // Infer rustc function from IP range
        let rustc_function = Self::infer_rustc_function(rustc_ips);
        
        for &ip in rustc_ips {
            let label = RustcLabel {
                source_snippet: source.to_string(),
                syn_node_type: syn_node.to_string(),
                rustc_ip: ip,
                rustc_function: rustc_function.clone(),
                semantic_label: semantic_label.to_string(),
            };
            
            self.labels.push(label);
            self.ip_to_label.insert(ip, semantic_label.to_string());
        }
    }
    
    fn infer_rustc_function(ips: &[u64]) -> String {
        if ips.is_empty() {
            return "unknown".to_string();
        }
        
        // Infer from IP range (simplified)
        let base = ips[0] & 0xFFFF_0000;
        match base {
            0x1000_0000 => "parse_module",
            0x2000_0000 => "type_check",
            0x3000_0000 => "trait_resolution",
            0x4000_0000 => "monomorphization",
            0x5000_0000 => "codegen",
            _ => "compiler_internal",
        }.to_string()
    }
    
    /// Query: Given IP, what's the semantic label?
    pub fn label_for_ip(&self, ip: u64) -> Option<&str> {
        self.ip_to_label.get(&ip).map(|s| s.as_str())
    }
    
    /// Save to parquet
    pub fn save_to_parquet(&self, path: &str) -> Result<(), String> {
        // Define schema
        let schema = Arc::new(Schema::new(vec![
            Field::new("source_snippet", DataType::Utf8, false),
            Field::new("syn_node_type", DataType::Utf8, false),
            Field::new("rustc_ip", DataType::UInt64, false),
            Field::new("rustc_function", DataType::Utf8, false),
            Field::new("semantic_label", DataType::Utf8, false),
        ]));
        
        // Create arrays
        let source_snippets: Vec<&str> = self.labels.iter()
            .map(|l| l.source_snippet.as_str())
            .collect();
        let syn_nodes: Vec<&str> = self.labels.iter()
            .map(|l| l.syn_node_type.as_str())
            .collect();
        let ips: Vec<u64> = self.labels.iter()
            .map(|l| l.rustc_ip)
            .collect();
        let functions: Vec<&str> = self.labels.iter()
            .map(|l| l.rustc_function.as_str())
            .collect();
        let labels: Vec<&str> = self.labels.iter()
            .map(|l| l.semantic_label.as_str())
            .collect();
        
        let source_array = Arc::new(StringArray::from(source_snippets)) as ArrayRef;
        let syn_array = Arc::new(StringArray::from(syn_nodes)) as ArrayRef;
        let ip_array = Arc::new(UInt64Array::from(ips)) as ArrayRef;
        let function_array = Arc::new(StringArray::from(functions)) as ArrayRef;
        let label_array = Arc::new(StringArray::from(labels)) as ArrayRef;
        
        // Create record batch
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![source_array, syn_array, ip_array, function_array, label_array],
        ).map_err(|e| e.to_string())?;
        
        // Write to parquet
        let file = std::fs::File::create(path)
            .map_err(|e| e.to_string())?;
        
        let props = WriterProperties::builder()
            .set_compression(parquet::basic::Compression::SNAPPY)
            .build();
        
        let mut writer = ArrowWriter::try_new(file, schema, Some(props))
            .map_err(|e| e.to_string())?;
        
        writer.write(&batch).map_err(|e| e.to_string())?;
        writer.close().map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    pub fn report(&self) {
        println!("\n📊 Auto-Labeling Report");
        println!("  Total labels: {}", self.labels.len());
        println!("  Unique IPs: {}", self.ip_to_label.len());
        
        // Count by semantic label
        let mut label_counts = HashMap::new();
        for label in &self.labels {
            *label_counts.entry(&label.semantic_label).or_insert(0) += 1;
        }
        
        println!("\n  Semantic labels:");
        let mut sorted: Vec<_> = label_counts.iter().collect();
        sorted.sort_by_key(|e| e.1);
        sorted.reverse();
        
        for (label, count) in sorted {
            println!("    {}: {}", label, count);
        }
        
        // Count by rustc function
        let mut function_counts = HashMap::new();
        for label in &self.labels {
            *function_counts.entry(&label.rustc_function).or_insert(0) += 1;
        }
        
        println!("\n  Rustc functions:");
        let mut sorted: Vec<_> = function_counts.iter().collect();
        sorted.sort_by_key(|e| e.1);
        sorted.reverse();
        
        for (func, count) in sorted.iter().take(5) {
            println!("    {}: {}", func, count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_auto_labeler() {
        let mut labeler = RustcAutoLabeler::new();
        labeler.learn_mapping("fn main() {}", "Fn", &[0x1000, 0x1010]);
        
        assert_eq!(labeler.label_for_ip(0x1000), Some("function_definition"));
    }
}
