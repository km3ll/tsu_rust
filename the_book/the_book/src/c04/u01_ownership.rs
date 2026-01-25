//! # What is Ownership?

fn ownership() {
	let n1 = r#"
	pod: Ownership
	- A set of rules that govern how a Rust program manages memory
	- Ownership addresses:
	  - Keeping track of what parts of code are using what data on the heap
	  - Minimizing the amount of duplicate data on the heap
	  - Cleaning up unused data on the heap
	---
	pod: Ownership Rules
	- Each value has an 'owner'
	- There an only be one owner at a time
	- When the owner goes out of scope, the value is dropped
	---"#;
	println!("{n1}");
}

fn ownership_stack_and_heap() {
	let n1 = r#"
	pod: The Stack And The Heap
	- Parts of memory available to your code to use at runtime
	- Because pointers to the heap have a known, fixed size, you can store them on the stack
	- When you want the actual data, you must follow the pointer (stack) to the heap
	- Think of a server at a restaurant taking orders from many tables
	---"#;
	println!("{n1}");
}

fn ownership_stack() {
	let n1 = r#"
	pod: The Stack
	- LIFO: Last In First Out
	- Stores values in the order it gets them and removes the values in the opposite order
	- Pushing onto the stack: adding data
	- Popping off the stack: removing data
	- All data on the stack must have a known, fixed size
	- Pushing to the stack is faster because the allocator never has to search for a place to store new data
	- The location to store data is always at the top
	- Values passed to a function and the function's local variables get pushed onto the stack.
	- When a function is over, its values and variables get popped off the stack
	- Think of a stack of plates
	---"#;
	println!("{n1}");
}

fn ownership_heap() {
	let n1 = r#"
	pod: The Heap
	- Data with an unknown size at compile time or a size that might change must be stored on the heap
	- The heap is less organized: you request a certain amount of space
	- Accessing data in the heap is generally slower because you have to follow a pointer
	- Think of being seated at a restaurant
	---
	pod: Allocating On The Heap
	- The memory allocator
	  - (1) finds an empty spot that is big enough
	  - (2) marks it as being in use
	  - (3) returns a pointer, which is the address of that location
	---"#;
	println!("{n1}");
}

fn ownership_variable_scope() {
	let n1 = r#"
	pod: Scope
	- The range within a program for which an item is valid
	- When an item comes into scope, it is valid
	- It remains valid until it goes out of scope
	---"#;
	println!("{n1}");
}

fn ownership_string_type() {
	let n1 = r#"
	pod: String Type
	- Manages data allocated on the heap
	- Stores an amount of text that is unknown at compile time
	---
	pod: Memory Allocator
	- Memory must be requested from the allocator at runtime
	- We need a way of returning this memory to the allocator
	- We need to pair exactly one 'allocate' with exactly one 'free'
	- Rust calls the 'drop' function to return memory automatically at the closing curly bracket
	---
	pod: RAII
	- Resource Acquisition Is Initialization: a pattern of deallocating resources
	---"#;
	println!("{n1}");

	let s1 = String::from("Hello");
	println!("String immutable s1: {s1}");

	let mut s2 = String::from("Hello");
	s2.push_str(", pod!");
	println!("String mutable s2: {s2}");
}

fn ownership_literal_allocation() {
	let n1 = r#"
	pod: String Literal
	- The content is known at compile time, so the text is hardcoded directly into the final executable
	---"#;
	println!("{n1}");

	let s1: &str = "Greetings!";
	println!("Literal allocation: s1: {s1}");
}

fn ownership_string_allocation() {
	let n1 = r#"
	pod: String In Memory
	- A String is made up of three parts
	  - A pointer to the memory that holds the contents
	  - A length (memory used in bytes)
	  - A capacity (total amount received from allocator)
	- This group of data is stored on the stack
	- A memory region on the heap holds the contents
	- Assigning s1 to s2 copies the pointer (stack), not the actual data (heap)
	---
	pod: Double Free Error
	- When s2 and s1 go out of scope, they will both try to free the same memory
	- Rust considers s1 as no longer valid (moved)
	---
	pod: Move (Shallow Copy)
	- Copying the pointer, length, and capacity
	- Also invalidating the first variable
	- Rust will never automatically create 'deep' copies of data
	---"#;
	println!("{n1}");

	let s1 = String::from("Hello");
	let s2 = s1;
	let s3 = s2.clone();
	println!("String in memory: s2: {s2}, cloned s3: {s3}");
}

