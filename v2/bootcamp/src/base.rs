pub fn base_() {
	println!("Base")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn _runs() {
		base_();
		assert!(true)
	}
}
