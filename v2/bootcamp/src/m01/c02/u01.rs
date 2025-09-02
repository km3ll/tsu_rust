pub fn memory_stack() {
	/**
	 * pod: Stack
	 * - Contents:
	 *   - Function arguments
	 *   - Local variables
	 *   - Known size at compile time
	 * - Size: dynamic / fixed upper limit
	 * - Lifetime: lifetime of function
	 * - Cleanup: automatic when function returns
	 */
	println!("Stack")
}

pub fn memory_heap() {
	/**
	 * pod: Heap
	 * - Contents:
	 *   - Values that live beyond a function's lifetime
	 *   - Values accessed by multiple threads
	 *   - Large values
	 *   - Unknown size at compile time
	 * - Size: dynamic
	 * - Lifetime: determined by programmer
	 * - Cleanup: manual
	 */
	println!("Heap")
}

pub fn memory_static() {
	/**
	 * pod: Static Memory
	 * - Contents:
	 *   - Program's binary
	 *   - Static variables
	 *   - String literals
	 * - Size: fixed size
	 * - Lifetime: lifetime of program
	 * - Cleanup: automatic when program terminates
	 */
	println!("Static")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn memory_stack_runs() {
		memory_stack();
		assert!(true)
	}

	#[test]
	fn memory_heap_runs() {
		memory_heap();
		assert!(true)
	}

	#[test]
	fn memory_static_runs() {
		memory_static();
		assert!(true)
	}
}
