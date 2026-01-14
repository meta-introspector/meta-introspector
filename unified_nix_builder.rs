// 🔥 UNIFIED NIX BUILDER - Uses canonical builder
// Migrated to use nix_canonical_builder.rs

use crate::nix_canonical_builder::{NixCanonicalBuilder, NixBuildRequest, NixBuildResult};

pub struct NixBuilder {
    canonical: NixCanonicalBuilder,
}

impl NixBuilder {
    pub fn new() -> Self {
        Self {
            canonical: NixCanonicalBuilder::new(),
        }
    }

    pub fn build(&self, args: &[&str]) -> Result<NixBuildResult, String> {
        self.canonical.build(NixBuildRequest {
            args: args.iter().map(|s| s.to_string()).collect(),
            env: vec![],
            working_dir: None,
        })
    }
}

impl Default for NixBuilder {
    fn default() -> Self {
        Self::new()
    }
}
