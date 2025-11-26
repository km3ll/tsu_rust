use rand::Rng;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;

fn game_start() {
	println!("[ Guess the number ]");
	let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

	loop {
		println!("Please input your guess");
		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		match guess.cmp(&secret_number) {
			Less => println!("Too small"),
			Greater => println!("Too big"),
			Equal => {
				println!("You win!");
				break;
			}
		}
	}
}

fn game_match() {
	let guess = rand::thread_rng().gen_range(1..=100);
	let secret = rand::thread_rng().gen_range(1..=100);
	println!("Guess: {guess} vs secret: {secret}.");

	match guess.cmp(&secret) {
		Less => println!("Too small"),
		Greater => println!("Too big"),
		Equal => println!("You win"),
	}
}

fn game_v1() {
	let n1 = r#"
	pod: Prelude
	- Set of items defined in the standard library that it brings into the scope of every program
	---
	pod: Associated Function
	- A function that’s implemented on a type
	- The :: syntax indicates that 'new' is an associated function of the String type
	---
	pod: References
	- Identified with ampersand '&'
	- A way to let multiple parts of your code access one piece of data without copying it
	---
	pod: Enumeration
	- A type that can be in one of multiple possible states
	- We call each possible state a 'variant'
	---
	pod: Enumeration: Result
	- Result's variants are Ok and Err
	- expect(msg: &str)
	---
	pod: Placeholder
	- Set of curly brackets {}
	---
	pod: Crate
	- A collection of Rust source code files
	- Binary crate: executable code
	- Library crate: code intended to be used in other programs and can't be executed on its own
	---
	pod: Semantic Versioning
	- A standard for writing library version numbers
	---
	pod: Crates.io
	- Repository of open source Rust projects
	---
	pod: Registry
	- A local copy of data from Crates.io
	---
	pod: Cargo.lock
	- Stores all the versions of project dependencies
	---"#;
	println!("{n1}");
}

fn game_v2() {
	let n1 = r#"
	pod: Range:
	- Inclusive (1..=100)
	---
	pod: Ordering Enum
	- Less / Greater / Equal
	---
	pod: Method: cmp()
	- Compares two values and returns an Ordering type
	---
	pod: Match expression
	- Made up of arms
	- Arms are patterns to match against and the code that should be run
	- The underscore (_) is a catch-all value
	---
	pod: Shadowing
	- Lets reusing a variable name rather than creating two unique variables
	---
	pod: Method: trim()
	- Eliminates new-line (\n) and carriage-return (\r)
	---
	pod: Loop
	- break: exits the loop
	- continue: goes to the next iteration
	---
	cmd:
	- cargo doc --open (local documentation)
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_game_v1() {
		game_v1();
	}

	#[test]
	fn run_game_v2() {
		game_v2();
	}

	#[test]
	fn run_game_match() {
		game_match();
	}

	#[test]
	fn run_game_start() {
		// game_start();
	}
}
