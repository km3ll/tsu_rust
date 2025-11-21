use std::io;

fn game_start() {
	println!("Guess the number!");
	println!("Please input your guess:");

	let mut guess = String::new();
	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");
	println!("You guessed: {guess}");
}

fn game_() {
	let n1 = r#"
	pod: 2.0 Guessing Game

	pod: Prelude
	- Set of items defined in the standard library that it brings into the scope of every program

	pod: Associated Function
	- A function that’s implemented on a type
	- The :: syntax indicates that 'new' is an associated function of the String type

	pod: References
	- Identified with ampersand '&'
	- A way to let multiple parts of your code access one piece of data without copying it

	pod: Enumeration
	- A type that can be in one of multiple possible states
	- We call each possible state a variant

	pod: Enumeration: Result
	- Result's variants are Ok and Err
	- expect(msg: &str)

	pod: Placeholder
	- Set of curly brackets {}

	pod: Crate
	- A collection of Rust source code files

	pod: Binary Crate
	- Executable code

	pod: Library Crate
	- Code that is intended to be used in other programs and can't be executed on its own.

	pod: Semantic Versioning
	- A standard for writing library version numbers

	pod: Crates.io
	- Repository of open source Rust projects

	pod: Registry
	- A local copy of data from Crates.io
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		game_();
	}

	#[test]
	fn run_game_start() {
		// game_start();
	}
}
