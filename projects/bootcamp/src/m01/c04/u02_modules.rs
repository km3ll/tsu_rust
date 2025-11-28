pub fn modules() {
	let n1 = r#"
	pod: Modules
	- Contain items: functions, structs, enums, traits, etc.
	- Explicitly defined using the 'mod' keyword
	- Not mapped to the file system
	- A single file could have multiple modules
	- Allow conditional compilation
	---
	pod: Sub-Modules
	- Must be declared within the parent module
	- Re-exported by adding 'pub' before 'use'
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_modules() {
		modules();
	}
}
