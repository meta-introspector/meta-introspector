// N-Char Value Lattice Concept Map Builder
// Extract all words from our 263 tools → n-char lattice → n-grams → Markov → harmonics

use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};
use rayon::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct ConceptMap {
    total_files: usize,
    total_words: usize,
    unique_words: usize,
    word_frequencies: HashMap<String, usize>,
    nchar_lattice: HashMap<usize, Vec<String>>,  // n -> words of length n
    ngrams: HashMap<String, usize>,  // n-gram -> frequency
    markov_transitions: HashMap<String, HashMap<String, f64>>,  // word -> next_word -> prob
    harmonic_frequencies: Vec<(String, f64)>,  // word -> frequency ratio
}

fn extract_words(content: &str) -> Vec<String> {
    content
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .filter(|s| !s.is_empty() && s.len() > 2)
        .map(|s| s.to_lowercase())
        .collect()
}

fn build_nchar_lattice(words: &[String]) -> HashMap<usize, Vec<String>> {
    let mut lattice: HashMap<usize, Vec<String>> = HashMap::new();
    
    for word in words {
        let len = word.len();
        lattice.entry(len).or_insert_with(Vec::new).push(word.clone());
    }
    
    // Sort and deduplicate each level
    for words in lattice.values_mut() {
        words.sort();
        words.dedup();
    }
    
    lattice
}

fn build_ngrams(words: &[String], n: usize) -> HashMap<String, usize> {
    let mut ngrams = HashMap::new();
    
    for window in words.windows(n) {
        let ngram = window.join(" ");
        *ngrams.entry(ngram).or_insert(0) += 1;
    }
    
    ngrams
}

fn build_markov_transitions(words: &[String]) -> HashMap<String, HashMap<String, f64>> {
    let mut transitions: HashMap<String, HashMap<String, usize>> = HashMap::new();
    
    for window in words.windows(2) {
        let from = &window[0];
        let to = &window[1];
        *transitions.entry(from.clone()).or_insert_with(HashMap::new).entry(to.clone()).or_insert(0) += 1;
    }
    
    // Convert counts to probabilities
    let mut probs = HashMap::new();
    for (from, nexts) in transitions {
        let total: usize = nexts.values().sum();
        let mut next_probs = HashMap::new();
        for (to, count) in nexts {
            next_probs.insert(to, count as f64 / total as f64);
        }
        probs.insert(from, next_probs);
    }
    
    probs
}

