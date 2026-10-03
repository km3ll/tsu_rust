// pod: Disable warning of unused code
#![allow(unused)]

pub fn greet() {
	println!("Hello, pod!")
}

// pod: Module for testing
#[cfg(test)]
mod basic {
	// pod: Import functions from parent module
	use super::*;

	#[test]
	fn greet_prints_message() {
		greet();
		assert!(true)
	}
}
