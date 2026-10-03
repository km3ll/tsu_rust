//! # Method Syntax

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	// methods
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn width(&self) -> bool {
		self.width > 0
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

impl Rectangle {
	fn square(size: u32) -> Self {
		Self {
			width: size,
			height: size,
		}
	}
}

fn methods_definition() {
	let n1 = r#"
	pod: Methods
	- Defined within the context of a struct, enum or trait object
	- Their first parameter is always `self`, which represents the instance
	- The `self` is an alias for the type that the impl block is for
	- Can take ownership of self, borrow immutably or borrow mutably
	- `&mut self` is used to transform `self` into something else preventing usage of the original instance
	- Can have multiple parameters added after the 'self' parameter
	---
	pod: Implementation Block
	- Everything within this block will be associated with the type
	- Each struct is allowed to have multiple impl blocks
	---"#;
	println!("{n1}");

	let r1 = Rectangle {
		width: 20,
		height: 40,
	};
	let area = r1.area();
	println!("Methods: area: {area}");
}

fn methods_getter() {
	let n1 = r#"
	pod: Getter
	- Giving a method the same name as one of the struct's fields
	- When we use parentethis we mean the method, otherwise we mean the field
	---"#;
	println!("{n1}");

	let r1 = Rectangle {
		width: 25,
		height: 50,
	};
	println!(
		"Methods: nonzero width: {}, width: {}",
		r1.width(),
		r1.width
	);
}

fn methods_ref_deref() {
	let n1 = r#"
	pod: Automatic Referencing and Dereferencing
	- Calling methods is one of the few places in Rust with this behavior
	- Rust automatically adds in `&`, `&mut` or `*` so objects match the signature of methods
	- Example: `p1.distance(&p2)` | `(&p1).distance(&p2)`
	---"#;
	println!("{n1}");
}

fn methods_more_parameters() {
	let r1 = Rectangle {
		width: 100,
		height: 200,
	};
	let r2 = Rectangle {
		width: 20,
		height: 40,
	};
	println!(
		"Methods: multiple params: r1 can hold r2: {}",
		r1.can_hold(&r2)
	);
}

fn methods_associated_functions() {
	let n1 = r#"
	pod: Associated Functions
	- All functions defined within impl blocks are associated functions
	- We can define functions that don't have `self` as their first parameter
	- They aren't methods, because they don't need an instance of the type to work (String::from)
	- Often used for constructors, often called new()
	- `Self` is an alias for the type that appears after the 'impl' keyword
	---"#;
	println!("{n1}");

	let r1 = Rectangle::square(50);
	println!("Methods: associated function r1: {r1:#?}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_methods_definition() {
		methods_definition();
	}

	#[test]
	fn run_methods_getter() {
		methods_getter();
	}

	#[test]
	fn run_methods_ref_deref() {
		methods_ref_deref();
	}

	#[test]
	fn run_methods_more_parameters() {
		methods_more_parameters();
	}

	#[test]
	fn run_methods_associated_functions() {
		methods_associated_functions();
	}
}
