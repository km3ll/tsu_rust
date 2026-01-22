//! # Processing a Series of Items with Iterators

fn iterators_definition() {
	let n1 = r#"
	pod: Iterator
	- Pattern that allows to perform some task on a sequence of items in turn
	- In Rust, iterators are lazy
	---
	pod: Zero-Cost Abstractions
	- Iterators, although a high-level abstraction, get compiled down to roughly the same code as if you'd written the lower-level code yourself
	- "What you don't use, you don't pay for. And further: What you do use, you couldn't hand code any better" -Bjarne Stroustrup
	---"#;
	println!("{n1}");

	let v1 = vec![1, 2, 3];
	let iter1 = v1.iter();

	println!("Iterators: lazy");
	for val in iter1 {
		println!("> Got {val}");
	}
}

fn iterators_trait() {
	let n1 = r#"
	pod: Trait: Iterator
	- Requires implementors to define one method: next()
	- next() returns one item at a time, wrapped in Some and when iteration is over, returns None
	- Calling the 'next' method changes the internal state of the iterator
	- Code using 'next' consumes or uses up the iterator
	- In for-loops the loop takes ownership of the iterator and makes it mutable behind the scenes
	---"#;
	println!("{n1}");

	let v2 = vec![4, 5, 6];
	let mut iter2 = v2.iter();

	println!("Iterators: next()");
	println!("> {:?}", iter2.next());
	println!("> {:?}", iter2.next());
	println!("> {:?}", iter2.next());
	println!("> {:?}", iter2.next());
}

fn iterators_variants() {
	let n1 = r#"
	pod: Iterator Variants
	- iter(): iterator over immutable references
	- iter_mut(): iterator over mutable references
	- into_iter(): iterator that takes ownership and returns owned values
	---"#;
	println!("{n1}");
}

fn iterators_consumers() {
	let n1 = r#"
	pod: Iterator Consumers
	- Methods that call next() and use up the iterator, such as sum()
	---"#;
	println!("{n1}");

	let v3 = vec![7, 8, 9];
	let iter3 = v3.iter();
	let total: i32 = iter3.sum();

	println!("Iterators: sum: {total}");

	// Error: value used after being moved
	// println!("Iterators: iter3: {iter3:?}");
}

fn iterators_adapters() {
	let n1 = r#"
	pod: Iterator Adapter
	- Methods that produce different iterators by changing some aspects of the original iterator
	- map() takes a closure and returns a new lazy iterator
	- We can use collect() to collect the resultant values into a collection data type
	---"#;
	println!("{n1}");

	let v4 = vec![11, 12, 13];
	let iter4 = v4.iter().map(|x| x * 3);
	let v5: Vec<i32> = iter4.collect();

	println!("Iterators: collect v5: {v5:?}");
}

#[derive(Debug, PartialEq)]
struct Shoe {
	size: u32,
	style: String,
}

fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
	shoes
		.into_iter()
		.filter(|shoe| shoe.size == shoe_size)
		.collect()
}

fn iterators_closures() {
	let shoes = vec![
		Shoe {
			size: 10,
			style: String::from("sneaker"),
		},
		Shoe {
			size: 13,
			style: String::from("sandal"),
		},
		Shoe {
			size: 10,
			style: String::from("boot"),
		},
	];

	let v6 = shoe_in_size(shoes, 10);
	println!("Iterators: closures: v6: {v6:#?}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_iterators_definition() {
		iterators_definition();
	}

	#[test]
	fn run_iterators_trait() {
		iterators_trait();
	}

	#[test]
	fn run_iterators_variants() {
		iterators_variants();
	}

	#[test]
	fn run_iterators_consumers() {
		iterators_consumers();
	}

	#[test]
	fn run_iterators_adapters() {
		iterators_adapters();
	}

	#[test]
	fn run_iterators_closures() {
		iterators_closures();
	}
}
