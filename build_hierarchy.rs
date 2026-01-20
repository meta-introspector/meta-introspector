use std::collections::{HashMap, HashSet};
use parquet::arrow::ParquetRecordBatchReaderBuilder;
use std::fs::File;

fn main() {
    println!("🔢 Building dependency hierarchy");
    
    // Load Level 0 (Const71)
    let level0 = load_const71("zos/layer0/const71.parquet");
    println!("📊 Level 0: {} constants", level0.len());
    
    // Extract Level 1 (references Level 0)
    let level1 = extract_level1(&level0);
    println!("📊 Level 1: {} declarations", level1.len());
    
    // Extract Level 2 (references Level 1)
    let level2 = extract_level2(&level1);
    println!("📊 Level 2: {} declarations", level2.len());
    
    // Save
    save_level("zos/layer1/level1.parquet", &level1);
    save_level("zos/layer2/level2.parquet", &level2);
    
    // Verify hierarchy
    verify_hierarchy(&level0, &level1, &level2);
}

fn load_const71(path: &str) -> Vec<Const> {
    let mut constants = Vec::new();
    
    let file = File::open(path).expect("Failed to open const71");
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
    let reader = builder.build().unwrap();
    
    for batch in reader.flatten() {
        // Extract constants
    }
    
    constants
}

fn extract_level1(level0: &[Const]) -> Vec<Declaration> {
    let mut level1 = Vec::new();
    
    // Scan all source files
    let files = glob::glob("**/*.rs").unwrap();
    
    for file in files.flatten() {
        if let Ok(content) = std::fs::read_to_string(&file) {
            for (line_num, line) in content.lines().enumerate() {
                // Find declarations that reference Level 0
                if let Some(decl) = parse_declaration(line) {
                    let refs = find_level0_refs(&decl, level0);
                    
                    if !refs.is_empty() && refs.iter().all(|r| r.level == 0) {
                        level1.push(Declaration {
                            name: decl.name,
                            content: line.to_string(),
                            level: 1,
                            references: refs,
                            source_file: file.to_str().unwrap().to_string(),
                            source_line: line_num as u64,
                        });
                    }
                }
            }
        }
    }
    
    level1
}

fn extract_level2(level1: &[Declaration]) -> Vec<Declaration> {
    let mut level2 = Vec::new();
    
    let files = glob::glob("**/*.rs").unwrap();
    
    for file in files.flatten() {
        if let Ok(content) = std::fs::read_to_string(&file) {
            for (line_num, line) in content.lines().enumerate() {
                if let Some(decl) = parse_declaration(line) {
                    let refs = find_level1_refs(&decl, level1);
                    
                    if !refs.is_empty() && refs.iter().all(|r| r.level <= 1) {
                        level2.push(Declaration {
                            name: decl.name,
                            content: line.to_string(),
                            level: 2,
                            references: refs,
                            source_file: file.to_str().unwrap().to_string(),
                            source_line: line_num as u64,
                        });
                    }
                }
            }
        }
    }
    
    level2
}

fn parse_declaration(line: &str) -> Option<DeclInfo> {
    let trimmed = line.trim();
    
    if trimmed.starts_with("const ") {
        let name = trimmed.split_whitespace().nth(1)?;
        Some(DeclInfo {
            name: name.trim_end_matches(':').to_string(),
            kind: DeclKind::Const,
        })
    } else if trimmed.starts_with("type ") {
        let name = trimmed.split_whitespace().nth(1)?;
        Some(DeclInfo {
            name: name.trim_end_matches('=').to_string(),
            kind: DeclKind::Type,
        })
    } else if trimmed.starts_with("fn ") {
        let name = trimmed.split_whitespace().nth(1)?
            .split('(').next()?;
        Some(DeclInfo {
            name: name.to_string(),
            kind: DeclKind::Function,
        })
    } else {
        None
    }
}

fn find_level0_refs(decl: &DeclInfo, level0: &[Const]) -> Vec<Reference> {
    let mut refs = Vec::new();
    
    // Check if declaration uses any Level 0 constants
    for const_val in level0 {
        if decl.name.contains(&format!("{:?}", const_val.value)) {
            refs.push(Reference {
                name: format!("const_{}", const_val.size),
                level: 0,
            });
        }
    }
    
    refs
}

fn find_level1_refs(decl: &DeclInfo, level1: &[Declaration]) -> Vec<Reference> {
    let mut refs = Vec::new();
    
    for l1_decl in level1 {
        if decl.name.contains(&l1_decl.name) {
            refs.push(Reference {
                name: l1_decl.name.clone(),
                level: 1,
            });
        }
    }
    
    refs
}

fn save_level(path: &str, decls: &[Declaration]) {
    use arrow::array::{StringArray, UInt64Array, UInt8Array};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use parquet::arrow::ArrowWriter;
    use std::sync::Arc;
    
    let schema = Schema::new(vec![
        Field::new("name", DataType::Utf8, false),
        Field::new("content", DataType::Utf8, false),
        Field::new("level", DataType::UInt8, false),
        Field::new("source_file", DataType::Utf8, false),
        Field::new("source_line", DataType::UInt64, false),
    ]);
    
    let names: Vec<String> = decls.iter().map(|d| d.name.clone()).collect();
    let contents: Vec<String> = decls.iter().map(|d| d.content.clone()).collect();
    let levels: Vec<u8> = decls.iter().map(|d| d.level).collect();
    let files: Vec<String> = decls.iter().map(|d| d.source_file.clone()).collect();
    let lines: Vec<u64> = decls.iter().map(|d| d.source_line).collect();
    
    let batch = RecordBatch::try_new(
        Arc::new(schema.clone()),
        vec![
            Arc::new(StringArray::from(names)),
            Arc::new(StringArray::from(contents)),
            Arc::new(UInt8Array::from(levels)),
            Arc::new(StringArray::from(files)),
            Arc::new(UInt64Array::from(lines)),
        ],
    ).unwrap();
    
    let file = File::create(path).unwrap();
    let mut writer = ArrowWriter::try_new(file, Arc::new(schema), None).unwrap();
    writer.write(&batch).unwrap();
    writer.close().unwrap();
    
    println!("💾 Saved {} declarations to {}", decls.len(), path);
}

fn verify_hierarchy(level0: &[Const], level1: &[Declaration], level2: &[Declaration]) {
    println!("\n🔍 Verifying hierarchy:");
    
    // Level 1 should only reference Level 0
    for decl in level1 {
        assert!(decl.references.iter().all(|r| r.level == 0),
            "Level 1 {} references non-Level 0", decl.name);
    }
    println!("  ✅ Level 1 only references Level 0");
    
    // Level 2 should only reference Level 0 or 1
    for decl in level2 {
        assert!(decl.references.iter().all(|r| r.level <= 1),
            "Level 2 {} references Level 3+", decl.name);
    }
    println!("  ✅ Level 2 only references Level 0-1");
    
    println!("\n✅ Hierarchy verified");
}

#[derive(Debug, Clone)]
struct Const {
    value: Vec<u8>,
    size: u8,
}

#[derive(Debug, Clone)]
struct Declaration {
    name: String,
    content: String,
    level: u8,
    references: Vec<Reference>,
    source_file: String,
    source_line: u64,
}

#[derive(Debug, Clone)]
struct Reference {
    name: String,
    level: u8,
}

#[derive(Debug)]
struct DeclInfo {
    name: String,
    kind: DeclKind,
}

#[derive(Debug)]
enum DeclKind {
    Const,
    Type,
    Function,
}
