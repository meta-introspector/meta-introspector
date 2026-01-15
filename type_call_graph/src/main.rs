use goblin::elf::{Elf, sym::STT_FUNC};
use capstone::prelude::*;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::Field;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CallEdge {
    caller: String,
    callee: String,
    caller_addr: u64,
    target_addr: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct FunctionInfo {
    name: String,
    address: u64,
    size: u64,
    type_patterns: Vec<String>,
    strings_used: Vec<String>,
    calls_to: Vec<String>,
    called_by: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TypeCallGraph {
    functions: HashMap<String, FunctionInfo>,
    edges: Vec<CallEdge>,
    stats: GraphStats,
}

#[derive(Debug, Serialize, Deserialize)]
struct GraphStats {
    total_functions: usize,
    type_functions: usize,
    total_edges: usize,
    type_edges: usize,
}

fn main() -> Result<()> {
    println!("🔍 Building Call Graph for AST/HIR/MIR Type Functions\n");
    
    let binary_path = std::env::args().nth(1)
        .unwrap_or_else(|| "/nix/store/eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee-rustc-1.83.0/lib/librustc_driver.so".to_string());
    
    println!("Reading binary: {}", binary_path);
    let binary = fs::read(&binary_path)?;
    let elf = Elf::parse(&binary)?;
    
    // Phase 1: Load type functions from parquet
    println!("\nPhase 1: Loading type functions from parquet...");
    let type_patterns = ["Expr", "Stmt", "Item", "Pat", "Ty", "Block", "Hir", "Mir", "Thir", 
                         "ast::", "hir::", "mir::", "thir::", "Body", "Def", "Res", "Node",
                         "Generic", "Trait", "Impl"];
    
    let type_functions = load_type_functions(&type_patterns)?;
    println!("  Found {} functions with type patterns", type_functions.len());
    
    // Phase 2: Build function address map
    println!("\nPhase 2: Building function symbol map...");
    let mut funcs: BTreeMap<u64, String> = BTreeMap::new();
    let mut func_sizes: HashMap<u64, u64> = HashMap::new();
    
    for sym in elf.syms.iter() {
        if sym.st_type() == STT_FUNC && sym.st_size > 0 {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                if !name.is_empty() {
                    funcs.insert(sym.st_value, name.to_string());
                    func_sizes.insert(sym.st_value, sym.st_size);
                }
            }
        }
    }
    println!("  Found {} total functions", funcs.len());
    
    // Phase 3: Disassemble and find calls
    println!("\nPhase 3: Disassembling and extracting calls...");
    let text_sec = elf.section_headers.iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name) == Some(".text"))
        .ok_or_else(|| anyhow::anyhow!(".text section not found"))?;
    
    let text_off = text_sec.sh_offset as usize;
    let text_addr = text_sec.sh_addr;
    let text_size = text_sec.sh_size as usize;
    let text_bytes = &binary[text_off..text_off + text_size];
    
    let cs = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode64)
        .detail(true)
        .build()?;
    
    let mut edges: Vec<CallEdge> = Vec::new();
    let insns = cs.disasm_all(text_bytes, text_addr)?;
    
    for insn in insns.iter() {
        if insn.id().0 == capstone::arch::x86::X86Insn::X86_INS_CALL as u32 {
            let src_addr = insn.address();
            
            let caller_name = funcs.range(..=src_addr)
                .next_back()
                .map(|(_, n)| n.clone())
                .unwrap_or_else(|| format!("sub_{:x}", src_addr));
            
            if let Ok(detail) = cs.insn_detail(&insn) {
                let arch_detail = detail.arch_detail();
                let ops = arch_detail.operands();
                
                for op in ops {
                    if let capstone::arch::ArchOperand::X86Operand(x86_op) = op {
                        if let capstone::arch::x86::X86OperandType::Imm(imm) = x86_op.op_type {
                            let target = imm as u64;
                            
                            let callee_name = funcs.range(..=target)
                                .next_back()
                                .filter(|(start, _)| {
                                    let size = *func_sizes.get(start).unwrap_or(&0x1000);
                                    target >= **start && target < **start + size
                                })
                                .map(|(_, n)| n.clone())
                                .unwrap_or_else(|| format!("sub_{:x}", target));
                            
                            edges.push(CallEdge {
                                caller: caller_name.clone(),
                                callee: callee_name,
                                caller_addr: src_addr,
                                target_addr: target,
                            });
                        }
                    }
                }
            }
        }
    }
    
    println!("  Found {} call edges", edges.len());
    
    // Phase 4: Build graph focused on type functions
    println!("\nPhase 4: Building type function graph...");
    let mut graph_funcs: HashMap<String, FunctionInfo> = HashMap::new();
    
    for (name, info) in &type_functions {
        let addr = funcs.iter()
            .find(|(_, n)| n == &name)
            .map(|(a, _)| *a)
            .unwrap_or(0);
        
        let size = func_sizes.get(&addr).copied().unwrap_or(0);
        
        graph_funcs.insert(name.clone(), FunctionInfo {
            name: name.clone(),
            address: addr,
            size,
            type_patterns: info.type_patterns.clone(),
            strings_used: info.strings_used.clone(),
            calls_to: Vec::new(),
            called_by: Vec::new(),
        });
    }
    
    // Add edges involving type functions
    let mut type_edges = Vec::new();
    for edge in &edges {
        let caller_is_type = graph_funcs.contains_key(&edge.caller);
        let callee_is_type = graph_funcs.contains_key(&edge.callee);
        
        if caller_is_type || callee_is_type {
            type_edges.push(edge.clone());
            
            if caller_is_type {
                if let Some(func) = graph_funcs.get_mut(&edge.caller) {
                    if !func.calls_to.contains(&edge.callee) {
                        func.calls_to.push(edge.callee.clone());
                    }
                }
            }
            
            if callee_is_type {
                if let Some(func) = graph_funcs.get_mut(&edge.callee) {
                    if !func.called_by.contains(&edge.caller) {
                        func.called_by.push(edge.caller.clone());
                    }
                }
            }
        }
    }
    
    let graph = TypeCallGraph {
        stats: GraphStats {
            total_functions: funcs.len(),
            type_functions: graph_funcs.len(),
            total_edges: edges.len(),
            type_edges: type_edges.len(),
        },
        functions: graph_funcs,
        edges: type_edges,
    };
    
    println!("\n📊 Graph Statistics:");
    println!("  Total functions: {}", graph.stats.total_functions);
    println!("  Type functions: {}", graph.stats.type_functions);
    println!("  Total call edges: {}", graph.stats.total_edges);
    println!("  Type-related edges: {}", graph.stats.type_edges);
    
    // Save graph
    let json = serde_json::to_string_pretty(&graph)?;
    fs::write("../type_call_graph.json", json)?;
    println!("\n✅ Saved to: type_call_graph.json");
    
    // Show top connected functions
    println!("\n🔗 Top 10 Most Connected Type Functions:");
    let mut func_list: Vec<_> = graph.functions.values().collect();
    func_list.sort_by_key(|f| std::cmp::Reverse(f.calls_to.len() + f.called_by.len()));
    
    for (i, func) in func_list.iter().take(10).enumerate() {
        println!("\n{}. {} (0x{:x})", i+1, &func.name[..func.name.len().min(80)], func.address);
        println!("   Patterns: {:?}", func.type_patterns);
        println!("   Calls {} functions, called by {}", func.calls_to.len(), func.called_by.len());
        if !func.strings_used.is_empty() {
            println!("   Strings: {:?}", &func.strings_used[..func.strings_used.len().min(2)]);
        }
    }
    
    Ok(())
}

