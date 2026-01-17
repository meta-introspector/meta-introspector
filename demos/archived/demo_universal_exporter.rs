// Demo: Universal Exporter

mod universal_exporter;
use universal_exporter::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📤 UNIVERSAL EXPORTER");
    println!("====================\n");
    println!("Export nix store to: GitHub Artifacts, Archive.org, Docker, HuggingFace\n");
    
    let exporter = UniversalExporter::new();
    
    // Example: Export mining results
    let nix_path = "/nix/store/abc123-branch-mining/";
    
    println!("📦 Creating export task for: {}", nix_path);
    let task = create_export_task("branch-mining", nix_path);
    
    println!("\n🎯 Export Targets:");
    for target in &task.targets {
        match target {
            ExportTarget::GitHubArtifact { repo, .. } => {
                println!("  • GitHub Artifacts: {}", repo);
            }
            ExportTarget::ArchiveOrg { identifier, .. } => {
                println!("  • Archive.org: https://archive.org/details/{}", identifier);
            }
            ExportTarget::DockerImage { registry, image, tag } => {
                println!("  • Docker: {}/{}:{}", registry, image, tag);
            }
            ExportTarget::HuggingFace { dataset, .. } => {
                println!("  • HuggingFace: https://huggingface.co/datasets/{}", dataset);
            }
        }
    }
    
    // Export to all targets
    println!("\n🚀 Exporting to all targets...");
    exporter.export(&task).await?;
    
    // Create GitHub workflow
    println!("\n📝 Creating GitHub workflow...");
    exporter.create_github_workflow(".github/workflows/export-nix-store.yml")?;
    
    // Create Dockerfile
    println!("🐳 Creating Dockerfile...");
    exporter.create_dockerfile(nix_path, "Dockerfile.nix-store")?;
    
    // Create Archive.org metadata
    println!("🏛️  Creating Archive.org metadata...");
    let metadata = exporter.create_archive_metadata("meta-introspector-mining");
    println!("{}", serde_json::to_string_pretty(&metadata)?);
    
    println!("\n✅ Export Complete!");
    
    println!("\n💡 Usage:");
    println!("  # GitHub Artifacts");
    println!("  gh run download <run-id> -n nix-store-export");
    
    println!("\n  # Archive.org");
    println!("  ia download meta-introspector-mining");
    
    println!("\n  # Docker");
    println!("  docker pull ghcr.io/meta-introspector/branch-mining:latest");
    println!("  docker run -it ghcr.io/meta-introspector/branch-mining:latest");
    
    println!("\n  # HuggingFace");
    println!("  from datasets import load_dataset");
    println!("  ds = load_dataset('introspector/rust/branch-mining')");
    
    println!("\n🌐 All nix store exports are now publicly accessible!");
    
    Ok(())
}
