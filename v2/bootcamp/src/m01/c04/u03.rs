pub fn modules_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		modules_();
		assert!(true)
	}
}
