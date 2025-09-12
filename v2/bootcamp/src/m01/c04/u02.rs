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
	 * - Must be declared within the parent module regardless of if
	 *   their content is defined inline or in a different file
	 */
	println!("sub-module");
}

pub fn modules_re_exporting() {
	/**
	 * pod: Re-Exporting by adding 'pub' before 'use'
	 */
	println!("re-exporting");
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

	#[test]
	fn run_modules_re_exporting() {
		modules_re_exporting();
		assert!(true)
	}
}
