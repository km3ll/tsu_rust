use std::{fs, io};

/**
 * pod: Closures
 * - Anonymous functions that you can pass into other functions
 */
fn read_first_line_v1(filename: &str) -> Result<Option<String>, io::Error> {
	// mismatched error
	// fs::read_to_string(filename).and_then(|s| {
	//   s.lines().next().map(|s| s.to_owned())
	// })
	fs::read_to_string(filename).map(|s| s.lines().next().map(|s| s.to_owned()))
}

/**
 * pod: Combinators
 * - Functions that perform operations on a value
 * - Can even change the value
 * - Allow to chain function calls
 *   - and_then()
 *   - map()
 *   - ok() converts a result into an option  
 */
fn read_first_line_v2(filename: &str) -> Option<String> {
	// mismatched error
	// fs::read_to_string(filename).and_then(|s| {
	//   s.lines().next().map(|s| s.to_owned())
	// })
	fs::read_to_string(filename)
		.ok()
		.and_then(|s| s.lines().next().map(|s| s.to_owned()))
}

fn result_and_option_with_map() {
	let r1: Result<Option<String>, io::Error> = read_first_line_v1("example.txt");
	println!("r1: {:?}", r1)
}

fn result_and_option_with_ok_and_then() {
	let r2: Option<String> = read_first_line_v2("example.txt");
	println!("r2: {:?}", r2)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_result_and_option_with_map() {
		result_and_option_with_map();
	}

	#[test]
	fn run_result_and_option_with_ok_and_then() {
		result_and_option_with_ok_and_then();
	}
}
