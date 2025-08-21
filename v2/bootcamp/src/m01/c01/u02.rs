pub fn greet() {
	// pod: println! is a macro
	println!("Hello, pod!");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn greet_runs() {
		greet();
		assert!(true)
	}
}
