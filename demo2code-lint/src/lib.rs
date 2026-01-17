#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

use rustc_ast::ast;
use rustc_hir as hir;
use rustc_lint::{EarlyContext, EarlyLintPass, LateContext, LateLintPass, LintContext};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::symbol::Symbol;

// Lint: Detect "demo", "mock", "fake" in names
declare_lint! {
    pub DEMO_CODE,
    Deny,
    "detects demo, mock, or fake code patterns"
}

declare_lint_pass!(Demo2CodeLint => [DEMO_CODE]);

impl EarlyLintPass for Demo2CodeLint {
    fn check_item(&mut self, cx: &EarlyContext<'_>, item: &ast::Item) {
        let name = item.ident.name.as_str().to_lowercase();
        
        let banned = ["demo", "mock", "fake", "stub", "placeholder", "test"];
        for pattern in &banned {
            if name.contains(pattern) {
                cx.lint(
                    DEMO_CODE,
                    format!("Demo2Code violation: '{}' contains banned pattern '{}'", item.ident.name, pattern),
                    |lint| lint.set_span(item.span)
                );
            }
        }
    }
    
    fn check_expr(&mut self, cx: &EarlyContext<'_>, expr: &ast::Expr) {
        // Check for hardcoded test values
        if let ast::ExprKind::Lit(lit) = &expr.kind {
            if let ast::LitKind::Int(val, _) = lit.kind {
                if val == 42 || val == 123 {
                    cx.lint(
                        DEMO_CODE,
                        format!("Demo2Code violation: hardcoded test value '{}'", val),
                        |lint| lint.set_span(expr.span)
                    );
                }
            }
            
            if let ast::LitKind::Str(s, _) = lit.kind {
                let str_val = s.as_str().to_lowercase();
                if str_val.contains("test") || str_val.contains("example") || str_val.contains("placeholder") {
                    cx.lint(
                        DEMO_CODE,
                        format!("Demo2Code violation: suspicious string '{}'", s),
                        |lint| lint.set_span(expr.span)
                    );
                }
            }
        }
    }
}

// Lint: Detect trivial functions
declare_lint! {
    pub TRIVIAL_FUNCTION,
    Warn,
    "detects functions that are too simple (< 5 statements)"
}

declare_lint_pass!(TrivialFunctionLint => [TRIVIAL_FUNCTION]);

impl EarlyLintPass for TrivialFunctionLint {
    fn check_item(&mut self, cx: &EarlyContext<'_>, item: &ast::Item) {
        if let ast::ItemKind::Fn(box ast::Fn { body: Some(body), .. }) = &item.kind {
            let stmt_count = count_statements(body);
            
            if stmt_count < 5 && !item.ident.name.as_str().starts_with("test_") {
                cx.lint(
                    TRIVIAL_FUNCTION,
                    format!("Function '{}' has only {} statements (minimum 5 required)", item.ident.name, stmt_count),
                    |lint| lint.set_span(item.span)
                );
            }
        }
    }
}

fn count_statements(block: &ast::Block) -> usize {
    block.stmts.len()
}

// Lint: Detect missing error handling
declare_lint! {
    pub MISSING_ERROR_HANDLING,
    Warn,
    "detects unwrap() without proper error handling"
}

declare_lint_pass!(ErrorHandlingLint => [MISSING_ERROR_HANDLING]);

impl EarlyLintPass for ErrorHandlingLint {
    fn check_expr(&mut self, cx: &EarlyContext<'_>, expr: &ast::Expr) {
        if let ast::ExprKind::MethodCall(call) = &expr.kind {
            let method_name = call.seg.ident.name.as_str();
            
            if method_name == "unwrap" || method_name == "expect" {
                cx.lint(
                    MISSING_ERROR_HANDLING,
                    "Use '?' operator instead of unwrap()/expect() for better error handling",
                    |lint| lint.set_span(expr.span)
                );
            }
        }
    }
}

// Lint: Detect excessive constants
declare_lint! {
    pub EXCESSIVE_CONSTANTS,
    Warn,
    "detects too many constant definitions (should use config)"
}

declare_lint_pass!(ConstantLint => [EXCESSIVE_CONSTANTS]);

impl EarlyLintPass for ConstantLint {
    fn check_crate(&mut self, cx: &EarlyContext<'_>, krate: &ast::Crate) {
        let mut const_count = 0;
        let mut total_items = 0;
        
        for item in &krate.items {
            total_items += 1;
            if matches!(item.kind, ast::ItemKind::Const(..)) {
                const_count += 1;
            }
        }
        
        if total_items > 0 {
            let ratio = const_count as f64 / total_items as f64;
            if ratio > 0.1 {
                cx.lint(
                    EXCESSIVE_CONSTANTS,
                    format!("Too many constants: {}/{} ({:.1}% > 10%)", const_count, total_items, ratio * 100.0),
                    |lint| lint
                );
            }
        }
    }
}

#[no_mangle]
pub fn register_plugins(reg: &mut rustc_lint::LintStore) {
    reg.register_early_pass(|| Box::new(Demo2CodeLint));
    reg.register_early_pass(|| Box::new(TrivialFunctionLint));
    reg.register_early_pass(|| Box::new(ErrorHandlingLint));
    reg.register_early_pass(|| Box::new(ConstantLint));
}
