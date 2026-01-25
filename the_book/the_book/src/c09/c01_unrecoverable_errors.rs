//! # Unrecoverable Errors with panic!

fn unrecoverable_errors() {
	let n1 = r#"
	pod: Errors
	- Recoverable: to report the problem and retry the operation
	- Unrecoverable: to immediately stop the program
	---"#;
	println!("{n1}");
}

fn unrecoverable_definition() {
	let n1 = r#"
	pod: Macro: panic!
	- Panics print a failure message, unwind, clean up the stack, and quit
	---
	pod: Unwinding vs Aborting
	- Unwinding: Rust walks back up the stack and cleans up the data from each function it encounters
	- Aborting: Rust ends the program without cleaning up
	- You can configure this behavior in Cargo.toml file within 'profile' section
	---
	pod: Backtrace
	- A list of all the functions that have been called to get to his point
	- We can set the RUST_BACKTRACE env variable to get a backtrace
	---
	pod: Debug Symbols
	- Enabled by default when using 'cargo build' or 'cargo run' without the '--release' flag
	---"#;
	println!("{n1}");

	// [profile.release]
	// panic = 'abort'
}

fn unrecoverable_panic_macro() {
	println!("Unrecoverable: panic!");
	panic!("Crash and burn!")
}

fn unrecoverable_code() {
	let vec1 = vec![1, 2, 3];
	let v1 = vec1[99];
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_unrecoverable_errors() {
		unrecoverable_errors();
	}

	#[test]
	fn run_unrecoverable_definition() {
		unrecoverable_definition();
	}

	#[test]
	#[should_panic]
	fn run_unrecoverable_panic_macro() {
		unrecoverable_panic_macro();
	}

	#[test]
	#[should_panic]
	fn run_unrecoverable_code() {
		unrecoverable_code();
	}
}
