//! # Paths for Referring to an Item in the Module Tree

mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {
			println!("Paths: added to waitlist");
		}
	}
}

mod restaurant {
	use std::ops::ControlFlow::Break;

	#[derive(Debug)]
	pub enum Appetizer {
		Soup,
		Salad,
	}

	#[derive(Debug)]
	pub struct Breakfast {
		pub toast: String,
		seasonal_fruit: String,
	}

	impl Breakfast {
		pub fn summer(toast: &str) -> Breakfast {
			Breakfast {
				toast: String::from(toast),
				seasonal_fruit: String::from("peaches"),
			}
		}
	}

	pub mod front_of_house {
		pub fn deliver_order() {
			println!("Paths: order delivered");
		}
	}

	pub mod back_of_house {
		pub fn fix_incorrect_order() {
			cook_order();
			super::front_of_house::deliver_order();
		}
		fn cook_order() {}
	}
}

fn eat_at_restaurant() {
	// Absolute path
	// crate::front_of_house::hosting::add_to_waitlist();

	// Relative path
	// front_of_house::hosting::add_to_waitlist();
}

fn paths_definition() {
	let n1 = r#"
	pod: Paths
	- Both absolute and relative paths have identifiers separated by double colons (::)
	- Absolute path: the full path starting from a crate root
	- Relative path: starts from the current module and uses 'self', 'super' or an identifier
	- Using the 'crate' name is like using '/' to start from the filesystem root
	- Our preference is to specify absolute paths
	---"#;
	println!("{n1}");
}

fn paths_modules() {
	let n1 = r#"
	pod: Modules
	- Items in parent module can't use the private items inside child modules
	- Items in child modules can use the items in their ancertor modules
	- Child modules can see the context in which they're defined
	- Making a module public doesn't make its contents public ('pub')
	---"#;
	println!("{n1}");

	front_of_house::hosting::add_to_waitlist();
}

fn paths_library_and_binary() {
	let n1 = r#"
	pod: Best Practices with Binary and Library
	- Library crate
	  - Code that can be shared
	  - The module tree should be defined in src/lib.rs
	  - Any public items can be used in the binary crate
	- Binary crate
	  - Code to start and executable that calls code in the library crate
	  - Becomes a user of the library crate. It can only use the public API
	---"#;
	println!("{n1}");
}

fn paths_super() {
	let n1 = r#"
	pod: Relative Paths with 'super'
	- 'super' starts a relative path in the parent module
	- It is like starting a filesystem path with the '..' syntax
	---"#;
	println!("{n1}");

	restaurant::back_of_house::fix_incorrect_order()
}

fn paths_structs() {
	let n1 = r#"
	pod: Paths of Structs
	- We can make each field public or not on a case-by-case basis
	- Because it has private fields, the struct needs to provide a public associated constructor
	---"#;
	println!("{n1}");

	let meal = restaurant::Breakfast::summer("Rye");
	println!("Paths: structs meal: {meal:?}");
}

fn paths_enums() {
	let n1 = r#"
	pod: Paths of Enums
	- Making an enum public, all of its variants are then public
	---"#;
	println!("{n1}");

	let appetizer = restaurant::Appetizer::Salad;
	println!("Paths: enums appetizer: {appetizer:?}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_paths_definition() {
		paths_definition();
	}

	#[test]
	fn run_paths_modules() {
		paths_modules();
	}

	#[test]
	fn run_paths_library_and_binary() {
		paths_library_and_binary();
	}

	#[test]
	fn run_paths_super() {
		paths_super();
	}

	#[test]
	fn run_paths_structs() {
		paths_structs();
	}

	#[test]
	fn run_paths_enums() {
		paths_enums();
	}
}
