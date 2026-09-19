//! # Multiple Error Types

use std::{error, fs, io, num::ParseIntError};

fn parse_file_v1(filename: &str) -> Result<i32, io::Error> {
    let s = fs::read_to_string(filename)?;
    // error: couldn't convert the error to io::Error
    // let i = s.parse()?;
    let i = 20;
    Ok(i)
}

fn parse_file_v2(filename: &str) -> Result<i32, Box<dyn error::Error>> {
    let s = fs::read_to_string(filename)?;
    // let i = s.parse()?;
    let i = 20;
    Ok(i)
}

#[derive(Debug)]
enum ParseFileError {
    File,
    Parse(ParseIntError),
}

fn parse_file_v3(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename).map_err(|e| ParseFileError::File)?;
    let i = s.parse().map_err(|e| ParseFileError::Parse(e))?;
    Ok(i)
}

fn error_types() {
    let n1 = r#"
    pod: Error Types with Trait
    - Any type that implements the Error trait
    - The `?` operator converts concrete errors into trait object
    - Callers won't know what concrete errors are being returned
    - Used when callers don't care about specific errors
    ---
    pod: enum: Error
    - Method: map_err()
    ---"#;
    println!("{n1}");
}

fn multiple_error_types_trait_object() {
    println!("multiple-errors: trait object");
}

fn multiple_error_types_error_enum() {
    let r1: Result<i32, ParseFileError> = parse_file_v3("example.txt");
    match r1 {
        Ok(i) => println!("i: {i}"),
        Err(e) => match e {
            ParseFileError::File => {
                println!("error: File")
            }
            ParseFileError::Parse(e) => {
                println!("error: {}", e)
            }
        },
    }
}

fn multiple_error_types_() {
    println!("Base");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_error_types() {
        error_types()
    }

    #[test]
    fn run_multiple_error_types_trait_object() {
        multiple_error_types_trait_object();
    }

    #[test]
    fn run_multiple_error_types_error_enum() {
        multiple_error_types_error_enum();
    }

    #[test]
    fn run_() {
        multiple_error_types_();
    }
}
