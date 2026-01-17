// 🌟 PERSONAL DATA SOVEREIGNTY: Your GitHub Stars = Your Dataset, No Silicon Valley
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sha2::Digest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalProfile {
    pub user_id: String,
    pub storage_preference: StorageLevel,
    pub github_stars: Vec<GitHubStar>,
    pub personal_datasets: HashMap<String, Dataset>,
    pub crud_apps: Vec<CrudApp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageLevel {
    Local,              // Free local storage
    Distributed,        // P2P network storage  
    Premium,            // Paid high-availability
    Sovereign,          // Your own infrastructure
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubStar {
    pub repo: String,
    pub starred_at: String,
    pub personal_notes: Option<String>,
    pub local_fork: Option<String>,
    pub content_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    pub name: String,
    pub source: DataSource,
    pub storage_level: StorageLevel,
    pub content_address: String,
    pub access_cost: u64, // lamports
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    GitHubStars,
    TodoList,
    PersonalNotes,
    CodeSnippets,
    BookmarksList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrudApp {
    pub name: String,
    pub emoji: String,
    pub storage_backend: StorageLevel,
    pub data_ca: String,
    pub app_ca: String,
}

pub struct PersonalDataSovereignty {
    profiles: HashMap<String, PersonalProfile>,
}

impl PersonalDataSovereignty {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }

    pub fn create_profile(&mut self, user_id: &str, storage_pref: StorageLevel) -> PersonalProfile {
        let profile = PersonalProfile {
            user_id: user_id.to_string(),
            storage_preference: storage_pref,
            github_stars: vec![],
            personal_datasets: HashMap::new(),
            crud_apps: vec![],
        };
        
        self.profiles.insert(user_id.to_string(), profile.clone());
        profile
    }

    pub fn import_github_stars(&mut self, user_id: &str, stars: Vec<String>) -> Result<(), String> {
        let profile = self.profiles.get_mut(user_id).ok_or("Profile not found")?;
        
        for repo in stars {
            let star = GitHubStar {
                repo: repo.clone(),
                starred_at: chrono::Utc::now().to_rfc3339(),
                personal_notes: None,
                local_fork: None,
                content_address: format!("ca_{}", sha2::Sha256::digest(repo.as_bytes()).iter().map(|b| format!("{:02x}", b)).collect::<String>()),
            };
            profile.github_stars.push(star);
        }

        // Create personal dataset from stars
        let dataset = Dataset {
            name: "My GitHub Stars".to_string(),
            source: DataSource::GitHubStars,
            storage_level: profile.storage_preference.clone(),
            content_address: format!("ca_stars_{}", user_id),
            access_cost: match profile.storage_preference {
                StorageLevel::Local => 0,
                StorageLevel::Distributed => 10,
                StorageLevel::Premium => 100,
                StorageLevel::Sovereign => 0,
            },
        };

        profile.personal_datasets.insert("github_stars".to_string(), dataset);
        Ok(())
    }

    pub fn create_crud_app(&mut self, user_id: &str, app_name: &str, emoji: &str) -> Result<CrudApp, String> {
        let profile = self.profiles.get_mut(user_id).ok_or("Profile not found")?;
        
        let app = CrudApp {
            name: app_name.to_string(),
            emoji: emoji.to_string(),
            storage_backend: profile.storage_preference.clone(),
            data_ca: format!("ca_data_{}_{}", user_id, app_name),
            app_ca: format!("ca_app_{}_{}", user_id, app_name),
        };

        profile.crud_apps.push(app.clone());
        Ok(app)
    }

    pub fn generate_personal_web_app(&self, user_id: &str) -> String {
        let profile = self.profiles.get(user_id).unwrap();
        
        format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>🌟 {}'s Personal Data Sovereignty</title>
    <style>
        body {{ font-family: monospace; background: #001; color: #0f0; }}
        .app {{ border: 1px solid #0f0; padding: 10px; margin: 10px; }}
        .storage-level {{ color: #ff0; }}
    </style>
</head>
<body>
    <h1>🌟 Welcome {}</h1>
    <p class="storage-level">Storage: {:?}</p>
    
    <h2>📊 Your Personal Datasets</h2>
    {}
    
    <h2>📱 Your CRUD Apps</h2>
    {}
    
    <h2>⭐ Your GitHub Stars ({})</h2>
    <p>Your starred repos are now your personal dataset!</p>
    <button onclick="syncStars()">Sync Latest Stars</button>
    
    <h2>🚀 No Silicon Valley Needed!</h2>
    <ul>
        <li>✅ Your data, your rules</li>
        <li>✅ Choose your storage level</li>
        <li>✅ Content-addressed everything</li>
        <li>✅ Pay only for what you use</li>
    </ul>
    
    <script>
        function syncStars() {{
            fetch('/sync-stars/{user_id}').then(r => r.json()).then(d => {{
                alert('Synced ' + d.count + ' stars to your personal dataset!');
            }});
        }}
    </script>
</body>
</html>
        "#, 
        user_id, 
        user_id,
        profile.storage_preference,
        self.render_datasets(&profile.personal_datasets),
        self.render_crud_apps(&profile.crud_apps),
        profile.github_stars.len(),
        user_id = user_id
        )
    }

    fn render_datasets(&self, datasets: &HashMap<String, Dataset>) -> String {
        datasets.iter().map(|(name, dataset)| {
            format!(r#"
            <div class="app">
                <h3>{}</h3>
                <p>Source: {:?} | Storage: {:?}</p>
                <p>CA: {}</p>
                <p>Cost: {} lamports</p>
            </div>
            "#, dataset.name, dataset.source, dataset.storage_level, 
               dataset.content_address, dataset.access_cost)
        }).collect::<Vec<_>>().join("")
    }

    fn render_crud_apps(&self, apps: &[CrudApp]) -> String {
        apps.iter().map(|app| {
            format!(r#"
            <div class="app">
                <h3>{} {}</h3>
                <p>Storage: {:?}</p>
                <p>Data CA: {}</p>
                <p>App CA: {}</p>
                <button onclick="location.href='/app/{}'">Open App</button>
            </div>
            "#, app.emoji, app.name, app.storage_backend, 
               app.data_ca, app.app_ca, app.app_ca)
        }).collect::<Vec<_>>().join("")
    }
}

// Demo function
pub fn demo_personal_sovereignty() {
    let mut sovereignty = PersonalDataSovereignty::new();
    
    // Create user profile
    let profile = sovereignty.create_profile("alice", StorageLevel::Distributed);
    println!("👤 Created profile for alice with distributed storage");
    
    // Import GitHub stars
    let stars = vec![
        "rust-lang/rust".to_string(),
        "nixos/nixpkgs".to_string(),
        "microsoft/vscode".to_string(),
    ];
    sovereignty.import_github_stars("alice", stars).unwrap();
    println!("⭐ Imported {} GitHub stars as personal dataset", 3);
    
    // Create personal CRUD apps
    let todo_app = sovereignty.create_crud_app("alice", "My Todos", "📝").unwrap();
    let notes_app = sovereignty.create_crud_app("alice", "My Notes", "📓").unwrap();
    
    println!("📱 Created CRUD apps:");
    println!("  {} {} - Data CA: {}", todo_app.emoji, todo_app.name, todo_app.data_ca);
    println!("  {} {} - Data CA: {}", notes_app.emoji, notes_app.name, notes_app.data_ca);
    
    // Generate personal web app
    let web_app = sovereignty.generate_personal_web_app("alice");
    println!("🌐 Generated personal web app (served from CA)");
    
    println!("\n🎯 RESULT: Complete data sovereignty!");
    println!("  ✅ GitHub stars → Personal dataset");
    println!("  ✅ CRUD apps → Content addressed");
    println!("  ✅ Storage choice → User controlled");
    println!("  ✅ No Silicon Valley → Self-sovereign");
}

fn main() {
    demo_personal_sovereignty();
}
