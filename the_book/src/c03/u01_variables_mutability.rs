//! # Variables and Mutability

use std::thread::Thread;

fn variables_core() {
	let n1 = r#"
	pod: Variables
	- Immutable by default
	- Adding `mut` conveys intent of changing its value
	---"#;
	println!("{n1}");

	let mut x = 5;
	x = 6;
	println!("Mutable value of x: {x}")
}

fn variables_constants() {
	let n1 = r#"
	pod: Constants
	- Declared using the `const` keyword
	- The type of value must be annotated
	- Can be declared in any scope
	- May be set to a constant expression, not the result of  value computed at runtime
	- Naming convention is to use all uppercase with underscore between words
	---"#;
	println!("{n1}");

	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
	println!("Const: {THREE_HOURS_IN_SECONDS}")
}

fn variables_shadowing() {
	let n1 = r#"
	pod: Shadowing
	- Declaring a new variable with the same name as a previous variable
	- The second variable takes any uses of the name until either it itself is shadowed or the scope ends
	---
	pod: Shadowing let keyword
	- We transform a value, but have immutability after the transformation is complete
	- We create a new variable, so we can change the type of the value
	---"#;
	println!("{n1}");

	let x = 5;
	let x = x + 1;
	{
		let x = x * 2;
		println!("Inner scope x (shadowed): {x}")
	}
	println!("Outer scope x: {x}");

	let spaces: &str = "    ";
	println!("Spaces: '{spaces}'");
	let spaces: usize = spaces.len();
	println!("Spaces (shadowed): '{spaces}'");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_variables_core() {
		variables_core();
	}

	#[test]
	fn run_variables_constants() {
		variables_constants();
	}

	#[test]
	fn run_variables_shadowing() {
		variables_shadowing()
	}
}
