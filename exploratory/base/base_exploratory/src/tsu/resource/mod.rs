use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct File {
	pub lines: Vec<String>,
}

impl File {
	pub fn from_file(file: &str) -> File {
		let lines = load_lines(file);
		File { lines }
	}
}

pub fn load_lines(file: &str) -> Vec<String> {
	let path = Path::new(file);
	let content =
		fs::read_to_string(path).expect(&format!("Something went wrong reading file: {}", file));
	content.lines().map(|line| line.to_string()).collect()
}
