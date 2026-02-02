//! # Treating Smart Pointers Like Regular References

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}

impl<T> Deref for MyBox<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

fn greet(name: &str) {
	println!("Deref: Hello, {name}!");
}

fn dereference() {
	let n1 = r#"
	pod: trait: Deref
	- Allows to customize the behavior of the 'dereference operator' (*)
	- A pointer is like an arrow to a value stored somewhere else
	- A number and a reference to a number are different types
	---
	pod: Deref Coercion
	- Converts a reference to a type that implements the Deref trait into a reference for another type
	- It happens automatically on arguments to functions or methods
	- Allows to write code that works for either references or smart pointers
	- Rust uses deref as many times as necessary to get a reference to match a parameter's type
	- Is resolved at compile time, so there is no runtime penalty
	---
	pod: Deref Coercion Cases
	- From &T to &U when T: Deref<Target=U>
	- From &mut T to & mut U when T: DerefMut<Target=U>
	- From &mut T to &U when T: Deref<Target=U>
	---"#;
	println!("{n1}");

	let x: i32 = 5;
	let y: &i32 = &x;
	println!("Deref: y: {y} -> x: {x}");
}

fn dereference_box() {
	let n1 = r#"
	pod: Box<T> Like Reference
	- Using a Box pointing to a copied value of x
	---"#;
	println!("{n1}");
	let x: i32 = 5;
	let y: Box<i32> = Box::new(x);

	println!("Deref: Box<y>: {y} -> x: {:?}", y);
	assert_eq!(5, *y)
}

fn dereference_my_box() {
	let n1 = r#"
	pod: MyBox
	- To enabling dereferencing we implement the Deref trait
	- We don't want to take ownership of the inner value inside MyBox<T>
	- `*y = *(y.deref())`
	---"#;
	println!("{n1}");

	let x = 5;
	let y: MyBox<i32> = MyBox::new(x);

	assert_eq!(5, *y);
	assert_eq!(5, *(y.deref()));
}

fn dereference_deref_coercion() {
	let n1: MyBox<&str> = MyBox::new("World");
	let n2: String = String::from("Pod");
	let n3: &str = "Ferris";

	greet(&n1);
	greet(&n2);
	greet(n3);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_dereference() {
		dereference();
	}

	#[test]
	fn run_dereference_box() {
		dereference_box()
	}

	#[test]
	fn run_dereference_my_box() {
		dereference_my_box()
	}

	#[test]
	fn run_dereference_deref_coercion() {
		dereference_deref_coercion()
	}
}
