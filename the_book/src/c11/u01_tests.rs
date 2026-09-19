//! # How to Write Tests

fn add(left: u64, right: u64) -> u64 {
	left + right
}

fn greeting(name: &str) -> &str {
	"Hello!"
}

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

struct Guess {
	value: i32,
}

impl Guess {
	pub fn new(value: i32) -> Self {
		if value < 1 {
			panic!("Value must be greater than or equal to 1, got {value}")
		} else if value > 100 {
			panic!("Value must be less than or equal to 100, got {value}")
		};
		Guess { value }
	}
}

fn tests_definition() {
	let n1 = r#"
	pod: Correctness
	- The extend to which our code does what we intend it to do
	---
	pod: Attributes
	- Metadata about pieces of code such as 'derive' and 'test'
	---
	pod: Tests
	- Setup, run and assert
	- Fail when something in the test function panics
	- Each test runs in a new thread, when the main thread sees that a test thread has died, the test is marked as failed
	- `assert!`, `assert_eq!`, `assert_ne!`
	- Assertion macros print their arguments using 'Debug' formatting
	- The values being compared must implement the 'PartialEq' and 'Debug' traits
	- A should_panic test would pass even if the test panics for a different reason from the one we were expecting
	---
	pod: Result in Tests
	- Enables the usage of question mark operator (?) in the body of tests
	- Convenient for tests that should fail if any operation within them return an 'Err' variant
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4)
	}

	#[test]
	#[should_panic]
	fn it_panics() {
		panic!("Make this test panic!")
	}

	#[test]
	fn larger_should_hold_smaller() {
		let larger = Rectangle {
			width: 8,
			height: 7,
		};
		let smaller = Rectangle {
			width: 5,
			height: 1,
		};
		assert!(larger.can_hold(&smaller))
	}

	#[test]
	fn smaller_cannot_hold_larger() {
		let larger = Rectangle {
			width: 8,
			height: 7,
		};
		let smaller = Rectangle {
			width: 5,
			height: 1,
		};
		assert!(!smaller.can_hold(&larger))
	}

	#[test]
	fn assert_using_eq_and_ne() {
		let result = add(2, 2);
		assert_eq!(result, 4);
		assert_ne!(result, 0);
	}

	#[test]
	#[should_panic]
	fn adds_custom_message() {
		let result = greeting("Ferris");
		assert!(
			result.contains("Ferris"),
			"Greeting did not contain name, value was `{result}`"
		)
	}

	#[test]
	#[should_panic(expected = "greater than or equal to 1")]
	fn is_panics_expected_v1() {
		Guess::new(-8);
	}

	#[test]
	#[should_panic(expected = "less than or equal to 100")]
	fn is_panics_expected_v2() {
		Guess::new(290);
	}

	#[test]
	fn it_uses_result() -> Result<(), String> {
		let result = add(2, 2);
		if result == 4 {
			Ok(())
		} else {
			Err(String::from("Two plus two did not equal foud"))
		}
	}

	#[test]
	fn run_tests_definition() {
		tests_definition();
	}
}
