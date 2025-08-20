pub fn greet() {
	println!("Hello, Cargo!");
}

// pod: Module for testing
#[cfg(test)]
mod c01_tests {
	// pod: Import functions from parent module
	use super::*;

	#[test]
	fn greet_prints_message() {
		greet();
		assert!(true);
	}
}
