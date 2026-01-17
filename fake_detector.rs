use std::collections::HashMap;
use std::fs;
use std::path::Path;
use syn::{visit::Visit, File, Item, Expr, Lit};

#[derive(Default)]
struct FakeCodeMetrics {
    total_tokens: usize,
    constants: usize,
    hardcoded_strings: usize,
    mock_patterns: usize,
    functions: usize,
    trivial_functions: usize,
    unwraps: usize,
    error_handlers: usize,
}

struct FakeDetector {
    metrics: FakeCodeMetrics,
    violations: Vec<String>,
}

impl FakeDetector {
    fn new() -> Self {
        Self {
            metrics: FakeCodeMetrics::default(),
            violations: Vec::new(),
        }
    }

    fn analyze_file(&mut self, path: &Path) -> std::io::Result<()> {
        let content = fs::read_to_string(path)?;
        let syntax = syn::parse_file(&content).map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
        })?;

        self.visit_file(&syntax);
        self.check_patterns(&content, path);
        
        Ok(())
    }

    fn check_patterns(&mut self, content: &str, path: &Path) {
        let banned = ["mock", "fake", "demo", "placeholder", "TODO", "FIXME", "stub"];
        
        for pattern in &banned {
            if content.to_lowercase().contains(pattern) {
                self.metrics.mock_patterns += content.matches(pattern).count();
                self.violations.push(format!(
                    "{}:{}: Contains banned pattern '{}'",
                    path.display(), 0, pattern
                ));
            }
        }

        // Check for hardcoded test values
        let test_values = ["42", "123", "test", "example"];
        for val in &test_values {
            if content.contains(val) {
                self.violations.push(format!(
                    "{}:{}: Suspicious hardcoded value '{}'",
                    path.display(), 0, val
                ));
            }
        }
    }

    fn compute_score(&self) -> f64 {
        let mut score = 100.0;

        // Penalize constants
        if self.metrics.total_tokens > 0 {
            let const_ratio = self.metrics.constants as f64 / self.metrics.total_tokens as f64;
            if const_ratio > 0.1 {
                score -= (const_ratio - 0.1) * 500.0;
            }
        }

        // Penalize hardcoded strings
        score -= (self.metrics.hardcoded_strings as f64) * 2.0;

        // Penalize mock patterns (severe)
        score -= (self.metrics.mock_patterns as f64) * 20.0;

        // Penalize trivial functions
        if self.metrics.functions > 0 {
            let trivial_ratio = self.metrics.trivial_functions as f64 / self.metrics.functions as f64;
            score -= trivial_ratio * 50.0;
        }

        // Penalize unwraps without error handling
        if self.metrics.unwraps > self.metrics.error_handlers {
            score -= ((self.metrics.unwraps - self.metrics.error_handlers) as f64) * 5.0;
        }

        score.max(0.0)
    }

    fn is_fake(&self) -> bool {
        self.compute_score() < 70.0 || self.metrics.mock_patterns > 0
    }

    fn report(&self) {
        println!("\n=== Fake Code Detection Report ===\n");
        println!("Metrics:");
        println!("  Total tokens: {}", self.metrics.total_tokens);
        println!("  Constants: {}", self.metrics.constants);
        println!("  Hardcoded strings: {}", self.metrics.hardcoded_strings);
        println!("  Mock patterns: {}", self.metrics.mock_patterns);
        println!("  Functions: {}", self.metrics.functions);
        println!("  Trivial functions: {}", self.metrics.trivial_functions);
        println!("  Unwraps: {}", self.metrics.unwraps);
        println!("  Error handlers: {}", self.metrics.error_handlers);
        println!();
        println!("Score: {:.1}/100", self.compute_score());
        println!("Status: {}", if self.is_fake() { "❌ FAKE" } else { "✅ REAL" });
        
        if !self.violations.is_empty() {
            println!("\nViolations:");
            for v in &self.violations {
                println!("  {}", v);
            }
        }
    }
}

impl<'ast> Visit<'ast> for FakeDetector {
    fn visit_item(&mut self, item: &'ast Item) {
        self.metrics.total_tokens += 1;
        
        match item {
            Item::Const(_) => self.metrics.constants += 1,
            Item::Fn(func) => {
                self.metrics.functions += 1;
                
                // Check if trivial (< 5 statements)
                if let Some(block) = &func.block.as_ref() {
                    if block.stmts.len() < 5 {
                        self.metrics.trivial_functions += 1;
                    }
                }
            }
            _ => {}
        }
        
        syn::visit::visit_item(self, item);
    }

    fn visit_expr(&mut self, expr: &'ast Expr) {
        match expr {
            Expr::Lit(lit) => {
                if matches!(lit.lit, Lit::Str(_)) {
                    self.metrics.hardcoded_strings += 1;
                }
            }
            Expr::MethodCall(call) => {
                if call.method == "unwrap" || call.method == "expect" {
                    self.metrics.unwraps += 1;
                }
            }
            Expr::Try(_) => {
                self.metrics.error_handlers += 1;
            }
            _ => {}
        }
        
        syn::visit::visit_expr(self, expr);
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: fake_detector <path>");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let mut detector = FakeDetector::new();
    let mut total_files = 0;
    let mut fake_files = 0;

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                total_files += 1;
                let mut file_detector = FakeDetector::new();
                
                if let Err(e) = file_detector.analyze_file(&path) {
                    eprintln!("Error analyzing {}: {}", path.display(), e);
                    continue;
                }
                
                if file_detector.is_fake() {
                    fake_files += 1;
                    println!("\n❌ FAKE: {}", path.display());
                    file_detector.report();
                }
            }
        }
    } else {
        total_files = 1;
        detector.analyze_file(path)?;
        detector.report();
        
        if detector.is_fake() {
            fake_files = 1;
        }
    }

    println!("\n=== Summary ===");
    println!("Total files: {}", total_files);
    println!("Fake files: {}", fake_files);
    println!("Real files: {}", total_files - fake_files);
    println!("Fake ratio: {:.1}%", (fake_files as f64 / total_files as f64) * 100.0);

    if fake_files > 0 {
        println!("\n❌ POLICY VIOLATION: Fake code detected");
        std::process::exit(1);
    } else {
        println!("\n✅ POLICY COMPLIANT: No fake code detected");
        std::process::exit(0);
    }
}
