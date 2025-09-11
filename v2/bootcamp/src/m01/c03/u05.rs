pub fn matching_expression() {
	/**
	 * pod: Match Expression
	 * - Flow-control operator
	 * - Allows to compare a value against the series of patterns (match arms)
	 *   to determine which code path to execute
	 * - Exhaustive on match arms
	 * - Patterns can be literal values, ranges, variable names, wildcards, etc
	 */
	let age: u32 = 32;
	match age {
		// Literal value
		1 => println!("Happy 1st Birthday!"),
		// Range
		13..19 => println!("You're a teenager!"),
		// With binding value
		x => println!("You are {x} years old!"), // Catch-all pattern (no binding value)
		                                         // _ => println!("Other age")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_matching_expression() {
		matching_expression();
		assert!(true)
	}
}
