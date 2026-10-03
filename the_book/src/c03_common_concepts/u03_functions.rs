//! # Functions

fn functions_core() {
	let n1 = r#"
	---
	pod: Snake Case
	- Conventional style for function and variable names
	---"#;
	println!("{n1}");
}

fn functions_statement() {
	let n1 = r#"
	---
	pod: Statements
	- Instructions that perform some action and do not return a value
	- Cannot assign a `let` statement to another variable
	- Adding a semicolon to the end of an expression turns it into a statement
	---"#;

	println!("Statement");
	let x = 6;
	println!(" > x: {x}");
}

fn functions_expression() {
	let n1 = r#"
	---
	pod: Expressions
	- Evaluate to a resultant value. Examples: `functions`, `macros`, and `scope-blocks`
	- Do not include ending semicolons (;)
	---"#;
	println!("{n1}");

	println!("Expression");
	let y = {
		let x = 3;
		x + 1
	};
	println!(" > y: {y}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_functions_statement() {
		functions_statement();
	}

	#[test]
	fn run_functions_expression() {
		functions_expression();
	}
}
