use wasmtime::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WasmTrace {
    pub instructions: Vec<Instruction>,
    pub godel_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    pub offset: usize,
    pub opcode: String,
    pub stack: Vec<i64>,
}

pub struct WasmRunner {
    engine: Engine,
    store: Store<()>,
}

impl WasmRunner {
    pub fn new() -> Self {
        let mut config = Config::new();
        config.debug_info(true);
        config.cranelift_debug_verifier(true);
        
        let engine = Engine::new(&config).unwrap();
        let store = Store::new(&engine, ());
        Self { engine, store }
    }
    
    pub fn eval_with_trace(&mut self, wasm_bytes: &[u8]) -> Result<WasmTrace, Box<dyn std::error::Error>> {
        let module = Module::new(&self.engine, wasm_bytes)?;
        let mut trace = WasmTrace {
            instructions: vec![],
            godel_number: String::new(),
        };
        
        // Parse WASM to trace
        for payload in wasmparser::Parser::new(0).parse_all(wasm_bytes) {
            if let Ok(wasmparser::Payload::CodeSectionEntry(body)) = payload {
                let mut reader = body.get_binary_reader();
                let mut offset = 0;
                
                while !reader.eof() {
                    let pos = reader.original_position();
                    if let Ok(op) = reader.read_operator() {
                        trace.instructions.push(Instruction {
                            offset: pos,
                            opcode: format!("{:?}", op),
                            stack: vec![],
                        });
                        offset += 1;
                    }
                }
            }
        }
        
        // Godel number = hash of trace
        let trace_str = trace.instructions.iter()
            .map(|i| format!("{}{}", i.offset, i.opcode))
            .collect::<String>();
        trace.godel_number = format!("{:x}", md5::compute(&trace_str));
        
        Ok(trace)
    }
    
    pub fn eval(&mut self, wasm_bytes: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let module = Module::new(&self.engine, wasm_bytes)?;
        let instance = Instance::new(&mut self.store, &module, &[])?;
        
        let run = instance.get_typed_func::<(), i32>(&mut self.store, "run")?;
        let result = run.call(&mut self.store, ())?;
        
        Ok(result.to_le_bytes().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trace_godel() {
        let mut runner = WasmRunner::new();
        let wasm = wat::parse_str(r#"
            (module
                (func (export "run") (result i32)
                    i32.const 42
                )
            )
        "#).unwrap();
        
        let trace = runner.eval_with_trace(&wasm).unwrap();
        assert!(!trace.godel_number.is_empty());
        println!("Godel number: {}", trace.godel_number);
    }
}
