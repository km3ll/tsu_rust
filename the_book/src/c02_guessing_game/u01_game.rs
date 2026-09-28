//! # Programming a Guessing Game

use rand::Rng;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;

fn game() {
	let n1 = r#"
	---
	pod: Prelude
	- Rust comes with a variety of things in its standard library. A balance needs to be struck.
	- The list of things that Rust automatically imports into every Rust program.

	pod: Associated Function
	- A function that is implemented on a type
	- The `::` syntax indicates that 'new' is an associated function of the String type

	pod: References
	- Identified with ampersand `&`
	- A way to let multiple parts of your code access one piece of data without copying it

	pod: Enumeration
	- A type that can be in one of multiple possible states
	- We call each possible state a `variant`

	pod: Enumeration: Result
	- Result's variants are `Ok` and `Err`

	pod: Placeholder
	- Set of curly brackets `{}`

	pod: Crate
	- A collection of Rust source code files
	- Binary crate: executable code
	- Library crate: code intended to be used in other programs and can't be executed on its own

	pod: Semantic Versioning
	- A standard for writing library version numbers

	pod: Crates.io
	- Repository of open source Rust projects

	pod: Registry
	- A local copy of data from `crates.io`

	pod: Cargo.lock
	- Automatically generated file
	- Stores all the versions of project dependencies
	- Serves as a deterministic snapshot of the resolved dependency graph at the time of a successful build
	- Ensures `reproducibility`: subsequent builds on any machine use the precise same dependency versions

	pod: Range
	- Inclusive range: `(1..=100)`

	pod: Ordering Enum
	- Variants: `Less`, `Greater`, and `Equal`

	method: `std::cmp()`
	- Compares two values and returns an `Ordering` type

	pod: Match Expression
	- Made up of arms
	- Arms are patterns to match against and the code that should be run
	- The underscore `_` is a `catch-all` value

	pod: Shadowing
	- Lets reusing a variable name rather than creating two unique variables

	method: `str::trim()`
	- Eliminates new-line (\n) and carriage-return (\r)

	pod: Loop
	- `break` exits the loop
	- `continue` goes to the next iteration

	cmd:
	- `cargo doc --open` # Opens the docs in a browser after the operation
	---"#;
	println!("{n1}");
}

fn game_match() {
	let guess = rand::thread_rng().gen_range(1..=100);
	let secret = rand::thread_rng().gen_range(1..=100);
	println!("guess: {guess} vs secret: {secret}");

	match guess.cmp(&secret) {
		Less => println!("Too small"),
		Greater => println!("Too big"),
		Equal => println!("You win"),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_game() {
		game();
	}

	#[test]
	fn run_game_match() {
		game_match();
	}
}
