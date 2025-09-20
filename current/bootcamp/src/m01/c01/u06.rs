// pod: The naming convention for functions is snake-case
pub fn functions_statement(x: u32) {
	// pod: Statements are instructions that do not return a value
	println!("Function called with: {}", x);
}

pub fn functions_expression(x: u32) -> u32 {
	// pod: Expressions are code that evaluate to a value
	let y: u32 = x * 2;

	/***
	 * pod: For a function to use the last expression as return type,
	 * we have to omit the semicolon. This syntax only works for
	 * the last expression in a function
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
