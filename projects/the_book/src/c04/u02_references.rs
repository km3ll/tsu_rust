//! # References and Borrowing

fn length(s: &String) -> usize {
	s.len()
}

fn change(s: &mut String) {
	s.push_str(", pod!")
}

/*
fn dangle() -> &String {
	let s = String::from("Hello");
	&s
} // s goes out of scope and is
*/

fn references() {
	let n1 = r#"
	pod: Reference
	- An address we can follow to access some data
	- That data is owned by some other variable
	- Unlike a pointer, a reference is guaranteed to point to a valid value for the life of that reference
	- Represented by ampersand '&', allowing to refer some value without taking ownership of it
	- Actual value is not dropped when the reference stops being used
	- Just as variables, references are immutable by default
	---
	pod: Dereferencing
	- Represented by asterisk '*'
	---
	pod: Borrowing
	- The action of creating a reference
	---"#;
	println!("{n1}");

	let s1 = String::from("Hello, pod!");
	let r1 = length(&s1);
	println!("Reference: s1: {s1}");
	println!("Reference: r1: {r1}");
}

fn references_mutable() {
	let n1 = r#"
	pod: Mutable References
	- If you have a mutable reference to a value, you cannot have other references to that value
	- We can use curly brackets to create new scopes, allowing multiple non-simultaneous references
	---
	pod: Data Race
	- Happens when
	  - (1) Two or more pointers access the same data at the same time
	  - (2) At least one of the pointers is being used to write to the data
	  - (3) There's no mechanism being used to synchronize access to the data
	---
	pod: Multiple References
	- Multiple immutable references are allowed
	- We cannot have multiple mutable references simultaneously
	- We cannot have a mutable reference while we have an immutable one
	---"#;
	println!("{n1}");

	let mut s1 = String::from("Hello");
	change(&mut s1);
	println!("Reference: mutable s1: {s1}");

	let mut s2 = String::from("Greetings");
	{
		let r1: &mut String = &mut s2;
	} // r1 goes out of scope

	let r2: &mut String = &mut s2;
	println!("Reference: mutable r2: {r2}");
}

fn references_scope() {
	let n1 = r#"
	pod: Reference Scope
	- Reference scope starts from where it is introduced until the last time it is used
	- When scopes don't overlap, we can have immutable references and after that a mutable reference
	---"#;
	println!("{n1}");

	let mut s1 = String::from("Hello, Ferris!");

	let r1 = &s1;
	let r2 = &s1;
	println!("Reference: immutable r1: {r1}, r2: {r2}");
	// r1 and r2 will not be used after this point

	let r3 = &mut s1;
	println!("Reference: mutable r3: {r3}");
}

fn references_dangling() {
	let n1 = r#"
	pod: Dangling Pointer (Reference)
	- A pointer that references a location in memory given to someone else
	- In Rust, the compiler guarantees that references will never be dangling references
	---"#;
	println!("{n1}");

	//dangle();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_references() {
		references();
	}

	#[test]
	fn run_references_mutable() {
		references_mutable();
	}

	#[test]
	fn run_references_scope() {
		references_scope();
	}

	#[test]
	fn run_references_dangling() {
		references_dangling();
	}
}
