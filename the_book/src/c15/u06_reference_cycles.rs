//! # Reference Cycles Can Leak Memory

use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

fn ref_cycles() {
	let n1 = r#"
	pod: Memory Leak
	- Memory that is never cleaned up
	- Items that refer to each other in a cycle. The reference count of each item never reaches Zero (0), and the values are never dropped
	---
	pod: Weak<T>
	- Strong references are how you can share ownership of an Rc<T> instance
	- Weak references don't express an ownership relationship, and their count doesn't affect when an Rc<T> instance is cleaned up
	- Any cycle involving some weak references will be broken once the strong reference count is Zero (0)
	- When you call Rc::downgrade you get a Weak<T>, which increases the weak_count
	- The Rc<T> uses weak_count to keep track of Weak<T> references
	- The upgrade method on a Weak<T> returns an Option<Rc<T>> to check whether the Rc<T> value has been dropped or not
	---
	pod: Tree Data Structure
	- A parent node should own its children
	- If a parent node is dropped, its child nodes should be dropped as well
	- A child node should refer to its parent, but not own it
	- If a child is dropped its parent node should still exist
	---"#;
	println!("{n1}");
}

#[derive(Debug)]
struct Node {
	value: i32,
	parent: RefCell<Weak<Node>>,
	children: RefCell<Vec<Rc<Node>>>,
}

fn weak_references() {
	println!("Weak References");

	let leaf = Rc::new(Node {
		value: 3,
		parent: RefCell::new(Weak::new()),
		children: RefCell::new(vec![]),
	});
	println!(" > leaf: {:?}", leaf);
	println!("   > parent: {:?}", leaf.parent.borrow().upgrade());
	println!("   > strong: {}", Rc::strong_count(&leaf));
	println!("   > weak: {}", Rc::weak_count(&leaf));

	let branch = Rc::new(Node {
		value: 5,
		parent: RefCell::new(Weak::new()),
		children: RefCell::new(vec![Rc::clone(&leaf)]),
	});
	println!(" > branch: {:?}", branch);
	println!("   > parent: {:?}", branch.parent.borrow().upgrade());
	println!("   > strong: {}", Rc::strong_count(&branch));
	println!("   > weak: {}", Rc::weak_count(&branch));

	println!("Weak References: branch linked to leaf");
	*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

	println!(" > leaf");
	println!("   > strong: {}", Rc::strong_count(&leaf));
	println!("   > weak: {}", Rc::weak_count(&leaf));

	println!(" > branch");
	println!("   > strong: {}", Rc::strong_count(&branch));
	println!("   > weak: {}", Rc::weak_count(&branch));
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_ref_cycles() {
		ref_cycles();
	}

	#[test]
	fn run_weak_references() {
		weak_references()
	}
}
