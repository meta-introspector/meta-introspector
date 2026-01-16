// Demo: Self-Deploying System

mod self_deploying_system;
use self_deploying_system::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 SELF-DEPLOYING SYSTEM");
    println!("========================\n");
    println!("QA → Local Prod → Oracle OCI\n");
    
    let system = SelfDeployingSystem::new();
    let configs = create_deployment_configs();
    
    println!("📋 Deployment Pipeline:");
    println!("  1. Build with nix");
    println!("  2. Deploy to QA (systemd)");
    println!("  3. Run health checks");
    println!("  4. Auto-promote to Local Prod");
    println!("  5. Prepare OCI deployment");
    println!("  6. Generate terraform config\n");
    
    // Deploy all services to QA
    for config in &configs {
        println!("🔧 Deploying: {}", config.name);
        println!("  Nix path: {}", config.nix_store_path);
        println!("  Auto-promote: {}\n", config.auto_promote);
        
        // Deploy to QA
        system.deploy_qa(config).await?;
        
        println!();
    }
    
    println!("✅ All services deployed!");
    
    println!("\n📊 Service Status:");
    println!("  QA Services:");
    println!("    • meta-introspector-qa.service");
    println!("    • zos-qa.service");
    println!("    • llm-proxy-qa.service");
    
    println!("\n  Prod Services:");
    println!("    • meta-introspector-prod.service");
    println!("    • zos-prod.service");
    
    println!("\n💡 Commands:");
    println!("  # Check status");
    println!("  systemctl status meta-introspector-qa");
    println!("  systemctl status meta-introspector-prod");
    
    println!("\n  # View logs");
    println!("  journalctl -u meta-introspector-qa -f");
    println!("  journalctl -u meta-introspector-prod -f");
    
    println!("\n  # Deploy to OCI");
    println!("  cd /tmp && terraform init");
    println!("  terraform apply -var-file=oci.tfvars");
    
    println!("\n☁️  OCI Deployment Ready:");
    println!("  • Terraform config: main.tf");
    println!("  • Variables: oci.tfvars");
    println!("  • Cloud-init scripts: *-cloud-init.sh");
    println!("  • Deployment tarballs: *-oci.tar.gz");
    
    println!("\n🔄 Self-Deployment Flow:");
    println!("  Local QA → Health Check → Local Prod → OCI Package → Oracle Cloud");
    
    Ok(())
}
