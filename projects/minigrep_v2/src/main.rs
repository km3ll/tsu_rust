#![allow(unused)]
use minigrep_v2::{search, search_case_insensitive};
use std::error::Error;
use std::{env, fs, process};

// pod: separating concerns in Binary projects
fn main() {
    let n1 = r#"
    pod: Iterator
    - By using env::args() we're passing ownership of the iterator to Config::build directly
    - 'args' can be any type that implements the Iterator trait and returns String items
    ---"#;
    println!("{n1}");
    
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("🦀 Problem parsing arguments: {err}");
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
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        let n1 = r#"
        pod: args
        - The first value in env::args() is the name of the program. 
        - We want to ignore that and get to the next value.
        ---"#;
        println!("{n1}");
        args.next();
 
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path")
        };
        
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
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