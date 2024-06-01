#![allow(unused)]

use std::{fs, io, error, num::ParseIntError};

fn main() {   
    println!("Hello World!");
}

fn implementations() {
    
    println!("Parsing file...");
    match parse_file_v2("example.txt") {
        Ok(i) => println!("{i}"),
        Err(e) => {
            match e {
                ParseFileError::File => {
                    println!("Error parsing file");
                },
                ParseFileError::Parse(error) => {
                    println!("Error parsing content: {}", error);
                }
            }
        }
    }

}

// Another approach is to create a custom error Enum with the variants of the 
// different types of errors a function could return.
enum ParseFileError {
    File,
    Parse(ParseIntError) // tupple varian
}

fn parse_file_v2(filename: &str) -> Result<i32, ParseFileError>{

    // read_to_string and parse methods return different errors
    let s = 
        fs::read_to_string(filename)
            .map_err(|e| ParseFileError::File)?;
    let i =
        s.parse()
            .map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)

}

// The function could return any type that implements the error trait.
// The downside is that callers of our functions won't know what concrete
// error types are being returned at compile time, and therefore cannot 
// handle individual errors differently.
fn parse_file_v1(filename: &str) -> Result<i32, Box<dyn error::Error>>{

    // read_to_string and parse methods return different errors
    let s = fs::read_to_string(filename)?;
    let i = s.parse()?;
    Ok(i)

}