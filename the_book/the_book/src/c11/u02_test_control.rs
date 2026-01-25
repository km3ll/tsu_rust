//! # Controlling How Tests Are Run

fn test_control_definition() {
	let n1 = r#"
	pod: Cargo test
	- Compiles code in test mode
	- Runs all the tests in parallel
	- Some conmmand line options go to 'cargo test' and others to the test binary
	- If a test passes, Rust captures anything printed to standard output by default
	- The module in which a test appears becomes part of the test's name
	- We can run all the tests in a module by filtering on the module's name
	- We can ignore a test unless specifically requested with #[ignore]
	---
	cmd:
	- cargo test --help
	- cargo test -- --help
	- cargo test -- --test-threads=1
	- cargo test -- --show-output
	- cargo test run_test_control_definition
	- cargo test -- --ignored
	- cargo test -- --include-ignored
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_test_control_definition() {
		test_control_definition();
	}

	#[test]
	#[ignore]
	fn run_test_control_ignored() {
		println!("Tests: ignored unless specifically requested");
	}
}
