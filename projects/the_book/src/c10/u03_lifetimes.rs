//! # Validating References with Lifetimes

use std::fmt::Display;

fn valid_lifetime() {
	let x = 5; // ----------+-- 'b
	//           |
	let r = &x; // --+-- 'a  |
	//   |       |
	println!("r: {r}"); //   |       |
	// --+       |
} // ----------+

fn longest_v1(x: &str, y: &str) {
	// Error: this function's return type contains a borrowed value,
	// but the signature does not say whether it is borrowed from `x` or `y`
	// if x.len() > y.len() { x } else { y }
}

fn longest_v2<'a>(x: &'a str, y: &'a str) -> &'a str {
	let n1 = r#"
	pod: Lifetime Annotations
	- Describe the relationships of the lifetimes of multiple references to each other
	- Do not affect lifetimes
	- Their names start with an apostrophe ('), are usually lowercased and very short
	  - &i32 = reference
	  - &'a i32 = refere with explicit lifetime
	  - &'a mut i32 = mutable reference with explicit lifetime
	---"#;
	println!("{n1}");
	if x.len() > y.len() { x } else { y }
}

#[derive(Debug)]
struct Excerpt<'a> {
	part: &'a str,
}

impl<'a> Excerpt<'a> {
	fn level(&self) -> i32 {
		30
	}

	fn announce(&self, msg: &str) -> &str {
		println!("Lifetimes: in method definitions msg: {msg}");
		self.part
	}
}

fn lifetimes_definition() {
	let n1 = r#"
	pod: Lifetimes
	- Ensure that references are valid as long as we need them to be
	- Every reference has a lifetime: the scope for which that reference is valid
	- Aim to prevent dangling references
	  - A referenced value goes out of scope before it's used
	  - The subject of the reference doesn't live as long as the reference
	---
	pod: Input Lifetimes
	- On function or method parameters
	---
	pod: Output Lifetimes
	- On return values
	---"#;
	println!("{n1}");
}

fn lifetimes_in_functions() {
	let n1 = r#"
	pod: Lifetime Annotations in Signatures
	- The returned reference will be valid as long as the parameter(s) are valid
	- We're specifying that the borrow checker should reject any values that don't adhere to these constraints
	- They become part of the contract of the function, much like types
	- The returned reference will also be valid for the length of the smaller of the lifetimes of x and y
	---"#;
	println!("{n1}");

	let x1 = "Hello";
	let y1 = "Ferris";
	let r1 = longest_v2(x1, y1);
	println!("Lifetimes: in signatures r1: {r1}");

	let x2: &str = "Hello Ferris";
	let r2: &str;
	{
		let y2: &str = "!";
		let r2 = longest_v2(x2, y2);
	}
	// Error: borrowed value r2 does not live long enough
	// println!("Lifetimes: in signatures r2: {r2}");
}

fn lifetimes_in_structs() {
	let n1 = r#"
	pod: Lifetimes in Structs
	- Structs can hold references but we need to add a lifetime annotation on every reference
	- An instance of the struct cannot outlive the reference it holds in its attributes
	---"#;
	println!("{n1}");

	let novel = String::from("Call me Ferris. Some years ago...");
	let sentence = novel.split(".").next().unwrap();
	{
		let excerpt = Excerpt { part: sentence };
		println!("Lifetimes: in structs excerpt: {excerpt:?}");
	}
}

fn lifetimes_elision() {
	let n1 = r#"
	pod: Lifetime Elision Rules
	- A set of particular cases that the compiler will consider, and if your code fits these cases, you don't need to write lifetimes explicitly
	- (1) The compiler assigns a different lifetime parameter to each parameter that is a reference
	- (2) If there is exactly one input lifetime parameter, that lifetime is asigned to all output lifetime parameters
	- (3) If there are multiple input lifetime parameters, but one of the is &self or &mut self because this is a method,
	  the lifetime of self is assigned to all output parameters.
	---"#;
	println!("{n1}");
}

fn lifetimes_in_methods() {
	let excerpt = Excerpt { part: "Hello." };
	excerpt.announce("Call me Ferris.");
}

fn lifetimes_static() {
	let n1 = r#"
	pod: Static Lifetime
	- The affected reference can live for the entire duration of the program
	- All the string literals have the 'static lifetime
	- The text of those strings is stored directly in the program's binary, which is always available
	---"#;
	println!("{n1}");

	let s1: &'static str = "I have a static lifetime.";
}

fn lifetimes_together() {
	fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
	where
		T: Display,
	{
		println!("Lifetimes: announcement ann: {ann}");
		if x.len() > y.len() { x } else { y }
	}
	println!("Lifetimes: all in one function");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_lifetimes_definition() {
		lifetimes_definition();
	}

	#[test]
	fn run_lifetimes_in_functions() {
		lifetimes_in_functions();
	}

	#[test]
	fn run_lifetimes_in_structs() {
		lifetimes_in_structs();
	}

	#[test]
	fn run_lifetimes_in_methods() {
		lifetimes_in_methods();
	}

	#[test]
	fn run_lifetimes_static() {
		lifetimes_static();
	}

	#[test]
	fn run_lifetimes_together() {
		lifetimes_together();
	}
}
