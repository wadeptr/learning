use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use chrono::Local;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Default, Deserialize, Serialize)]
struct SampleState {
    reviewed_tasks: Vec<String>,
}

const DEFAULT_SAMPLE_SIZE: usize = 5;
const SAMPLE_STATE_FILE: &str = ".quiz-sample-state.toml";

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

fn sample_state_path(kb_path: &Path) -> PathBuf {
    kb_path
        .parent()
        .expect("exercises.toml must have a parent directory")
        .join(SAMPLE_STATE_FILE)
}

fn load_sample_state(path: &Path) -> SampleState {
    let Ok(content) = fs::read_to_string(path) else {
        return SampleState::default();
    };
    toml::from_str(&content).unwrap_or_else(|e| {
        eprintln!(
            "Warning: cannot parse sample history at {}: {}. Starting a new cycle.",
            path.display(),
            e
        );
        SampleState::default()
    })
}

fn save_sample_state(path: &Path, state: &SampleState) -> io::Result<()> {
    let content =
        toml::to_string_pretty(state).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(path, content)
}

fn select_sample(exercises: &[Exercise], count: usize, state: &mut SampleState) -> Vec<Exercise> {
    state
        .reviewed_tasks
        .retain(|task| exercises.iter().any(|exercise| &exercise.task == task));

    let mut unseen: Vec<&Exercise> = exercises
        .iter()
        .filter(|exercise| !state.reviewed_tasks.contains(&exercise.task))
        .collect();

    if unseen.is_empty() {
        state.reviewed_tasks.clear();
        unseen = exercises.iter().collect();
    }

    let mut rng = rand::thread_rng();
    unseen.shuffle(&mut rng);

    let mut sample = Vec::new();
    if count > 1 {
        let mut unseen_levels: Vec<String> = Vec::new();
        for exercise in &unseen {
            if !unseen_levels.contains(&exercise.level) {
                unseen_levels.push(exercise.level.clone());
            }
        }
        unseen_levels.shuffle(&mut rng);

        for level in unseen_levels.into_iter().take(count) {
            if let Some(index) = unseen.iter().position(|exercise| exercise.level == level) {
                sample.push(unseen.swap_remove(index).clone());
            }
        }
    }

    let remaining = count.saturating_sub(sample.len());
    sample.extend(unseen.into_iter().take(remaining).cloned());
    sample.shuffle(&mut rng);
    state
        .reviewed_tasks
        .extend(sample.iter().map(|exercise| exercise.task.clone()));
    sample
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
        println!(
            "{}  ────────────────────────────────────────────────────{}",
            DIM, RESET
        );
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
    println!(
        "{}  ────────────────────────────────────────────────────{}",
        DIM, RESET
    );
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
                eprintln!(
                    "Topic '{}' not found. Available: {}",
                    name,
                    available.join(", ")
                );
                return;
            }
        },
        None => match select_topic(&topics) {
            Some(t) => t,
            None => return,
        },
    };

    let (topic_name, kb_path) = topic;

    let sample_size = match args.get(2).map(String::as_str) {
        Some("--sample") => match args.get(3) {
            Some(raw) => match raw.parse::<usize>() {
                Ok(0) | Err(_) => {
                    eprintln!("Sample size must be a positive integer.");
                    return;
                }
                Ok(size) => Some(size),
            },
            None => Some(DEFAULT_SAMPLE_SIZE),
        },
        Some(arg) => {
            eprintln!(
                "Unknown option '{}'. Usage: quiz [topic] [--sample [count]]",
                arg
            );
            return;
        }
        None => None,
    };

    if args.len() > 4 {
        eprintln!("Too many arguments. Usage: quiz [topic] [--sample [count]]");
        return;
    }

    let date = Local::now().format("%b %d %Y").to_string();

    println!();
    println!(
        "{}{}  ══════════════════════════════════════════════════{}",
        BOLD, CYAN, RESET
    );
    println!(
        "{}{}  Quiz: {}  —  {}{}",
        BOLD, CYAN, topic_name, date, RESET
    );
    println!(
        "{}{}  ══════════════════════════════════════════════════{}",
        BOLD, CYAN, RESET
    );
    println!();

    let content = fs::read_to_string(kb_path)
        .unwrap_or_else(|e| panic!("Cannot read {}: {}", kb_path.display(), e));
    let kb: KnowledgeBase = toml::from_str(&content)
        .unwrap_or_else(|e| panic!("Cannot parse {}: {}", kb_path.display(), e));

    if let Some(size) = sample_size {
        let state_path = sample_state_path(kb_path);
        let mut state = load_sample_state(&state_path);
        let sample = select_sample(&kb.exercises, size, &mut state);

        if let Err(e) = save_sample_state(&state_path, &state) {
            eprintln!(
                "Cannot save sample history at {}: {}",
                state_path.display(),
                e
            );
            return;
        }

        println!(
            "  Random review: {} unseen exercise{} selected ({} of {} reviewed this cycle).",
            sample.len(),
            if sample.len() == 1 { "" } else { "s" },
            state.reviewed_tasks.len(),
            kb.exercises.len()
        );
        run_quiz(sample);
        return;
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn exercise(level: &str, task: &str) -> Exercise {
        Exercise {
            level: level.to_string(),
            task: task.to_string(),
            hint: String::new(),
        }
    }

    fn exercises() -> Vec<Exercise> {
        vec![
            exercise("data-structures", "linked list"),
            exercise("data-structures", "hash table"),
            exercise("algorithms", "binary search"),
            exercise("algorithms", "breadth-first search"),
        ]
    }

    #[test]
    fn samples_do_not_repeat_until_the_cycle_is_complete() {
        let exercises = exercises();
        let mut state = SampleState::default();

        let first = select_sample(&exercises, 2, &mut state);
        let second = select_sample(&exercises, 2, &mut state);
        let first_tasks: HashSet<&str> = first.iter().map(|ex| ex.task.as_str()).collect();
        let second_tasks: HashSet<&str> = second.iter().map(|ex| ex.task.as_str()).collect();

        assert!(first_tasks.is_disjoint(&second_tasks));
        assert_eq!(state.reviewed_tasks.len(), exercises.len());

        let third = select_sample(&exercises, 2, &mut state);
        assert_eq!(third.len(), 2);
        assert_eq!(state.reviewed_tasks.len(), 2);
    }

    #[test]
    fn sample_includes_each_level_when_space_allows() {
        let exercises = exercises();
        let mut state = SampleState::default();

        let sample = select_sample(&exercises, 2, &mut state);
        let levels: HashSet<&str> = sample.iter().map(|ex| ex.level.as_str()).collect();

        assert_eq!(levels, HashSet::from(["data-structures", "algorithms"]));
    }

    #[test]
    fn final_sample_uses_only_the_unseen_remainder() {
        let exercises = exercises();
        let mut state = SampleState::default();

        let _ = select_sample(&exercises, 3, &mut state);
        let final_sample = select_sample(&exercises, 3, &mut state);

        assert_eq!(final_sample.len(), 1);
        assert_eq!(state.reviewed_tasks.len(), exercises.len());
    }
}
