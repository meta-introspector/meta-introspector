use anyhow::Result;
use arrow::array::*;
use arrow::datatypes::*;
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, debug, error};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProbeEvent {
    pub timestamp_ns: u64,
    pub probe_id: String,
    pub process_id: u32,
    pub thread_id: u64,
    pub function_name: String,
    pub event_type: EventType,
    pub data_payload: Vec<u8>,
    pub stack_trace: Option<Vec<String>>,
    pub cpu_id: u16,
    pub duration_ns: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EventType {
    FunctionEntry,
    FunctionExit,
    MemoryAlloc,
    MemoryFree,
    NetworkSend,
    NetworkReceive,
    FileOpen,
    FileRead,
    FileWrite,
    FileClose,
    SystemCall,
    Custom(String),
}

pub struct ParquetStreamer {
    output_dir: std::path::PathBuf,
    event_receiver: mpsc::Receiver<ProbeEvent>,
    event_sender: mpsc::Sender<ProbeEvent>,
    batch_size: usize,
    compression_level: i32,
}

impl ParquetStreamer {
    pub async fn new(output_dir: &Path) -> Result<Self> {
        tokio::fs::create_dir_all(output_dir).await?;
        
        let (event_sender, event_receiver) = mpsc::channel(10000);
        
        Ok(Self {
            output_dir: output_dir.to_path_buf(),
            event_receiver,
            event_sender,
            batch_size: 1000,
            compression_level: 6,
        })
    }
    
    pub fn get_sender(&self) -> mpsc::Sender<ProbeEvent> {
        self.event_sender.clone()
    }
    
    pub async fn start_streaming(&mut self) -> Result<()> {
        info!("🌊 Starting Parquet streaming to: {:?}", self.output_dir);
        
        let mut batch_buffer = Vec::with_capacity(self.batch_size);
        let mut file_counter = 0u64;
        
        while let Some(event) = self.event_receiver.recv().await {
            batch_buffer.push(event);
            
            // Write batch when full or on timeout
            if batch_buffer.len() >= self.batch_size {
                self.write_batch(&batch_buffer, file_counter).await?;
                batch_buffer.clear();
                file_counter += 1;
            }
        }
        
        // Write remaining events
        if !batch_buffer.is_empty() {
            self.write_batch(&batch_buffer, file_counter).await?;
        }
        
        Ok(())
    }
    
    async fn write_batch(&self, events: &[ProbeEvent], file_counter: u64) -> Result<()> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("probe_events_{}_{:06}.parquet", timestamp, file_counter);
        let filepath = self.output_dir.join(filename);
        
        debug!("📝 Writing batch of {} events to: {:?}", events.len(), filepath);
        
        // Convert events to Arrow RecordBatch
        let record_batch = self.events_to_record_batch(events)?;
        
        // Write to Parquet with compression
        let file = std::fs::File::create(&filepath)?;
        let props = WriterProperties::builder()
            .set_compression(parquet::basic::Compression::ZSTD(
                parquet::basic::ZstdLevel::try_new(self.compression_level)?
            ))
            .set_write_batch_size(self.batch_size)
            .build();
        
        let mut writer = ArrowWriter::try_new(file, record_batch.schema(), Some(props))?;
        writer.write(&record_batch)?;
        writer.close()?;
        
        info!("✅ Wrote {} events to: {:?}", events.len(), filepath);
        Ok(())
    }
    
    fn events_to_record_batch(&self, events: &[ProbeEvent]) -> Result<RecordBatch> {
        let schema = self.create_schema();
        
        // Extract data from events
        let mut timestamps = Vec::with_capacity(events.len());
        let mut probe_ids = Vec::with_capacity(events.len());
        let mut process_ids = Vec::with_capacity(events.len());
        let mut thread_ids = Vec::with_capacity(events.len());
        let mut function_names = Vec::with_capacity(events.len());
        let mut event_types = Vec::with_capacity(events.len());
        let mut data_payloads = Vec::with_capacity(events.len());
        let mut cpu_ids = Vec::with_capacity(events.len());
        let mut durations = Vec::with_capacity(events.len());
        
        for event in events {
            timestamps.push(event.timestamp_ns as i64);
            probe_ids.push(event.probe_id.as_str());
            process_ids.push(event.process_id);
            thread_ids.push(event.thread_id);
            function_names.push(event.function_name.as_str());
            event_types.push(format!("{:?}", event.event_type));
            data_payloads.push(event.data_payload.as_slice());
            cpu_ids.push(event.cpu_id);
            durations.push(event.duration_ns.map(|d| d as i64));
        }
        
        // Create Arrow arrays
        let timestamp_array = Int64Array::from(timestamps);
        let probe_id_array = StringArray::from(probe_ids);
        let process_id_array = UInt32Array::from(process_ids);
        let thread_id_array = UInt64Array::from(thread_ids);
        let function_name_array = StringArray::from(function_names);
        let event_type_array = StringArray::from(event_types);
        let data_payload_array = BinaryArray::from(data_payloads);
        let cpu_id_array = UInt16Array::from(cpu_ids);
        let duration_array = Int64Array::from(durations);
        
        // Create RecordBatch
        let record_batch = RecordBatch::try_new(
            schema,
            vec![
                Arc::new(timestamp_array),
                Arc::new(probe_id_array),
                Arc::new(process_id_array),
                Arc::new(thread_id_array),
                Arc::new(function_name_array),
                Arc::new(event_type_array),
                Arc::new(data_payload_array),
                Arc::new(cpu_id_array),
                Arc::new(duration_array),
            ],
        )?;
        
        Ok(record_batch)
    }
    
    fn create_schema(&self) -> Arc<Schema> {
        Arc::new(Schema::new(vec![
            Field::new("timestamp_ns", DataType::Int64, false),
            Field::new("probe_id", DataType::Utf8, false),
            Field::new("process_id", DataType::UInt32, false),
            Field::new("thread_id", DataType::UInt64, false),
            Field::new("function_name", DataType::Utf8, false),
            Field::new("event_type", DataType::Utf8, false),
            Field::new("data_payload", DataType::Binary, true),
            Field::new("cpu_id", DataType::UInt16, false),
            Field::new("duration_ns", DataType::Int64, true),
        ]))
    }
}

