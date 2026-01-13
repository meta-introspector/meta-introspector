use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BorrowCheckerSauce {
    pub flavor: SauceType,
    pub intensity: f64,
    pub meme_density: f64,
    pub injection_points: Vec<InjectionPoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SauceType {
    Classic,      // Original borrow checker
    Spicy,        // Extra strict
    Mild,         // Relaxed rules  
    Sweet,        // GC-like
    Umami,        // Reference counting
    None,         // Raw pointers only
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InjectionPoint {
    pub location: String,
    pub sauce_amount: f64,
    pub meme_coefficient: f64,
}

pub struct SauceInjector;

impl SauceInjector {
    pub fn inject_borrow_sauce(
        raw_rust: &str, 
        sauce: &BorrowCheckerSauce
    ) -> Result<String> {
        match sauce.flavor {
            SauceType::None => Ok(raw_rust.replace("&", "*").replace("mut ", "mut_ptr ")),
            SauceType::Classic => Ok(raw_rust.to_string()), // No change
            SauceType::Spicy => Self::add_extra_lifetimes(raw_rust),
            SauceType::Mild => Self::remove_some_borrows(raw_rust),
            SauceType::Sweet => Self::add_gc_annotations(raw_rust),
            SauceType::Umami => Self::add_rc_everywhere(raw_rust),
        }
    }
    
    fn add_extra_lifetimes(code: &str) -> Result<String> {
        Ok(code.replace("fn ", "fn <'a, 'b, 'c> "))
    }
    
    fn remove_some_borrows(code: &str) -> Result<String> {
        Ok(code.replace("&mut ", "").replace("&", ""))
    }
    
    fn add_gc_annotations(code: &str) -> Result<String> {
        Ok(format!("#[gc]\n{}", code))
    }
    
    fn add_rc_everywhere(code: &str) -> Result<String> {
        Ok(code.replace("String", "Rc<String>").replace("Vec<", "Rc<Vec<"))
    }
    
    pub fn detect_sauce_level(code: &str) -> BorrowCheckerSauce {
        let borrow_count = code.matches('&').count() as f64;
        let lifetime_count = code.matches('\'').count() as f64;
        let unsafe_count = code.matches("unsafe").count() as f64;
        
        let intensity = (borrow_count + lifetime_count) / code.len() as f64;
        let meme_density = if unsafe_count > 0 { 0.0 } else { intensity };
        
        let flavor = match intensity {
            x if x > 0.1 => SauceType::Spicy,
            x if x > 0.05 => SauceType::Classic,
            x if x > 0.01 => SauceType::Mild,
            _ => SauceType::None,
        };
        
        BorrowCheckerSauce {
            flavor,
            intensity,
            meme_density,
            injection_points: vec![],
        }
    }
}

pub struct MemeSauceAnalyzer;

impl MemeSauceAnalyzer {
    pub fn compute_meme_purity(sauce: &BorrowCheckerSauce) -> f64 {
        // How much of the "safety" is just elaborate meme vs actual utility
        match sauce.flavor {
            SauceType::None => 0.0,        // Pure utility, no meme
            SauceType::Mild => 0.2,        // Mostly utility
            SauceType::Classic => 0.6,     // Balanced meme/utility
            SauceType::Spicy => 0.9,       // Mostly meme
            SauceType::Sweet => 0.95,      // Almost pure meme
            SauceType::Umami => 0.8,       // Heavy meme content
        }
    }
    
    pub fn sauce_removal_safety(code: &str) -> f64 {
        // How safe is it to remove the borrow checker sauce?
        let ptr_operations = code.matches("*").count();
        let array_access = code.matches("[").count();
        let unsafe_blocks = code.matches("unsafe").count();
        
        let danger_score = (ptr_operations + array_access) as f64 / code.len() as f64;
        
        if unsafe_blocks > 0 {
            0.0 // Already unsafe, sauce removal is safe
        } else {
            1.0 - danger_score.min(1.0)
        }
    }
}
