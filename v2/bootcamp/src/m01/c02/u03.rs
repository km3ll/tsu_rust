pub fn ownership_pass_into_function() {
	fn print_string(p1: String) {
		println!("p1: {p1}")
	} // p1 is dropped here

	/**
	 * pod: Moving ownership INTO functions
	 * - Values are moved by default
	 * - Passing a variable into a function has the same effect
	 *   as assigning one variable to another variable
	 */
	let s1: String = String::from("John");
	print_string(s1);
	// println!("s1: {s1}"); error: borrow of moved value 's1'
}

pub fn ownership_clone_into_function() {
	fn print_string(p1: String) {
		println!("p1: {p1}")
	} // p1 is dropped here

	/**
	 * pod: Cloning before calling a function
	 * - Instead of moving ownership, we first cloine s2 and then
	 *   move ownership of that clone into the function
	 */
	let s2: String = String::from("Wick");
	print_string(s2.clone());
	println!("s2: {s2}");
}

pub fn ownership_out_of_function() {
	fn generate_string() -> String {
		String::from("Ferris")
	}

	/**
	 * pod: Moving ownership OUT of functions
	 * - Ownership of the generated function is tranferred to s3
	 */
	let s3: String = generate_string();
	println!("s3: {s3}");
}

pub fn ownership_in_and_out_functions() {
	fn add_to_string(mut p1: String) -> String {
		// pod: use method 'push_str' to append to a String
		// p1.push_str(" is awesome!"); error: cannot borrow 'p1' as mutable
		p1.push_str(" is awesome!");
		p1
	}

	/**
	 * pod: Functions taking ownership and giving it back
	 */
	let s1: String = String::from("Ferris");
	let s2 = add_to_string(s1);
	println!("s2: {s2}")
}

pub fn ownership_stack_datatypes() {
	fn print_integer(i1: i32) {
		println!("i1: {i1}");
	}

	/**
	 * pod: Stack-Only data types
	 * - The value of x was clone into z
	 * - The value of x was also cloned into i
	 */
	let x: i32 = 30;
	let z: i32 = x;
	print_integer(x);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_ownership_pass_into_function() {
		ownership_pass_into_function();
		assert!(true)
	}

	#[test]
	fn run_ownership_clone_into_function() {
		ownership_clone_into_function();
		assert!(true)
	}

	#[test]
	fn run_ownership_out_of_function() {
		ownership_out_of_function();
		assert!(true)
	}

	#[test]
	fn run_ownership_in_and_out_functions() {
		ownership_in_and_out_functions();
		assert!(true)
	}

	#[test]
	fn run_ownership_stack_datatypes() {
		ownership_stack_datatypes();
		assert!(true)
	}
}