// Streaming reader for real-time analysis
pub struct ParquetStreamReader {
    data_dir: std::path::PathBuf,
    file_watcher: tokio::sync::watch::Receiver<Vec<std::path::PathBuf>>,
}

impl ParquetStreamReader {
    pub async fn new(data_dir: &Path) -> Result<Self> {
        let (tx, rx) = tokio::sync::watch::channel(Vec::new());
        
        // Watch for new Parquet files
        let data_dir_clone = data_dir.to_path_buf();
        tokio::spawn(async move {
            let mut last_files = std::collections::HashSet::new();
            
            loop {
                if let Ok(entries) = tokio::fs::read_dir(&data_dir_clone).await {
                    let mut current_files = Vec::new();
                    let mut entries = entries;
                    
                    while let Ok(Some(entry)) = entries.next_entry().await {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "parquet" {
                                current_files.push(entry.path());
                            }
                        }
                    }
                    
                    // Check for new files
                    let current_set: std::collections::HashSet<_> = current_files.iter().collect();
                    if current_set != last_files {
                        let _ = tx.send(current_files.clone());
                        last_files = current_set;
                    }
                }
                
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });
        
        Ok(Self {
            data_dir: data_dir.to_path_buf(),
            file_watcher: rx,
        })
    }
    
    pub async fn stream_events(&mut self) -> Result<mpsc::Receiver<ProbeEvent>> {
        let (tx, rx) = mpsc::channel(1000);
        
        let mut watcher = self.file_watcher.clone();
        
        tokio::spawn(async move {
            while watcher.changed().await.is_ok() {
                let files = watcher.borrow().clone();
                
                for file_path in files {
                    if let Err(e) = Self::read_parquet_file(&file_path, &tx).await {
                        error!("Failed to read Parquet file {:?}: {}", file_path, e);
                    }
                }
            }
        });
        
        Ok(rx)
    }
    
    async fn read_parquet_file(
        file_path: &Path, 
        sender: &mpsc::Sender<ProbeEvent>
    ) -> Result<()> {
        use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
        
        let file = std::fs::File::open(file_path)?;
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
        let mut reader = builder.build()?;
        
        while let Some(batch_result) = reader.next() {
            let batch = batch_result?;
            
            // Convert RecordBatch back to ProbeEvents
            let events = Self::record_batch_to_events(&batch)?;
            
            for event in events {
                if sender.send(event).await.is_err() {
                    break; // Receiver dropped
                }
            }
        }
        
        Ok(())
    }
    
    fn record_batch_to_events(batch: &RecordBatch) -> Result<Vec<ProbeEvent>> {
        let mut events = Vec::new();
        
        let timestamps = batch.column(0).as_any().downcast_ref::<Int64Array>().unwrap();
        let probe_ids = batch.column(1).as_any().downcast_ref::<StringArray>().unwrap();
        let process_ids = batch.column(2).as_any().downcast_ref::<UInt32Array>().unwrap();
        let thread_ids = batch.column(3).as_any().downcast_ref::<UInt64Array>().unwrap();
        let function_names = batch.column(4).as_any().downcast_ref::<StringArray>().unwrap();
        let event_types = batch.column(5).as_any().downcast_ref::<StringArray>().unwrap();
        let data_payloads = batch.column(6).as_any().downcast_ref::<BinaryArray>().unwrap();
        let cpu_ids = batch.column(7).as_any().downcast_ref::<UInt16Array>().unwrap();
        let durations = batch.column(8).as_any().downcast_ref::<Int64Array>().unwrap();
        
        for i in 0..batch.num_rows() {
            let event_type = match event_types.value(i) {
                "FunctionEntry" => EventType::FunctionEntry,
                "FunctionExit" => EventType::FunctionExit,
                "MemoryAlloc" => EventType::MemoryAlloc,
                "MemoryFree" => EventType::MemoryFree,
                "NetworkSend" => EventType::NetworkSend,
                "NetworkReceive" => EventType::NetworkReceive,
                "FileOpen" => EventType::FileOpen,
                "FileRead" => EventType::FileRead,
                "FileWrite" => EventType::FileWrite,
                "FileClose" => EventType::FileClose,
                "SystemCall" => EventType::SystemCall,
                other => EventType::Custom(other.to_string()),
            };
            
            events.push(ProbeEvent {
                timestamp_ns: timestamps.value(i) as u64,
                probe_id: probe_ids.value(i).to_string(),
                process_id: process_ids.value(i),
                thread_id: thread_ids.value(i),
                function_name: function_names.value(i).to_string(),
                event_type,
                data_payload: data_payloads.value(i).to_vec(),
                stack_trace: None, // TODO: Implement stack trace deserialization
                cpu_id: cpu_ids.value(i),
                duration_ns: if durations.is_null(i) { None } else { Some(durations.value(i) as u64) },
            });
        }
        
        Ok(events)
    }
}
