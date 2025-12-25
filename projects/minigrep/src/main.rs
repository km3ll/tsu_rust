#![allow(unused)]
use std::{env, fs};

fn minigrep() {
    let n1 = r#"
	pod: grep
	- [G]lobally search a [R]egular [E]xpression and [P]rint
	- Two hyphens in 'cargo run --' to indicate the following arguments are for our program
	---
	pod: args
	- The first value in the vector is the name of our binary
	---
	pod: problems
	- (1) The main function has multiple resposabilities
	- (2) Configuration variables are not grouped together
	- (3) Error message does not share detailed information
	- (4) Error-handling code is defined in multiple places
	---
	pod: errors
	- A call to panic!() is more appropriate for a programming problem thatn a usage problem
	---"#;
    println!("{n1}");
}

// pod: separating concerns in Binary projects
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("🦀 searching for {}", config.query);
    println!("🦀 in file {}", config.file_path);

    let contents =
        fs::read_to_string(config.file_path).expect("🦀 should have been able to read the file");

    println!("🦀 with text:\n{contents}");
}

// pod: grouping configuration values
#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // pod: creating a constructor for Config
    fn new(args: &[String]) -> Self {
        // pod: improving the error message
        if args.len() < 3 {
            panic!("🦀 not enough arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}

/*
pod: extracting the argument parser

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path  = args[2].clone();

    Config { query, file_path }
}
 */
