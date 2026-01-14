// perf-runtime/src/probe.rs
// Parquet data capture for any value or type

use std::fs::{self, File};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use parquet::file::properties::WriterProperties;
use parquet::file::writer::SerializedFileWriter;
use parquet::schema::parser::parse_message_type;
use arrow::array::{ArrayRef, StringArray, Int64Array, Float64Array};
use arrow::datatypes::{Schema, Field, DataType};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;

const PROBE_DIR: &str = "data/probes";

pub struct ProbeSession {
    name: String,
    timestamp: u64,
    inputs: Vec<ProbeValue>,
    output: Option<ProbeValue>,
}

#[derive(Debug, Clone)]
pub struct ProbeValue {
    pub name: String,
    pub type_name: String,
    pub value_str: String,
    pub timestamp: u64,
}

impl ProbeSession {
    pub fn start(name: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            name: name.to_string(),
            timestamp,
            inputs: Vec::new(),
            output: None,
        }
    }
    
    pub fn capture_input<T: std::fmt::Debug>(&mut self, name: &str, value: &T) {
        self.inputs.push(ProbeValue {
            name: name.to_string(),
            type_name: std::any::type_name::<T>().to_string(),
            value_str: format!("{:?}", value),
            timestamp: self.timestamp,
        });
    }
    
    pub fn capture_output<T: std::fmt::Debug>(&mut self, value: &T) {
        self.output = Some(ProbeValue {
            name: "output".to_string(),
            type_name: std::any::type_name::<T>().to_string(),
            value_str: format!("{:?}", value),
            timestamp: self.timestamp,
        });
    }
    
    pub fn write_parquet(&self) {
        // Create probe directory
        fs::create_dir_all(PROBE_DIR).ok();
        
        // Create parquet file
        let file_path = format!("{}/probe_{}_{}.parquet", PROBE_DIR, self.name, self.timestamp);
        
        if let Err(e) = self.write_parquet_file(&file_path) {
            eprintln!("Failed to write parquet: {}", e);
        }
    }
    
    fn write_parquet_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Define schema
        let schema = Arc::new(Schema::new(vec![
            Field::new("name", DataType::Utf8, false),
            Field::new("type_name", DataType::Utf8, false),
            Field::new("value", DataType::Utf8, false),
            Field::new("timestamp", DataType::Int64, false),
            Field::new("is_output", DataType::Int64, false),
        ]));
        
        // Collect all values (inputs + output)
        let mut all_values = self.inputs.clone();
        if let Some(output) = &self.output {
            all_values.push(output.clone());
        }
        
        // Build arrays
        let names: Vec<_> = all_values.iter().map(|v| v.name.as_str()).collect();
        let type_names: Vec<_> = all_values.iter().map(|v| v.type_name.as_str()).collect();
        let values: Vec<_> = all_values.iter().map(|v| v.value_str.as_str()).collect();
        let timestamps: Vec<_> = all_values.iter().map(|v| v.timestamp as i64).collect();
        let is_outputs: Vec<_> = all_values.iter().enumerate()
            .map(|(i, _)| if i == all_values.len() - 1 && self.output.is_some() { 1i64 } else { 0i64 })
            .collect();
        
        let name_array = Arc::new(StringArray::from(names)) as ArrayRef;
        let type_array = Arc::new(StringArray::from(type_names)) as ArrayRef;
        let value_array = Arc::new(StringArray::from(values)) as ArrayRef;
        let timestamp_array = Arc::new(Int64Array::from(timestamps)) as ArrayRef;
        let is_output_array = Arc::new(Int64Array::from(is_outputs)) as ArrayRef;
        
        // Create record batch
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![name_array, type_array, value_array, timestamp_array, is_output_array],
        )?;
        
        // Write to parquet
        let file = File::create(path)?;
        let mut writer = ArrowWriter::try_new(file, schema, None)?;
        writer.write(&batch)?;
        writer.close()?;
        
        Ok(())
    }
}

/// Capture a single value to parquet (inline usage)
pub fn probe_capture<T: std::fmt::Debug>(name: &str, value: &T) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let probe_value = ProbeValue {
        name: name.to_string(),
        type_name: std::any::type_name::<T>().to_string(),
        value_str: format!("{:?}", value),
        timestamp,
    };
    
    // Create a single-value session
    let mut session = ProbeSession {
        name: "inline_probe".to_string(),
        timestamp,
        inputs: vec![probe_value],
        output: None,
    };
    
    session.write_parquet();
}
