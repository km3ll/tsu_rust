pub fn strings_() {
	println!("Base")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		strings_();
		assert!(true)
	}
}
