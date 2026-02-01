//! # RefCell<T> and the Interior Mutability Pattern

use crate::c15::u05_refcell_smart_pointer::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn refcell() {
	let n1 = r#"
	pod: Interior Mutability
	- Design pattern that allows to mutate data even when there are mutable references to that data
	- With `Box<T>` the borrowing rules are enforced at runtime. With `RefCell<T>` these invariants are enforced at runtime
	- With RefCell if you break the borrowing rules, your program will panic and exit
	---
	pod: Box<T>, Rc<T> or RefCell<T>
	- Rc<T> enables multiple owners of the same data
	- Box<T> and RefCell<T> have single owners
	- Box<T> allows immutable or mutable borrows checked at compile time
	- Rc<T> allows only immutable borrows checked at compile time
	- RefCell<T> allows immutable or mutable borrows checked at runtime
	- You can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable
	---"#;
	println!("{n1}");
}

pub trait Messenger {
	fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
	messenger: &'a T,
	value: usize,
	max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
	T: Messenger,
{
	pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
		LimitTracker {
			messenger,
			value: 0,
			max,
		}
	}

	pub fn set_value(&mut self, value: usize) {
		self.value = value;

		let percentage_of_max = self.value as f64 / self.max as f64;

		if percentage_of_max >= 1.0 {
			self.messenger.send("Error: you are over your quota!")
		} else if percentage_of_max >= 0.9 {
			self.messenger
				.send("Urgent Warning: you've used up over 90% of your quota!");
		} else if percentage_of_max >= 0.75 {
			self.messenger
				.send("Warning: you've used up over 75% of your quota!");
		}
	}
}

struct MockMessenger {
	sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
	fn new() -> MockMessenger {
		MockMessenger {
			sent_messages: RefCell::new(vec![]),
		}
	}
}

impl Messenger for MockMessenger {
	fn send(&self, msg: &str) {
		self.sent_messages.borrow_mut().push(String::from(msg));
	}
}

fn refcell_mocks() {
	let n1 = r#"
	pod: Mock Objects
	- Specific types that record what happens during a test so that you can assert that the correct action took place
	- For example, to keep track of a user's quota for the number of API calls they're allowed to make
	---
	pod: RefCell<T>
	- Keeps track of how many Ref<T> (`borrow()`) and RefMut<T> (`borrow_mut()`) smart pointers are currently active
	---"#;
	println!("{n1}");
}

#[derive(Debug)]
enum List {
	Cons(Rc<RefCell<i32>>, Rc<List>),
	Nil,
}

fn refcell_lists() {
	let value = Rc::new(RefCell::new(5));

	let list_a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
	let list_b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&list_a));
	let list_c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&list_a));

	println!("RefCell: lists before");
	println!(" > a: {:?}", list_a);
	println!(" > b: {:?}", list_b);
	println!(" > c: {:?}", list_c);

	*value.borrow_mut() += 10;
	println!("RefCell: lists after");
	println!(" > a: {:?}", list_a);
	println!(" > b: {:?}", list_b);
	println!(" > c: {:?}", list_c);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_it_sends_an_over_75_percent_warning_message() {
		let mock_messenger = MockMessenger::new();
		let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
		limit_tracker.set_value(80);

		assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
	}

	#[test]
	fn run_refcell() {
		refcell();
	}

	#[test]
	fn run_refcell_mocks() {
		refcell_mocks()
	}

	#[test]
	fn run_refcell_lists() {
		refcell_lists()
	}
}
