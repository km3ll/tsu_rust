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
	- Because pointers to the heap has a known, fixed size, you can store them on the stack,
	  but when you want the actual data, you must follow the pointer
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
	- Pushing to the stack is faster because the allocator never has to search for a place to
	  store new data: the location is always at the top
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
	// TODO
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
}
