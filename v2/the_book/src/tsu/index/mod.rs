use std::fs;
use std::path::Path;
use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Note {
    pub content: Vec<String>,
}

impl Note {
    pub fn new(lines: Vec<String>) -> Note {
        let content = lines.into_iter()
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