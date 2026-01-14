// 🔥 LIBREPORTING - Unified Reporting System
// Consistent headers, footers, and formatting across all tools
//
// ## Usage
//
// ```rust
// use meta_introspector::{report_start, report_section, report_count, report_summary, report_end};
//
// let start_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
//
// report_start!("My Tool", "project_name");
// report_section!("Processing binaries");
// report_count!("Found", 32);
// report_summary!("Binaries" => 32, "Libraries" => 82);
// report_end!("My Tool", "project_name", start_time);
// ```
//
// ## Output Format
//
// ```
// 🔥 MY TOOL
// =======================================
// 📊 Project: project_name
// ⏰ Timestamp: 1768339103
//
// 🔧 Processing binaries...
//   📊 Found: 32
//
// 🎯 SUMMARY:
// ==============================
// 📋 Binaries: 32
// 📋 Libraries: 82
//
// ✅ MY TOOL COMPLETED
// 📁 Project: project_name
// ⏰ Duration: 5 seconds
// =======================================
// ```

use std::time::{SystemTime, UNIX_EPOCH};

pub struct Report {
    pub title: String,
    pub timestamp: u64,
    pub project: String,
    sections: Vec<ReportSection>,
}

pub struct ReportSection {
    pub name: String,
    pub items: Vec<String>,
}

impl Report {
    pub fn new(title: &str, project: &str) -> Self {
        Self {
            title: title.to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            project: project.to_string(),
            sections: Vec::new(),
        }
    }

    pub fn header(&self) {
        println!("🔥 {}", self.title.to_uppercase());
        println!("=======================================");
        println!("📊 Project: {}", self.project);
        println!("⏰ Timestamp: {}", self.timestamp);
        println!();
    }

    pub fn section(&mut self, name: &str) -> &mut ReportSection {
        self.sections.push(ReportSection {
            name: name.to_string(),
            items: Vec::new(),
        });
        self.sections.last_mut().unwrap()
    }

    pub fn summary(&self) {
        println!();
        println!("🎯 SUMMARY:");
        println!("==============================");
        for section in &self.sections {
            println!("📋 {}: {} items", section.name, section.items.len());
        }
    }

    pub fn footer(&self) {
        println!();
        println!("✅ {} COMPLETED", self.title.to_uppercase());
        println!("📁 Project: {}", self.project);
        println!("⏰ Duration: {} seconds", 
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - self.timestamp);
        println!("=======================================");
    }
}

impl ReportSection {
    pub fn add(&mut self, item: &str) {
        self.items.push(item.to_string());
        println!("  - {}", item);
    }

    pub fn add_count(&mut self, name: &str, count: usize) {
        let item = format!("{}: {}", name, count);
        self.items.push(item.clone());
        println!("  📊 {}", item);
    }

    pub fn add_file(&mut self, path: &str, size: u64) {
        let item = format!("{} ({} bytes)", path, size);
        self.items.push(item.clone());
        println!("  📄 {}", item);
    }
}

// Convenience macros for easy reporting
#[macro_export]
macro_rules! report_start {
    ($title:expr, $project:expr) => {{
        println!("🔥 {}", $title.to_uppercase());
        println!("=======================================");
        println!("📊 Project: {}", $project);
        println!("⏰ Timestamp: {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
        println!();
    }};
}

#[macro_export]
macro_rules! report_section {
    ($name:expr) => {
        println!("🔧 {}...", $name);
    };
}

#[macro_export]
macro_rules! report_item {
    ($item:expr) => {
        println!("  - {}", $item);
    };
}

#[macro_export]
macro_rules! report_count {
    ($name:expr, $count:expr) => {
        println!("  📊 {}: {}", $name, $count);
    };
}

#[macro_export]
macro_rules! report_file {
    ($path:expr, $size:expr) => {
        println!("  📄 {} ({} bytes)", $path, $size);
    };
}

#[macro_export]
macro_rules! report_summary {
    ($($name:expr => $count:expr),*) => {
        println!();
        println!("🎯 SUMMARY:");
        println!("==============================");
        $(
            println!("📋 {}: {}", $name, $count);
        )*
    };
}

#[macro_export]
macro_rules! report_end {
    ($title:expr, $project:expr, $start_time:expr) => {
        println!();
        println!("✅ {} COMPLETED", $title.to_uppercase());
        println!("📁 Project: {}", $project);
        println!("⏰ Duration: {} seconds", 
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() - $start_time);
        println!("=======================================");
    };
}
