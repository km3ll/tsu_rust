//! # Using Box<T> to Point to Data on the Heap

use crate::c15::u01_box_smart_pointer::List::{Cons, Nil};

fn smart_pointers() {
	let n1 = r#"
	pod: Pointer
	- A variable that contains an address in memory.
	- This address 'points at' some other data
	---
	pod: Smart Pointer
	- Data structure that acts like a pointer and also has additional medatada and capabilities
	- Owns the data it points to
	---
	pod: trait: Deref
	- Allows smart pointers to behave like a pointer so your code works with either references or smart pointers
	---
	pod: trait: Drop
	- Allows smart pointers to customize the code that runs when a smart pointer goes out of scope
	---"#;
	println!("{n1}");
}

fn box_smart_pointer() {
	let n1 = r#"
	pod: Box<T>
	- Allows to store data on the heap
	- What remains on the stack is the pointer to the heap data
	- Used when:
	  - You have a type whose size can't be known at compile time, and you want to use a value of that type in a context that requires an exact size
	  - You have a large amount of data, and you want to transfer ownership but ensure that the data won't be copied
	  - You want to own a value, and you care only that it's a type that implements a particular trait rather than being of a specific type
	- A box pointer's size doesn't change based on the amount of data it's pointing to.
	---"#;
	println!("{n1}");

	let b1: Box<i32> = Box::new(10);
	println!("Box: b1: {b1}");

	{
		let b2: Box<String> = Box::new(String::from("Hello, Ferris"));
		println!("Box: b2: {b2}");
	} // The box (stack) and the data (heap) are both dropped here
}

fn box_memory_allocation() {
	enum Message {
		Quit,
		Move { x: i32, y: i32 },
		Write(String),
		ChangeColor(i32, i32, i32),
	}

	let n1 = r#"
	pod: Memory Allocation
	- Rust goes through each of the variants to see which variant needs the most space
	- Because only one variant will be used, the most space a Message value will need is the space it would take to store the largest of its variants
	---"#;
	println!("{n1}");
}

#[derive(Debug)]
enum List {
	Cons(i32, Box<List>),
	Nil,
}

fn box_recursive_types() {
	let n1 = r#"
	pod: Recursive Types with Box
	- Rust can't know how much space a recursive type needs
 	- The nesting of values of recursive types could theoretically continue infinitely
 	---
 	pod: Cons List
 	- A data structure that is made up of nested pairs (current value, next pair)
 	- List of  1, 2, 3 = (1, (2, (3, Nil)))
 	---
 	pod: Indirection
 	- Instead of storing a value directly, we should change the data structure to store the value indirectly by storing a pointer to the value instead
 	- Cons size = i32 + Box< usize >
	---"#;
	println!("{n1}");

	let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
	println!("Box: list: {:#?}", list);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_smart_pointers() {
		smart_pointers();
	}

	#[test]
	fn run_box_smart_pointer() {
		box_smart_pointer();
	}

	#[test]
	fn run_box_memory_allocation() {
		box_memory_allocation()
	}

	#[test]
	fn run_box_recursive_types() {
		box_recursive_types();
	}
}
