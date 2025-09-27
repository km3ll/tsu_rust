/**
 * pod: Naming convention
 * - Snake-case for functions
 */
pub fn functions_statement(x: u32) {
	/**
	 * pod: Statements
	 * - Instructions that do not return a value
	 */
	println!("Function called with: {}", x);
}

pub fn functions_expression(x: u32) -> u32 {
	/**
	 * pod: Expressions
	 * - Code that evaluate to a value
	 */
	let y: u32 = x * 2;

	/***
	 * pod: Function's return type
	 * - For a function to use the last expression as return type,
	 *   we have to omit the semicolon. This syntax only works for
	 *   the last expression in a function
	 */
	println!("New value of y: {}", y);
	y
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_functions_statement() {
		functions_statement(11);
	}

	#[test]
	fn run_functions_expression() {
		functions_expression(11);
	}
}
