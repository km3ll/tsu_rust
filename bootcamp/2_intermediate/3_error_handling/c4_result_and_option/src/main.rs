#![allow(unused)]
use std::{fs, io};


fn main() {

    // Methods such as ok(), map(), and and_then() are called combinators.
    // Combinators are functions which perform operations on a value and
    // can even change the value. You can chain function calls with them.
    println!("Reading first line...");
    let first_line = read_first_line_v2("example.txt").unwrap();
    println!("{}", first_line);

}

fn read_first_line_v2(filename: &str) -> Option<String> {

    // We call the and_then function on the result type.
    // If we get the Ok() variant, the value stored inside that Ok variant
    // is passed to a clossurem, which in turn returns a result type.

    // The ok() method takes a result type and converts it to an option type
    fs::read_to_string(filename).ok().and_then(|s| {
        s.lines().next().map(|s| s.to_owned())
    })
    
}

fn read_first_line_v1(filename: &str) -> Result<Option<String>, io::Error> {

    // We call the and_then function on the result type.
    // If we get the Ok() variant, the value stored inside that Ok variant
    // is passed to a clossurem, which in turn returns a result type.

    fs::read_to_string(filename).map(|s| {
        s.lines().next().map(|s| s.to_owned())
    })
    
}

