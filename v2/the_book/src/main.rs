#![allow(unused)]
use std::fs;
use std::io::Result;
use std::path::Path;
// pod: Module Import
mod config;
mod c01;

fn main() {
	load_config();
	load_file();
}

fn load_config() {
	// Load configuration
	let cfg = config::AppConfig::from_file()
		.expect("Failed to load config file");
	println!("Server will run on {}:{}", cfg.server.host, cfg.server.port);
}

fn load_file() {
	let name = "resources/messages.txt";
	let path = Path::new(name);
	let content = fs::read_to_string(path)
		.expect(&format!("Something went wrong reading file: {}", name));
	let lines: Vec<&str> = content.lines().collect();

	println!("Line count: {}", lines.len());
	for line in lines {
		println!("{}", line);
	}
}