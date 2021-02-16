use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::process::Command;

fn lookup(commit_id: &str) -> String {
    let output = Command::new("git")
        .args(&["show", "--oneline", commit_id])
        .output()
        .expect("Failed to execute git blame");

    let re = Regex::new(r"(?i)Merge\s+(?:pull\s+request|pr)\s+\#?(\d+)").unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    for line in stdout.lines() {
        if let Some(id) = re
            .captures(line)
            .and_then(|cap| cap.get(1))
            .map(|s| s.as_str())
        {
            return format!("PR #{:<5}", id.to_string());
        }
    }

    commit_id.to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: git-blame-pr filename");
        return;
    }

    let mut cache: HashMap<String, String> = HashMap::new();

    let output = Command::new("git")
        .args(&["blame", "--first-parent", &args[1]])
        .output()
        .expect("Failed to execute git blame");

    let stdout = String::from_utf8(output.stdout).unwrap();
    for line in stdout.lines() {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        let commit_id = parts[0];
        let rest_parts: Vec<&str> = parts[1].splitn(2, ") ").collect();
        let source = rest_parts[1];

        if !cache.contains_key(commit_id) {
            let id = lookup(commit_id);
            cache.insert(commit_id.to_string(), id);
        }

        let value = cache.get(commit_id).unwrap();
        println!("{} {}", value, source);
    }
}
