//! # Rc<T> The Reference Counted Smart Pointer

use crate::c15::u04_rc_smart_pointer::List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug)]
enum List {
	Cons(i32, Rc<List>),
	Nil,
}

fn rc_smart_pointer() {
	let n1 = r#"
	pod: Rc<T> Reference Counting
	- Keeps track of the number of immutable references to a value to determine whether or not the value is still in use.
	- When the last person leaves the room, they turn off the TV
	- To allocate some data on the heap for multiple parts to read and we can't determine at compile time which part will finish using the data last
	- Is only for use in single-threaded scenarios
	- Example: two lists that both share ownership of a third list
	- Every time we call Rc::clone, the reference count to the data within Rc increases
	- The call to Rc::clone only increments the reference count
	- The implementation of the Drop trait decreases the reference count automatically when an Rc value goes out of scope
	---"#;
	println!("{n1}");
}

fn rc_cons() {
	println!("Rc: counts of a");

	let a: Rc<List> = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
	println!(" > after a: {}", Rc::strong_count(&a));

	{
		let b = Cons(3, Rc::clone(&a));
		println!(" > after b inner scope: {}", Rc::strong_count(&a));
	}

	println!(" > after b outer scope: {}", Rc::strong_count(&a));

	let c = Cons(4, Rc::clone(&a));
	println!(" > after c: {}", Rc::strong_count(&a));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_rc_smart_pointer() {
		rc_smart_pointer();
	}

	#[test]
	fn run_rc_cons() {
		rc_cons()
	}
}
