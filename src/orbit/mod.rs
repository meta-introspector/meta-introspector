//! Orbit computation and visualization
//! Tracks convergence to automorphic eigenvector

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

/// Single orbit point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Orbit {
    /// Iteration number
    pub iteration: u64,
    
    /// System size (bytes)
    pub size: u64,
    
    /// Number of duplicates removed
    pub duplicates_removed: u64,
    
    /// Galois field coverage (0.0 to 1.0)
    pub gf_coverage: f64,
    
    /// Number of necessary bytes
    pub necessary_bytes: u64,
}

/// Orbit trajectory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitTrajectory {
    /// All orbit points
    pub orbits: Vec<Orbit>,
    
    /// Did we converge?
    pub converged: bool,
    
    /// Final eigenvector size
    pub eigenvector_size: u64,
}

impl OrbitTrajectory {
    pub fn new() -> Self {
        Self {
            orbits: Vec::new(),
            converged: false,
            eigenvector_size: 0,
        }
    }
    
    /// Add orbit point
    pub fn add(&mut self, orbit: Orbit) {
        self.orbits.push(orbit);
    }
    
    /// Mark as converged
    pub fn converge(&mut self, size: u64) {
        self.converged = true;
        self.eigenvector_size = size;
    }
    
    /// Save to JSON
    pub fn save(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| e.to_string())?;
        
        let mut file = File::create(path)
            .map_err(|e| e.to_string())?;
        
        file.write_all(json.as_bytes())
            .map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    /// Generate visualization script
    pub fn generate_plot_script(&self, output: &str) -> Result<(), String> {
        let script = format!(r#"
import matplotlib.pyplot as plt
import json

# Load orbit data
with open('{}', 'r') as f:
    data = json.load(f)

orbits = data['orbits']

# Extract data
iterations = [o['iteration'] for o in orbits]
sizes = [o['size'] for o in orbits]
duplicates = [o['duplicates_removed'] for o in orbits]
coverage = [o['gf_coverage'] for o in orbits]

# Create figure
fig, ((ax1, ax2), (ax3, ax4)) = plt.subplots(2, 2, figsize=(12, 8))

# Plot 1: System size
ax1.plot(iterations, sizes, 'b-', linewidth=2)
ax1.set_xlabel('Iteration')
ax1.set_ylabel('System Size (bytes)')
ax1.set_title('Convergence to Eigenvector')
ax1.grid(True)

# Plot 2: Duplicates removed
ax2.plot(iterations, duplicates, 'r-', linewidth=2)
ax2.set_xlabel('Iteration')
ax2.set_ylabel('Duplicates Removed')
ax2.set_title('Deduplication Progress')
ax2.grid(True)

# Plot 3: Galois field coverage
ax3.plot(iterations, coverage, 'g-', linewidth=2)
ax3.set_xlabel('Iteration')
ax3.set_ylabel('GF Coverage')
ax3.set_title('Galois Field Saturation')
ax3.axhline(y=1.0, color='k', linestyle='--', label='100% coverage')
ax3.legend()
ax3.grid(True)

# Plot 4: Phase space
ax4.plot(sizes, coverage, 'purple', linewidth=2)
ax4.set_xlabel('System Size')
ax4.set_ylabel('GF Coverage')
ax4.set_title('Phase Space Trajectory')
ax4.grid(True)

# Add convergence marker
if data['converged']:
    ax4.plot(sizes[-1], coverage[-1], 'ro', markersize=10, label='Eigenvector')
    ax4.legend()

plt.tight_layout()
plt.savefig('{}')
print('Saved visualization to {}')
"#, output, output.replace(".json", ".png"), output.replace(".json", ".png"));

        let mut file = File::create(output.replace(".json", ".py"))
            .map_err(|e| e.to_string())?;
        
        file.write_all(script.as_bytes())
            .map_err(|e| e.to_string())?;
        
        Ok(())
    }
}

/// Compute orbit trajectory
pub fn compute_trajectory(db: &crate::provenance::ProvenanceDB) -> OrbitTrajectory {
    let mut trajectory = OrbitTrajectory::new();
    let mut iteration = 0;
    let initial_size = db.size();
    
    loop {
        // Current state
        let size = db.necessary_size();
        let duplicates_removed = initial_size - size;
        let gf_coverage = compute_gf_coverage(db);
        
        let orbit = Orbit {
            iteration,
            size,
            duplicates_removed,
            gf_coverage,
            necessary_bytes: size,
        };
        
        trajectory.add(orbit);
        
        // Check convergence
        let duplicates = db.find_duplicates();
        if duplicates.is_empty() {
            trajectory.converge(size);
            break;
        }
        
        iteration += 1;
        
        // Safety: max 1000 iterations
        if iteration > 1000 {
            break;
        }
    }
    
    trajectory
}

/// Compute Galois field coverage from unique signatures
fn compute_gf_coverage(db: &crate::provenance::ProvenanceDB) -> f64 {
    let unique_sigs = db.find_duplicates().len();
    let total_bytes = db.size();
    
    if total_bytes == 0 {
        return 0.0;
    }
    
    (unique_sigs as f64) / (total_bytes as f64)
}
