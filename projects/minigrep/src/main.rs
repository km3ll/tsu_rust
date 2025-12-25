#![allow(unused)]
use std::{env, fs};

fn main() {
    let n1 = r#"
	pod: grep
	- [G]lobally search a [R]egular [E]xpression and [P]rint
	- Two hyphens in 'cargo run --' to indicate the following arguments are for our program
	---
	pod: args
	- The first value in the vector is the name of our binary
	---"#;

    let args: Vec<String> = env::args().collect();

    let query: &str = &args[1];
    let file_path: &str = &args[2];

    println!("🦀 searching for {query}");
    println!("🦀 in file {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("🦀 should have been able to read the file");

    println!("🦀 with text:\n{contents}");

}
