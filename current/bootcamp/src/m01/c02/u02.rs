pub fn ownership_strategy() {
	/**
	 * pod: Ownership
	 * A strategy to manage memory through a set of rules checked at compile time
	 *
	 * - Each value has a variable that is called its 'owner'
	 * - There can only be one owner at a time
	 * - When the owner gets out of scope, the value is dropped
	 */
	println!("Ownership");
}

pub fn ownership_scope() {
	/**
	 * s1: stack frame (pointer) to the value in the heap
	 * pod: actual string allocated on the heap
	 *
	 * s1 is the owner of the data stored on the heap
	 * when s1 goes out of scope, then the data is cleaned up
	 */
	let s1: String = String::from("Ferris"); // heap-allocated String
	println!("s1: {s1}");

	{
		let s2: String = String::from("Ferris");
	} // s2 is dropped
	 //println!("s2: {s2}"); error: cannot find value 's2' in this scope
}

pub fn ownership_moving() {
	/**
	 * pod: Moving ownership
	 * - Values are moved by default
	 * - s3 is invalidated
	 * - s4 is now the owner of the String
	 */
	let s3: String = String::from("Pod");
	let s4: String = s3;
	// println!("s3: {s3}"); error: borrow of moved value 's3'
	println!("s4: {s4}");
}

pub fn ownership_cloning() {
	/**
	 * pod: Cloning values
	 * - s5 has his own copy of s4 String
	 */
	let s4: String = String::from("Rust");
	let s5: String = s4.clone();
	println!("s4: {s4}");
	println!("s5: {s5}");
}

pub fn ownership_primitive_types() {
	/**
	 * pod: Primitive types
	 * - Primitives that are entirely stored on the stack such as:
	 *   integers, floating point numbers, booleans or characters
	 *   are cloned by default.
	 * - These types are cheap to clone so there's no material
	 *   difference between cloning and moving the values.
	 */
	let x: i8 = 10;
	let y: i8 = x;
	println!("x: {x}");
	println!("y: {y}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_ownership_strategy() {
		ownership_strategy();
	}

	#[test]
	fn run_ownership_scope() {
		ownership_scope();
	}

	#[test]
	fn run_ownership_moving() {
		ownership_moving();
	}

	#[test]
	fn run_ownership_cloning() {
		ownership_cloning();
	}

	#[test]
	fn run_ownership_primitive_types() {
		ownership_primitive_types();
	}
}
