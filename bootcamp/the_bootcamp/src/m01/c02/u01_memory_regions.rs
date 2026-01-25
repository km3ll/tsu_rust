pub fn memory_stack() {
	let n1 = r#"
	pod: Stack
	- Contents
	  - Function arguments
	  - Local variables
	  - Known size at compile time
	- Size
	  - Dynamic
	  - Fixed upper limit
	- Lifecycle
	  - Lifetime of a function
	- Cleanup
	  - Automatic when function returns
	---"#;
	println!("{n1}");
}

pub fn memory_heap() {
	let n1 = r#"
	pod: Heap
	- Contents
	  - Values that live beyond a function's lifetime
	  - Values accessed by multiple threads
	  - Large values
	  - Unknown size at compile time
	- Size
	  - Dynamic
	- Lifetime
	  - Determined by programmer
	- Cleanup
	  - Manual
	---"#;
	println!("{n1}");
}

pub fn memory_static() {
	let n1 = r#"
	pod: Static
	- Contents
	  - Program's binary
	  - Static variables
	  - String literals
	- Size
	  - Fixed size
	- Lifetime
	  - Lifetime of programm
	- Cleanup
	  - Automatic when program terminates
	---"#;
	println!("{n1}");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_memory_stack() {
		memory_stack();
	}

	#[test]
	fn run_memory_heap() {
		memory_heap();
	}

	#[test]
	fn run_memory_static() {
		memory_static();
	}
}
