//! # Command Line Program
//!
//! - Accepting Command Line Arguments
//! - Reading a File
//! - Refactoring to Improve Modularity and Error Handling
//! - Adding Functionality with Test Driven Development (TDD)
//! - Working with Environment Variables
//! - Redirecting Errors to Standard Error

fn minigrep() {
	let n1 = r#"
	pod: grep
	- [G]lobally search a [R]egular [E]xpression and [P]rint
	- Two hyphens in 'cargo run --' to indicate the following arguments are for our program
	---
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_minigrep() {
		minigrep();
	}
}
