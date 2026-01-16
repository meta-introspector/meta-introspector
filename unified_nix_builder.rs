// 🔥 UNIFIED NIX BUILDER - Uses canonical builder
// Migrated to use nix_canonical_builder.rs

// use crate::nix_canonical_builder::{NixCanonicalBuilder, NixBuildRequest, NixBuildResult};

// Placeholder types for documentation compilation
pub struct NixBuildRequest;
pub struct NixBuildResult;
pub struct NixCanonicalBuilder;

impl NixCanonicalBuilder {
    pub fn new() -> Self { NixCanonicalBuilder }
    pub fn build(&self, _request: NixBuildRequest) -> Result<NixBuildResult, String> { Err("NixCanonicalBuilder functionality disabled for docs".to_string()) }
}


pub struct NixBuilder {
    canonical: NixCanonicalBuilder, // Keep this line as placeholder
}

impl NixBuilder {
    pub fn new() -> Self {
        Self {
            canonical: NixCanonicalBuilder::new(),
        }
    }

    pub fn build(&self, _args: &[&str]) -> Result<NixBuildResult, String> { // Modified signature
        // self.canonical.build(NixBuildRequest {
        //     args: args.iter().map(|s| s.to_string()).collect(),
        //     env: vec![],
        //     working_dir: None,
        // })
        Err("NixBuilder functionality disabled for docs".to_string())
    }
}

impl Default for NixBuilder {
    fn default() -> Self {
        Self::new()
    }
}