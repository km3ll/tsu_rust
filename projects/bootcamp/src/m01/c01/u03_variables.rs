#[allow(unused)]

pub fn variable_creation() {
	let n1 = r#"
	pod: Immutability
	- Variables are immutable by default
	---"#;
	println!("{n1}");

	let a1: i16 = 5;
	let a2: f32 = 5.0;
}

pub fn variable_mutability() {
	let n1 = r#"
	pod: Mutability
	- Add 'mut' keyword after 'let' to modify a single variable
	---"#;
	println!("{n1}");

	let mut m1: i16 = 4;
	m1 = 6;
}

pub fn variable_shadowing() {
	let n1 = r#"
	pod: Shadowing
	- You create two separate variables
	---"#;
	println!("{n1}");

	let s1: i32 = 10;
	let s1: i32 = 20;
}

pub fn variable_scope() {
	let n1 = r#"
	pod: Scopes
	- Outer
	- Inner
	  - Variable lives within the scope of brackets {}
	---"#;
	println!("{n1}");

	let d1: i16 = 40;
	{
		let d1: i16 = 30;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_variable_creation() {
		variable_creation();
		assert!(true)
	}

	#[test]
	fn run_variable_mutability() {
		variable_mutability();
	}

	#[test]
	fn run_variable_scope() {
		variable_scope();
	}
}
