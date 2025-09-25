pub fn borrowing_definition() {
	/**
	 * pod: Borrowing
	 * - The act of creating a reference
	 * - References are pointers with rules/restrictions
	 * - References do not take ownership
	 *
	 * Why borrow?
	 * - Performance
	 * - When ownership is not needed/desired
	 *
	 * Rules
	 * - At any given time, you can have either one mutable reference
	 *   or any number of immutable references
	 * - References must always be valid
	 *
	 * References
	 * - Created with an ampersand
	 */
	println!("Borrowing");
}

pub fn borrowing_immutable_reference() {
	fn print_string(r1: &String) {
		println!("r1: {r1}")
	}

	/**
	 * pod: Immutable reference
	 */
	let s1: String = String::from("Ferris");
	let r1: &String = &s1;
	print_string(r1);
}

pub fn borrowing_mutable_reference() {
	fn add_to_string(r2: &mut String) {
		/**
		 * pod: Automatic Dereferencing
		 * - We don't need to dereference r2
		 * - The asterisk '*' is a dereference operator
		 */
		r2.push_str(" is awesome!");
		// (*r2).push_str(" is awesome!");
	}

	/**
	 * Mutable reference
	 * - In order to declare a mutable reference the variable itself
	 *   has to be mutable
	 */
	let mut s2: String = String::from("Ferris");
	let r2: &mut String = &mut s2;
	add_to_string(r2);
	println!("s2: {s2}");
}

pub fn borrowing_dangling_reference() {
	/**
	 * pod: Dangling reference
	 *
	 * fn generate_string() -> &String {
	 *   let s: String = String::from("Ferris");
	 *   &s //dangling reference
	 * } // s is dropped
	 *
	 * let s3 = generate_string();
	 */
	println!("Dangling reference")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_borrowing_definition() {
		borrowing_definition();
	}

	#[test]
	fn run_borrowing_immutable_reference() {
		borrowing_immutable_reference();
	}

	#[test]
	fn run_borrowing_mutable_reference() {
		borrowing_mutable_reference();
	}

	#[test]
	fn run_borrowing_dangling_reference() {
		borrowing_dangling_reference();
	}
}
