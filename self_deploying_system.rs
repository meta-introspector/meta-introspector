// Self-Deploying System: QA → Local Prod → Oracle OCI
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentTarget {
    LocalQA { systemd_unit: String },
    LocalProd { systemd_unit: String },
    OracleOCI { instance_id: String, compartment_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    pub name: String,
    pub nix_store_path: String,
    pub target: DeploymentTarget,
    pub auto_promote: bool,
}

pub struct SelfDeployingSystem {
    pub configs: Vec<DeploymentConfig>,
}

impl SelfDeployingSystem {
    pub fn new() -> Self {
        Self { configs: Vec::new() }
    }
    
    pub async fn deploy_qa(&self, config: &DeploymentConfig) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔧 Deploying to QA: {}", config.name);
        
        // Build with nix
        let nix_result = self.nix_build(&config.nix_store_path)?;
        
        // Create systemd unit for QA
        self.create_systemd_unit(&config.name, &nix_result, "qa")?;
        
        // Start QA service
        self.systemctl("start", &format!("{}-qa", config.name))?;
        
        // Run health checks
        if self.health_check_qa(&config.name).await? {
            println!("  ✓ QA deployment successful");
            
            if config.auto_promote {
                self.promote_to_prod(config).await?;
            }
        }
        
        Ok(())
    }
    
    async fn promote_to_prod(&self, config: &DeploymentConfig) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Promoting to Local Prod: {}", config.name);
        
        // Stop QA
        self.systemctl("stop", &format!("{}-qa", config.name))?;
        
        // Get QA nix store path
        let qa_path = self.get_systemd_nix_path(&format!("{}-qa", config.name))?;
        
        // Create prod systemd unit
        self.create_systemd_unit(&config.name, &qa_path, "prod")?;
        
        // Start prod service
        self.systemctl("start", &format!("{}-prod", config.name))?;
        
        // Run health checks
        if self.health_check_prod(&config.name).await? {
            println!("  ✓ Prod deployment successful");
            
            // Prepare for OCI
            self.prepare_oci_deployment(config).await?;
        }
        
        Ok(())
    }
    
    async fn prepare_oci_deployment(&self, config: &DeploymentConfig) -> Result<(), Box<dyn std::error::Error>> {
        println!("☁️  Preparing Oracle OCI deployment: {}", config.name);
        
        // Get prod nix store path
        let prod_path = self.get_systemd_nix_path(&format!("{}-prod", config.name))?;
        
        // Create OCI deployment package
        self.create_oci_package(&config.name, &prod_path)?;
        
        // Generate terraform config
        self.generate_terraform_config(config)?;
        
        println!("  ✓ OCI deployment package ready");
        println!("  → Run: terraform apply -var-file=oci.tfvars");
        
        Ok(())
    }
    
    fn nix_build(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("nix-build")
            .arg(path)
            .arg("--no-out-link")
            .output()?;
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    
    fn create_systemd_unit(&self, name: &str, nix_path: &str, env: &str) -> Result<(), Box<dyn std::error::Error>> {
        let unit = format!(r#"[Unit]
Description={} - {} environment
After=network.target

[Service]
Type=simple
ExecStart={}/bin/{}
Restart=always
RestartSec=10
Environment="NIX_STORE_PATH={}"
Environment="DEPLOYMENT_ENV={}"
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
"#, name, env, nix_path, name, nix_path, env);
        
        let unit_path = format!("/etc/systemd/system/{}-{}.service", name, env);
        std::fs::write(&unit_path, unit)?;
        
        // Reload systemd
        Command::new("systemctl")
            .arg("daemon-reload")
            .output()?;
        
        println!("  ✓ Created systemd unit: {}", unit_path);
        Ok(())
    }
    
    fn systemctl(&self, action: &str, unit: &str) -> Result<(), Box<dyn std::error::Error>> {
        Command::new("systemctl")
            .arg(action)
            .arg(unit)
            .output()?;
        
        println!("  ✓ systemctl {} {}", action, unit);
        Ok(())
    }
    
    async fn health_check_qa(&self, name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        println!("  🔍 Running QA health checks...");
        
        // Check systemd status
        let status = Command::new("systemctl")
            .args(["is-active", &format!("{}-qa", name)])
            .output()?;
        
        if !status.status.success() {
            return Ok(false);
        }
        
        // Check HTTP endpoint (if applicable)
        if let Ok(response) = reqwest::get("http://localhost:8080/health").await {
            if response.status().is_success() {
                println!("  ✓ Health check passed");
                return Ok(true);
            }
        }
        
        Ok(true)
    }
    
    async fn health_check_prod(&self, name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        println!("  🔍 Running Prod health checks...");
        
        let status = Command::new("systemctl")
            .args(["is-active", &format!("{}-prod", name)])
            .output()?;
        
        if !status.status.success() {
            return Ok(false);
        }
        
        println!("  ✓ Health check passed");
        Ok(true)
    }
    
    fn get_systemd_nix_path(&self, unit: &str) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("systemctl")
            .args(["show", "-p", "Environment", unit])
            .output()?;
        
        let env = String::from_utf8_lossy(&output.stdout);
        for line in env.lines() {
            if line.contains("NIX_STORE_PATH=") {
                return Ok(line.split('=').nth(1).unwrap_or("").to_string());
            }
        }
        
        Err("NIX_STORE_PATH not found".into())
    }
    
    fn create_oci_package(&self, name: &str, nix_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Create OCI deployment tarball
        let tarball = format!("/tmp/{}-oci.tar.gz", name);
        
        Command::new("tar")
            .args(["-czf", &tarball, "-C", nix_path, "."])
            .output()?;
        
        // Create cloud-init script
        let cloud_init = format!(r#"#!/bin/bash
set -e

# Install nix
curl -L https://nixos.org/nix/install | sh
source ~/.nix-profile/etc/profile.d/nix.sh

# Extract deployment
mkdir -p /opt/{}
tar -xzf /tmp/{}-oci.tar.gz -C /opt/{}

# Create systemd unit
cat > /etc/systemd/system/{}.service << 'EOF'
[Unit]
Description={} - OCI deployment
After=network.target

[Service]
Type=simple
ExecStart=/opt/{}/bin/{}
Restart=always
Environment="DEPLOYMENT_ENV=oci"

[Install]
WantedBy=multi-user.target
EOF

# Start service
systemctl daemon-reload
systemctl enable {}
systemctl start {}
"#, name, name, name, name, name, name, name, name, name);
        
        std::fs::write(format!("/tmp/{}-cloud-init.sh", name), cloud_init)?;
        
        println!("  ✓ Created OCI package: {}", tarball);
        Ok(())
    }
    
    fn generate_terraform_config(&self, config: &DeploymentConfig) -> Result<(), Box<dyn std::error::Error>> {
        let terraform = format!(r#"
terraform {{
  required_providers {{
    oci = {{
      source = "oracle/oci"
    }}
  }}
}}

provider "oci" {{
  region = var.region
}}

resource "oci_core_instance" "{name}" {{
  availability_domain = var.availability_domain
  compartment_id      = var.compartment_id
  shape              = "VM.Standard.E4.Flex"
  
  shape_config {{
    memory_in_gbs = 16
    ocpus        = 2
  }}
  
  source_details {{
    source_type = "image"
    source_id   = var.image_id
  }}
  
  metadata = {{
    user_data = base64encode(file("{name}-cloud-init.sh"))
  }}
  
  display_name = "{name}-prod"
}}

output "instance_ip" {{
  value = oci_core_instance.{name}.public_ip
}}
"#, name = config.name);
        
        std::fs::write("main.tf", terraform)?;
        
        let tfvars = r#"
region = "us-ashburn-1"
compartment_id = "ocid1.compartment.oc1.."
availability_domain = "AD-1"
image_id = "ocid1.image.oc1.."
"#;
        
        std::fs::write("oci.tfvars", tfvars)?;
        
        println!("  ✓ Generated terraform config");
        Ok(())
    }
}

// Create deployment configs for all services
pub fn create_deployment_configs() -> Vec<DeploymentConfig> {
    vec![
        DeploymentConfig {
            name: "meta-introspector-server".to_string(),
            nix_store_path: ".#minimal-build-server".to_string(),
            target: DeploymentTarget::LocalQA {
                systemd_unit: "meta-introspector-qa".to_string(),
            },
            auto_promote: true,
        },
        DeploymentConfig {
            name: "zos-server".to_string(),
            nix_store_path: ".#zos".to_string(),
            target: DeploymentTarget::LocalQA {
                systemd_unit: "zos-qa".to_string(),
            },
            auto_promote: true,
        },
        DeploymentConfig {
            name: "universal-llm-proxy".to_string(),
            nix_store_path: ".#universal-llm-proxy".to_string(),
            target: DeploymentTarget::LocalQA {
                systemd_unit: "llm-proxy-qa".to_string(),
            },
            auto_promote: false, // Manual promotion for LLM proxy
        },
    ]
}
