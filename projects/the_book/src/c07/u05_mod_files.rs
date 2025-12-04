fn mod_files() {
	let n1 = r#"
	pod: Module Files
	- 'mod' is not an 'include' operation
	- 'mod' declares a module
	- Rust looks in a file with the same name as the module (src/front_of_house.rs)
	- We moved each module's code to a separate file
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_mod_files() {
		mod_files();
	}
}
