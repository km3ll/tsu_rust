pub fn base_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		base_();
		assert!(true)
	}
}