fn ownership_scope_assignment() {
	let n1 = r#"
	pod: Scope And Assignment
	- When assigning a completely new value to an existing variable, Rust frees the original variable's memory
	---"#;
	println!("{n1}");

	let mut s1 = String::from("Hello");
	s1 = String::from("Hola");
	println!("Scope assignment: s1: {s1}");
}

fn ownership_clone() {
	let n1 = r#"
	pod: Clone (Deep Copy)
	- The heap data does get copied
	---"#;
	println!("{n1}");

	let s1 = String::from("Hola");
	let s2 = s1.clone();
	println!("Clone: s1: {s1}, s2: {s2}");
}

fn ownership_stack_data_copy() {
	let n1 = r#"
	pod: Copy Trait
	- Types that have known size at compile time are stored on the stack, so copies are quick to make
	- If a type implements the Copy trait, variables that use it do not move
	- Cannot be used if the type, or any of its parts, implements the Drop trait
	- Types that implement Copy: integers, boolean, floating-point, char, tuples (with types implementing Copy as well)
	---"#;
	println!("{n1}");

	let x1 = 5;
	let x2 = x1;
	println!("Copied primitive: x1: {x1}, x2: {x2} ");
}

fn ownership_functions() {
	let n1 = r#"
	pod: Ownership And Functions
	- Passing a variable to a function will move or copy, just as assignment does
	---"#;
	println!("{n1}");

	fn takes_ownership(s1: String) {
		println!("Ownership: taken in function: s1: {s1}");
	}

	fn makes_copy(u1: u16) {
		println!("Ownership: copied in function: u1: {u1}");
	}

	let s = String::from("Hello");
	takes_ownership(s);
	// println!("s: {s}"); // value used after being moved

	let u1: u16 = 10;
	makes_copy(u1);
	println!("Ownership: original u1: {u1}");
}

fn ownership_return_values() {
	let n1 = r#"
	pod: Return Values
	- Returning values can also transfer ownership
	---"#;
	println!("{n1}");

	fn gives_ownership() -> String {
		String::from("Hello, pod!")
	}

	fn takes_and_gives_back(s1: String) -> String {
		println!("Ownership: taken and given: s1: {s1}");
		s1
	}

	let s1 = gives_ownership();
	println!("Ownership: given: s1: {s1}");

	let s2 = String::from("Hello!");
	let s3 = takes_and_gives_back(s2);
	println!("Ownership: given and taken: s3: {s3}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_ownership() {
		ownership();
	}

	#[test]
	fn run_ownership_stack_and_heap() {
		ownership_stack_and_heap();
	}

	#[test]
	fn run_ownership_stack() {
		ownership_stack();
	}

	#[test]
	fn run_ownership_heap() {
		ownership_heap();
	}

	#[test]
	fn run_ownership_variable_scope() {
		ownership_variable_scope();
	}

	#[test]
	fn run_ownership_string_type() {
		ownership_string_type();
	}

	#[test]
	fn run_ownership_literal_allocation() {
		ownership_literal_allocation();
	}

	#[test]
	fn run_ownership_string_allocation() {
		ownership_string_allocation();
	}

	#[test]
	fn run_ownership_scope_assignment() {
		ownership_scope_assignment();
	}

	#[test]
	fn run_ownership_clone() {
		ownership_clone();
	}

	#[test]
	fn run_ownership_stack_data_copy() {
		ownership_stack_data_copy();
	}

	#[test]
	fn run_ownership_functions() {
		ownership_functions();
	}

	#[test]
	fn run_ownership_return_values() {
		ownership_return_values();
	}
}
