pub fn greet() {
	// pod: println! is a macro
	println!("Hello, pod!");
}

#[cfg(test)]
mod u02_tests {
	use super::*;

	#[test]
	fn greet_runs() {
		greet();
		assert!(true)
	}

}
