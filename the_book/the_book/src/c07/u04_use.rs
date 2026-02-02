//! # Bringing Paths Into Scope with the use keyword

mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist(name: &str) {
			println!("Use: {name} added to waitlist");
		}
	}
}

fn use_definition() {
	let n1 = r#"
	pod: Keyword 'use'
	- Creates a shortcut to a path
	- Paths brought into scope with 'use' also check privacy
	---"#;
	println!("{n1}");

	use crate::c07::u04_use::front_of_house::hosting::add_to_waitlist;
	add_to_waitlist("Ferris")
}

fn use_idiomatic() {
	let n1 = r#"
	pod: Idiomatic 'use' Paths
	- Specifying the parent when calling a function makes it clear that the function is not local
	- When bringing in structs, enums, and other items, it's idiomatic to specify the full path
	---"#;
	println!("{n1}");

	use crate::c07::u04_use::front_of_house::hosting;
	hosting::add_to_waitlist("Ferris");

	use std::collections::HashMap;
	let mut map: HashMap<u8, u8> = HashMap::new();
	map.insert(1, 2);
	println!("Use: hashmap: {map:?}");
}

fn use_new_names() {
	let n1 = r#"
	pod: Keyword 'as'
	- Sets a new local name, or alias
	- Brings two types of the same name into the same scope
	---"#;
	println!("{n1}");

	use std::fmt::Result;
	use std::io::Result as IoResult;
	println!("Use: alias: IoResult");
}

fn use_re_exporting() {
	let n1 = r#"
	pod: Re-Exporting Names with 'pub use'
	- When the internal structure of your code differs from how callers would think about the domain
	- We can write our code with one structure but expose a different structure
	---"#;
	println!("{n1}");
}

fn use_nested_paths() {
	let n1 = r#"
	pod: Nested Paths with 'self'
	- Specifying the common part of the path first
	- The common first path can be merged with 'self'
	---"#;
	println!("{n1}");

	// use std::{ cmp::Ordering, io };
	// use std::io::{self, Write};
}

fn use_glob_operator() {
	let n1 = r#"
	pod: The Glob Operator '*'
	- To bring all public items defined in a path into scope
	- Often used when testing to bring everything under test into the tests module
	---"#;
	println!("{n1}");

	println!("Use: glob operator: use::std::collections::*;");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_use_definition() {
		use_definition();
	}

	#[test]
	fn run_use_idiomatic() {
		use_idiomatic();
	}

	#[test]
	fn run_use_new_names() {
		use_new_names();
	}

	#[test]
	fn run_use_re_exporting() {
		use_re_exporting();
	}

	#[test]
	fn run_use_nested_paths() {
		use_nested_paths();
	}

	#[test]
	fn run_use_glob_operator() {
		use_glob_operator();
	}
}
