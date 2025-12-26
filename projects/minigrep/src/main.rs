#![allow(unused)]
use minigrep::{search, search_case_insensitive};
use std::error::Error;
use std::{env, fs, process};

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
	- A call to panic!() is more appropriate for a programming problem than a usage problem
	- Many programmers expect 'new' functions to never fail
	- A nonzero exit status is a convention to signal to the process that called our program that the program exited with an error state
	- Programs panic by calling .expect() method
	---
	pod: Macro: unimplemented!()
	- Indicates unimplemented code by panicking with a message of "not implemented"
	---
	pod: Multi-line String
	- The backslash after the openning double quote tells Rust not to put a new line character at the beginning of the contents of this string literal
	---
	pod: Vector lifetimes
	- The data referenced by a slice needs to be valid for the reference to be valid
	- 'content' is the only parameter that should be connected to the return value using lifetime syntax
	---
	pod: Function: to_lowercase()
	- Calling it creates new data rather than referencing existing data
	---
	pod: Standard outputs in terminal
	- Standard output: for general information
	- Standard error: for error messages
	- This distinction enables users to choose to direct the successful output to a file but still print error messages to the screen
	---
	pod: Macro: eprintln!()
	- Prints to the standard error, with a newline
	---"#;
    println!("{n1}");
}

// pod: separating concerns in Binary projects
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("🦀 problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("🦀 searching for {}", config.query);
    println!("🦀 in file {}", config.file_path);

    // pod: handling error returned from 'run' in main
    if let Err(err) = run(config) {
        eprintln!("🦀 application error: {err}");
        process::exit(1);
    }
}

// pod: grouping configuration values
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // pod: creating a constructor for Config
    // pod: returning a Result instead of calling panic!
    /*
    fn new(args: &[String]) -> Self {
        // pod: improving the error message
        if args.len() < 3 {
            panic!("🦀 not enough arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
     */
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // pod: returning error from run
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/*
pod: extracting logic from main

fn run(config: Config) {
    // pod: returning error from run
    let contents = fs::read_to_string(config.file_path)
        .expect("🦀 should have been able to read the file");

    println!("🦀 with text:\n{contents}");
}

pod: extracting the argument parser

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path  = args[2].clone();

    Config { query, file_path }
}
 */
