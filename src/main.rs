use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use chrono::Local;
use rand::seq::SliceRandom;
use serde::Deserialize;

// ── Terminal styling ──────────────────────────────────────────────────────────

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";

// ── Data model ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone)]
struct Exercise {
    level: String,
    task: String,
    hint: String,
}

#[derive(Debug, Deserialize)]
struct KnowledgeBase {
    exercises: Vec<Exercise>,
}

// ── Filesystem ────────────────────────────────────────────────────────────────

/// Returns (topic_name, path_to_exercises.toml) for every eligible subdirectory.
fn discover_topics(root: &Path) -> Vec<(String, PathBuf)> {
    let Ok(entries) = fs::read_dir(root) else {
        return Vec::new();
    };
    let mut topics: Vec<(String, PathBuf)> = entries
        .flatten()
        .filter_map(|e| {
            let path = e.path();
            if !path.is_dir() {
                return None;
            }
            let kb = path.join("exercises.toml");
            kb.exists().then(|| {
                let name = path.file_name()?.to_str()?.to_string();
                Some((name, kb))
            })?
        })
        .collect();
    topics.sort_by(|a, b| a.0.cmp(&b.0));
    topics
}

// ── I/O helpers ───────────────────────────────────────────────────────────────

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_lowercase()
}

fn ask_yn(label: &str) -> bool {
    prompt(&format!("  Include {}? [y/n] ", label)) == "y"
}

// ── Topic selection ───────────────────────────────────────────────────────────

fn select_topic<'a>(topics: &'a [(String, PathBuf)]) -> Option<&'a (String, PathBuf)> {
    match topics.len() {
        0 => {
            eprintln!("No topics found. Add a subdirectory containing exercises.toml.");
            None
        }
        1 => Some(&topics[0]),
        _ => {
            println!("{}  Available topics:{}", BOLD, RESET);
            for (i, (name, _)) in topics.iter().enumerate() {
                println!("    {}{}{}){} {}", CYAN, BOLD, i + 1, RESET, name);
            }
            println!();
            loop {
                let raw = prompt("  Select a topic [1]: ");
                let input = if raw.is_empty() { "1".to_string() } else { raw };
                match input.parse::<usize>() {
                    Ok(n) if n >= 1 && n <= topics.len() => return Some(&topics[n - 1]),
                    _ => println!("  Enter a number between 1 and {}.", topics.len()),
                }
            }
        }
    }
}

// ── Quiz engine ───────────────────────────────────────────────────────────────

fn run_quiz(mut exercises: Vec<Exercise>) {
    let mut rng = rand::thread_rng();
    exercises.shuffle(&mut rng);
    let total = exercises.len();

    println!();
    println!(
        "  {} exercises loaded.  {}[Enter]{} → hint   {}[Enter]{} → next   {}q{} → quit",
        total, BOLD, RESET, BOLD, RESET, BOLD, RESET
    );

    for (i, ex) in exercises.iter().enumerate() {
        println!();
        println!("{}  ────────────────────────────────────────────────────{}", DIM, RESET);
        println!();
        println!("  {}{}{}/{}{}", CYAN, BOLD, i + 1, total, RESET);
        println!();
        for line in word_wrap_preserving_lines(&ex.task, 58) {
            if line.is_empty() {
                println!();
            } else {
                println!("  {}{}{}", BOLD, line, RESET);
            }
        }
        println!();

        if prompt("  > ") == "q" {
            println!("\n{}  Ended at {}/{}. {}", DIM, i + 1, total, RESET);
            return;
        }

        println!();
        println!("  {}{}Hint:{}", YELLOW, BOLD, RESET);
        println!();
        for line in ex.hint.trim().lines() {
            println!("    {}{}{}", GREEN, line, RESET);
        }
        println!();

        if prompt("  > ") == "q" {
            println!("\n{}  Ended at {}/{}. {}", DIM, i + 1, total, RESET);
            return;
        }
    }

    println!();
    println!("{}  ────────────────────────────────────────────────────{}", DIM, RESET);
    println!();
    println!("  {}{}All done! Great work.{}", BOLD, GREEN, RESET);
    println!();
}

/// Word-wraps one logical line at `width` characters.
fn word_wrap(s: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in s.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current.clone());
            current = word.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

/// Word-wraps text while preserving explicit line breaks from the exercise file.
fn word_wrap_preserving_lines(s: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for source_line in s.trim().lines() {
        let source_line = source_line.trim();
        if source_line.is_empty() {
            lines.push(String::new());
        } else if let Some(item) = source_line.strip_prefix("- ") {
            for (i, line) in word_wrap(item, width.saturating_sub(2)).iter().enumerate() {
                if i == 0 {
                    lines.push(format!("- {}", line));
                } else {
                    lines.push(format!("  {}", line));
                }
            }
        } else {
            lines.extend(word_wrap(source_line, width));
        }
    }
    lines
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cwd = std::env::current_dir().expect("Cannot read current directory");
    let topics = discover_topics(&cwd);

    // Optional topic name as first CLI arg: `quiz vim`
    let topic = match args.get(1) {
        Some(name) => match topics.iter().find(|(t, _)| t == name) {
            Some(t) => t,
            None => {
                let available: Vec<&str> = topics.iter().map(|(t, _)| t.as_str()).collect();
                eprintln!("Topic '{}' not found. Available: {}", name, available.join(", "));
                return;
            }
        },
        None => match select_topic(&topics) {
            Some(t) => t,
            None => return,
        },
    };

    let (topic_name, kb_path) = topic;
    let date = Local::now().format("%b %d %Y").to_string();

    println!();
    println!("{}{}  ══════════════════════════════════════════════════{}", BOLD, CYAN, RESET);
    println!("{}{}  Quiz: {}  —  {}{}", BOLD, CYAN, topic_name, date, RESET);
    println!("{}{}  ══════════════════════════════════════════════════{}", BOLD, CYAN, RESET);
    println!();

    let content = fs::read_to_string(kb_path)
        .unwrap_or_else(|e| panic!("Cannot read {}: {}", kb_path.display(), e));
    let kb: KnowledgeBase = toml::from_str(&content)
        .unwrap_or_else(|e| panic!("Cannot parse {}: {}", kb_path.display(), e));

    // Collect levels in order of first appearance — TOML file controls the ordering.
    let mut levels: Vec<String> = Vec::new();
    for ex in &kb.exercises {
        if !levels.contains(&ex.level) {
            levels.push(ex.level.clone());
        }
    }

    let mut pool: Vec<Exercise> = Vec::new();
    for level in &levels {
        if ask_yn(level) {
            pool.extend(kb.exercises.iter().filter(|e| &e.level == level).cloned());
        }
    }
    println!();

    if pool.is_empty() {
        println!("  Nothing selected. Goodbye.");
        return;
    }

    run_quiz(pool);
}
