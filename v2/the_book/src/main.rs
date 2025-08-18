#![allow(unused)]
use std::fs;
use std::io::Result;
use std::path::Path;
use tsu::config::AppConfig;
use crate::tsu::index::{load_file, Note};

// config: Module Import
mod tsu;

fn main() {
	let config: AppConfig = load();
	start(config);
	//load_note("resources/index/one-piece/note.txt");
	//let lines = load_file("resources/index/tag/character.txt");
	//for line in &lines {
	//	println!("{}", line);
	//}
	//let note = Note::new(lines);

}

fn load() -> AppConfig {
	AppConfig::from_file().expect("Failed to load config file")
}

fn start(config: AppConfig) {
	// tags
	let characters = load_file("resources/index/tag/character.txt");
	let places = load_file("resources/index/tag/place.txt");
	let technique = load_file("resources/index/tag/technique.txt");

	// note
	let lines = load_file("resources/index/one-piece/note.txt");
	let note = Note::new(lines);
	for line in note.content {
		println!("{}", line);
	}
}