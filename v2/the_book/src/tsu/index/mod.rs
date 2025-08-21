use std::fs;
use std::path::Path;
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Note {
    pub content: String,
}

impl Note {
    pub fn new(lines: Vec<String>) -> Note {
        let content: String = lines.into_iter()
            .filter_map(|line| if is_invalid_line(&line) { None } else { Some(line) })
            .collect();
        Note { content }
    }
}

pub fn is_invalid_line(line: &String) -> bool {
    line.trim().is_empty() ||
    line.trim().starts_with('#') ||
    line.trim().starts_with("--")
}

pub fn load_file(file: &str) -> Vec<String> {
    let path = Path::new(file);
    let content = fs::read_to_string(path)
        .expect(&format!("Something went wrong reading file: {}", file));
    content.lines().map(|line| line.to_string()).collect()
}

pub fn list_candidates(line: &String) -> Vec<String> {
    let mut candidates: Vec<String> = line
        .split("-")
        .filter_map(|token| if is_candidate(token) { Some(token.trim().to_string()) } else { None })
        .map(|token| format_candidate(&token))
        .collect();
    candidates.sort();
    candidates.dedup();
    candidates
}

pub fn is_candidate(token: &str) -> bool {
    if (token.is_empty()) { return false; }
    let trimmed = token.trim();
    match trimmed.chars().next() {
        Some(c) => c.is_uppercase(),
        None => false
    }
}

pub fn format_candidate(token: &str) -> String {
    match token.split(":").next() {
        Some(token) => token.to_string(),
        None => "".to_string()
    }
}