use std::{error, fs, io, num::ParseIntError};

fn parse_file_v1(filename: &str) -> Result<i32, io::Error> {
	let s = fs::read_to_string(filename)?;
	// error: couldn't convert the error to io::Error
	// let i = s.parse()?;
	let i = 20;
	Ok(i)
}

/**
 * pod: Error types with trait
 * - Any type that implements the Error trait
 * - The ? operator converts concrete errors into trait object
 * - Callers won't know what concrete error are being returned
 * - Used then callers don't care about specific errors
 */
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

/**
 * pod: Error enum
 * - map_err()
 */
fn parse_file_v3(filename: &str) -> Result<i32, ParseFileError> {
	let s = fs::read_to_string(filename).map_err(|e| ParseFileError::File)?;
	let i = s.parse().map_err(|e| ParseFileError::Parse(e))?;
	Ok(i)
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
