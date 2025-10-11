fn basic_error_handling_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		basic_error_handling_();
	}
}
