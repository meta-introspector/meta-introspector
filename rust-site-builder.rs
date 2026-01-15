use std::fs;
use std::path::Path;

fn main() {
    let output_dir = Path::new("_site");
    fs::create_dir_all(output_dir).unwrap();

    // Copy all HTML files
    for entry in fs::read_dir(".").unwrap().flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "html") {
            let filename = path.file_name().unwrap();
            fs::copy(&path, output_dir.join(filename)).unwrap();
            println!("Copied: {:?}", filename);
        }
    }

    // Copy reports directory
    if Path::new("reports").exists() {
        copy_dir_recursive("reports", &output_dir.join("reports")).unwrap();
        println!("Copied: reports/");
    }

    // Copy data files
    if Path::new("data").exists() {
        fs::create_dir_all(output_dir.join("data")).unwrap();
        for file in ["git-metrics-report.json", "investor-report-2025.json", "git-sources-registry.json"] {
            let src = Path::new("data").join(file);
            if src.exists() {
                fs::copy(&src, output_dir.join("data").join(file)).unwrap();
                println!("Copied: data/{}", file);
            }
        }
    }

    // Generate index if not exists
    if !Path::new("index.html").exists() {
        let index = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Meta-Introspector</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { 
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; 
            background: linear-gradient(180deg, #230052 0%, #301860 50%, #CC5500 100%);
            color: #F5F5DC; 
            padding: 40px 20px; 
            min-height: 100vh;
        }
        .container { max-width: 1200px; margin: 0 auto; text-align: center; }
        .logo { 
            width: 300px; 
            height: 300px; 
            margin: 0 auto 30px; 
            border-radius: 50%;
            box-shadow: 0 0 40px rgba(0, 255, 255, 0.5), 0 0 80px rgba(220, 20, 60, 0.3);
            animation: pulse 3s ease-in-out infinite;
        }
        @keyframes pulse {
            0%, 100% { box-shadow: 0 0 40px rgba(0, 255, 255, 0.5), 0 0 80px rgba(220, 20, 60, 0.3); }
            50% { box-shadow: 0 0 60px rgba(0, 255, 255, 0.8), 0 0 120px rgba(220, 20, 60, 0.5); }
        }
        h1 { 
            font-size: 3em; 
            color: #00FFFF; 
            margin-bottom: 20px; 
            text-shadow: 0 0 20px #00FFFF, 0 0 40px #00FFFF;
        }
        p { color: #FFD700; font-size: 1.2em; }
        .links { display: flex; gap: 20px; justify-content: center; margin-top: 40px; flex-wrap: wrap; }
        .btn { 
            background: linear-gradient(135deg, #DC143C 0%, #FF4500 100%);
            color: #FFF8DC; 
            padding: 15px 30px; 
            border-radius: 6px; 
            text-decoration: none; 
            font-weight: 500;
            box-shadow: 0 0 15px #DC143C;
            transition: all 0.3s;
        }
        .btn:hover { 
            box-shadow: 0 0 30px #FF4500, 0 0 50px #DC143C;
            transform: scale(1.05);
        }
    </style>
</head>
<body>
    <div class="container">
        <img src="solfunmeme-logo.png" alt="SOLFUNMEME" class="logo">
        <h1>📊 Meta-Introspector</h1>
        <p>SOLFUNMEME/ZOS (Zero Ontology System) - Git Activity Analysis & Reports</p>
        <div class="links">
            <a href="investor-report-2025.html" class="btn">2025 Investor Report</a>
            <a href="reports/" class="btn">All Reports</a>
            <a href="https://huggingface.co/datasets/introspector/git-activity" class="btn">Dataset</a>
        </div>
    </div>
</body>
</html>"#;
        fs::write(output_dir.join("index.html"), index).unwrap();
        println!("Generated: index.html");
    }

    println!("\n✅ Site built in _site/");
}

fn copy_dir_recursive(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_recursive(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