fn compute_harmonic_frequencies(word_freq: &HashMap<String, usize>) -> Vec<(String, f64)> {
    let total: usize = word_freq.values().sum();
    let mut harmonics: Vec<(String, f64)> = word_freq
        .iter()
        .map(|(word, count)| (word.clone(), *count as f64 / total as f64))
        .collect();
    
    harmonics.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    harmonics
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗺️  N-CHAR VALUE LATTICE CONCEPT MAP BUILDER\n");
    
    // Collect all our Rust files
    let files: Vec<_> = fs::read_dir(".")?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("rs"))
        .map(|e| e.path())
        .collect();
    
    println!("📂 Found {} Rust files", files.len());
    
    // Extract all words in parallel
    println!("📝 Extracting words...");
    let all_words: Vec<String> = files.par_iter()
        .filter_map(|path| fs::read_to_string(path).ok())
        .flat_map(|content| extract_words(&content))
        .collect();
    
    println!("   Total words: {}", all_words.len());
    
    // Build word frequency map
    println!("📊 Building frequency map...");
    let mut word_freq: HashMap<String, usize> = HashMap::new();
    for word in &all_words {
        *word_freq.entry(word.clone()).or_insert(0) += 1;
    }
    
    println!("   Unique words: {}", word_freq.len());
    
    // Build n-char lattice
    println!("🔷 Building n-char lattice...");
    let nchar_lattice = build_nchar_lattice(&word_freq.keys().cloned().collect::<Vec<_>>());
    
    for (n, words) in nchar_lattice.iter() {
        println!("   {}-char words: {}", n, words.len());
    }
    
    // Build n-grams (2-grams, 3-grams)
    println!("🔗 Building n-grams...");
    let bigrams = build_ngrams(&all_words, 2);
    let trigrams = build_ngrams(&all_words, 3);
    
    println!("   2-grams: {}", bigrams.len());
    println!("   3-grams: {}", trigrams.len());
    
    // Build Markov transitions
    println!("🔀 Building Markov transitions...");
    let markov = build_markov_transitions(&all_words);
    
    println!("   Transition states: {}", markov.len());
    
    // Compute harmonic frequencies
    println!("🎵 Computing harmonic frequencies...");
    let harmonics = compute_harmonic_frequencies(&word_freq);
    
    println!("   Top 10 frequencies:");
    for (word, freq) in harmonics.iter().take(10) {
        println!("     {} → {:.4}", word, freq);
    }
    
    // Build concept map
    let concept_map = ConceptMap {
        total_files: files.len(),
        total_words: all_words.len(),
        unique_words: word_freq.len(),
        word_frequencies: word_freq,
        nchar_lattice,
        ngrams: bigrams,
        markov_transitions: markov,
        harmonic_frequencies: harmonics,
    };
    
    // Save as JSON
    println!("\n💾 Saving concept map...");
    let json = serde_json::to_string_pretty(&concept_map)?;
    fs::write("data/concept_map.json", json)?;
    
    println!("✅ Saved data/concept_map.json");
    
    // Generate report
    let mut report = String::from("# Concept Map Report\n\n");
    report.push_str(&format!("**Files analyzed**: {}\n", concept_map.total_files));
    report.push_str(&format!("**Total words**: {}\n", concept_map.total_words));
    report.push_str(&format!("**Unique words**: {}\n\n", concept_map.unique_words));
    
    report.push_str("## N-Char Lattice\n\n");
    let mut lengths: Vec<_> = concept_map.nchar_lattice.keys().collect();
    lengths.sort();
    for n in lengths {
        let count = concept_map.nchar_lattice[n].len();
        report.push_str(&format!("- **{}-char words**: {}\n", n, count));
    }
    
    report.push_str("\n## Top 20 Words by Frequency\n\n");
    report.push_str("| Word | Count | Frequency |\n");
    report.push_str("|------|-------|----------|\n");
    for (word, freq) in concept_map.harmonic_frequencies.iter().take(20) {
        let count = concept_map.word_frequencies[word];
        report.push_str(&format!("| {} | {} | {:.4} |\n", word, count, freq));
    }
    
    report.push_str("\n## Markov Transition Examples\n\n");
    let mut markov_examples: Vec<_> = concept_map.markov_transitions.iter().take(10).collect();
    markov_examples.sort_by_key(|(word, _)| *word);
    
    for (from, nexts) in markov_examples {
        report.push_str(&format!("\n**{}** →\n", from));
        let mut next_sorted: Vec<_> = nexts.iter().collect();
        next_sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        for (to, prob) in next_sorted.iter().take(5) {
            report.push_str(&format!("  - {} ({:.3})\n", to, prob));
        }
    }
    
    report.push_str("\n## Harmonic Analysis\n\n");
    report.push_str("Frequency distribution follows power law:\n\n");
    report.push_str("```\n");
    for (i, (word, freq)) in concept_map.harmonic_frequencies.iter().take(20).enumerate() {
        let rank = i + 1;
        let expected = 1.0 / rank as f64;
        let ratio = freq / expected;
        report.push_str(&format!("Rank {}: {} freq={:.4} expected={:.4} ratio={:.2}\n", 
            rank, word, freq, expected, ratio));
    }
    report.push_str("```\n");
    
    fs::write("CONCEPT_MAP_REPORT.md", report)?;
    println!("✅ Saved CONCEPT_MAP_REPORT.md");
    
    Ok(())
}
