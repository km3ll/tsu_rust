mod front_of_house {
	mod hosting {
		fn add_to_waitlist() {
			println!("Paths: added to waitlist");
		}
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
	- Our preference is to specify absilute paths
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
}

fn path_library_and_binary() {
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
	fn run_path_library_and_binary() {
		path_library_and_binary();
	}
}
