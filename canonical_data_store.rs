//! # Canonical Data Store
//! 
//! THE ONLY PLACE for crossbeam + parquet data storage.
//! Reuses proven patterns from markov_resonance_analyzer.

use crossbeam::channel::{bounded, Receiver, Sender};
use arrow::array::{StringArray, UInt64Array, Float64Array};
use arrow::record_batch::RecordBatch;
use arrow::datatypes::{DataType, Field, Schema};
use parquet::arrow::ArrowWriter;
use std::sync::Arc;
use std::fs;
use serde::{Serialize, Deserialize};

/// Canonical data store with crossbeam workers and parquet output
pub struct CanonicalDataStore<T: Clone + Send + 'static> {
    sender: Sender<T>,
    receiver: Receiver<T>,
    num_workers: usize,
}

impl<T: Clone + Send + 'static> CanonicalDataStore<T> {
    pub fn new(buffer_size: usize) -> Self {
        let (sender, receiver) = bounded(buffer_size);
        let num_workers = num_cpus::get() * 2; // 2x for I/O bound
        
        Self {
            sender,
            receiver,
            num_workers,
        }
    }
    
    pub fn sender(&self) -> Sender<T> {
        self.sender.clone()
    }
    
    pub fn receiver(&self) -> Receiver<T> {
        self.receiver.clone()
    }
    
    pub fn num_workers(&self) -> usize {
        self.num_workers
    }
}

/// Save grammar data to parquet (reuses markov pattern)
pub fn save_grammar_parquet(
    grammars: &[(String, String, u64, usize, f64)], // (function, lmfdb_label, signature, states, score)
    output_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("function_name", DataType::Utf8, false),
        Field::new("lmfdb_label", DataType::Utf8, false),
        Field::new("signature", DataType::UInt64, false),
        Field::new("states", DataType::UInt64, false),
        Field::new("score", DataType::Float64, false),
    ]));
    
    let file = fs::File::create(output_path)?;
    let mut writer = ArrowWriter::try_new(file, schema.clone(), None)?;
    
    // Write in batches of 100k rows (proven size from markov)
    let batch_size = 100_000;
    for chunk in grammars.chunks(batch_size) {
        let functions: Vec<&str> = chunk.iter().map(|(f, _, _, _, _)| f.as_str()).collect();
        let labels: Vec<&str> = chunk.iter().map(|(_, l, _, _, _)| l.as_str()).collect();
        let signatures: Vec<u64> = chunk.iter().map(|(_, _, s, _, _)| *s).collect();
        let states: Vec<u64> = chunk.iter().map(|(_, _, _, st, _)| *st as u64).collect();
        let scores: Vec<f64> = chunk.iter().map(|(_, _, _, _, sc)| *sc).collect();
        
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(StringArray::from(functions)),
                Arc::new(StringArray::from(labels)),
                Arc::new(UInt64Array::from(signatures)),
                Arc::new(UInt64Array::from(states)),
                Arc::new(Float64Array::from(scores)),
            ],
        )?;
        
        writer.write(&batch)?;
    }
    
    writer.close()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_data_store() {
        let store = CanonicalDataStore::<String>::new(100);
        assert!(store.num_workers() > 0);
    }
}

fn main() {
    println!("canonical_data_store - add usage here");
}
