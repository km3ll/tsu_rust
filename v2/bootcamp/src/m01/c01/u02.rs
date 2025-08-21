pub fn hello_pod() {
	// pod: println! is a macro
	println!("Hello, pod!");
}

#[cfg(test)]
mod u02_tests {
	use super::*;
	#[test]
	fn hello_pod_runs() {
		hello_pod();
		assert!(true)
	}
}
