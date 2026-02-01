//! # RefCell<T> and the Interior Mutability Pattern

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
		} else if percentage_of_max >= 9.0 {
			self.messenger
				.send("Urgent Warning: you've used up over 90% of your quota!");
		} else if percentage_of_max >= 7.5 {
			self.messenger
				.send("Warning: you've used up over 75% of your quota!");
		}
	}
}

fn refcell_mocks() {
	let n1 = r#"
	pod: Mock Objects
	- Specific types that record what happens during a test so that you can assert that the correct action took place
	- For example, to keep track of a user's quota for the number of API calls they're allowed to make
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_refcell() {
		refcell();
	}
}
