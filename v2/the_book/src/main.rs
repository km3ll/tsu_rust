#![allow(unused)]
use std::fs;
use std::io::Result;
use std::path::Path;
use tsu::config::AppConfig;
use crate::tsu::index::*;
use std::collections::HashSet;

// config: Module Import
mod tsu;

fn main() {
	let config: AppConfig = load();
	start(config);
}

fn load() -> AppConfig {
	AppConfig::from_file().expect("Failed to load config file")
}

fn start(config: AppConfig) {
	// tags
	let characters = load_file("resources/index/tag/character.txt");
	let concepts = load_file("resources/index/tag/concept.txt");
	let fruits = load_file("resources/index/tag/fruit.txt");
	let places = load_file("resources/index/tag/place.txt");
	let techniques = load_file("resources/index/tag/technique.txt");

	// note
	let lines = load_file("resources/index/one-piece/note.txt");
	let note = Note::new(lines);
	let candidates = list_candidates(&note.content);

	let checked = uncommon_elements(characters, candidates);
	let checked = uncommon_elements(places, checked);
	let checked = uncommon_elements(techniques, checked);
	let checked = uncommon_elements(fruits, checked);
	let checked = uncommon_elements(concepts, checked);

	println!("Uncategorized: ");
	for i in checked {
		println!(" - {}", i);
	}
}

fn uncommon_elements(a: Vec<String>, b: Vec<String>) -> Vec<String> {
	let set_a: HashSet<_> = a.into_iter().collect();
	let set_b: HashSet<_> = b.into_iter().collect();
	set_a
		.symmetric_difference(&set_b) // items in A or B but not both
		.cloned()                     // clone &String → String
		.collect()
}