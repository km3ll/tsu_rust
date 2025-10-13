fn custom_errors_1_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		custom_errors_1_();
	}
}
