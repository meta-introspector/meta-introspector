// Auto-generated instrumented wrappers will be included here

include!(concat!(env!("OUT_DIR"), "/instrumented_wrappers.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wrapper_generation() {
        // Wrappers are generated at compile time
        println!("Instrumented wrappers loaded!");
    }
}
