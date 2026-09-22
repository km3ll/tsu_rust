//! # Hello, Tests!

pub fn add_two(a: u64) -> u64 {
	let n1 = r#"
	---
	pod: #[cfg(test)]
	- Compiles and runs test code only when you run `cargo test`, not when you run `cargo build`

	pod: The tests Directory
	- Definet at the top level of projects, next to `src`
	- Cargo looks for integration test files in this directory
	- Cargo compiles each test file as an individual crate
	---"#;
	println!("{n1}");
	internal_adder(a, 2)
}

fn internal_adder(left: u64, right: u64) -> u64 {
	left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_add_two() {
		let r1: u64 = add_two(8);
		assert_eq!(10, r1);
	}
}
