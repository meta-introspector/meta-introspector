// Git pack file scanner
// Reads compressed git objects at byte level

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::collections::HashMap;

#[derive(Debug)]
pub struct PackHeader {
    pub signature: [u8; 4],
    pub version: u32,
    pub num_objects: u32,
}

#[derive(Debug)]
pub struct PackObject {
    pub obj_type: u8,
    pub size: usize,
    pub offset: usize,
    pub data: Vec<u8>,
}

pub fn read_pack_header(file: &mut File) -> std::io::Result<PackHeader> {
    let mut sig = [0u8; 4];
    file.read_exact(&mut sig)?;
    
    let mut ver_bytes = [0u8; 4];
    file.read_exact(&mut ver_bytes)?;
    let version = u32::from_be_bytes(ver_bytes);
    
    let mut num_bytes = [0u8; 4];
    file.read_exact(&mut num_bytes)?;
    let num_objects = u32::from_be_bytes(num_bytes);
    
    Ok(PackHeader {
        signature: sig,
        version,
        num_objects,
    })
}

pub fn read_pack_object(file: &mut File, offset: usize) -> std::io::Result<PackObject> {
    file.seek(SeekFrom::Start(offset as u64))?;
    
    // Read type and size (variable length encoding)
    let mut byte = [0u8; 1];
    file.read_exact(&mut byte)?;
    
    let obj_type = (byte[0] >> 4) & 0x07;
    let mut size = (byte[0] & 0x0f) as usize;
    let mut shift = 4;
    
    while byte[0] & 0x80 != 0 {
        file.read_exact(&mut byte)?;
        size |= ((byte[0] & 0x7f) as usize) << shift;
        shift += 7;
    }
    
    // Read compressed data
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    
    Ok(PackObject {
        obj_type,
        size,
        offset,
        data,
    })
}

pub fn decompress_object(data: &[u8]) -> Result<Vec<u8>, String> {
    use flate2::read::ZlibDecoder;
    use std::io::Read;
    
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)
        .map_err(|e| format!("Decompression failed: {}", e))?;
    
    Ok(decompressed)
}

pub fn find_byte_pattern(data: &[u8], pattern: &[u8]) -> Vec<usize> {
    let mut positions = Vec::new();
    
    for i in 0..data.len().saturating_sub(pattern.len()) {
        if &data[i..i + pattern.len()] == pattern {
            positions.push(i);
        }
    }
    
    positions
}

pub fn discover_patterns(data: &[u8], min_freq: usize) -> HashMap<Vec<u8>, usize> {
    let mut patterns = HashMap::new();
    
    // Scan for 2-16 byte patterns
    for window_size in 2..=16 {
        for window in data.windows(window_size) {
            *patterns.entry(window.to_vec()).or_insert(0) += 1;
        }
    }
    
    // Filter by frequency
    patterns.retain(|_, &mut count| count >= min_freq);
    patterns
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pattern_discovery() {
        let data = b"fn main() { fn test() { fn foo() {";
        let patterns = discover_patterns(data, 2);
        
        assert!(patterns.contains_key(b"fn ".as_slice()));
    }
}
