pub fn option_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		option_();
		assert!(true)
	}
}
