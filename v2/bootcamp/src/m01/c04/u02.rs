pub fn modules_module() {
	/**
	 * pod: Modules
	 * - Contain items: functions, structs, enums, traits, etc
	 * - Explicitly defined using the 'mod' keyword
	 * - Not mapped to the file system
	 * - A single file could have multiple modules
	 * - Allow conditional compilation
	 */
	println!("module");
}

pub fn modules_sub_module() {
	/**
	 * pod: Sub-modules
	 * - Must be declared within the parent module
	 */
	println!("sub-module");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_modules_module() {
		modules_module();
		assert!(true)
	}

	#[test]
	fn run_modules_sub_module() {
		modules_sub_module();
		assert!(true)
	}
}