fn load_type_functions(patterns: &[&str]) -> Result<HashMap<String, TypeFunctionInfo>> {
    let file = fs::File::open("../string_usage.parquet")?;
    let reader = SerializedFileReader::new(file)?;
    
    let mut functions: HashMap<String, TypeFunctionInfo> = HashMap::new();
    
    for i in 0..reader.num_row_groups() {
        let row_group = reader.get_row_group(i)?;
        let rows = row_group.get_row_iter(None)?;
        
        for row_result in rows {
            let row = row_result?;
            let mut string_val = String::new();
            let mut func_name = String::new();
            
            for (name, field) in row.get_column_iter() {
                if let Field::Str(s) = field {
                    if name == "string_value" {
                        string_val = s.to_string();
                    } else if name == "function_name" {
                        func_name = s.to_string();
                    }
                }
            }
            
            let mut matched_patterns = Vec::new();
            for pattern in patterns {
                if func_name.contains(pattern) {
                    matched_patterns.push(pattern.to_string());
                }
            }
            
            if !matched_patterns.is_empty() {
                let entry = functions.entry(func_name.clone()).or_insert_with(|| {
                    TypeFunctionInfo {
                        type_patterns: matched_patterns.clone(),
                        strings_used: Vec::new(),
                    }
                });
                
                if !string_val.is_empty() && !entry.strings_used.contains(&string_val) {
                    entry.strings_used.push(string_val);
                }
            }
        }
    }
    
    Ok(functions)
}

#[derive(Debug)]
struct TypeFunctionInfo {
    type_patterns: Vec<String>,
    strings_used: Vec<String>,
}
