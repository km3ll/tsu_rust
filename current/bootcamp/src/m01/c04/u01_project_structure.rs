pub fn structure_package() {
	/**
	 * pod: package
	 * - Created when 'cargo new' is executed
	 * - A package contains one or more crates
	 * - Package rules
	 *   - At least 1 crate
	 *   - At most 1 library crate
	 *   - Any number of binary crates
	 */
	println!("package");
}

pub fn structure_crate() {
	/**
	 * pod: crate
	 * - A tree of modules that produces a library or binary
	 *   - Library: code that you can share with other crates
	 *   - Binary: code that you can run
	 */
	println!("crate");
}

pub fn structure_module() {
	/**
	 * pod: module
	 * - Control the organization, control and privacy
	 */
	println!("module");
}

pub fn structure_overview() {
	println!(" package (cargo new)");
	println!(" ├── library (crate)");
	println!(" │   └── root module (crate root)");
	println!(" │       └── sub-module");
	println!(" └── binary (crate)");
	println!("     └── root module (crate root)");
	println!("         └── sub-module");

	/**
	 * pod: Binary Crate
	 * - By default a new package comes with a binary crate that
	 *   starts at main.rs
	 *
	 * - Convention: if a main.rs file exists in the source directory,
	 *   then it will be the crate root of a binary crate with the same
	 *   name as the package.
	 */
	 
	/** 
	 * pod: Library Crate
	 * - Convention: if a bin.rs file exists in the source directory,
	 *   then it will be the crate root of a library crate.
	 *
	 * - The package can contain both, which is a common pattern for
	 *   CLI applications
	 *
	 * - For more than 1 binary crate we should create a subdirectory
	 *   within the src directory called 'bin'
	 */
	println!("");
	println!("root");
	println!("├── bin");
	println!("│   └── another_main.rs"); // binary
	println!("├── lib.rs"); // library
	println!("└── main.rs"); // binary
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
	fn run_structure_module() {
		structure_module();
	}

	#[test]
	fn run_structure_overview() {
		structure_overview();
	}
}
