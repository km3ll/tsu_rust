fn packages_definition() {
	let n1 = r#"
	pod: Module System
	- Rust features that allow to manage code organization
	  - Package: feature that lets you build, test, and share crates
	  - Crate: a tree of modules that produces a library or executable
	  - Module: to control the organization, scope, and privacy of paths
	  - Path: a way of naming an item (struct, function, module)
	---"#;
	println!("{n1}");
}

fn packages_and_crates() {
	let n1 = r#"
	pod: Crate
	- The smallets amount of code that the compiler considers at a time
	- Can contain modules
	---
	pod: Binary Crate
	- Programs you can compile to an executable that you can run
	- Must have a main() function
	- Example: command-line program or server
	---
	pod: Library Crate
	- Functionality intended to be shared with multiple projects
	- Don't compile to an executable
	- Don't have a main() function
	- Example: the rand (random) crate
	---
	pod: Crate Root
	- Source file that the compiler starts from and makes up the root module
	---
	pod: Package
	- Bundle of one or more crates that provides functionality
	- Contains a Cargo.toml file describing how to build those crates
	- Cargo is actually a package containing the binary crate for the command line tool
	- Must contain at least one crate (binary or library)
	- Can contain many binary crates, but at most one library crate
	- Multiple binary crates are placed in the src/bin directory (binary crate per file)
	---"#;
	println!("{n1}");
}

fn packages_new_project() {
	let n1 = r#"
	pod: Cargo new
	- Cargo.toml file gives us a package
	- src/main.rs: crate root of a binary crate (same name as package)
	- src/lib.rs: crate root of a library crate (same name as package)
	- Both src/main.rs and src/lib.rs mean two crates (binary and library)
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_packages_definition() {
		packages_definition();
	}

	#[test]
	fn run_packages_and_crates() {
		packages_and_crates();
	}

	#[test]
	fn run_packages_new_project() {
		packages_new_project();
	}
}
