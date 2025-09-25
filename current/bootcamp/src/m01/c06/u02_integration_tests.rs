pub fn integration_tests() {
	println!("project: p08_tests");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_integration_tests() {
		integration_tests();
	}
}
