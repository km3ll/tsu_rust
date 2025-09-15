pub fn cargo_features() {
	println!("Base");
}

pub fn cargo_() {
	println!("Base");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn run_() {
		cargo_();
		assert!(true)
	}
}
