pub fn greet() {
	// pod: println! is a macro
	println!("Hello, pod!");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_greet() {
		greet();
	}
}
