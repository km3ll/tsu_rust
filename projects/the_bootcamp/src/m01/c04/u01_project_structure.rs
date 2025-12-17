pub fn structure_package() {
	let n1 = r#"
	pod: Package
	- Created when 'cargo new' is executed
	- A package contains one or more crates
	- Package rules
	  - At least 1 crate
	  - At most 1 library crate
	  - Any number of binary crates
	---"#;
	println!("{n1}");
}

pub fn structure_crate() {
	let n1 = r#"
	pod: Crate
	- A tree of modules that produces a library or binary
	- Library: code that you can share with other crates
	- Binary: code that you can run
	---"#;
	println!("{n1}");
}

pub fn structure_overview() {
	let n1 = r#"
	pod: Project Tree
	package (cargo new)
	├── library (crate)
	│   └── root module (crate root)
	│       └── sub-module
	└── binary (crate)
		└── root module (crate root)
			└── sub-module
	---"#;
	println!("{n1}");

	let n1 = r#"
	pod: Binary Crate
	- By default a new package comes with a binary crate that starts at main.rs
	- If a main.rs file exists in the source directory, then it will be the crate root of a binary crate with the same name as the package
	---"#;
	println!("{n1}");

	let n1 = r#"
	pod: Library Crate
	- If a bin.rs file exists in the source directory, then it will be the crate root of a library crate
	- The package can contain both, which is a common pattern for CLI applications
	- For more than 1 binary crate we should create a subdirectory within the src directory called 'bin'
	---"#;
	println!("{n1}");

	let n1 = r#"
	pod: Project Tree
	root
	├── bin
	│   └── another_main.rs // binary
	├── lib.rs // library
	└── main.rs // binary
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_structure_package() {
		structure_package();
	}

	#[test]
	fn run_structure_crate() {
		structure_crate();
	}

	#[test]
	fn run_structure_overview() {
		structure_overview();
	}
}
